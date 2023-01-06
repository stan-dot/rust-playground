use generics::aggregator::{longest, Summary, Tweet};

struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let largest = get_largest(number_list);
    // println!("The largest number is {}", largest);
    // let p = Point { x: 5, y: 10 };

    // println!("p.x = {}", p.x());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// fn get_largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for number in &list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// // monomorphization - compiler does to speed up

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn largest<T>(list:&[T])-> &T{
//     let mut largest = &list[0];
//     for item in &list {
//         if item > largest {
//             largest = item ;
//         }
//     }
//     largest

// }
