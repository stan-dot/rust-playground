use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use proc_macro;
use std::fmt;
use std::ops::Add;
use std::slice;

// use advanced::Human;
// unsafe allows
// dereference a raw pointer
// access or modify a mutable static variable
// implement unsafe trait
// access fields of `union`
// on a block by block basis - should keep small
// enclose in safe abstraction, give safe api

// no lifetime annotation here
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");

    // raw pointers
    // no cleanup, can be null, can beinvalid, can ignore borrowing
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("Name is {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // let person = Human;
    // Pilot::fly(&person); // fully qualified syntax
    // Wizard::fly(&person); // fully qualified syntax
    // person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // <Type as Trait>function(receiver_if_method, next_arg, ...);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    //ADVANCED TYPES
    let x: i32 = 5;
    let y: Kilometers = 5;
    // no typechecking benefit
    println!("x + y = {}", x + y);

    // higher order functions
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // these two are equivalent
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // macros code

    Pancakes::hello_macro();
}

unsafe fn dangerous() {}

// we know this does not overlap, but Rust doesn't
// this function is a safe abstraction over an unsafe action
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    // this assert is key, it makes this code safe = prevents memory leak
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
    // (&mut values[..mid], &mut values[mid..])
}

// this C defines abi - calling at assembly level.
// FFI = Foreign Function Interface
extern "C" {
    fn abs(input: i32) -> i32;
}

// also external for running from a different language
// mangling makes function name less human readable

// here it's safe
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust functino from C!");
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

// overloading operators

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// definitino of the add trait, rhs = right hand side
// trait Add<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// ADVANCED TYPES
// that is not a newtype, but a synomy

type Kilometers = i32;

// alias example used in the std library
// type Result<T> = std::result::Result<T, std::io::Error>;

// never type - will never return = diverging fnuction
// fn bar() -> ! {}
// continue, panic, and loop also have this

// dynamically sized types = unsized types

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// macros are metaprogramming

#[macro_export]
macro_rules! vec{
    // first arm; the only arm as it's a simple macro
    // $ is a variable
    ( $( $x:expr), *)=>{

        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);

            )* // zero or more
            temp_vec
        }
    }
}

fn macro_test() {
    // prinln!, vec! = variable number of parameters
    // harder to maintain
    // let v: Vec<u32> = vec![1,2,3];
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    // ad, that will be generate
}

// procedural macros  = not matching as in declarative, just process
// 3 kinds, custom derive, attribute-like, function-like
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}

#[derive(HelloMacro)]
struct Pancakes;
