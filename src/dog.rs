//! Our application logic about dogs

use crate::diesel::establish_connection;
use crate::diesel::models::Dog;
use crate::diesel::schema::dogs::dsl::*;
use diesel::PgConnection;
use diesel::prelude::*;
use itertools::Itertools;

pub fn handle_dog_request() -> String {
    // Ideally, we wouldn't open a new connection for each request
    let mut connection = establish_connection();
    dog_list(&mut connection)
}

fn dog_list(connection: &mut PgConnection) -> String {
    let dog_names = dogs
        .limit(5)
        .select(Dog::as_select())
        .load(connection)
        .expect("Error loading dogs")
        .iter()
        .map(|dog| dog.name.clone())
        .join(", ");
    format!("Names of dogs in the database: {}", dog_names)
}
