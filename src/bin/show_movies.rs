extern crate spidy;
extern crate diesel;

use self::spidy::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use spidy::schema::movies::dsl::*;

    let connection = establish_connection();
    let results = movies.filter(published.eq(true))
        .limit(5)
        .load::<Movie>(&connection)
        .expect("Error loading movies");

    println!("Displaying {} movies", results.len());
    for movie in results {
        println!("{}", movie.title);
        println!("------------\n");
        println!("{}", movie.description);
    }
}
