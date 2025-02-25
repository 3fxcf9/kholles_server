use crate::md_to_html::md_to_html;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Proof {
    pub pid: <Self as ProofTrait>::ProofIdType,
    pub title: String,
    pub authors: Vec<String>,
    pub date: String, // TODO: Change
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub content: String,
}

pub trait ProofTrait {
    type ProofIdType;
}

impl ProofTrait for Proof {
    type ProofIdType = u32;
}

impl Proof {
    pub fn as_html_proof(&self) -> Proof {
        Proof {
            content: md_to_html(self.content.as_str()),
            ..self.clone()
        }
    }
}

impl Proof {
    pub fn get_html(&self) -> String {
        md_to_html(self.content.as_str())
    }
}

pub trait WeekTrait {
    type WeekNumberType;
}

impl WeekTrait for Week {
    type WeekNumberType = u8;
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Week {
    pub number: <Self as WeekTrait>::WeekNumberType,
    pub date: String, // TODO: Change
    pub description: String,
    pub tags: Vec<String>,
    pub proofs: Vec<<Proof as ProofTrait>::ProofIdType>,
}
