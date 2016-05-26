extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
  let conn = establish_connection();

  println!("Title:");
  let mut title = String::new();
  stdin().read_line(&mut title).unwrap();
  let title = &title.replace("\n", "");
  println!("\nOk! Let's write {}. (Press {} when finished)\n", title, EOF);
  let mut body = String::new();
  stdin().read_to_string(&mut body).unwrap();

  let post = create_post(&conn, title, &body);
  println!("Saved post with title {} and id {}", title, post.id);
}


#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
