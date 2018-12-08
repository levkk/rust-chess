extern crate reqwest;
extern crate serde_json;

use serde_json::value::Value as JsonValue;

// Networking
use std::{net, thread, time, collections};
use std::io::{Read, Write};

// String
use std::str;

//Display
use std::fmt;

// Input
use helpers::input;

// Protocol
use protocol::Message;

// Retry attempts for http connection
const RETRY_ATTEMPTS_HTTP: i32 = 5;




/// Connection interface
pub trait Connection {
  fn send_message(&mut self, message: &str) -> bool;
  fn wait_for_message(&mut self) -> Result<String, String>;
  fn get_message(&self) -> Result<String, String>;
}

/// Echo connection
/// Doesn't do much.
pub struct EchoConnection {

}

impl EchoConnection {
  /// New connection
  pub fn new() -> Self {
    EchoConnection{}
  }
}

impl Connection for EchoConnection {
  ///
  fn send_message(&mut self, _message: &str) -> bool {
    true
  }

  ///
  fn wait_for_message(&mut self) -> Result<String, String> {
    Ok(String::from("make_move e7e5"))
  }

  ///
  fn get_message(&self) -> Result<String, String> {
    Ok(String::from("Nothing"))
  }
}

/// Tcp connection
pub struct TcpConnection {
  stream: net::TcpStream,
}

enum TcpConnectionDelimiter {
  EndOfMessage,
}

impl fmt::Display for TcpConnectionDelimiter {
  //
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = match *self {
      TcpConnectionDelimiter::EndOfMessage => write!(f, "\r\n"), // Just like HTTP
    };

    Ok(())
  }
}

impl TcpConnection {
  ///
  pub fn new(host: &str) -> Result<TcpConnection, String> {
    // Attempt a connection to server
    match net::TcpStream::connect(host) {
      Ok(stream) => {
        let connection = TcpConnection{stream};
        Ok(connection)
      },

      Err(err) => {
        let error = format!("{}", err);
        Err(error)
      },
    }
  }

  ///
  pub fn host(host: &str) -> Result<TcpConnection, String> {
    // Create a listening socket
    let listener = match net::TcpListener::bind(host) {
      Ok(listener) => listener,
      Err(err) => panic!("Connection > Host could not bind to address: {}, {}", host, err),
    };

    // Wait for the other player to connect
    println!("Waiting for the other player to connect...");

    let (stream, addr) = match listener.accept() {
      Ok((stream, addr)) => (stream, addr),
      Err(err) => panic!("TcpConnection > Host could not accept: {}", err),
    };

    println!("Client connected from: {}", addr);

    Ok(TcpConnection{stream})
  }
}

impl Connection for TcpConnection {
  ///
  fn send_message(&mut self, message: &str) -> bool {
    let mut data = String::from(message);

    // Append the end of message delimiter
    data.push_str(&TcpConnectionDelimiter::EndOfMessage.to_string());

    let mut raw_data = data.as_bytes();

    println!("TcpConnection > Sending {}", str::from_utf8(&raw_data).unwrap());
    
    match self.stream.write(&mut raw_data) {
      Ok(_) => true,
      Err(err) => {
        println!("TcpConnection > Writing error: {}", err);
        false
      },
    }
  }

  /// Receive a message from another player
  fn wait_for_message(&mut self) -> Result<String, String> {
    let mut message = String::new();

    while !message.contains(&TcpConnectionDelimiter::EndOfMessage.to_string()) {
      let mut buffer = [0u8; 512];
      
      match self.stream.read(&mut buffer) {
        Ok(_) => (),
        Err(err) => panic!("Connection error: {}", err),
      };

      match str::from_utf8(&buffer) {
        Ok(data) => message.push_str(&data),
        Err(err) => panic!("Decoding error: {}", err),
      };

      println!("TcpConnection > Got message: {}", String::from_utf8_lossy(&buffer));

      // If we receive an empty message, usually the connection is terminated.
      if buffer[0] as char == '\0' {
        break;
      }
    }

    let message = message
      .replace('\0', "")
      .replace(&TcpConnectionDelimiter::EndOfMessage.to_string(), "");

    Ok(message)
  }

