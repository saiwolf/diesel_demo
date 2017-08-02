extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    let mut n = String::new();
    println!("What's your User ID?");
    stdin()
        .read_line(&mut n)
        .expect("Failed to get ID");
    let user_id: i32 = n.trim().parse().expect("Invalid input. Needs to be an integer!");

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character

    println!("\nOk! Let's write {}! (Press {} when finished)\n", title, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let _ = create_post(&connection, title, &body, user_id);
    println!("\nSaved draft {}", title)
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
