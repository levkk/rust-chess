#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::status;
use rocket::{State, Data, Outcome};
use rocket::request::{self, Request, FromRequest};
use rocket::data::{self, FromData};
use rocket::http::{self, Status, ContentType, RawStr};
use rocket::Outcome::*;

extern crate rocket_contrib;

use rocket_contrib::Json;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::sync::Mutex;
use std::io::Read;

type ClientList = Mutex<HashMap<String, Client>>;

/// Errors
static CLIENT_EXISTS_ERROR: &'static str =
  "{\"error\": \"This client already exists.\"}";


/// Message made by a client on the chess board
#[derive(Serialize, Deserialize, Clone, Default)]
struct Message {
  message: String,
}

impl Message {
  pub fn new(message: &str) -> Self {
    Self{message: message.to_string()}
  }
}


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

  pub fn set_board(&mut self, board: &str) {
    self.board = board.to_string();
  }
}

mod clients {
  use super::*;

  #[get("/clients")]
  fn list(client_list: State<ClientList>) -> Json<Vec<Client>> {
    let all_clients = client_list.lock().unwrap()
      .values()
      .map(|client| client.clone())
    .collect();

    Json(all_clients)
  }


  #[post("/clients", format = "application/json", data = "<client>")]
  fn create(
    client: Json<Client>, // Proper client check
    state: State<ClientList>
  ) -> Result<status::Created<Json<Client>>, status::BadRequest<&'static str>> {
    
    let mut client_list = state.lock().unwrap();

    if client_list.contains_key(&client.name.clone()) {
      return Err(
        status::BadRequest(
          Some(CLIENT_EXISTS_ERROR)
        )
      );
    }

    client_list
      .insert(
        client.name.clone(),
        client.clone(),
    );

    let url = format!("/clients/{}", client.name.clone());

    Ok(status::Created(
      url,
      Some(Json(client.clone()))
    ))
  }

  #[get("/clients/<client>")]
  fn retrieve(client: &RawStr, state: State<ClientList>) -> Option<Json<Client>> {
    let mut client_list = state.lock().unwrap();
    let key = String::from(client.as_str());

    if !client_list.contains_key(&key) {
      return None;
    }

    Some(Json(client_list.get(&key).unwrap().clone()))
  }

  #[post("/clients/<client>/message", format = "application/json", data = "<message>")]
  fn make_message(
    client: &RawStr, message: Json<Message>, state: State<ClientList>
  ) -> Option<Json<Message>> {
    let mut client_list = state.lock().unwrap();
    let key = client.to_string();

    if !client_list.contains_key(&key) {
      return None;
    }

    let mut client = client_list.get_mut(&key).unwrap();
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

    let mut client = client_list.get_mut(&key).unwrap();
    client.board = board.clone();

    Some(board)
  }

  trait Filterable<K, V> {
    fn get_filter_mut(&mut self, func: &Fn(&V) -> bool) -> Option<&mut V>;
    fn get_filter(&self, func: &Fn(&V) -> bool) -> Option<V>;
  }

  impl <K: std::cmp::Eq + std::hash::Hash, V: std::clone::Clone>Filterable<K, V> for HashMap<K, V> {
    fn get_filter_mut(&mut self, func: &Fn(&V) -> bool) -> Option<&mut V> {
      for (key, mut value) in self.iter_mut() {
        if func(&value.clone()) {
          return Some(value);
        }
      }

      None
    }

    fn get_filter(&self, func: &Fn(&V) -> bool) -> Option<V> {
      for (key, value) in self.iter() {
        if func(&value.clone()) {
          return Some(value.clone());
        }
      }

      None
    }
  }

  #[get("/clients/<client>/find_buddy")]
  fn find_buddy(client: &RawStr, state: State<ClientList>) -> Option<Json<Client>> {
    let mut client_list = state.lock().unwrap();
    let key = String::from(client.as_str());

    if !client_list.contains_key(&key) {
      return None;
    }

    let mut other_player_name = String::new();
    let mut other_player = Client::default();

    match client_list.get_filter_mut(&|val: &Client| {
      val.name != key && val.other_player.len() == 0
    }) {
      Some(player) => {
        player.other_player = key.clone();
        other_player_name = player.name.clone();
        other_player = player.clone();
      },
      None => return None,
    };

    match client_list.get_mut(&key) {
      Some(player) => player.other_player = other_player_name,
      None => return None,
    };

    Some(Json(other_player))
  }
}


// #[post("/clients/<client>/update_board", format = "application/json", data = "<board>")]
// fn update_board(client: &RawStr, board: String, state: State<ClientList>) -> Option<String> {
//   None
// }


#[catch(400)]
fn bad_request(req: &Request) -> String {
  String::from("{}")
}

fn main() {
  let client_list = Mutex::new(HashMap::<String, Client>::new());
  client_list.lock().unwrap().insert(String::from("one"), Client::new("one"));

    rocket::ignite()
      .mount("/", routes![
        clients::list,
        clients::create,
        clients::retrieve,
        clients::make_message,
        clients::update_board,
        clients::find_buddy,
        ])
      .manage(client_list)
      .catch(catchers![bad_request])
      .launch();
    println!("Hello, world!");
}
