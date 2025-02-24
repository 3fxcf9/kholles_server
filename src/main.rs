use gray_matter::engine::YAML;
use gray_matter::Matter;
use kholles_server::error::{CustomError, ErrorType};
use kholles_server::md_to_html::md_to_html;
use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
struct Proof {
    pid: <Self as ProofTrait>::ProofIdType,
    title: String,
    authors: Vec<String>,
    date: String, // TODO: Change
    tags: Vec<String>,
    #[serde(skip_deserializing)]
    content: String,
}

trait ProofTrait {
    type ProofIdType;
}

impl ProofTrait for Proof {
    type ProofIdType = u32;
}

impl Proof {
    fn as_html_proof(&self) -> Proof {
        Proof {
            content: md_to_html(self.content.as_str()),
            ..self.clone()
        }
    }
}

impl Proof {
    fn get_html(&self) -> String {
        md_to_html(self.content.as_str())
    }
}

trait WeekTrait {
    type WeekNumberType;
}

impl WeekTrait for Week {
    type WeekNumberType = u8;
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
struct Week {
    number: <Self as WeekTrait>::WeekNumberType,
    date: String, // TODO: Change
    description: String,
    tags: Vec<String>,
    proofs: Vec<<Proof as ProofTrait>::ProofIdType>,
}

fn list_proofs(
    dir: &Path,
    files: &mut HashMap<<Proof as ProofTrait>::ProofIdType, Proof>,
) -> Result<(), CustomError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                list_proofs(&path, files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let contents = fs::read_to_string(&path)?;
                let result = Matter::<YAML>::new().parse(&contents);
                let proof = result.data.unwrap().deserialize::<Proof>();
                match proof {
                    Err(error) => {
                        return Err(CustomError::new(
                            ErrorType::ServerError,
                            format!("Error at file {}: {}", path.to_str().unwrap(), error),
                        ));
                    }
                    Ok(p) => {
                        files.insert(
                            p.pid,
                            Proof {
                                content: result.content,
                                ..p
                            },
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn list_weeks(dir: &Path, weeks: &mut Vec<Week>) -> Result<(), CustomError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                list_weeks(&path, weeks)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let contents = fs::read_to_string(&path)?;
                match serde_yaml::from_str::<Week>(&contents) {
                    Ok(week) => {
                        weeks.push(week);
                    }
                    Err(error) => {
                        return Err(CustomError::new(
                            ErrorType::ServerError,
                            format!(
                                "Error parsing yaml file {}: {}",
                                path.to_str().unwrap(),
                                error
                            ),
                        ));
                    }
                }
            }
        }
    }
    weeks.sort_by_key(|w| w.number);
    Ok(())
}

#[get("/")]
fn index_endpoint() -> Result<Template, CustomError> {
    Ok(Template::render(
        "index",
        context! {
            title: "My Rocket Website",
            html: "WIP"
        },
    ))
}

#[get("/proof/list")]
fn proof_list_endpoint() -> Result<Template, CustomError> {
    let mut files = HashMap::new();
    list_proofs(Path::new("."), &mut files)?;

    Ok(Template::render(
        "proof-list",
        context! {
            proofs: files.into_values().map(|e| e.clone()).collect::<Vec<Proof>>(),
        },
    ))
}

#[get("/proof/<pid>")]
fn proof_view_endpoint(pid: <Proof as ProofTrait>::ProofIdType) -> Result<Template, CustomError> {
    let mut proofs = HashMap::new();
    list_proofs(Path::new("."), &mut proofs)?;

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
    let mut weeks = vec![];
    list_weeks(Path::new("."), &mut weeks)?;

    Ok(Template::render(
        "week-list",
        context! {
            weeks: weeks.iter().map(|e| e.clone()).collect::<Vec<Week>>(),
        },
    ))
}

#[get("/week/<number>")]
fn week_view_endpoint(
    number: <Week as WeekTrait>::WeekNumberType,
) -> Result<Template, CustomError> {
    let mut weeks = vec![];
    list_weeks(Path::new("."), &mut weeks)?;
    let mut proofs = HashMap::new();
    list_proofs(Path::new("."), &mut proofs)?;

    if let Some(w) = weeks.get((number.max(1) - 1) as usize) {
        let week_proofs: Vec<Option<Proof>> = w
            .proofs
            .iter()
            .map(|pid| match proofs.get(pid) {
                Some(p) => Some(p.as_html_proof()),
                None => None,
            })
            .collect();
        // let week_proofs: Vec<Option<Proof>> = w.proofs.iter().map(|pid| proofs.get(pid)).collect();

        Ok(Template::render(
            "week-view",
            context! {
                week:w,
                proofs: week_proofs,
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
            ],
        )
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
