use std::io::{self, Write};

/// ユーザーからの入力を取得する
/// 注：入力を受け取って前後の空白を削除した文字列を返す
pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?; // 出力をフラッシュ (即時表示するため)
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // 入力文字列の前後にある空白を削除して返す
}