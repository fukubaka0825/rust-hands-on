// forでも関数型的なアプローチでも実行速度が変わらない！！！
fn main() {
    let result = (0..100)
        .map(fizzbuzzfn) //(num|fizzbuzzfn(num))の省略形
        .fold(String::from(""), |acc, line|format!("{}\n{}",acc,line));
    println!{"{}",result};
}


fn fizzbuzzfn(num: i32) -> String{
    // アーリーリターンの場合はreturnをかく
    return 1;
    // セミコロンを書かないとreturn扱いになります
    if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    }
}