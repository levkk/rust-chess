
/// Connection interface
pub trait Connection {
  fn send_message(&self, message: &str) -> bool;
  fn wait_for_message(&self) -> Result<String, String>;
  fn get_message(&self) -> Result<String, String>;
}

pub struct EchoConnection {

}

impl EchoConnection {
  ///
  pub fn new() -> Self {
    EchoConnection{}
  }
}

impl Connection for EchoConnection {
  ///
  fn send_message(&self, _message: &str) -> bool {
    true
  }

  ///
  fn wait_for_message(&self) -> Result<String, String> {
    Ok(String::from("make_move e7e5"))
  }

  ///
  fn get_message(&self) -> Result<String, String> {
    Ok(String::from("Nothing"))
  }
}