// Hide some warnings
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

// ORM
extern crate diesel;

// .env manager
extern crate dotenv;

// Good time library
// TODO: use it
// extern crate chrono;

// Diesel imports
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;

// .env
use dotenv::dotenv;

// Standard library
use std::env;
use std::time::SystemTime;

// Diesel-generated database schema
use schema::*;

#[table_name="clients"]
#[derive(Clone, Queryable, Identifiable, Associations, AsChangeset)]
pub struct Client {
  pub id: i64,
  pub name: String,
  pub rank: Option<i64>,
  pub online: bool,
  pub last_login: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name="clients"]
pub struct NewClient {
  pub name: String,
  pub rank: Option<i64>,
  pub online: bool,
  pub last_login: Option<SystemTime>,
}

#[derive(Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="games"]
pub struct Game {
  pub id: i64,
  pub started_at: Option<SystemTime>,
  pub ended_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
  pub started_at: Option<SystemTime>,
  pub ended_at: Option<SystemTime>,
}

#[derive(Clone, Serialize, Deserialize, Queryable, Identifiable, Associations, AsChangeset)]
#[table_name="client_games"]
#[belongs_to(Client)]
#[belongs_to(Game)]
pub struct ClientGame {
  pub id: i64,
  pub client_id: i64,
  pub game_id: i64,
}

#[derive(Insertable)]
#[table_name="client_games"]
pub struct NewClientGame {
  pub client_id: i64,
  pub game_id: i64,
}

// Generate the create method
macro_rules! c {
  ($table:tt, $InsertableObject:tt) => (
    pub fn create(object: $InsertableObject, connection: &PgConnection) -> Result<Self, String> {
      // Establish a transaction
      match connection.transaction::<Self, Error, _>(|| {

        // Insert into the database
        diesel::insert_into($table::table)
        .values(&object)
        .execute(connection)?;

        // Select it back
        $table::table
        .order($table::id.desc())
        .first::<Self>(connection)

      }) {
        Ok(object) => Ok(object),
        Err(err) => Err(format!("{}", err)),
      }
    }
  )
}

// Generate list, retrieve, update, and destroy methods
macro_rules! rud {
  ($table:tt) => (
    pub fn list(limit: i64, offset: i64, connection: &PgConnection) -> Vec<Self> {
      match $table::table
        .limit(limit)
        .offset(offset)
        .load::<Self>(connection) {
        Ok(objects) => objects,
        Err(_) => vec![],
      }
    }

    pub fn retrieve(id: i64, connection: &PgConnection) -> Option<Self> {
      match $table::table
        .filter($table::id.eq(id))
        .first::<Self>(connection) {
          Ok(object) => Some(object),
          Err(_) => None,
        }
    }

    pub fn update(&self, object: Self, connection: &PgConnection) -> bool {
      match diesel::update(self).set(&object).execute(connection) {
        Ok(_) => true,
        Err(_) => false,
      }
    }

    pub fn delete(id: i64, connection: &PgConnection) -> bool {
      match diesel::delete(
        $table::table.filter($table::id.eq(id))
      )
      .execute(connection) {
        Ok(_) => true,
        Err(_) => false,
      }
    }
  );
}

// Create, retrieve, update, destroy
macro_rules! crud {
  ($table:tt, $InsertableObject:tt) => (
    c!($table, $InsertableObject);
    rud!($table);
  );
}

impl Client {
  crud!(clients, NewClient);

  pub fn get(name: String, connection: &PgConnection) -> Option<Self> {
    match clients::table
      .filter(clients::name.eq(&name))
      .first::<Client>(connection) {
      Ok(client) => Some(client),
      Err(_) => None,
    }
  }

  pub fn exists(name: String, connection: &PgConnection) -> bool {
    match Self::get(name, connection) {
      Some(_) => true,
      None => false,
    }
  }

  // pub fn list(limit: i64, connection: &PgConnection) -> Vec<Self> {
  //   match clients::table
  //     .limit(limit)
  //     .load::<Client>(connection) {
  //       Ok(clients) => clients,
  //       Err(_) => vec![],
  //     }
  // }

  pub fn online(limit: i64, connection: &PgConnection) -> Vec<Self> {
    match clients::table
      .filter(clients::online.eq(true))
      .limit(limit)
      .load::<Client>(connection) {
        Ok(clients) => clients,
        Err(_) => vec![],
      }
  }

  pub fn logout(&self, connection: &PgConnection) -> bool {
    match diesel::update(self)
      .set(clients::online.eq(false))
      .execute(connection) {
        Ok(rows) => rows == 1,
        Err(_) => false,
    }
  }
}

pub fn connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set in .env or in the environment.");

  PgConnection::establish(&database_url)
    .expect(&format!("Cannot connect to database: {}", database_url))
}
