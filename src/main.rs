mod qcli;
use qcli::Serv;
fn main() {
  let aoz = cli!{Serv::Aoz where &["veed", "c"]}; 
  let out = aoz.go().unwrap_or(String::from("FATAL ERROR!"));
  println!("{}", out);
}