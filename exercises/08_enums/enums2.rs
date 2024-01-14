// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move {x : i32, y : i32},
    Echo {a : String},
    ChangeColor {r: i32,g: i32,b: i32}, Quit,
    
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo{a: String::from("hello world")},
        Message::ChangeColor{r: 200, g: 255, b: 255},
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
