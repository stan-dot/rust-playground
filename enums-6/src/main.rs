enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin{
    Penny, Nickel, Dime, Quarter(UsState),
}

#[derive(Debug)]
enum UsState{
Alabama, Alaska, 
}

fn value_in_cents(coin:Coin)->u8{

  match coin{
      Coin::Penny=>{
        println!("Lucky penny!");  
          1
      },
      Coin::Nickel=>5,
      Coin::Dime=>10,
      Coin::Quarter(state)=>{
        println!("state quarter from {:?}!", state);
          25
      },
  }
}

fn plus_one(x:Option<i32>)->Option<i32>{
match x{
None => None,
Some(i) => Some(i+1),
}

}

struct QuitMessage;
struct MoveMessage{
    x: i32,
    y:i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);


impl Message{
    fn call(&self){

    }
}

enum IpAddr{
    V4(u8,u8, u8, u8),
    V6(String),
}

fn main(){

  let home = IpAddr::V4(127,0,0,1);
  let loopback = IpAddr::V6(String::from("::1"));

  let m = Message::Write(String::from("hello"));
  m.call();

  let some_number = Some(5);
  let some_char = Some('e');
  let absent_number: Option<i32> = None;

  value_in_cents(Coin::Quarter(UsState::Alaska));

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("five, six, none: {:?}, {:?}, {:?}", five, six, none);
 //  let mut count = 0;

}
