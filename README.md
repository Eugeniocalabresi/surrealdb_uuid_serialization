# SurrealDB UUID Behavior Project
This is a basic Rust project designed to demonstrate how SurrealDB behaves differently when handling literal queries versus SDK queries involving UUIDs.


- First query: if I insert a record with a UUID via sdk binding, the value will be saved as Bytes array. then I will be able to deserialize it back to the original UUID. 
- Second query: if I insert a record with a UUID via literal query (using the 'u' prefix), the value will be saved as a SurrealDB UUID. then I will be able to deserialize it back to the original UUID.
- Third query: if I insert a record with a UUID via literal query (without the 'u' prefix), the value will be saved as a Strand and I will not be able to deserialize it back to the original UUID.