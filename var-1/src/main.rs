fn main() {
    // デフォルトでイミュータブル let mutするとミュータブルな変数を作れる const/var
    let answer: &str = "cram"; // 型注釈は省略化
    println!("How can a clam {} in a clean cream can?", answer);

    // 値の再代入
    let mut x = 1;
    x = 2;
    println!("{}", x); // 2を表示したいが

    // String: ヒープメモリにアロケートされるリサイズ可能なUTF-8のテキストを保持する
    // &str: いわゆるスライス。read-onlyなので、Stringとちがってリサイズはできない
    let this_is_str = "高速爆するとstr";
    let this_is_string = "高速爆するとString".to_string();
    let mut this_is_mut_string = "高速爆するとミュータブルString".to_string();

    println!{"{}", this_is_mut_string}

}
