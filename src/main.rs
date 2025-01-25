use std::{io, fs};
mod config;
mod diff_processor;
mod code_analyzer;
use std::path::PathBuf;

use config::get_input;
use diff_processor::create_temp_files;
use code_analyzer::{count_lines_in_directory, filter_lines};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ユーザーから情報を取得
    let source_path = get_input("ソースコードのパスを入力してください: ")?;
    let start_revision = get_input("開始リビジョンを入力してください: ")?;
    let end_revision = get_input("終了リビジョンを入力してください: ")?;
    let user_name = get_input("ユーザー名を入力してください: ")?;

    // config.jsonは実行モジュールと同じ階層に配置する想定
    let config_path = "config.json";

    // config.json が存在するか確認
    if !std::path::Path::new(config_path).exists() {
        eprintln!("エラー: config.json が見つかりません。実行ファイルと同じディレクトリに配置してください。");
        return Err("config.json not found".into());
    }

    // 一時ファイル作成
    let temp_dir_path = create_temp_files(&source_path, &start_revision, &end_revision, &user_name, config_path)?;
    let temp_file_path = temp_dir_path.join("temp_file.txt");

    // 変更行のフィルタリング
    if let Err(e) = filter_lines(&PathBuf::from(&source_path), &temp_file_path){
        eprintln!("filter_linesでエラーが発生しました: {}",e);
        return Err(e.into());
    }

    // カウントする
    let count_dir_path = temp_dir_path.join("CountFile");
    let total_lines = count_lines_in_directory(&count_dir_path)?;

    // 出力
    println!("総ステップ数: {}", total_lines);
    println!("一時ディレクトリパス: {}", temp_dir_path.display());
    println!("ユーザー名: {}", user_name);
    println!("エンターキーを押して続行...");
    io::stdin().read_line(&mut String::new())?;

    // 一時ディレクトリを削除
    fs::remove_dir_all(&temp_dir_path)?;

    Ok(())
}