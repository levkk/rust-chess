//
use std::net;
use std::io;
use std::fmt;

// Regex
extern crate regex;
use regex::RegexSet;

// Game
use game::Game;
use board::Color;

enum Message {
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
struct Client {
  game: Game,
  server: String,
  connected: bool,
  in_game: bool,
  color: Color,
}

impl Client {

  /// Create a new client for multiplayer.
  ///
  /// Parameters:
  /// `game`: Game (will take ownership)
  pub fn new(game: Game, server: &str) -> Self {
    Client{
      game,
      server: String::from(server),
      connected: false,
      in_game: false,
      color: Color::Nil,
    }
  }

  ///
  fn connect(&mut self) -> Result<net::TcpStream, io::Error> {
    match net::TcpStream::connect(&self.server) {
      Ok(stream) => {
        self.connected = true;

        Ok(stream)
      },

      Err(err) => {
        println!("Could not connect to {}: {}", self.server, err);

        Err(err)
      },
    }
  }

  ///
  // fn send_message(&self, stream: &mut net::TcpStream, message: Message, payload: &str) -> io::Result<usize>  {
  //   match message {
  //     Message::Bye => {
  //       stream.write(&format!("{}", Message::Bye).as_bytes())
  //     },

  //     Message::MakeMove => {
  //       stream.write(&format!("{} {}", Message::MakeMove, payload).as_bytes())
  //     },
  //   }
  // }

  ///
  fn handle_reply(&mut self, message: &str) -> Result<(), String> {

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
        Ok(())
      },
      Some(&1) => {
        match self.game.make_move(&message[10..]) { // The text after "make_move "
          Ok(_) => Ok(()),
          Err(err) => Err(String::from(err)),
        }
      },
      Some(&_) => {
        Err(String::from("Got unhandled matched message."))
      },
      None => {
        Err(String::from("Got message matching nothing."))
      },
    }
  }

  ///
  pub fn play(&mut self) {
    if !self.connected {
      let _stream = self.connect().unwrap();
    }
  }
}

#[cfg(test)]
mod test {

  use client::Client;
  use game::Game;

  #[test]
  fn test_messages() {

    let mut client = Client::new(Game::new(), "test");

    match client.handle_reply("make_move") {
      Ok(_) => panic!("Not supposed to accept this message"),
      Err(err) => {},
    };

    match client.handle_reply("make_move e2e4") {
      Ok(_) => {},
      Err(err) => panic!("Made a valid move. {}", err),
    };

    match client.handle_reply("bye") {
      Ok(_) => {},
      Err(err) => panic!("Valid good bye message. {}", err),
    };
  }
}