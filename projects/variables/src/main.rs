fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("spaces len is {}.", spaces);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
