extern crate rust_apex;
extern crate serde_json;

use serde_json::Value;

fn main() {
    rust_apex::handle_func(|input: Value, _| Ok(input));
}
