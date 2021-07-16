fn main() {
    let num = 10;
    let fizzbuzz = if num % 15 == 0 {
        "FizzBuzz".to_string()
    } else if num % 3 == 0 {
        "Fizz".to_string()
    } else if num % 5 == 0 {
        "Buzz".to_string()
    } else {
        num.to_string()
    };
    println!("{}",fizzbuzz);
    // for式を使って0~99の数字を使ってイテレートする iteraterが実装されて型であればforで回せる
    for num in 0..100 {
        let fizzbuzz = if num % 15 == 0 {
            "FizzBuzz".to_string()
        } else if num % 3 == 0 {
            "Fizz".to_string()
        } else if num % 5 == 0 {
            "Buzz".to_string()
        } else {
            num.to_string()
        };
        println!("{} {}",fizzbuzz,fizzbuzzfn(num));
    }
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

// Unit = void
fn void(){
    let num = 10;
    num.to_string();
}