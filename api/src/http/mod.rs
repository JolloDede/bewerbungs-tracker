use axum::{extract::State, http::StatusCode, response::Html};
use rip_templating::{Component, DOCTYPE, html};
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

    Ok(page_template("Index", index_page(d_contacts).0))
}

pub fn page_template(title: &str, children: String) -> Html<String> {
    html! {
        {DOCTYPE}
        html {
            lang: "de",
            head {
                meta { charset: "UTF-8" }
                meta { name: "viewport" content: "width=device-width, initial-scale=1.0" }
                meta { name: "color-scheme" content: "light dark" }
                link { rel: "stylesheet" href: "/assets/css/style.css" }
                title { {title} }
            }
            body {
                header {
                    class: "container",
                    nav {
                        ul {
                            li {
                                a {
                                    href: "/",
                                    b { "Home" }
                                }
                            }
                            li {
                                a {
                                    href: "/firmas",
                                    b { "Firmen" }
                                }
                            }
                            li {
                                a {
                                    href: "/contacts",
                                    b { "Kontakte" }
                                }
                            }
                        }
                        ul {
                            li {
                                button {
                                    onclick: "window.localtion.push(\"/contact/add\"",
                                    "Kontakt hinzufügen"
                                }
                            }
                            li {
                                button {
                                    onclick: "window.localtion.push(\"/firma/add\"",
                                    "Firma hinzufügen"
                                }
                            }
                        }
                    }
                }
                main {
                    {children}
                }
                script {
                    src: "/assets/js/htmx-2.0.8.min.js"
                }
            }
        }
    }
}

fn index_page(contacts: Vec<DisplayContact>) -> Html<String> {
    let firma_count = contacts.iter().count();
    let types: Vec<ContactKV> = ContactType::iter()
        .map(|c_type| ContactKV {
            key: c_type.as_ref().to_string(),
            value: c_type.as_ref().to_string(),
        })
        .collect();

    html! {
        h1 {
            { format!("Todos {}", firma_count) }
        }
        table {
            thead {
               tr {
                   th { scope: "col", "Firma" }
                   th { scope: "col", "Datum" }
                   th { scope: "col", "Status" }
               }
            }
            tbody {
                for contact in contacts {
                    tr {
                        th {
                            scope: "row",
                            a {
                                href: { format!("/firma/{}", contact.firma_id) },
                                {contact.firma_id}
                            }
                        }
                        td {
                            style: {
                                match contact.age {
                                    7..14 => "background-color: yellow;",
                                    n if n > 14 => "background-color: red;",
                                    _ => "",
                                }
                            }
                            {contact.date}
                        }
                        td {
                            input {
                                r#type: "text",
                                name: "firma",
                                id: "firma",
                                value: {contact.firma_id},
                                style: "display: none;",
                            }
                            select {
                                name: "typ",
                                id: "typ",
                                hx_post: "/contact/add",
                                hx_include: "previous #firma",
                                hx_on_htmx_after_request: "window.location.href = window.location.pathname + window.location.search + (window.location.search ? '&' : '?') + 't=' + Date.now();",
                                for c_type in &types {
                                    option {
                                        value: { c_type.key },
                                        selected: c_type.key == contact.r#type,
                                        {c_type.value}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
