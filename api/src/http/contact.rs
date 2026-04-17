use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
};
use chrono::Utc;
use entity::contact::Model as ContactModel;
use entity::firma::Model as FirmaModel;
use sea_orm::{ActiveValue::Set, EntityTrait, FromQueryResult};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

use crate::AppState;

pub async fn get_contact_form(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let firmas = entity::prelude::Firma::find()
        .all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load firmas!"))?;
    let mut types = Vec::new();
    for typ in ContactType::iter() {
        types.push(ContactKV {
            key: typ.as_ref().to_string(),
            value: typ.as_ref().to_string(),
        });
    }

    let contact_template = ContactFormTemplate {
        firmas: firmas,
        types: types,
    };

    let res = contact_template
        .render()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(res))
}

#[derive(Template)]
#[template(path = "contact_form.html")]
struct ContactFormTemplate {
    firmas: Vec<FirmaModel>,
    types: Vec<ContactKV>,
}

#[derive(EnumIter, EnumString, AsRefStr)]
pub enum ContactType {
    Erfasst,
    Bewerbung,
    Nachfrage,
    Vorstellungsgespräch,
    Absage,
}

struct ContactKV {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostContact {
    firma: String,
    typ: String,
}

pub async fn post_contact(
    state: State<AppState>,
    form: Form<PostContact>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let form = form.0;
    let uid = uuid::Uuid::now_v7();
    let created_at = Utc::now().date_naive();
    let firma = uuid::Uuid::parse_str(form.firma.as_str()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to parse firma uid!",
        )
    })?;

    let contact = entity::contact::ActiveModel {
        id: Set(uid),
        date: Set(created_at),
        r#type: Set(form.typ),
        fk_firma: Set(firma),
        ..Default::default()
    };

    let res = entity::contact::Entity::insert(contact)
        .exec(&state.db)
        .await;

    match res {
        Ok(_res) => Ok(Redirect::to("/")),
        Err(err) => {
            dbg!(err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, ""))
        }
    }
}

pub async fn get_contact_list(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let contacts = entity::contact::Entity::find()
        .find_also_related(entity::firma::Entity)
        .all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load firmas!"))?;

    let mut disp_contact = Vec::new();
    for contact in contacts {
        disp_contact.push(DisplayListContact {
            id: contact.0.id.to_string(),
            firma: contact.1.unwrap().name,
            date: contact.0.date.to_string(),
            status: contact.0.r#type,
        });
    }

    let firma_temp = ContactListTemplate {
        contacts: disp_contact,
    };
    let res = firma_temp
        .render()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(res))
}

#[derive(Template)]
#[template(path = "contact_list.html")]
struct ContactListTemplate {
    contacts: Vec<DisplayListContact>,
}

#[derive(FromQueryResult)]
struct DisplayListContact {
    id: String,
    firma: String,
    date: String,
    status: String,
}
