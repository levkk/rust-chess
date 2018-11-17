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
use diesel::result::{Error, DatabaseErrorKind};
use diesel::pg::PgConnection;

// .env
use dotenv::dotenv;

// Standard library
use std::env;
use std::time::SystemTime;
use std;

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

// Create
pub trait C<T: Sized> {
  fn create(object: Self, connection: &PgConnection) -> Result<T, String>;
}

// Read, update, destroy
pub trait RUD {
  fn list(limit: i64, offset: i64, connection: &PgConnection) -> Vec<Self> where Self: std::marker::Sized;
  fn retrieve(id: i64, connection: &PgConnection) -> Option<Self> where Self: std::marker::Sized;
  fn update(&self, object: Self, connection: &PgConnection) -> bool where Self: std::marker::Sized;
  fn delete(object: Self, connection: &PgConnection) -> bool where Self: std::marker::Sized;
  fn save(&mut self, connection: &PgConnection);
}

macro_rules! c {
  ($table:tt, $QueryableObject:tt) => (

    fn create(object: Self, connection: &PgConnection) -> Result<$QueryableObject, String> {
      // Establish a transaction
      match connection.transaction::<$QueryableObject, Error, _>(|| {

        // Insert into the database
        diesel::insert_into($table::table)
        .values(&object)
        .execute(connection)?;

        // Select it back
        $table::table
        .order($table::id.desc())
        .first::<$QueryableObject>(connection)

        // Insert the new row into the table
        // match diesel::insert_into($table::table)
        // .values(&object)
        // .execute(connection) {
        //   Ok(_) => {
        //     // Select that row back
        //     match $table::table
        //     .order($table::id.desc())
        //     .first::<$QueryableObject>(connection) {
        //       // Transaction completes successfully
        //       Ok(object) => Ok(object),

        //       // We didn't find the row we just inserted, which means the insert failed but database
        //       // didn't report it. Odd, very unlikely?
        //       Err(err) => {
        //         // println!("INSERT for the transaction failed: {}", err);
        //         Err(err)
        //       },
        //     }
        //   },

        //   // Insert failed because of a contrait or a syntax error
        //   Err(err) => {
        //     // println!("INSERT failed because of a constraint or a syntax error: {}", err);
        //     Err(err)
        //   },
        // }
      }) {
        Ok(object) => Ok(object),
        Err(err) => Err(format!("{}", err)),
      }
    }
  )
}

macro_rules! rud {
  ($table:tt) => (
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

// impl C for NewClientGame {
//   c!(client_games);
// }

impl RUD for ClientGame {
  rud!(client_games);
}

impl ClientGame {
  // pub fn new(client: &Client, game: &Game) -> Self {
  //   ClientGame{
  //     id: None,
  //     client_id: client.id.unwrap(),
  //     game_id: game.id.unwrap(),
  //   }
  // }
}

impl C<Client> for NewClient {
  c!(clients, Client);
}

impl RUD for Client {
  rud!(clients);
}

impl Client {
  pub fn create(client: NewClient, connection: &PgConnection) -> Result<NewClient, String> {
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
