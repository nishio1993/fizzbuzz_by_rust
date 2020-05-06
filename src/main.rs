fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    let mut x:i32 = 0;

    while x <= 100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }

        x += 1;
    }
}