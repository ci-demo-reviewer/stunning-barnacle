#![allow(dead_code)]

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read from stdin");

    line.trim().into()
}

fn main() {
    let name = read_line();
    let time = read_line().parse().unwrap();

    let greeter = Greeter::new(name);

    println!("{}", greeter.get_greeting(time));
}

struct Greeter {
    name: String,
}

impl Greeter {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_greeting(&self, hour: u8) -> String {
        if hour > 4 && hour < 11 {
            format!("Good morning {}!", self.name)
        } else if hour < 17 {
            todo!("Implement")
        } else {
            todo!("Implement")
        }
    }
}

#[test]
fn test_morning_greeting() {
    let greeter = Greeter::new("Test".into());
    assert!(greeter.get_greeting(6).contains("morning"));
}
