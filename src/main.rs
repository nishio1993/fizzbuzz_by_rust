fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    /*
    * for a in 1 .. 101で1以上101未満ループ
    * for a in 1 ..=100で1以上100以下ループ
    * for a in (1 .. 101).rev()で逆順
    * for a in b.iter()で配列のループ
    */
    for x in 1 .. 101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}