  ///
  fn get_message(&self) -> Result<String, String> {
    Ok(String::from(""))
  }
}


/// Local player
pub struct SelfConnection {

}

impl SelfConnection {
  pub fn new() -> SelfConnection {
    SelfConnection{}
  }
}

impl Connection for SelfConnection {

  /// Ignore the message
  fn send_message(&mut self, _message: &str) -> bool {
    true
  }

  fn wait_for_message(&mut self) -> Result<String, String> {
    print!(" Other player > ");

    let value = input();

    if value == "exit" {
      Ok(Message::Bye.to_string())
    }

    else if value.len() != 4 {
      Ok(format!("{}", Message::BadMessage))
    }
    
    else {
      Ok(format!("{} {}", Message::MakeMove, value))
    }
  }

  fn get_message(&self) -> Result<String, String> {
    Ok(String::from("Nothing"))
  }
}

pub struct HttpConnection {
  endpoint: String,
  client: reqwest::Client,
  name: String,
  location: String,
  previous_message: String,
  other_player: String,
}

use reqwest::header;

impl HttpConnection {
  pub fn new(endpoint: &str, client_name: &str) -> Result<HttpConnection, String> {
    let join_url = format!("{}/clients", endpoint);

    let client = reqwest::Client::new();
    let body = json!({
      "name": client_name,
    });

    let location = match client.post(&join_url).json(&body).send() {
      Ok(mut res) => {
        let headers = res.headers().clone();
        println!("{:?}", headers.clone());
        let location_header = String::from(headers[header::LOCATION].to_str().unwrap());
        location_header
      }
      Err(err) => return Err(err.to_string()),
    };

    // println!("Connection: {}", location);

    Ok(HttpConnection{
      endpoint: String::from(endpoint),
      client,
      location,
      name: String::from(client_name),
      previous_message: String::from(""),
      other_player: String::from("/clients/14"),
    })
  }
}

impl Connection for HttpConnection {
  fn send_message(&mut self, message: &str) -> bool {
    println!("Sending http message: {}", message);
    let endpoint = format!("{}/{}/message", self.endpoint, self.location);
    println!("Endpoint: {}", endpoint);
    self.client.post(&endpoint)
      .json(&json!({"message": message}))
      .send()
    .unwrap();

      true
  }

  fn wait_for_message(&mut self) -> Result<String, String> {
    // let mut attempts = RETRY_ATTEMPTS_HTTP;

    loop {
      let mut response = self.client
        .get(&format!("{}/{}", self.endpoint, self.other_player))
        .send()
      .unwrap();

      if response.status().is_server_error() {
        println!("Server error: {}", response.status());
      }

      if response.status().is_client_error() {
        panic!("Client error: {}", response.status());
      }

      let client: JsonValue = match response.json() {
        Ok(client) => client,
        Err(err) => panic!("Bad JSON from server: {}", err),
      };

      println!("Client: {}", client);

      let message = String::from(
        client["nextMessage"]
        .as_object()
        .unwrap()["message"]
        .as_str()
        .unwrap()
      );

      if message != self.previous_message {
        self.previous_message = message.clone();
        return Ok(message);
      }

      println!("No new message. Waiting a second to retry...");

      thread::sleep(time::Duration::from_millis(1000));
    }
  }

  fn get_message(&self) -> Result<String, String> {
    Ok(String::from("Nothing"))
  }
}

impl Drop for HttpConnection {
  fn drop(&mut self) {
    self.client.delete(&format!("{}/clients/{}", self.endpoint, self.name)).send();
  }
}

// impl Connection for HttpConnection {
//   fn send_message(&mut self, message: &str) -> bool {

//   }
// }
