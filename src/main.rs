pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// use self::diesel::prelude::*;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .limit(5)
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        let checkbox = if task.completed { "X" } else { " " };
        let date_string = task
            .due_date
            .map(|d| format!(" (due {})", d.to_string()))
            .unwrap_or_default();
        println!("[{}] {}{}", checkbox, task.title, date_string);
    }
}
