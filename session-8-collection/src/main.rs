use std::collections::HashMap;

// fn main() {
//     // let mut vec: Vec<i32> = Vec::new();
//     let mut vec = vec![1, 2, 3, 4];
//     vec.push(5);
//     println!("{:?}", vec);

//     // get element

//     let third: &i32 = &vec[2];
//     println!("{:?}", third);

//     match vec.get(2) {
//         Some(third) => println!("{:?}", third),
//         None => println!("There is none third element"),
//     }

//     for i in &mut vec {
//         println!("{}", i);
//         *i += 50;
//     }

//     println!("{:?}", vec);

//     #[derive(Debug)]
//     enum SpreadSheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadSheetCell::Int(3),
//         SpreadSheetCell::Float(3.14),
//         SpreadSheetCell::Text(String::from("3.14159")),
//     ];

//     println!("{:?}", row)
// }

fn main() {
    // let data = "Init String";
    // let str = String::from(data);
    // let s = data.to_string();
    // println!("{}", data);
    // println!("{}", s);
    // println!("{}", str);

    // let mut s = String::from("foo");
    // s.push_str(" bar");
    // println!("{}", s);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);
    // let ss = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", ss);

    //----------------
    // hashMap

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Red"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // println!("{:?}", &score);

    // scores.entry(team_name).or_insert(100);

    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
