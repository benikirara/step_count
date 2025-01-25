use regex::Regex;
use std::{
    fs::{self, File},
    io::{self, BufRead, Write},
    path::Path,
};

/// 変更行を整形する
pub fn filter_lines(original_path: &Path, temp_file_path: &Path) -> io::Result<()> {
    // 元ファイルを開く
    let original_file = File::open(original_path)?;
    // バッファ付きリーダーを作成
    let reader = io::BufReader::new(original_file);

    // 一時ファイルを作成
    let temp_file = File::create(temp_file_path)?;
    // 一時ファイルへの書き込みを行う変数
    let mut writer = temp_file;

    // コメント行を検出する正規表現
    let comment_regex = Regex::new(r"^//").unwrap();
    // 波括弧のみの行を検出する正規表現
    let bracket_regex = Regex::new(r"^(\{|\}|};)$").unwrap();

    for line_result in reader.lines() {
        // 行を取得
        let line = line_result?;

        // "+" で始まる行（追加された行）を処理 (ただし、"+++" で始まる行は除く)
        if line.starts_with("+") && !line.starts_with("+++") {
            // 先頭の "+" を削除し、前後の空白を削除
            let actual_line = line[1..].trim();

            // 空行、コメント行、波括弧のみの行でない場合、一時ファイルに書き込む
            if !actual_line.is_empty()
                && !comment_regex.is_match(actual_line)
                && !bracket_regex.is_match(actual_line)
            {
                writeln!(writer, "{}", actual_line)?;
            }
        }
    }
    Ok(())
}

/// ディレクトリ内のファイルの行数を合計する
pub fn count_lines_in_directory(count_dir_path: &Path) -> Result<usize, std::io::Error> {
    let mut total_lines = 0;

    for entry in fs::read_dir(count_dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            // ファイルの行数を合計に加算
            total_lines += reader.lines().count();
        }
    }
    Ok(total_lines)
}
