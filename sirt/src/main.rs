use libsirt::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LongBlock {
    one: Int,
    two: Bool,
    three: String,
    four: Bool,
    five: Bool,
}

fn main() {
    let i = r#"
        LongBlock {
            one: int(1)
            two: bool(true)
            three: text("hello")
            four: bool(yes)
            five: bool(no)
        }
"#;

    let a: Result<LongBlock, _> = libsirt::from_str(i);
    let m = a.unwrap();
    dbg!(&m);
    println!("{}", libsirt::to_pretty_string("LongBlock", &m).unwrap());
}
