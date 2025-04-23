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
    let _: Vec<Foo> = db.insert("foo").content(foo.clone()).await.unwrap();

    // print the result of the sdk query
    let query = db.query("SELECT * FROM foo").await.unwrap();
    print!("first query: {:#?}", query);

    // empty the table
    let _: Vec<Foo> = db.delete("foo").await.unwrap();

    // create the table with the literal query with 'u' prefix
    let _ = db
        .query(format!(
            "CREATE foo SET foo_id = u'{}', name = '{}', age = {}",
            foo.foo_id, foo.name, foo.age
        ))
        .await
        .unwrap();

    // print the result of the literal query with 'u' prefix
    let query = db.query("SELECT * FROM foo").await.unwrap();
    print!("second query: {:#?}", query);

    // empty the table
    let _: Vec<Foo> = db.delete("foo").await.unwrap();

    // create the table with the literal query without 'u' prefix
    let _ = db
        .query(format!(
            "CREATE foo SET foo_id = '{}', name = '{}', age = {}",
            foo.foo_id, foo.name, foo.age
        ))
        .await
        .unwrap();

    // print the result of the literal query without 'u' prefix
    let query = db.query("SELECT * FROM foo").await.unwrap();
    print!("third query: {:#?}", query);

}
