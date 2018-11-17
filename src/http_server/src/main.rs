// HTTP RESTful server

// Required by Rocket
#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]
 
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate syn;

// Modules
mod db;
mod schema;
mod util;

// CRUD
#[allow(unused_imports)]
use db::C;
#[allow(unused_imports)]
use db::RUD;

// Rocket
use rocket::response::status;
use rocket::State;
use rocket::request::Request;
use rocket::http::RawStr;
use rocket_contrib::Json;

// Std lib
use std::collections::HashMap;
use std::sync::Mutex;

#[allow(unused_imports)]
use util::Filterable;

//
type ClientList = Mutex<HashMap<String, Client>>;

/// Errors
static CLIENT_EXISTS_ERROR: &'static str =
  "{\"error\": \"This client already exists.\"}";


/// Message made by a client on the chess board
#[derive(Serialize, Deserialize, Clone, Default)]
struct Message {
  message: String,
}

// impl Message {
//   // pub fn new(message: &str) -> Self {
//   //   Self{message: message.to_string()}
//   // }
// }


/// A connected client
#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct Client {
  /// Name of the client must be unique
  name: String,
  /// Serialized board (JSON)

  #[serde(default = "String::default")]
  board: String,
  /// Next message

  #[serde(default = "Message::default")]
  next_message: Message,
  /// Other player

  #[serde(default = "String::default")]
  other_player: String,
}

impl Client {
  pub fn new(name: &str) -> Client {
    Client{
      name: name.to_string(),
      board: String::new(),
      next_message: Message::default(),
      other_player: String::new(),
    }
  }

  // pub fn set_board(&mut self, board: &str) {
  //   self.board = board.to_string();
  // }
}

// Serialization
impl From<db::Client> for Client {
  fn from(client: db::Client) -> Self {
    Client::new(&client.name)
  }
}

// Deserialization
impl From<Client> for db::NewClient {
  fn from(client: Client) -> Self {
    db::NewClient {
      name: client.name,
      rank: None,
      online: true,
      last_login: None,
    }
  }
}


mod clients {
  use super::*;

  #[get("/clients")]
  fn list() -> Json<Vec<Client>> {
    let conn = db::connection();
    let all_clients = db::Client::online(25, &conn)
      .iter()
      .map(|client| Client::from(client.clone()))
      .collect();

    Json(all_clients)
  }


  #[post("/clients", format = "application/json", data = "<client>")]
  fn create(
    client: Json<Client>,
  ) -> Result<status::Created<Json<Client>>, status::BadRequest<&'static str>> {
    
    // Establish connection to DB
    let conn = db::connection();

    // Deserialize
    let db_client = db::NewClient::from(client.clone());

    // Execute
    match db::NewClient::create(db_client, &conn) {
      Ok(client) => Ok(
        status::Created(
          format!("/clients/{}", client.id.to_string()), // Generate the URL
          Some(Json(Client::from(client))),
        )
      ),
      Err(_err) => Err(
        status::BadRequest(
          Some(CLIENT_EXISTS_ERROR)
        )
      ),
    }
  }

  #[get("/clients/<id>")]
  fn retrieve(id: &RawStr) -> Option<Json<Client>> {
    let conn = db::connection();

    match id.as_str().parse::<i64>() {
      // Valid integer
      Ok(id) => {
        match db::Client::retrieve(id, &conn) {
          // Found the client
          Some(client) => Some(Json(Client::from(client))),

          // Nope!
          None => None,
        }
      },

      // Not a valid integer
      Err(_) => None,
    }
  }

  #[delete("/clients/<client>")]
  fn delete(client: &RawStr) -> Result<status::NoContent, status::NotFound<&'static str>> {

    let conn = db::connection();

    match db::Client::get(client.to_string(), &conn) {
      Some(client) => {
        if client.logout(&conn) {
          Ok(status::NoContent)
        }

        else {
          Err(status::NotFound("Client does not exist."))
        }
      },
      None => Err(status::NotFound("Client does not exist.")),
    }
  }

  #[post("/clients/<client>/message", format = "application/json", data = "<message>")]
  fn message(
    client: &RawStr, message: Json<Message>, state: State<ClientList>
  ) -> Option<Json<Message>> {
    let mut client_list = state.lock().unwrap();
    let key = client.to_string();

    if !client_list.contains_key(&key) {
      return None;
    }

    let client = client_list.get_mut(&key).unwrap();
    let message = message.into_inner();
    client.next_message = message.clone();

    Some(Json(message))
  }

  #[post("/clients/<client>/update_board", format = "application/json", data = "<board>")]
  fn update_board(
    client: &RawStr,
    board: String,
    state: State<ClientList>
  ) -> Option<String> {
    let mut client_list = state.lock().unwrap();
    let key = client.to_string();

    if !client_list.contains_key(&key) {
      return None;
    }

    let client = client_list.get_mut(&key).unwrap();
    client.board = board.clone();

    Some(board)
  }

  #[get("/clients/<client>/find_buddy")]
  fn find_buddy(client: &RawStr, state: State<ClientList>) -> Option<Json<Client>> {
    let mut client_list = state.lock().unwrap();
    let key = String::from(client.as_str());

    if !client_list.contains_key(&key) {
      return None;
    }

    #[allow(unused_assignments)]
    let mut other_player = Client::default();

    match client_list.get_filter_mut(&|val: &Client| {
      val.name != key && val.other_player.len() == 0
    }) {
      Some(player) => {
        player.other_player = key.clone();
        other_player = player.clone();
      },
      None => return None,
    };

    match client_list.get_mut(&key) {
      Some(player) => {
        player.other_player = other_player.name.clone();
      },
      None => return None, // impossible
    };

    Some(Json(other_player))
  }
}


#[catch(400)]
fn bad_request(_req: &Request) -> String {
  String::from("{}")
}

fn main() {
  let client_list = Mutex::new(HashMap::<String, Client>::new());
  client_list.lock().unwrap().insert(String::from("one"), Client::new("one"));

    rocket::ignite()
      .mount("/", routes![
        clients::list,
        clients::create,
        clients::delete,
        clients::retrieve,
        clients::message,
        clients::update_board,
        clients::find_buddy,
        ])
      .manage(client_list)
      .catch(catchers![bad_request])
      .launch();
    println!("Hello, world!");
}
