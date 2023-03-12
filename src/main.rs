mod library;
mod models;

use crate::{library::database::DatabaseRepo, models::person_model::Person};

fn main() {
    let data = Person {
        id: Some("12345678910".to_string()),
        name: String::from("Anderson Silva"),
        location: String::from("Curitiba"),
        title: String::from("Programador Pleno"),
    };

    let db: DatabaseRepo<Person> =
        DatabaseRepo::init(&String::from("Testedb"), &String::from("pessoas"));
    // db.insert(& data);
    // let res = db.select_by_id(& "640e2953366d0d7026a29f01".to_string());
    // println!("Item foi deletado? => {}", deletado);

    db.update(bson::doc!{"location": "SJP"}, bson::doc! {"location": "Curitiba 2"});
    // db.delete_by_id(&"12345".to_string(), false);
    let res = db.select(bson::doc! {});
    // db.delete(bson::doc! {"name": "Anderson 2"});
    println!("Result => {:?}", res);
}
