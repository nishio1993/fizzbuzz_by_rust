fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    for x in 1 .. 101 {
        let s:String = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz".to_string(),   //文字列リテラルは「文字列スライス」という型
            (0, _) => "Fizz".to_string(),       //静的に領域が確保されるため、Stringとは違う型になる
            (_, 0) => "Buzz".to_string(),       //Stringは可変長文字列である
            (_, _) => x.to_string()
        };  //式なので「;」が必要
        println!("{}", s);
    }
}