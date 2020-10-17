mod qcli;
use qcli::Serv;
fn main() {
  let aoz = cli![Serv::Aoz where {program: "veed", lang: "c"}]; 
  let out = aoz.go();
  println!("{}, {}", aoz, out.unwrap());
}