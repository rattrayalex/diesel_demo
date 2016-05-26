extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::diesel_demo::models::Post;
use diesel::prelude::*;
use std::env::args;

fn main() {
  use diesel_demo::schema::posts::dsl::{posts, published};

  let id = args().nth(1)
    .expect("id required")
    .parse::<i32>().expect("invalid ID");

  let conn = establish_connection();

  let post = diesel::update(posts.find(id))
    .set(published.eq(true))
    .get_result::<Post>(&conn)
    .expect(&format!("Unable to find post {}", id));

  println!("Published post '{}'!", post.title);
}
