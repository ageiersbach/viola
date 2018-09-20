#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rusqlite;
use rusqlite::Connection;

mod journal;

fn main() {
    let connection = Connection::open("../db.sqlite3").unwrap();
    let stmt = connection.execute("SELECT * FROM journals;", &[]).unwrap();
    println!("Hello, world! ${}", &stmt);
}
