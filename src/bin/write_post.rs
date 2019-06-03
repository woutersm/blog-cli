extern crate blog_cli;
extern crate diesel;

use self::blog_cli::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Title?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..title.len() - 1];
    println!("\nOk! Let's write {} (Press CTRL-D when finished)\n", title);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();

    let post = create_post(&connection, title, &body);

    println!("\nSaved draft {} with id {}", title, post.id);
}
