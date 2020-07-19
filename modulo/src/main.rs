#[macro_use]
extern crate lazy_static;

use modulo_sys::form::types::*;

mod parser;
mod generator;

fn main() {
    let layout = r#"
Hey {{name}},
{{message}}

Looking forward to hearing from you!

By the way, have you checked out {{website}}?

Cheers :)
    "#;

    let structure = parser::layout::parse_layout(layout);
    let form = generator::generate(structure);

    modulo_sys::form::show(form)
}
