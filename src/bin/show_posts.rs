extern crate blog_cli;
extern crate diesel;

use self::models::*;
use blog_cli::*;
use diesel::prelude::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
