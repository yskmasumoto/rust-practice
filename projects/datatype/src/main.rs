use std::io;

fn main() {
    // addition
    // 足し算
    let sum: i8 = 5 + 10;

    println!("sum is {}.", sum);

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;

    println!("difference is {}.", difference);

    // multiplication
    // 掛け算
    let product = 4 * 30;

    println!("product is {}.", product);

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
                         // 結果は0

    println!("quotient is {}.", quotient);
    println!("floored is {}.", floored);

    // remainder
    // 余り
    let remainder = 43 % 5;

    println!("remainder is {}.", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation
                         // 明示的型注釈付きで
    
    println!("t is {}.", t);
    println!("f is {}.", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫

    println!("c is {}.", c);
    println!("z is {}.", z);
    println!("heart_eyed_cat is {}.", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
    println!("The value of z is: {}.", z);

    let w: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = w.0;

    let six_point_four = w.1;

    let one = w.2;

    println!("The value of five_hundred is: {}.", five_hundred);
    println!("The value of six_point_four is: {}.", six_point_four);
    println!("The value of one is: {}.", one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _b = [3; 5];

    let first = a[0];
    let second = a[1];

    println!("The first index of a is: {}.", first);
    println!("The second index of a is: {}.", second);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number"); // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );
}
