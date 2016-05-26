extern crate diesel_demo;
extern crate diesel;

use diesel::prelude::*;
use self::diesel_demo::{establish_connection};
use std::env::args;


fn main() {
  use self::diesel_demo::schema::posts::dsl::*;

  let target = args().nth(1)
    .expect("must include title arg to delete")
    .parse::<String>().expect("da faq, must be a str");
  let pattern = format!("%{}%", target);

  let conn = establish_connection();
  let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(&conn)
    .expect("Error deleting posts");

  println!("Deleted {} posts", num_deleted);

}
