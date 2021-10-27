use mongodb::{bson::doc, bson::Document, options::ClientOptions, Client};

#[allow(dead_code)]
pub async fn connect() -> mongodb::error::Result<()> {

    let db_constructed_url: &str = concat!(
        "mongodb+srv://",
        "dimitriskl",
        ":",
        "CqW3pyvtd8943sr",
        "@",
        "CqW3pyvtd8943sr",
        "rust-mongo.bxs8v.mongodb.net",
        "/",
        "myFirstDatabase?retryWrites=true&w=majority",
    );

    //let mut client_options = ClientOptions::parse("mongodb+srv://dimitriskl:CqW3pyvtd8943sr@rust-mongo.bxs8v.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let mut client_options = ClientOptions::parse(db_constructed_url).await?;

    client_options.app_name = Some("Rust Demo".to_string());
    
    let client = Client::with_options(client_options)?;
    
    client
        .database("rust-mongo")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("rust-mongo");
    // Get a handle to a collection in the database.
    let users_collection = db.collection::<Document>("users");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "mydb.books" collection.
    users_collection.insert_many(docs, None).await?;


    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok(())
}

/*
pub async fn new_users(Users) -> Result<(), ()> {

}
*/

#[allow(dead_code)]
struct User {
    id: i16
}
#[allow(dead_code)]
struct Users {
    users: Vec<User>
}