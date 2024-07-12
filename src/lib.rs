
pub mod models;
pub mod schema;

use crate::models::{NewPost, Post};

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}




pub fn show_posts() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

        // The following will return all users as a `QueryResult<Vec<User>>`
}

// pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str)  {
//     use crate::schema::posts;

//     let new_post = NewPost { title, body };


//     diesel::insert_into(posts::table)
//     .values(&new_post)
//     .get_result(conn)
//     .expect("Error saving new post");

//     // diesel::insert_into(posts::table)
//     //     .values(&new_post)
//     //     .execute(conn)
//     //     .expect("Error saving new post");

//     //posts::table.order(posts::id.desc()).first(conn).unwrap()

// }

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str)  {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn).expect("error");
//         .returning(Post::as_returning())
//         .get_result(conn)
//         .expect("Error saving new post")
}