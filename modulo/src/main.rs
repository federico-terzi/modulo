use modulo_sys::form::types::*;

fn main() {
    println!("Hello, world!");
    modulo_sys::form::show(Form {
        title: "test title".to_owned(),
        fields: vec![
            Field::row(vec![
                Field::label("test label"),
                Field::text("name", ""),
            ]),
            Field::label("third label"),
            Field::text("greetings", "Best Regards"),
        ],
    })
}
