extern crate rust_chess;

use rust_chess::connection::*;

use std::thread;

use std::net::TcpListener;
use std::io::{Read, Write};

#[test]
fn test_tcp_connection() {
  // Create simple TCP server.
  let server = match TcpListener::bind("127.0.0.1:54345") {
    Ok(server) => server,
    Err(err) => panic!("Error: {}", err),
  };

  // Start the client on a separate thread.
  let handle = thread::spawn(move || {
    // Tcp Connection
    let mut conn = match TcpConnection::new("127.0.0.1:54345") {
      Ok(conn) => conn,
      Err(err) => panic!("Could not create TcpConnection: {}", err),
    };

    // Send the first message
    let result = conn.send_message(&String::from("make_move e2e4"));

    // Make sure it got sent
    assert_eq!(result, true);

    // Wait for reply
    match conn.wait_for_message() {
      Ok(msg) => assert_eq!(msg, "bye"),
      Err(err) => panic!("Reply error: {}", err),
    };
  });

  // Accept connection from client
  let (mut stream, addr) = server.accept().unwrap();
  println!("Got new conection from: {}", addr);

  // Wait for first message
  let mut buf = [0u8; 512];

  // Read message
  match stream.read(&mut buf) {
    Ok(_) => (),
    Err(err) => panic!("Could not read from stream: {}", err),
  };

  // Make sure it is the message the client sent.
  assert!(String::from_utf8_lossy(&buf).starts_with("make_move e2e4\r\n")); // \r\n is the end of message marker

  // Send back reply
  match stream.write(b"bye\r\n") {
    Ok(_) => (),
    Err(err) => panic!("Could not write: {}", err),
  };

  println!("Wrote reply.");

  handle.join().unwrap();
}