# Sirt
Sirt is a basic data format that is designed to be dead simple
to use. At a high level, a Sirt format is made up of "Blocks",
and each block is made up of basic data types seen in most
programming languages. In Sirt, these types are known as
**Values**.

# Libsirt
Libsirt is the core library that, under the hood, parses the contents
of the file or input into Blocks. The most crucial part is that it 
implements Serde's Serialise and Deserialise, allowing users of this
library to construct a data structure that represents the defined Sirt format,
and also represent the data structure into text.

# Example
An example of how a Sirt format is structured can be seen below:

```toml
App {
    title: text("Sirt")
    useNativeTitleBar: bool(true)
    features: list(text("featureOne"), text("featureTwo"), text("featureThree"))
}
```

in application code:
```rs
use serde::{Deserialize, Serialize};
use libsirt::{types::*, from_str};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct App {
    title: Text, // Text is just an alias to String
    use_native_title_bar: Bool, // Bool is just an alias to bool
    features: List<Text> // List is just an alias to Vec
}

let file = "..."; // loaded from file
let app: Result<App, _> = from_str::<App>(file);
let app = app.unwrap();

assert_eq!(app.title, "Sirt".to_string());
assert_eq!(app.use_native_title_bar, true);
assert_eq!(app.features, vec!["featureOne".to_string(), "featureTwo".to_string(), "featureThree".to_string()]);
```
