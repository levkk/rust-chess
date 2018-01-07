//
extern crate rust_chess;

//
use rust_chess::client::*;
use rust_chess::protocol::*;

// thread
use std::thread;

#[test]
fn tcp_client_and_host_test() {

  //Client
  thread::spawn(|| {
    let mut client = Client::new("tcp://127.0.0.1:54345");

    client.send_message(Message::Hello, &String::from("lev"));
  });

  // This thread will be the host
  let mut host = Client::host("tcp://0.0.0.0:54345");

  // Expect hello from peer
  let hello = host.wait_for_message();

  assert_eq!(hello.0, Message::Hello);
  assert_eq!(hello.1, "lev");
}