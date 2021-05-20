use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}
impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}
fn get_str_in() -> String {
    let mut user_in = String::new();
    stdin()
        .read_line(&mut user_in)
        .expect("Failed to read line");
    user_in.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = get_str_in();
    let mut allow_them_in = false;
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}
