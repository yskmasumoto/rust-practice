fn main() {
    println!("Hello, world!");

    another_function();

    other_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let y2 = 3;
        y2 + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(5);
    println!("The value of z is: {}", z);
}

fn another_function() {
    println!("Another function.");  // 別の関数
}

fn other_function(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}