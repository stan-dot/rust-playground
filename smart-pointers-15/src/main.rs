use crate::List::{Cons, Nil};
use crate::ReferenceCountList::{Cons2, Nil2};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref behaviour

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // using Box<T> like a reference
    let x1 = 5;
    let y1 = Box::new(x);
    assert_eq!(5, x1);
    assert_eq!(5, *y1);

    // custom MyBox

    let x2 = 5;
    let y2 = MyBox::new(x);
    assert_eq!(5, x2);
    assert_eq!(5, *y2);

    // MyBox with dereference
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let m2 = MyBox::new(String::from("Rust2"));
    hello(&(*m)[..]); // using standard library Deref on string, if Rust didn't have deref coertion.

    // drop (like componentOnUnload)
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("Other stuff"),
    };
    println!("CustomSmartPointers created.");

    // force dropping
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created");
    // c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");

    // Reference counting with error
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a)); // here it complains that it's already used
    // let c = Cons(4, Box::new(a));

    // reference counting
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // could do cloning, but here there is not a deep copy


    // refCell when the compiler is overly conservative and thinks it won't compile

    // interior mutaiblity

    let x3 = 5;
    let y3 = &mut x3;


}

fn hello(name: &str) {
    println!("Hello, {name}");
}

// recursive types need the Box. such as cons list

// pseudocode (1, (2, (3, Nil)))
// most often Vec<T> is better

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum ReferenceCountList {
    Cons2(i32, Rc<ReferenceCountList>),
    Nil2,
}
