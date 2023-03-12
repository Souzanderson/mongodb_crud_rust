use std::env;
use bson::Document;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use serde::{de::DeserializeOwned, Serialize};

pub struct DatabaseRepo<T: Serialize + DeserializeOwned + Sync + Send + Unpin> {
    col: Collection<T>,
}

impl<T: Serialize + DeserializeOwned + Sync + Send + Unpin> DatabaseRepo<T> {
    pub fn init(db_name:&String, collection:&String) -> Self {
        dotenv().ok();
        let uri = match env::var("DB_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        println!("[INFO] Connect to => {}", uri);
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(&db_name);
        let col: Collection<T> = db.collection(&collection);
        DatabaseRepo { col }
    }

    pub fn handle_id(&self, id: &String, is_obj: bool) -> Document {
        if is_obj {
            let obj_id = ObjectId::parse_str(id).unwrap();
            return doc! {"_id": obj_id};
        } else {
            return doc! {"_id": id};
        }
    }

    pub fn insert(&self, values: &T) -> Result<InsertOneResult, Error> {
        let value = self
            .col
            .insert_one(values, None)
            .ok()
            .expect("Error creating value");
        Ok(value)
    }

    pub fn update(&self, values: Document, filter: Document) -> Result<UpdateResult, Error> {
        let update = doc! {"$set": values};
        let value = self
            .col
            .update_many(filter, update, None)
            .ok()
            .expect("Error update value");
        Ok(value)
    }

    pub fn select_by_id(&self, id: &String, is_obj: bool) -> Option<T> {
        let filter = self.handle_id(id, is_obj);
        let details = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        match details {
            Some(value) => Some(value),
            _ => None,
        }
    }

    pub fn select(&self, filter: Document) -> Vec<T> {
        let cursors = self
            .col
            .find(filter, None)
            .ok()
            .expect("Error getting user's detail");
        // let response = cursors.map(|item| item.unwrap()).collect();
        let response = cursors.map(|doc| doc.unwrap()).collect::<Vec<T>>();
        response
    }

    pub fn delete(&self, filter: Document) -> bool {
        let cursors = self
            .col
            .delete_many(filter, None)
            .ok()
            .expect("Error getting user's detail");
        // let response = cursors.map(|item| item.unwrap()).collect();
        match Ok::<DeleteResult, Error>(cursors) {
            Ok(res) => {
                if res.deleted_count > 0 {
                    true
                } else {
                    false
                }
            }
            _ => {
                println!("[ERORR] Erro ao excluir dado!");
                false
            }
        }
    }

    pub fn delete_by_id(&self, id: &String, is_obj: bool) -> bool {
        let filter = self.handle_id(id, is_obj);
        let cursors = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        // let response = cursors.map(|item| item.unwrap()).collect();
        match Ok::<DeleteResult, Error>(cursors) {
            Ok(res) => {
                if res.deleted_count > 0 {
                    true
                } else {
                    false
                }
            }
            _ => {
                println!("[ERORR] Erro ao excluir dado!");
                false
            }
        }
    }
}
