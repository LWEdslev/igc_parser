use regex::{Regex};
use igc_parser::records;

fn main() {
    let b = "B0941395152202N00032723WA0011400150";
    println!("{:?}", records::Record::parse(b).unwrap());
}
