mod code_analyzer;
mod diff_processor;

use code_analyzer::{count_lines_in_directory, filter_lines};
use diff_processor::create_temp_files;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::Result;

/// コンフィグファイルのパス.`config.json`は実行モジュールと同じ階層に配置する想定
const CONFIG_PATH: &str = "config.json";

/// ユーザーからのインプットを管理する構造体
struct UserRequestData {
    /// ソースコードパス
    pub source_path: String,
    /// 開始リビジョン
    pub start_revision: String,
    /// 終了リビジョン
    pub end_revision: String,
    /// ユーザー名
    pub user_name: String,
}

impl UserRequestData {
    pub fn new() -> Result<Self> {
        Ok(Self {
            source_path: Self::get_input("ソースコードのパスを入力してください: ")?,
            start_revision: Self::get_input("開始リビジョンを入力してください: ")?,
            end_revision: Self::get_input("終了リビジョンを入力してください: ")?,
            user_name: Self::get_input("ユーザー名を入力してください: ")?,
        })
    }

    /// ユーザーからの入力を取得する
    /// 注：入力を受け取って前後の空白を削除した文字列を返す
    fn get_input(prompt: &str) -> Result<String> {
        print!("{prompt}");
        io::stdout().flush()?; // 出力をフラッシュ (即時表示するため)
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string()) // 入力文字列の前後にある空白を削除して返す
    }
}

#[allow(dead_code)]
fn step_count_exec() -> Result<(), Box<dyn std::error::Error>> {
    // ユーザーから情報を取得
    let user_request_data = UserRequestData::new()?;

    // config.json が存在するか確認
    if !Path::new(CONFIG_PATH).try_exists()? {
        eprintln!("エラー: config.json が見つかりません。実行ファイルと同じディレクトリに配置してください。");
        return Err("config.json not found".into());
    }

    // 一時ファイル作成
    let temp_dir_path = create_temp_files(&user_request_data, CONFIG_PATH)?;
    let temp_file_path = temp_dir_path.join("temp_file.txt");

    // 変更行のフィルタリング
    if let Err(e) = filter_lines(
        &PathBuf::from(&user_request_data.source_path),
        &temp_file_path,
    ) {
        eprintln!("filter_linesでエラーが発生しました: {}", e);
        return Err(e.into());
    }

    // カウントする
    let count_dir_path = temp_dir_path.join("CountFile");
    let total_lines = count_lines_in_directory(&count_dir_path)?;

    // 出力
    println!("総ステップ数: {}", total_lines);
    println!("一時ディレクトリパス: {}", temp_dir_path.display());
    println!("ユーザー名: {}", user_request_data.user_name);
    println!("エンターキーを押して続行...");
    io::stdin().read_line(&mut String::new())?;

    // 一時ディレクトリを削除
    fs::remove_dir_all(&temp_dir_path)?;

    Ok(())
}
