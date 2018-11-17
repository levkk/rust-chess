#![allow(proc_macro_derive_resolution_fallback)]

extern crate diesel;

extern crate dotenv;
extern crate chrono;

extern crate syn;
extern crate proc_macro;

use diesel::prelude::*;
use diesel::result::{Error, DatabaseErrorKind};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;
use std;



// use diesel::sql_types::Nullable;

// Use the generated schema macros
use schema::*;

#[table_name="clients"]
#[derive(Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations, AsChangeset)]
pub struct Client {
  pub id: Option<i64>,
  pub name: String,
  pub rank: Option<i64>,
  pub online: bool,
  pub last_login: Option<SystemTime>,
}

#[table_name="games"]
#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations, AsChangeset)]
pub struct Game {
  pub id: Option<i64>,
  pub started_at: Option<SystemTime>,
  pub ended_at: Option<SystemTime>,
}

#[table_name="client_games"]
#[derive(Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations, AsChangeset)]
#[belongs_to(Client)]
#[belongs_to(Game)]
pub struct ClientGame {
  pub id: Option<i64>,
  pub client_id: i64,
  pub game_id: i64,
}

pub trait CRUD {
  fn create(object: Self, connection: &PgConnection) -> Result<Self, String> where Self: std::marker::Sized;
  fn list(limit: i64, offset: i64, connection: &PgConnection) -> Vec<Self> where Self: std::marker::Sized;
  fn retrieve(id: i64, connection: &PgConnection) -> Option<Self> where Self: std::marker::Sized;
  fn update(&self, object: Self, connection: &PgConnection) -> bool where Self: std::marker::Sized;
  fn delete(object: Self, connection: &PgConnection) -> bool where Self: std::marker::Sized;
  fn save(&mut self, connection: &PgConnection);
}

macro_rules! crud {
  ($table:tt) => (
    fn create(object: Self, connection: &PgConnection) -> Result<Self, String> {
      match diesel::insert_into($table::table)
        .values(&object)
        .execute(connection) {
          Ok(_) => Ok(object),
          Err(err) => Err(format!("Error: {}", err)),
        }
    }

    fn list(limit: i64, offset: i64, connection: &PgConnection) -> Vec<Self> {
      match $table::table
        .limit(limit)
        .offset(offset)
        .load::<Self>(connection) {
        Ok(objects) => objects,
        Err(_) => vec![],
      }
    }

    fn retrieve(id: i64, connection: &PgConnection) -> Option<Self> {
      match $table::table
        .filter($table::id.eq(id))
        .first::<Self>(connection) {
          Ok(object) => Some(object),
          Err(_) => None,
        }
    }

    fn update(&self, object: Self, connection: &PgConnection) -> bool {
      match diesel::update(self).set(&object).execute(connection) {
        Ok(_) => true,
        Err(_) => false,
      }
    }

    fn delete(object: Self, connection: &PgConnection) -> bool {
      match diesel::delete(
        $table::table.filter($table::id.eq(object.id))
      )
      .execute(connection) {
        Ok(_) => true,
        Err(_) => false,
      }
    }

    fn save(&mut self, _connection: &PgConnection) {
      unimplemented!();
      // match self.id {
      //   None => {
      //     connection
      //       .build_transaction()
      //       .read_write()
      //       .run(|| {
      //         let _ = Self::create(self.clone(), connection).unwrap();
      //         match $table::table.order($table::id.desc()).first::<Self>(connection) {
      //           Ok(object) => {
      //             self.id = object.id.clone();
      //             Ok(())
      //           },
      //           Err(_) => {
      //             panic!("Database error: no rows after successful insert.");
      //           },
      //         }
      //       });
      //   },
      //   Some(id) => unimplemented!(),
      // };
    }
  )
}

impl CRUD for ClientGame {
  crud!(client_games);
}

impl ClientGame {
  pub fn new(client: &Client, game: &Game) -> Self {
    ClientGame{
      id: None,
      client_id: client.id.unwrap(),
      game_id: game.id.unwrap(),
    }
  }
}

impl Client {
  pub fn create(client: Self, connection: &PgConnection) -> Result<Self, String> {
    match diesel::insert_into(clients::table)
      .values(&client)
      .execute(connection) {
        Ok(_) => Ok(client),
        Err(err) => match err {
          Error::DatabaseError(error_type, _) => match error_type {
            DatabaseErrorKind::UniqueViolation => {
              Err(String::from("Unique constraint violation!"))
            },

            _ => Err(format!("Database internal error: {:?}", error_type))
          },

          _ => Err(format!("Database error: {}", err)),
        }
      }
  }

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

  pub fn list(limit: i64, connection: &PgConnection) -> Vec<Self> {
    match clients::table
      .limit(limit)
      .load::<Client>(connection) {
        Ok(clients) => clients,
        Err(_) => vec![],
      }
  }

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
