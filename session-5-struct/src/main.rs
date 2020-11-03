// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of then rectangle is {} square pixels.",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//----------------

// use tuple
// fn main() {
//     let react1 = (30, 50);

//     println!(
//         "The area of then rectangle is {} square pixels.",
//         area(react1)
//     )
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//----------------

// #[derive(Debug)]
// struct Reactangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let react1 = Reactangle {
//         width: 30,
//         height: 50,
//     };
//     println!("react1 is {:#?}", react1)
// }

//----------------

// #[derive(Debug)]
// struct Reactangle {
//     width: u32,
//     height: u32,
// }

// impl Reactangle {
//     fn area(&self) -> u32 {
//         &self.width * &self.height
//     }
// }

// fn main() {
//     let react1 = Reactangle {
//         width: 30,
//         height: 50,
//     };

//     println!("Reactangle is {:?}", react1.area())
// }

//----------------

// #[derive(Debug)]
// struct Reactangle {
//     width: u32,
//     height: u32,
// }

// impl Reactangle {
//     fn can_hold(&self, other: &Reactangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let react1 = Reactangle {
//         width: 30,
//         height: 50,
//     };
//     let react2 = Reactangle {
//         width: 10,
//         height: 40,
//     };
//     let react3 = Reactangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can react1 hold react2? {}", react1.can_hold(&react2));
//     println!("Can react1 hold react3? {}", react1.can_hold(&react3));
// }

//----------------

#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn square(size: u32) -> Reactangle {
        Reactangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Reactangle::square: {:?}", Reactangle::square(3))
}
