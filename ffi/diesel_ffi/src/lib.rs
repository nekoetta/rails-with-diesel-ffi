mod schema;

#[macro_use]
extern crate diesel;
extern crate libc;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::NaiveDateTime;
use csv::WriterBuilder;
use std::error::Error;
use dotenv;
use std::env;
use libc::*;
use std::ffi::{CString};

use schema::*;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[no_mangle]
fn take_csv() -> *const c_char {
    let connection = establish_connection();
    let results = somethings::table.inner_join(users::table)
        .select((somethings::sentence, users::name))
        .limit(5_000_000)
        .load::<SomethingWithUser>(&connection)
        .expect("Error loading somethings");

    let csv_data = make_csv(&results).unwrap();
    CString::new(csv_data).unwrap().into_raw()
}

fn make_csv(data: &Vec<SomethingWithUser>) -> Result<String, Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    for some_info in data {
        wtr.write_record(&[&some_info.name, &some_info.sentence])?;
    }
    let data = String::from_utf8(wtr.into_inner()?)?;
    Ok(data)
}


#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[table_name = "users"]
struct User {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "somethings"]
struct Something {
    pub id: i32,
    pub user_id: i32,
    pub int: i32,
    pub sentence: String,
    pub date: Option<NaiveDateTime>,
    pub nest: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable)]
struct SomethingWithUser {
    pub sentence: String,
    pub name: String,
}


extern crate fake;
extern crate serde;
extern crate serde_json;
extern crate rand;

use diesel::{Insertable};
use chrono::{DateTime, Utc};
use fake::{Fake, Faker};
use fake::faker::name::raw::*;
use fake::faker::lorem::raw::*;
use fake::locales::*;
use diesel::dsl::insert_into;
use rand::{thread_rng, Rng};

#[no_mangle]
fn seed() {
    let connection = establish_connection();
    let mut rng = thread_rng();

    use schema::users::dsl::*;
    let mut new_users:  Vec::<UserCreate> = vec!();
    for _n in 0..100 {
        let fake_date = Faker.fake::<DateTime::<Utc>>();
        let user = UserCreate {
            name: Name(EN).fake::<String>(),
            created_at: fake_date.clone(),
            updated_at: fake_date.clone(),
        };

        new_users.push(user);
    }
    insert_into(users).values(&new_users).execute(&connection).unwrap();


    use schema::somethings::dsl::*;
    let mut new_somethings:  Vec::<SomethingCreate> = vec!();
    for n in 1..5_000_001 {
        let fake_date = Faker.fake::<DateTime::<Utc>>();
        let something = SomethingCreate {
            user_id: rng.gen_range(1..100),
            int: Faker.fake::<i32>(),
            sentence: Sentence(EN, 5..8).fake(),
            date: Faker.fake::<DateTime::<Utc>>(),
            nest: Faker.fake::<i32>(),
            created_at: fake_date.clone(),
            updated_at: fake_date.clone(),
        };
        new_somethings.push(something);
        if n % 5_000 == 0 {
            insert_into(somethings).values(&new_somethings).execute(&connection).unwrap();
            new_somethings.clear();
            println!("{} finished", n);
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
struct UserCreate {
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="somethings"]
struct SomethingCreate {
    user_id: i32,
    int: i32,
    sentence: String,
    date: DateTime::<Utc>,
    nest: i32,
    created_at: DateTime::<Utc>,
    updated_at: DateTime::<Utc>,
}