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

use chrono::naive::NaiveDateTime;
use std::io::stdin;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    // insert_task_from_user();
    display_tasks();
    complete_task();
}

fn insert_task_from_user() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    let mut due_date = None;
    loop {
        println!("Enter a date in the form of \"YYYY-MM-DD\"");
        println!("Or enter a blank line to skip");
        let mut due_date_string = String::new();
        stdin().read_line(&mut due_date_string).unwrap();
        let due_date_string = &due_date_string[..due_date_string.len() - 1];
        eprintln!("due_date_string: \"{}\"", due_date_string);
        if due_date_string.is_empty() {
            break;
        }
        let string_to_parse = format!("{} 00:00", due_date_string);
        match NaiveDateTime::parse_from_str(&string_to_parse, "%Y-%m-%d %H:%M") {
            Ok(date) => {
                due_date = Some(date);
                break;
            }
            // Err(_) => {
            Err(e) => {
                eprintln!("Err: {:?}", e);
                println!("That was not a valid date.")
            }
        }
    }

    let task = create_task(&connection, Some(title), due_date);
    println!("\nSaved task \"{}\" with id {}", title, task.id);
}

fn complete_task() {
    use self::schema::tasks::dsl::{completed, tasks};

    let mut id = String::new();
    println!("Enter the ID of the task to complete");
    stdin().read_line(&mut id).unwrap();
    let id = &id[..id.len() - 1];
    let id = id.parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let task = diesel::update(tasks.find(id))
        .set(completed.eq(true))
        .get_result::<Task>(&connection)
        .expect(&format!("Unable to find task {}", id));
    println!("Completed task {}", task.title);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

pub fn create_task<'a>(
    conn: &PgConnection,
    title: Option<&'a str>,
    due_date: Option<NaiveDateTime>,
) -> Task {
    use schema::tasks;

    let new_task = NewTask { title, due_date };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
        .expect("Error saving new task")
}

fn display_tasks() {
    display_up_to_n_tasks(10);
}

fn display_up_to_n_tasks(max_num_tasks: i64) {
    use self::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .order_by(due_date)
        .then_order_by(title)
        .limit(max_num_tasks)
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        let checkbox = if task.completed { "X" } else { " " };
        let date_string = task
            .due_date
            .map(|d| format!(" (due {})", d.to_string()))
            .unwrap_or_default();
        println!(
            "[{}] #{: <3} {}{}",
            checkbox, task.id, task.title, date_string
        );
    }
}
