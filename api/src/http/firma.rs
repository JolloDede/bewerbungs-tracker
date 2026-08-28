use std::fmt::Debug;

use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
};
use chrono::Utc;
use entity::firma::Model as FirmaModel;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::{AppState, http::ContactType};

pub async fn get_firma_form() -> Result<Html<String>, (StatusCode, &'static str)> {
    let firma_temp = FirmaFormTemplate { firma: None };

    let res = firma_temp
        .render()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(res))
}

#[derive(Template)]
#[template(path = "firma_form.html")]
struct FirmaFormTemplate {
    firma: Option<FirmaModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostFirma {
    name: String,
    plzort: String,
    stellenbezeichnung: String,
    text: String,
    urls: String,
}

pub async fn post_firma(
    state: State<AppState>,
    form: Form<PostFirma>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let form = form.0;
    let uid = uuid::Uuid::now_v7();
    let created_at = Utc::now().naive_local();

    let firma = entity::firma::ActiveModel {
        id: Set(uid),
        name: Set(form.name),
        plzort: Set(form.plzort),
        stellenbezeichung: Set(form.stellenbezeichnung),
        text: Set(form.text),
        urls: Set(form.urls),
        create_at: Set(created_at),
    };

    let res = entity::firma::Entity::insert(firma).exec(&state.db).await;

    match res {
        Ok(_res) => {
            let contact = entity::contact::ActiveModel {
                id: Set(uuid::Uuid::now_v7()),
                date: Set(Utc::now().date_naive()),
                r#type: Set(ContactType::Erfasst.as_ref().to_string()),
                fk_firma: Set(uid),
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
        Err(err) => {
            dbg!(err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, ""))
        }
    }
}

pub async fn get_firma_list(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let firmas = entity::firma::Entity::find()
        .all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load firmas!"))?;

    let firma_temp = FirmaListTemplate { firmas: firmas };
    let res = firma_temp
        .render()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(res))
}

#[derive(Template)]
#[template(path = "firma_list.html")]
struct FirmaListTemplate {
    firmas: Vec<FirmaModel>,
}
