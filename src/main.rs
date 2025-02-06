mod user_request;
mod git_diff;
mod file_filter;
mod line_counter;
mod config;

use std::error::Error;
use std::env;
use std::fs;

use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    // ユーザーからの入力を取得
    let user_request = user_request::UserRequest::from_user_input()?;

    // config.jsonを読み込む (実行時のカレントディレクトリに配置されている想定)
    let config = Config::from_file_or_exit("config.json");

    // Gitリポジトリのディレクトリに移動
    env::set_current_dir(&user_request.source_path)?;

    // 変更されたファイルのリストを取得
    let changed_files = git_diff::get_changed_files(&user_request.git_rev, &user_request.author_name)?;

    // ユーザーの一時ディレクトリにCountTempFileを作成
    let mut output_file_path = env::temp_dir();
    output_file_path.push("CountTempFile");
    
    if !output_file_path.exists() {
        fs::create_dir_all(&output_file_path)?;
    }

    // 各変更ファイルに対して追加された行を取得・保存
    for file in changed_files.lines() {
        let added_lines = git_diff::get_added_lines(&user_request.git_rev, file)?;
        file_filter::filter_and_save(output_file_path.to_str().unwrap(), file, &added_lines)?;
    }

    // バックアップ用のディレクトリを作成
    let backup_path = output_file_path.join("Countfile");
    if backup_path.exists() {
        fs::create_dir_all(&backup_path)?;
    }

    // 必要なファイルを移動
    file_filter::move_files(output_file_path.to_str().unwrap(), backup_path.to_str().unwrap(), &config)?;

    // 総行数をカウント
    let total_lines = line_counter::count_lines(backup_path.to_str().unwrap())?;
    println!("\n総ステップ数: {}", total_lines);

    // CountTempFileのパスを表示し、終了メッセージ表示
    println!("一時ディレクトリパス: {}", output_file_path.display());
    println!("\nエンターキーを押して終了...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // CountTempFileディレクトリを削除
    fs::remove_dir_all(output_file_path)?;

    Ok(())
}
