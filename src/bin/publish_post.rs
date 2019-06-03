extern crate blog_cli;
extern crate diesel;

use self::blog_cli::*;
use self::diesel::prelude::*;
use self::models::Post;
use std::env::args;

fn main() {
    use blog_cli::schema::posts::dsl::{posts, published};
    let id = args()
        .nth(1)
        .expect("Id must be provided")
        .parse::<i32>()
        .expect("Invalid Id");

    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post with Id: {}", id));

    println!("Updating post with title: {}", post.title);
}
