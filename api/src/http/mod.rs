use askama::Template;
use axum::{extract::State, http::StatusCode, response::Html};
use sea_orm::{FromQueryResult, Statement};
use sqlx::types::chrono::{NaiveDateTime, Utc};

mod firma;
pub use firma::*;
mod contact;
pub use contact::*;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::AppState;

#[derive(FromQueryResult)]
struct DisplayContact {
    firma_id: Uuid,
    firma: String,
    date: NaiveDateTime,
    r#type: String,
    age: u32,
}

pub async fn index(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let mut d_contacts = DisplayContact::find_by_statement(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"
            WITH RankedContacts AS (
                SELECT
                    f.id AS firma_id,
                    f.name AS firma,
                    c.date,
                    c.type,
                    ROW_NUMBER() OVER (PARTITION BY f.id ORDER BY c.date DESC) AS rn
                FROM firma f
                LEFT JOIN contact c ON f.id = c.fk_firma
            )
            SELECT firma_id, firma, date, type, 0 AS age
            FROM RankedContacts
            WHERE rn = 1
              AND (type != 'absage' OR type IS NULL)
            ORDER BY date ASC;
        "#,
    ))
    .all(&state.db)
    .await
    .map_err(|err| {
        dbg!(err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load contacts")
    })?;

    let today = Utc::now().date_naive();
    for contact in d_contacts.iter_mut() {
        let age = contact
            .date
            .date()
            .signed_duration_since(today)
            .num_days()
            .unsigned_abs() as u32;
        contact.age = age;
    }

    let mut types = Vec::new();
    for typ in ContactType::iter() {
        types.push(ContactKV {
            key: typ.as_ref().to_string(),
            value: typ.as_ref().to_string(),
        });
    }

    let firma_count = d_contacts.iter().count();
    let index = IndexTemplate {
        contacts: d_contacts,
        types: types,
        firma_count,
    };
    let res = index
        .render()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    return Ok(Html(res));
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    contacts: Vec<DisplayContact>,
    types: Vec<ContactKV>,
    firma_count: usize,
}
