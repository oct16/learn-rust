
// #[derive(Debug)]
// enum IpAddKind {
//     V4, V6
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddKind,
//     address: String
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddKind::V4,
//         address: String::from("127.0.0.1")
//     };

//     let looppack = IpAddr {
//         kind: IpAddKind::V6,
//         address: String::from("::1")
//     };

//     println!("Address V4 is {:?}, V6 is {:?}", home, looppack)

// }

// --------------

// match 
// #[derive(Debug)]
// #[allow(dead_code)]
// enum UseState {
//     Alabama,
//     Alaska,
// }

// #[allow(dead_code)]
// enum Coin {
//     Penny, 
//     Nickel,
//     Dime,
//     Quarter(UseState),
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

// fn main() {
//     println!("{}", value_in_cents(Coin::Quarter(UseState::Alabama)))
// }

// --------------

//Options

fn plus_one(x: Option<i32>) ->Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main (){ 
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}", six, none)
}