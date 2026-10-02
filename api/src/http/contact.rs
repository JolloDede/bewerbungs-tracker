use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
};
use chrono::Utc;
use entity::firma::Model as FirmaModel;
use rip_templating::html;
use sea_orm::{ActiveValue::Set, EntityTrait, FromQueryResult, QueryOrder};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};
use uuid::Uuid;

use crate::{AppState, http::page_template};

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

    Ok(page_template(
        "Kontakt formular",
        contact_form(firmas, types).0,
    ))
}

fn contact_form(firmas: Vec<FirmaModel>, types: Vec<ContactKV>) -> Html<String> {
    html! {
        form {
            action: "",
            method: "POST",
            label {
                span { "Firma:" }
            }
            select {
                name: "firma",
                id: "firma",
                for firma in firmas {
                    option {
                        value: {firma.id},
                        {firma.name}
                    }
                }
            }
        }
        label {
           span { "Typ:" }
           select {
               name: "typ",
               id: "typ",
               for c_type in types {
                   option {
                       value: c_type.key,
                       {c_type.value}
                   }
               }
           }
        }
        input {
            r#type: "submit",
            value: "Speichern",
        }
    }
}

#[derive(EnumIter, EnumString, AsRefStr)]
pub enum ContactType {
    Erfasst,
    Bewerbung,
    Nachfrage,
    Vorstellungsgespräch,
    Absage,
}

pub struct ContactKV {
    pub key: String,
    pub value: String,
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
    let created_at = Utc::now().naive_utc();
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
        .order_by_desc(entity::contact::Column::Date)
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

    Ok(page_template("Kontaktliste", contact_list(disp_contact).0))
}

fn contact_list(contacts: Vec<DisplayListContact>) -> Html<String> {
    html! {
       div {
           for contact in contacts {
               article {
                   class: "firma",
                   div {
                       h2 { {contact.firma} }
                       p { {contact.date} }
                       h3 {
                           style: "margin-top: 0;",
                           {contact.status}
                       }
                   }
                   div {
                       button {
                           class: "contrast",
                           hx_delete: {format!("contact/{}", contact.id)},
                           hx_trigger: "click",
                           hx_swap: "delete",
                           hx_target: "closest article",
                           "Delete"
                       }
                   }
               }
           }
       }
    }
}

#[derive(FromQueryResult)]
struct DisplayListContact {
    id: String,
    firma: String,
    date: String,
    status: String,
}

pub async fn delete_contact(Path(id): Path<String>, state: State<AppState>) -> StatusCode {
    let Ok(uuid) = Uuid::parse_str(id.as_str()) else {
        return StatusCode::NOT_FOUND;
    };
    let Ok(_) = entity::contact::Entity::delete_by_id(uuid)
        .exec(&state.db)
        .await
    else {
        return StatusCode::NOT_FOUND;
    };

    StatusCode::OK
}
