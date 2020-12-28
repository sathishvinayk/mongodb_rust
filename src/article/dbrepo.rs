use crate::article::{Article, DB, env};
use std::io::{ErrorKind};
use futures::stream::StreamExt;
use mongodb::{
    Collection,
    results::{DeleteResult, UpdateResult},
    bson::{doc, oid::ObjectId, from_bson, Bson::Document},
};
const COLL: &str = "articles";

// Implemetation for the Database
impl DB {
    // Get the collection name
    fn get_collection(&self) -> Collection {
        let dbname = env::var("DB_NAME").expect("DB_NAME env var must be set"); 
        self.client.database(&dbname).collection(COLL)
    }

    // Insert Doc
    pub async fn insert(&self, post: Article) {
        let data = doc! {
            "name": post.name,
            "author": post.author
        };
        self.get_collection().insert_one(data, None).await.unwrap();
    }

    // Read Doc
    pub async fn read(&self, name: String) -> Result<Option<Article>, ErrorKind>{
        let article = doc! { "name": name };
        let cursor = self.get_collection().find_one(article, None).await;
        match cursor {
            Ok(data) => match data {
                Some(article) => match from_bson(Document(article)) {
                    Ok(result) => Ok(Some(result)),
                    Err(_) => Err(ErrorKind::InvalidInput),
                }
                None => Ok(None)
            },
            Err(_) => Err(ErrorKind::Other)
        }
    }

    // Read all Documents
    pub async fn read_all(&self) -> Result<Vec<Article>, ErrorKind> {
        let mut cursor = self.get_collection().find(None, None).await.unwrap();
        let mut collections: Vec<Article> = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(data) => match from_bson(Document(data)) {
                    Ok(result) => collections.push(result),
                    Err(_) => return Err(ErrorKind::InvalidInput)
                }
                Err(_) => return Err(ErrorKind::Other)
            }
        }
        Ok(collections)
    }

    // Delete one document
    pub async fn delete(&self, id: &str) -> Result<DeleteResult, ErrorKind>{
        let objectid = ObjectId::with_string(id).map_err(|_| ErrorKind::InvalidData)?;
        let data = doc! {
            "_id": objectid
        };
        let delete_record = self.get_collection().delete_one(data, None).await;
        match delete_record {
            Ok(x) => Ok(x),
            Err(_) => Err(ErrorKind::Other)
        }
    }

    // Update document
    pub async fn update(&self, id: &str, article: Article) -> Result<UpdateResult, ErrorKind> {
        let objectid = ObjectId::with_string(id).map_err(|_| ErrorKind::InvalidData)?;
        let data = doc! {
            "_id": objectid
        };
        let new_post = doc! {
            "name": article.name,
            "author": article.author
        };

        let updated_record = self.get_collection().update_one(data, new_post, None).await;

        match updated_record {
            Ok(x) => Ok(x),
            Err(_) => Err(ErrorKind::Other)
        }
    }
}

