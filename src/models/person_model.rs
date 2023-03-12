use mongodb::bson::{oid::ObjectId, doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub location: String,
    pub title: String,
}


impl Person{
    pub fn toJSON(&self) -> Document{
        return doc!{
            "name": self.name.clone(),
            "location": self.location.clone(),
            "title": self.title.clone()
        }
    }
}