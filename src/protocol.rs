// Display
use std::fmt;

/// Message headers
#[derive(Debug, PartialEq)]
pub enum Message {
  Hello,
  Bye,
  BadMessage,
  MakeMove,
}

impl fmt::Display for Message {
  //
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = match *self {
      Message::Hello => write!(f, "hello"),
      Message::Bye => write!(f, "bye"),
      Message::BadMessage => write!(f, "bad_msg"),
      Message::MakeMove => write!(f, "make_move"),
    };

    Ok(())
  }
}

/// Message Regex to match them when they arrive
/// over the pipe.
pub enum MessageRegex {
  Hello,
  Bye,
  BadMessage,
  MakeMove,
}

impl fmt::Display for MessageRegex {
  //
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let _ = match *self {
      MessageRegex::Hello => write!(f, r"{} [A-Za-z0-9]+", Message::Hello),
      MessageRegex::Bye => write!(f, r"{}$", Message::Bye),
      MessageRegex::BadMessage => write!(f, r"{}$", Message::BadMessage),
      MessageRegex::MakeMove => write!(f, r"{} [A-Ha-h][1-8][A-Ha-h][1-8]$", Message::MakeMove),
    };

    Ok(())
  }
}