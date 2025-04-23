use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::local::Mem};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Foo {
    pub foo_id: Uuid,
    pub name: String,
    pub age: u32,
}

#[tokio::main]
async fn main() {
    let foo = Foo {
        foo_id: Uuid::now_v7(),
        name: "John Doe".to_string(),
        age: 30,
    };

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("group").use_db("group").await.unwrap();

    // insert record with the sdk
    let _: Vec<Foo> = db.insert("fooBytes").content(foo.clone()).await.unwrap();

    // print the result of the sdk query
    let query = db.query("SELECT * FROM fooBytes").await.unwrap();
    print!("first query: {:#?}", query);

    // create the table with the literal query with 'u' prefix
    let _ = db
        .query(format!(
            "CREATE fooUuid SET foo_id = u'{}', name = '{}', age = {}",
            foo.foo_id, foo.name, foo.age
        ))
        .await
        .unwrap();

    // print the result of the literal query with 'u' prefix
    let query = db.query("SELECT * FROM fooUuid").await.unwrap();
    print!("second query: {:#?}", query);

    // create the table with the literal query without 'u' prefix
    let _ = db
        .query(format!(
            "CREATE fooStrand SET foo_id = '{}', name = '{}', age = {}",
            foo.foo_id, foo.name, foo.age
        ))
        .await
        .unwrap();

    // print the result of the literal query without 'u' prefix
    let query = db.query("SELECT * FROM fooStrand").await.unwrap();
    print!("third query: {:#?}", query);

    let fooBytes: Vec<Foo> = db.select("fooBytes").await.unwrap();

    let fooUuid: Vec<Foo> = db.select("fooUuid").await.unwrap();

    // this will panic because of: Db(Serialization("failed to deserialize; expected a byte array, found \"01966288-7237-7c03-8275-fc53e0c72590\""))
    let fooStrand: Vec<Foo> = db.select("fooStrand").await.unwrap();
}
