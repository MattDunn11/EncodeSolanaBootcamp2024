// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!



#[derive(Debug)]
enum Message {
    Move(i32, i32),
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
      // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move(10, 30),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
