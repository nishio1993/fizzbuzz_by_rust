fn main() {
    fizzbuzz();
}

fn fizzbuzz() {
    let fizzbuzz = |x: i32| -> String {     //クロージャの文法は|変数:型| -> 返り値型 {}
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),  //format!の返り値はstd::string::String
            (0, _) => format!("Fizz"),      //format!無しの文字列リテラルは&str
            (_, 0) => format!("Buzz"),
            (_, _) => x.to_string()
        }
    };  //クロージャ定義は関数定義と違い、末尾に「;」が必要
    let println = |x| { //クロージャの変数、返り値は型省略可能
        println!("{}", x);
    };

    (1 ..= 100).map(fizzbuzz).for_each(println);
}