// Display
use std::fmt;

// Regex
extern crate regex;
use regex::RegexSet;

// Game
// use board::Color;

use connection::{Connection, EchoConnection};

pub enum Message {
  Bye,
  MakeMove,
}

impl fmt::Display for Message {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = match *self {
      Message::Bye => write!(f, "bye"),
      Message::MakeMove => write!(f, "make_move"),
    };

    Ok(())
  }
}

enum MessageRegex {
  Bye,
  MakeMove,
}

impl fmt::Display for MessageRegex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = match *self {
      MessageRegex::Bye => write!(f, r"{}", Message::Bye),
      MessageRegex::MakeMove => write!(f, r"{} [A-Ha-h][1-8][A-Ha-h][1-8]", Message::MakeMove),
    };

    Ok(())
  }
}

// Client
pub struct Client {
  // server: String,
  connection: Box<Connection>, // Connection size is not known at compile time
  // connected: bool,
  // in_game: bool,
  // color: Color,
}

impl Client {

  /// Create a new client for multiplayer.
  ///
  /// Parameters:
  /// `game`: Game (will take ownership)
  pub fn new(server: &str) -> Self {
    let connection: Box<Connection>;

    if server.starts_with("echo") {
      connection = Box::new(EchoConnection::new());
    }

    else {
      connection = Box::new(EchoConnection::new());
    }
        
    Client{
      // server: String::from(server),
      connection: connection,

      // color: Color::Nil,
    }
  }

  ///
  pub fn send_message(&self, message: Message, payload: &str) {
    let contents: String;

    match message {
      Message::Bye => {
        contents = Message::Bye.to_string();
      },
      
      Message::MakeMove => {
        contents = format!("{} {}", Message::MakeMove, payload);
      }
    }

    let _ = self.connection.send_message(&contents);
  }

  ///
  pub fn receive_message(&mut self) -> String {
    // This will block until something arrives
    // over the pipe. This may not always be what we want
    // so we can use Connection::get_message() isntead.
    let message = self.connection.wait_for_message().unwrap(); // TODO: Handle error

    // Handle the reply
    self.handle_reply(&message).unwrap()
  }

  ///
  fn handle_reply(&mut self, message: &str) -> Result<String, String> {

    lazy_static! {
      static ref MESSAGES: RegexSet =  RegexSet::new(&[
        MessageRegex::Bye.to_string(),
        MessageRegex::MakeMove.to_string(),
      ]).unwrap();
    }
    
    let matches: Vec<usize> = MESSAGES.matches(message).into_iter().collect();

    if matches.len() != 1 {
      return Err(format!("Unknown message received: {}", message));
    }

    let message_match = matches.iter().next();

    match message_match {
      Some(&0) => {
        Ok(String::from("exit"))
      },
      Some(&1) => {
        Ok(String::from(&message[10..]))
      },
      Some(&_) => {
        Err(String::from("Got unhandled matched message."))
      },
      None => {
        Err(String::from("Got message matching nothing."))
      },
    }
  }
}

#[cfg(test)]
mod test {

  use client::Client;

  #[test]
  fn test_messages() {
    let mut client = Client::new("echo");

    match client.handle_reply("make_move") {
      Ok(_) => panic!("Not supposed to accept this message"),
      Err(err) => {},
    };

    match client.handle_reply("make_move e2e4") {
      Ok(input) => assert_eq!(input, "e2e4"),
      Err(err) => panic!("Made a valid move. {}", err),
    };

    match client.handle_reply("bye") {
      Ok(input) => assert_eq!(input, "exit"),
      Err(err) => panic!("Valid good bye message. {}", err),
    };
  }
}