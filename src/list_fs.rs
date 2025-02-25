use crate::error::{CustomError, ErrorType};
use crate::types::*;
use gray_matter::{engine::YAML, Matter};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

const LISTEN_PATH_ENV_NAME: &str = "FS_LISTEN_PATH";
const PROOF_SUBFOLDER_NAME: &str = "proofs";
const WEEK_SUBFOLDER_NAME: &str = "weeks";

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

fn list_weeks(
    dir: &Path,
    weeks: &mut HashMap<<Week as WeekTrait>::WeekNumberType, Week>,
) -> Result<(), CustomError> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                list_weeks(&path, weeks)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let contents = fs::read_to_string(&path)?;
                match serde_yaml::from_str::<Week>(&contents) {
                    Ok(week) => {
                        weeks.insert(week.number, week);
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
    Ok(())
}

pub fn get_proof_list() -> Result<HashMap<<Proof as ProofTrait>::ProofIdType, Proof>, CustomError> {
    let mut files = HashMap::new();
    list_proofs(
        Path::new(&env::var(LISTEN_PATH_ENV_NAME).unwrap_or(".".to_string()))
            .join(PROOF_SUBFOLDER_NAME)
            .as_path(),
        &mut files,
    )?;
    Ok(files)
}
pub fn get_week_list() -> Result<HashMap<<Week as WeekTrait>::WeekNumberType, Week>, CustomError> {
    let mut weeks = HashMap::new();
    list_weeks(
        Path::new(&env::var(LISTEN_PATH_ENV_NAME).unwrap_or(".".to_string()))
            .join(WEEK_SUBFOLDER_NAME)
            .as_path(),
        &mut weeks,
    )?;
    Ok(weeks)
}
