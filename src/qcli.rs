extern crate reqwest;
use std::fmt;
use reqwest::blocking as http;

#[derive(fmt::Debug)]
pub enum Serv<'a> {
  Aoz { program: &'a str, lang: &'a str }
}

impl fmt::Display for Serv<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl Serv<'_> {
  fn url(&self) -> String {
    match self {
      Serv::Aoz(data) => {
        let mut prog = "";
        if data.len() != 1 {
          prog = &data[1];
        }
        format!("http://all-of-zem.firefish.repl.co/{}/{}", data[0], prog)
      }
    }
  }
  
  pub fn go(&self) -> Result<String, reqwest::Error> { 
    let res = http::get(&self.url())?
      .text();

    res
  }
}

#[macro_export]
macro_rules! cli {
  ( $serv:path where $opt:expr ) => {
    {
      let srv = $serv $opt;
      println!("sent a request to {}", srv);
      srv
    }
  };
}
