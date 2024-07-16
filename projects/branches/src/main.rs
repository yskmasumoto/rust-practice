fn main() {
    // let number = 3;
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);


    if number < 5 {
        println!("condition was true");       // 条件は真でした
    } else {
        println!("condition was false");      // 条件は偽でした
    }

    if number != 0 {
        println!("number was something other than zero");   // 数値は0以外の何かです
    }

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    let mut countdown = 3;
    while countdown != 0 {
        println!("{}!", countdown);

        countdown -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index += 1;
    }

    
    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}