#[derive(Debug)]
enum Message {
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

#[derive(Debug)]
enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

#[derive(Debug)]
enum Money {
    PEN,
    USD,
    EUR
}

fn main() {

    let x: Message = Message::Move { x: 3, y: 4 };

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    let color: Message = Message::ChangeColor(255,233,100);

    let text: Message = Message::Write("Hello".to_string());

    println!("Text {:?}", text);

    println!("Color {:?}", color);

    println!("Moneda {:?}", Money::PEN);

}
