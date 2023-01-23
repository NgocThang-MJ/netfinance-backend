use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Attribute {
    MustHave,
    NiceToHave,
    Wasted,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Attribute::MustHave => write!(f, "MustHave"),
            Attribute::NiceToHave => write!(f, "NiceToHave"),
            Attribute::Wasted => write!(f, "Wasted"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub price: String,
    pub attribute: Attribute,
    pub date: String,
}
