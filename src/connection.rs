
// Networking
use std::net;
use std::io::{Read, Write};

// String
use std::str;

//Display
use std::fmt;

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
      if buffer.len() == 0 {
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

#[cfg(test)]
mod tests {

}