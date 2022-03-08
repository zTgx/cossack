use cossack::status::{Status, Code, code, strings};

fn main() {
    let s = Status::new(Code::KOK);
    let c = code(s.code);
    println!("status : {}", c);
    println!("status : {}", strings(c));
}