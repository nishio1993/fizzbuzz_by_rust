fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    for x in 1 .. 101 {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"), // 「|」で区切ると複数指定
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x)  //「_」でワイルドカード
        }
    }
}