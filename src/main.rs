use kholles_server::error::{CustomError, ErrorType};

use kholles_server::list_fs::{get_proof_list, get_week_list};
use kholles_server::types::*;
use kholles_server::webhook;

use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};
use std::collections::HashSet;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index_endpoint() -> Result<Template, CustomError> {
    Ok(Template::render("index", context! {}))
}

#[get("/proof/list")]
fn proof_list_endpoint() -> Result<Template, CustomError> {
    let files = get_proof_list()?;

    let mut proofs = files
        .into_values()
        .map(|e| e.clone())
        .collect::<Vec<Proof>>();
    proofs.sort_unstable();

    Ok(Template::render(
        "proof-list",
        context! {
            proofs,
        },
    ))
}

#[get("/proof/<pid>")]
fn proof_view_endpoint(pid: <Proof as ProofTrait>::ProofIdType) -> Result<Template, CustomError> {
    let proofs = get_proof_list()?;

    if let Some(p) = proofs.get(&pid) {
        Ok(Template::render(
            "proof-view",
            context! {
                html: p.get_html(),
                proof: p,
            },
        ))
    } else {
        Err(CustomError::new(
            ErrorType::ServerError,
            "Proof not found".to_string(),
        ))
    }
}

#[get("/week/list")]
fn week_list_endpoint() -> Result<Template, CustomError> {
    let weeks = get_week_list()?;

    let mut week_list = weeks.values().map(|e| e.clone()).collect::<Vec<Week>>();
    week_list.sort_by_key(|w| w.number);

    Ok(Template::render(
        "week-list",
        context! {
            weeks: week_list,
        },
    ))
}

#[get("/week/newest")]
fn week_newest_endpoint() -> Result<Template, CustomError> {
    let weeks = get_week_list()?;

    let week_list = weeks.values().map(|e| e.clone()).collect::<Vec<Week>>();
    let newest = week_list
        .iter()
        .max_by_key(|w| w.number)
        .ok_or(CustomError::new(
            ErrorType::ServerError,
            "Could’nt find proof".to_string(),
        ))?;

    let proofs = get_proof_list()?;
    let mut authors = HashSet::new();
    let week_proofs: Vec<Option<Proof>> = newest
        .proofs
        .iter()
        .map(|pid| match proofs.get(pid) {
            Some(p) => {
                for author in p.authors.clone() {
                    authors.insert(author);
                }

                return Some(p.as_html_proof());
            }
            None => None,
        })
        .collect();

    let mut sorted_authors: Vec<String> = authors.into_iter().collect();
    sorted_authors.sort_unstable();

    Ok(Template::render(
        "week-view",
        context! {
            week: newest,
            proofs: week_proofs,
            authors: sorted_authors,
        },
    ))
}

#[get("/week/<number>")]
fn week_view_endpoint(
    number: <Week as WeekTrait>::WeekNumberType,
) -> Result<Template, CustomError> {
    let weeks = get_week_list()?;
    let proofs = get_proof_list()?;

    if let Some(w) = weeks.get(&number) {
        let mut authors = HashSet::new();
        let week_proofs: Vec<Option<Proof>> = w
            .proofs
            .iter()
            .map(|pid| match proofs.get(pid) {
                Some(p) => {
                    for author in p.authors.clone() {
                        authors.insert(author);
                    }

                    return Some(p.as_html_proof());
                }
                None => None,
            })
            .collect();

        let mut sorted_authors: Vec<String> = authors.into_iter().collect();
        sorted_authors.sort_unstable();

        Ok(Template::render(
            "week-view",
            context! {
                week: w,
                proofs: week_proofs,
                authors: sorted_authors,
            },
        ))
    } else {
        Err(CustomError::new(
            ErrorType::ServerError,
            "Week not found".to_string(),
        ))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index_endpoint,
                proof_list_endpoint,
                proof_view_endpoint,
                week_list_endpoint,
                week_view_endpoint,
                week_newest_endpoint,
                webhook::handle_webhook
            ],
        )
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
