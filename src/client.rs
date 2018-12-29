// Regex
extern crate regex;
use regex::RegexSet;

// Game
// use board::Color;

use connection::{
  Connection, EchoConnection, TcpConnection,
  SelfConnection, HttpConnection,
};

// Messages and Regexes
use protocol::{Message, MessageRegex};

// Client
pub struct Client {
  // server: String,
  connection: Box<Connection>, // Connection size is not known at compile time
  // connected: bool,
  // in_game: bool,
  // color: Color,
  pub host: bool,
  name: String,
}

impl Client {

  /// Create a new client for multiplayer.
  ///
  /// Parameters:
  /// `game`: Game (will take ownership)
  pub fn new(server: &str) -> Client {
    let connection: Box<Connection>;

    if server.starts_with("echo") {
      connection = Box::new(EchoConnection::new());
    }

    else if server.starts_with("tcp://") {
      let tcp_connection = match TcpConnection::new(&server[6..]) {
        Ok(conn) => conn,
        Err(err) => panic!("Could not connect to server: {}", err),
      };

      connection = Box::new(tcp_connection);
    }

    else if server.starts_with("http://") {
      let http_client = match HttpConnection::new(&server, "my_very_random_name_1") {
        Ok(client) => client,
        Err(err) => panic!("Could not connect to server: {}", err),
      };

      connection = Box::new(http_client);
    }

    else {
      connection = Box::new(SelfConnection::new());
    }
        
    Client{
      // server: String::from(server),
      connection,
      host: false,
      name: String::default(),

      // color: Color::Nil,
    }
  }

  /// Host a peer-to-peer game.
  ///
  /// Parameters:
  /// `addr`: &str Properly formatted address, e.g. tcp://0.0.0.0:54345
  pub fn host(addr: &str) -> Client {
    let connection: Box<Connection>;

    // TCP
    if addr.starts_with("tcp://") {
      let listener = match TcpConnection::host(&addr[6..]) {
        Ok(conn) => conn,
        Err(err) => panic!("Could not create server: {}",  err),
      };

      connection = Box::new(listener);
    }

    // Dummy echo
    else {
      connection = Box::new(EchoConnection::new());
    }

    Client{
      connection,
      host: true,
      name: String::default(),
    }
  }

  /// Send a message to the remote peer
  ///
  /// Parameters:
  /// `message`: Message type
  /// `payload`: &str, UTF-8 formatted
  pub fn send_message(&mut self, message: Message, payload: &str) {

    let contents = match message {
      Message::Hello => {
        format!("{} {}", Message::Hello, payload)
      },

      Message::Bye => {
        Message::Bye.to_string()
      },

      Message::BadMessage => {
        format!("{} {}", Message::BadMessage, payload)
      }
      
      Message::MakeMove => {
        format!("{} {}", Message::MakeMove, payload)
      },
    };

    println!("Sending message in client: {}", message);

    let _ = self.connection.send_message(&contents);
  }

  /// Wait for answer from peer and block until it arrives.
  pub fn wait_for_message(&mut self) -> (Message, String) {
    // This will block until something arrives
    // over the pipe. This may not always be what we want
    // so we can use Connection::get_message() isntead.
    let message = self.connection.wait_for_message().unwrap(); // TODO: Handle error

    // Handle the reply
    self.handle_reply(&message).unwrap()
  }

  /// Async wait_for_message (non-blocking)
  pub fn get_message(&mut self) -> Result<(Message, String), String> {
    match self.connection.get_message() {
      Ok(message) => self.handle_reply(&message),
      Err(_) => Err(String::from("No message received yet.")),
    }
  }

  /// Handles peer reply
  ///
  /// Parameters:
  /// `message`: &str reply from peer
  fn handle_reply(&mut self, message: &str) -> Result<(Message, String), String> {

    lazy_static! {
      static ref MESSAGES: RegexSet =  RegexSet::new(&[
        MessageRegex::Bye.to_string(),
        MessageRegex::MakeMove.to_string(),
        MessageRegex::Hello.to_string(),
        MessageRegex::BadMessage.to_string(),
      ]).unwrap();
    }
    
    let matches: Vec<usize> = MESSAGES.matches(message).into_iter().collect();

    // Should match only one message    
    if matches.len() != 1 {
      return Err(format!("Unknown message received: {}", message));
    }

    let message_match = matches.iter().next();

    match message_match {
      Some(&0) => {
        Ok((Message::Bye, String::from("exit")))
      },

      Some(&1) => {
        Ok((Message::MakeMove, String::from(&message[10..])))
      },

      Some(&2) => {
        Ok((Message::Hello, String::from(&message[6..])))
      },

      Some(&3) => {
        Ok((Message::BadMessage, String::from("")))
      },

      Some(&_) => {
        panic!("Client > Handle reply : Received valid message that is not handled by client.");
      },
      
      None => {
        Err(String::from("Got message matching nothing."))
      },
    }
  }

  pub fn set_name(&mut self, name: &str) {
    self.name = String::from(name);
  }
}

#[cfg(test)]
mod test {

  use client::Client;
  use protocol::*;

  // Test regex handling of replies
  #[test]
  fn test_messages() {
    // Create a dummy client
    let mut client = Client::new("echo");

    // Message missing payload
    match client.handle_reply(&Message::MakeMove.to_string()) {
      Ok(_) => panic!("Not supposed to accept this message"),
      Err(_) => (),
    };

    // Good message with payload
    match client.handle_reply(&format!("{} e2e4", Message::MakeMove)) {
      Ok(input) => assert_eq!(input.1, "e2e4"),
      Err(err) => panic!("Made a valid move. {}", err),
    };

    // Good message with no payload
    match client.handle_reply(&Message::Bye.to_string()) {
      Ok(input) => assert_eq!(input.1, "exit"),
      Err(err) => panic!("Valid good bye message. {}", err),
    };

    // Bad message with payload
    match client.handle_reply(&format!("{} random_text", Message::Bye)) {
      Ok(msg) => panic!("Not supposed to accept this message: {}", msg.0),
      Err(_) => (),
    };
  }
}
