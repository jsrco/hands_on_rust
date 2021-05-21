use std::io::stdin;

#[derive(Debug)]
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
    let mut visitor_list = vec![
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?"),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = get_str_in();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
