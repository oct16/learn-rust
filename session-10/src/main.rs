// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![10, 32, 56, 221, 12, 456, 12];

//     let result = largest(&number_list);

//     println!("The largest number is {}", result);

//     let number_list = vec![10, 32, 56, 2221, 12, 456, 12];

//     let result = largest(&number_list);

//     println!("The largest number is {}", result);

//     let char_list = vec!['a', 'b', 'c', '🍉', 'f'];

//     let result = largest_char(&char_list);

//     println!("The largest char is {}", result);
// }

use std::fmt::Display;

fn main() {}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
