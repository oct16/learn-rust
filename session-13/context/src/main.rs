// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;
//     // fn equal_to_x(z: i32) -> bool {z == x};
//     // can't capture dynamic environment in a fn item;

//     let y = 4;

//     assert!(equal_to_x(y));
// }


// FnOnce 闭包可以从所处的环境获取变量
// FnMut 可以从环境中可变地借用值并对他们进行修改
// Fn 可以从环境中不可变地借用值

// 闭包强制获取所有权，在参数列表前添加 move 关键字

fn main() {

    let x= vec![1,2,3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);
    // borrow of moved value: `x`
    // value borrowed here after moverustc(E0382)

    let y = vec![1,2,3];

    assert!(equal_to_x(y));

}