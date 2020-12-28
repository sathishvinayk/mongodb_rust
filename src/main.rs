mod article;
use article::{Article};

// This function will do all the REST.
async fn operations() {
    // Initiate the DB
    let client = article::init().await.unwrap();
    let _db = article::DB {
        client: client
    };
    
    let post1 = Article {
        name: "python".to_string(),
        author: "david".to_string()
    };
    let post2 = Article {
        name: "node".to_string(),
        author: "joyce".to_string()
    };
    
    // Insert documents
    _db.insert(post1).await;
    _db.insert(post2).await;

    // Read Document
    let value = _db.read("python".to_string()).await;
    println!("value {:?}", value);

    // Read All Document
    let collections = _db.read_all().await;
    println!("{:?}", collections);

    // Update Document
    let new_post = Article {
        name: "rust".to_string(),
        author: "william".to_string()
    };

    let update_doc = _db.update("5fe98cc20086bed2008e0841", new_post).await;

    println!("Updated Post {:?}", update_doc);
}

#[tokio::main]
async fn main() {
    operations().await;
}
