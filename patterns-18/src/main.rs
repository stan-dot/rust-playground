fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // with *match* compiler checks for exhaustiveness
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // for loop
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // these are also patterns
    let x0 = 5;
    // number of items on the left and right needs to match
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    // cannot assign a certain (irrefutable) value to if, only to if let

    // if let Some(x) = some_option_value{
    //     println!("{}", x);
    // }

    if let x = 5 {
        println!("{}", x);
    }

    let x2 = 0;

    match x2 {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // shadowing y3
    let x3 = Some(5);
    let y3 = 10;
    match x3 {
        Some(50) => println!("Got 50"),
        Some(y3) => println!("Matched, y = {y3}"),
        _ => println!("Defautl case, x = {:?}", x3),
    }
    println!("at the end, x={:?}, y = {y3}", x3);
    // it is second arm that is executed
    // let x4 = 1;
    // match x4 {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     // that was unstable at a time
    //     4..6 => println!("from four to six"),
    //     _ => println!("anything"),
    // }
    // ranges only allowed with numeric and `char` values
    let x5 = 'c';
    match x5 {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("Something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y: 0 } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis,({x}, {y})"),
        // this lacks 0,0 case
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, b, g) => {
            println!("Change the color o red {r}, green {g}, and blue {b}");
        }
        Message::DifferentColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}");
        }
        Message::DifferentColor(Color::Rgb(r, b, g)) => {
            println!("Change the color o red {r}, green {g}, and blue {b}");
        }
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);
    // this will set value only if of them is None

    // numbers
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
        // obviously that branch doesn't happen, unless there's less than 5 elements
        (first, .., last) => {
            println!("some numbers: {first}, {last}");
        }
    }

    // unused variable
    let _x6 = 5;

    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => {
            println!("x is {}", x)
        }
    }

    // using match guard
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x7 = 4;
    let y7 = false;
    match x7 {
        // here the check is if ANY of the numbers, not just 6
        4 | 5 | 6 if y => {
            println!("yes")
        }
        _ => println!("no"),
    }

    let msg = ShortMessage::Hello{id:5};
    match msg {
        ShortMessage::Hello{
            id: id_variable @3..=7,
        } => println!("Found an id in rnage: {}", id_variable),
        ShortMessage::Hello{
            id: @10..=12,
        } => println!("Found an id in another rnage"),
        ShortMessage::Hello{id} => println!("Found some other id: {}", id),
    }
}

enum ShortMessage {
    Hello { id: i32 },
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    DifferentColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
