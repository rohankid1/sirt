use libsirt::{Block, Value};

#[derive(Debug, Clone)]
pub struct BlockItem {
    pub block: Block,
    pub description: Option<String>,
    pub syntax: Option<String>,
}

impl BlockItem {
    fn gen_desc(&mut self) {
        if self.description.is_none() {
            self.description = Some(format!(
                "// RUST REPRESENTATION\n\n{}",
                repr_block(&self.block),
            ));
        }
    }

    fn gen_syn(&mut self) {
        if self.syntax.is_none() {
            self.syntax = Some(format!(
                "### SYNTAX BREAKDOWN ###\n\n{}",
                explain_syntax(&self.block)
            ));
        }
    }

    pub fn item_description(&mut self) -> &str {
        self.gen_desc();
        self.description.as_ref().unwrap()
    }

    pub fn item_syntax(&mut self) -> &str {
        self.gen_syn();
        self.syntax.as_ref().unwrap()
    }
}

pub fn infer_type_str(ty: &Value) -> String {
    match ty {
        Value::Bool(_) => "bool".into(),
        Value::Int(_) => "i64".into(),
        Value::Text(_) => "String".into(),
        Value::List(l) => {
            let mut output = String::from("Vec<");
            output.push_str(
                &l.first()
                    .map(infer_type_str)
                    .unwrap_or("<Unknown>".to_string()),
            );
            output.push('>');
            output
        }
    }
}

pub fn repr_block(block: &Block) -> String {
    let mut output = String::new();

    output.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    output.push_str(&format!("struct {} {{\n", block.get_name()));
    for (name, value) in block.get_fields() {
        let value = infer_type_str(value);
        output.push_str(&format!("    {name}: {value}"));
        output.push('\n');
    }
    output.push('}');

    output
}

fn list_depth(value: Option<&Value>, num: usize) -> usize {
    match value {
        Some(Value::List(l)) => list_depth(l.first(), num + 1),
        _ => num,
    }
}

pub fn explain_syntax(block: &Block) -> String {
    let mut output = String::new();

    let name_len = block.get_name().len();
    output.push_str(&format!("{}\n", block.get_name()));
    output.push_str(&"^".repeat(name_len));
    output.push_str(" type: identifier\n\n");
    output.push_str("FIELDS:\n");

    for (name, value) in block.get_fields().iter() {
        let inferred = infer_type_str(value);
        output.push_str(&format!("{name} -> {inferred}\n"));
        output.push_str("----------\n");
        output.push_str(&format!("depth: {}\n", list_depth(Some(value), 1)));
        output.push_str(&format!("{name} -> identifier\n{inferred} -> type\n"));
        if inferred.contains("<Unknown>") {
            output.push_str("note: cannot infer type of list,\nas there are no items inside it");
        }
        output.push_str("\n\n\n");
    }

    output
}
