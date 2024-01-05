enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32, 
    yk: i32,
}
struct WriteMessage(message);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        //method body would be defined here
    }
}

fn main() {

}
