use libsirt::{from_str, from_str_named, from_str_named_iter};
use serde::Deserialize;

#[test]
fn test_from_str() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Point {
        x: i64,
        y: i64,
    }

    let i = r#"Point { x: int(10) y: int(5) }"#;
    let expected = Point { x: 10, y: 5 };

    assert_eq!(expected, from_str(i).unwrap());
}

#[test]
fn test_from_str_named() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Window {
        fullscreen: bool,
        title: String,
    }

    // bool can be "true", "false", "yes", or "no"
    let i = r#"Other { color: text("red") }  Window { fullscreen: bool(no) title: text("Sirt Application") }"#;
    let expected = Window {
        fullscreen: false,
        title: "Sirt Application".to_string(),
    };

    assert_eq!(expected, from_str_named(i, "Window").unwrap());
}

#[test]
fn test_from_str_named_iter() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Config {
        name: String,
        value: bool,
    }

    let i = r#"
    Person {}
    ABC {}

    Config {
        name: text("enableBlur")
        value: bool(true)
    }

    Config {
        name: text("enableLogging")
        value: bool(false)
    }

    Config {
        name: text("compactMode")
        value: bool(yes)
    }
    "#;

    let blur = Config {
        name: "enableBlur".to_string(),
        value: true,
    };
    let logging = Config {
        name: "enableLogging".to_string(),
        value: false,
    };
    let compact = Config {
        name: "compactMode".to_string(),
        value: true,
    };

    let configs: Result<Vec<Config>, _> = from_str_named_iter::<Config>(i, "Config")
        .unwrap()
        .collect();
    let mut iter = configs.unwrap().into_iter();

    assert_eq!(iter.next(), Some(blur));
    assert_eq!(iter.next(), Some(logging));
    assert_eq!(iter.next(), Some(compact));
}
