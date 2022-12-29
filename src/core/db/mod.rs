use mongodb::{Collection, IndexModel};
use mongodb::bson::Document;
use mongodb::options::{CreateIndexOptions, IndexOptions};

pub(crate) mod repository;


async fn make_indexes<T>(collection: &Collection<T>, indexes: Vec<Document>) {
    let create_options = CreateIndexOptions::builder().build();
    let options = IndexOptions::builder().background(true).build();
    for index in indexes {
        let m = IndexModel::builder().keys(index).options(Some(options.clone())).build();
        if let Err(e) = collection.create_index(m, create_options.clone()).await {
            println!("Failed to create index: {:#?}", e);
        }
    }
}
