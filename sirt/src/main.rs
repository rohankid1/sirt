use libsirt::{from_str, from_str_named};
use serde::{Deserialize, Serialize};

const EX_ONE: &'static str = include_str!("../../examples/one.ro");

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct One {
    number: i64,
    string: String,
    is_mc_cool: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Shitpostlands {
    emperor: String,
    emperor_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    age: i64,
    can_fly: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Second {
    more: String,
}

impl Person {
    fn from_examples() -> Self {
        from_str(EX_ONE).unwrap()
    }
}

impl Second {
    fn from_example() -> Self {
        from_str_named(EX_ONE, "Second").unwrap()
    }
}

fn main() {
    let i = r#"
        One {
            number: int(500)
            string: text("hi mc")
            isMcCool: bool(yes)
        }

        Shitpostlands {
            emperor: text("Jirb")
            emperorStatus: bool(false)
        }
"#;

    let a: One = from_str_named(i, "One").unwrap();
    let b: Shitpostlands = from_str_named(i, "Shitpostlands").unwrap();
    let c = Person::from_examples();
    let d = Second::from_example();

    println!("{a:?}");
    println!("{b:?}");
    dbg!(c);
    dbg!(d);
}
