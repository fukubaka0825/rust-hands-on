use std::env::args;
use std::fs::read_to_string;


struct GrepArgs {
    pattern: String,
    path: String,
}

// 構造体に対する実装を定義するimpl
impl GrepArgs {
    fn new(pattern: String,path: String) -> GrepArgs {
        GrepArgs {pattern,path}
    }
}



fn grep(content: String,pattern:String){
    for line in content.lines(){
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

//すねいくケース
fn run(state: GrepArgs) {
    // Resultをパターンマッチにかけている
    match read_to_string(state.path) {
        Ok(content) => grep(content,state.pattern),
        Err(reason) => println!("{}",reason),
    }
}

fn main() {
    // //
    // match args().nth(1) {
    //     // Some,NoneはOption型で、値がない可能性があることを示します
    //     Some(path) => run_cat(path),
    //     None => println!("No path is specified!")
    // }
    // // ある値の時にだけ処理がしたいif let　unwrapで問答無用で処理も可能
    // if let Some(path) = args().nth(1) {
    //     run(path)
    // }
    let pattern = std::env::args().nth(1);
    let path = std::env::args().nth(2);

    // タプルは組み合わせみたいなもの、OptionのStringとOptionのStringで両方ある場合runを走らせる
    match(pattern, path) {
        (Some(pattern), Some(path)) => run(GrepArgs::new(pattern,path)),
         _ => println!("pattern or path is not specified!"),
    }
}

// ?キーワード -> 常にパターンマッチングが必要ではなく、？でエラーを伝播して後続にエラーハンドリングを任せられる

