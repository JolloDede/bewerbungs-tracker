use std::fmt::Debug;

use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
};
use chrono::Utc;
use entity::firma::Model as FirmaModel;
use rip_templating::html;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    http::{ContactType, index_page, page_template},
};

pub async fn get_firma_form() -> Result<Html<String>, (StatusCode, &'static str)> {
    Ok(page_template("Firma formular", firma_form(None).0))
}

fn firma_form(firma: Option<FirmaModel>) -> Html<String> {
    let (firma_name, stellen_bezeichnung, plzort) = match &firma {
        Some(firma) => (
            firma.name.clone(),
            firma.stellenbezeichnung.clone(),
            firma.plzort.clone(),
        ),
        None => ("".to_string(), "".to_string(), "".to_string()),
    };

    return html! {
       form {
           method: if firma.is_some() { "PUT" } else { "POST" },
           h2 {
               if let Some(firma) = &firma {
                   {format!("Firma {} verändern", firma.name)}
               } else {
                    "Neue Firma erfassen"
               }
           }
           label {
               span { "Firmenname:" }
               input {
                   autofocus: true,
                   r#type: "text",
                   name: "name",
                   id: "name",
                   value: &firma_name,
               }
           }
           label {
               span { "Stellenbezeichnung:" }
               input {
                   r#type: "text",
                   name: "stellenbezeichnung",
                   id: "stellenbezeichnung",
                   value: &stellen_bezeichnung,
               }
           }
           label {
               span { "PLZ Ort:" }
               input {
                   r#type: "text",
                   name: "plzort",
                   id: "plzort",
                   value: &plzort,
               }
           }
           label {
               span { "Urls:" }
               textarea {
                   name: "urls",
                   id: "urls",
                   if let Some(firma) = &firma {
                       {firma.urls}
                   }
               }
           }
           label {
               span { "Text:" }
               textarea {
                   name: "text",
                   id: "text",
                   if let Some(firma) = &firma {
                       {firma.text}
                   }
               }
           }
           button {
               r#type: "submit",
               "Speichern"
           }
       }
    };
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
        stellenbezeichnung: Set(form.stellenbezeichnung),
        text: Set(form.text),
        urls: Set(form.urls),
        create_at: Set(created_at),
    };

    let res = entity::firma::Entity::insert(firma).exec(&state.db).await;

    match res {
        Ok(_res) => {
            let contact = entity::contact::ActiveModel {
                id: Set(uuid::Uuid::now_v7()),
                date: Set(Utc::now().naive_utc()),
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

    Ok(page_template("Firma list", firma_list(firmas).0))
}

fn firma_list(firmas: Vec<FirmaModel>) -> Html<String> {
    html! {
        div {
            for firma in firmas {
               article {
                   class: "firma",
                   div {
                       style: "flex-grow: 1;",
                       h3 { {firma.name} }
                       p {
                           role: "button",
                           class: "outline",
                           {firma.text}
                       }
                   }
                   div {
                       button {
                           onclick: "window.location.href = '/firma/{{ firma.id }}'",
                           "Edit"
                       }
                   }
               }
            }
        }
    }
}

pub async fn get_firma(
    Path(id): Path<String>,
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let Ok(uuid) = Uuid::parse_str(id.as_str()) else {
        return Err((StatusCode::NOT_FOUND, "Havent found firma with this id"));
    };
    let Ok(Some(res)) = entity::firma::Entity::find_by_id(uuid).one(&state.db).await else {
        return Err((StatusCode::NOT_FOUND, "Havent found firma with this id"));
    };

    Ok(page_template("Firma", firma_form(Some(res)).0))
}
