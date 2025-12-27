use crate::md_to_html::md_to_html;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub struct Proof {
    pub pid: <Self as ProofTrait>::ProofIdType,
    pub title: String,
    pub note: Option<String>,
    pub authors: Vec<String>,
    #[serde(deserialize_with = "deserialize_date")]
    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDate,
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub content: String,
}

pub trait ProofTrait {
    type ProofIdType;
}

impl ProofTrait for Proof {
    type ProofIdType = u64;
}

impl Ord for Proof {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse cmp for easier sorting (we want the newest first)
        (other.date, other.pid).cmp(&(self.date, self.pid))
    }
}

impl PartialOrd for Proof {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
    #[serde(skip_deserializing)]
    pub number: <Self as WeekTrait>::WeekNumberType,
    #[serde(deserialize_with = "deserialize_date")]
    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDate,
    pub description: String,
    pub proofs: Vec<<Proof as ProofTrait>::ProofIdType>,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveDate::parse_from_str(&String::deserialize(deserializer)?, "%d/%m/%Y")
        .map_err(serde::de::Error::custom)
}

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%d/%m/%Y").to_string())
}
