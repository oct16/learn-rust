// use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main() {
//     // 在堆上存i32值
//     let b = Box::new(5);
//     println!("{}", b);

//     let list = Cons(1,
//         Box::new(Cons(2,
//             Box::new(Cons(3,
//                 Box::new(Cons(4,
//                     Box::new(Cons(5,
//                         Box::new(Nil))))))))));
// }

// fn main() {
//     // let x = 5;
//     // let y = &x;
//     // assert_eq!(x, 5);
//     // assert_eq!(*y, 5);

//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(x, 5);
//     assert_eq!(*y, 5);
// }

// ----------------
// custom smart pointer

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(x, 5);
//     assert_eq!(*y, 5);
// }

// use std::ops::Deref;
// struct MyBox<T>(T);
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
//     let m = Box::new(String::from("Rust"));
//     hello(&m);
// }

// fn hello(name: &str) {
//     println!("hello, {}", name)
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer With Data {}", self.data);
    }
}

fn main() {
    // 丢弃顺序和创建顺序相反
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(d);
    println!("CustomSmartPinter created");
}
