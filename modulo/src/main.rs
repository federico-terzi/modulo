#[macro_use] extern crate lazy_static;

mod parser;
mod config;
mod generator;

fn main() {
    let data = r#"
    layout: |
      Hey {{name}},
      {{message}}

      Looking forward to hearing from you!

      {{end}}

      Cheers :)
    fields:
      name:
        default: "John"
      message:
        multiline: true
      end:
        type: choice
        values:
          - "Looking forward to hearing from you"
          - "Let me know what you think"
          - "Let me know if that helps"
          - "Thanks for the help!"
    "#;

    let config: config::FormConfig = serde_yaml::from_str(data).unwrap(); // TODO: remove unwrap
    let form = generator::generate(config);
    let values = modulo_sys::form::show(form);

    let output = serde_json::to_string(&values).expect("unable to encode values as JSON");
    println!("{}", output);
}
