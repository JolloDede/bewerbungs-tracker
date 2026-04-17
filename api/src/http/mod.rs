use askama::Template;
use axum::{extract::State, http::StatusCode, response::Html};
use sea_orm::{FromQueryResult, Statement};
use sqlx::types::chrono::{NaiveDate, Utc};

mod firma;
pub use firma::*;
mod contact;
pub use contact::*;
use uuid::Uuid;

use crate::AppState;

#[derive(FromQueryResult)]
struct DisplayContact {
    firma_id: Uuid,
    firma: String,
    date: NaiveDate,
    r#type: String,
    age: u32,
}

pub async fn index(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let mut d_contacts = DisplayContact::find_by_statement(
        Statement::from_string(sea_orm::DatabaseBackend::Sqlite,
            r#"
           	SELECT f.id as firma_id, f.name as firma, c.date, c.type, 0 as age FROM firma f LEFT JOIN (
                SELECT fk_firma, MAX(date) as max_date FROM contact GROUP BY fk_firma
            ) latest_contact ON f.id = latest_contact.fk_firma
            INNER JOIN contact c ON c.fk_firma = latest_contact.fk_firma AND c.date = latest_contact.max_date
            WHERE c.Type != 'absage'
            ORDER BY c.date ASC;
        "#,
        )
    ).all(&state.db)
    .await
    .map_err(|err| {dbg!(err); (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load contacts")})?;

    let today = Utc::now().date_naive();
    for contact in d_contacts.iter_mut() {
        let age = contact.date.signed_duration_since(today).num_days().abs() as u32;
        contact.age = age;
    }

    let index = IndexTemplate {
        contacts: d_contacts,
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
}
