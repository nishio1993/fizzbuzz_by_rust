fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    for x in 1 .. 101 {
        let tmp:String; //s:&str = &x.to_string();の場合、sの参照先はmatch式ブロックの中にあり、値が破棄されているためコンパイル不可能
        let s:&str = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            (_, _) => {tmp = x.to_string(); &tmp}   //ブロック外の変数に値を入れることで、参照先がライフタイムを終えていないことを保証する
        };
        println!("{}", s);
    }
}