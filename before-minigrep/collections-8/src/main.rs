use std::collections::HashMap;


enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![100, 32, 57];
    v.push(5);
    let third:&i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third{
        Some(third)=> println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    for i in &mut v{
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    let mut s = String::from("foo");
    let s2= "bar";
    s.push_str(s2);
    
    println!("s2 is {s2}");
    let joint = format!("{s}-{s2}");
    println!("joint string: {}", joint);

    for c in joint.chars(){
        println!("{c}");
    }

    for b in joint.bytes(){
    println!("{b}");
    }


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    for (key, value) in &scores{
        println!("{key}: {value}");
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let text = "hello world wonderful world";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace(){
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_map);

}
