pub mod aggregator {

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }

        fn summarize_author(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    }
    //
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    // T: Display + Clone,
    // U: Clone + Debug,
    // {
    // unimplemented!()
    // }
    //
    // todo go back in chapter 17
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}:{}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    fn lifetimes() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }


    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            return x;
        }
        y
    }
}
