mod code_analyzer;
mod diff_processor;

use code_analyzer::{count_lines_in_directory, filter_lines};
use diff_processor::create_temp_files;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

/// コンフィグファイルのパス.`config.json`は実行モジュールと同じ階層に配置する想定
const CONFIG_FILE_NAME: &str = "config.json";

/// ユーザーからのインプットを管理する構造体
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestData {
    /// ソースコードパス
    pub source_path: String,
    /// 開始リビジョン
    pub start_revision: String,
    /// 終了リビジョン
    pub end_revision: String,
    /// ユーザー名
    pub user_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepCountResultData {
    total_lines: usize,
    temp_dir_path: PathBuf,
}

#[tauri::command]
pub fn exec(user_request_data: UserRequestData) -> Result<StepCountResultData, String> {
    // デバッグ時は`src-tauri/{CONFIG_FILE_NAME}`を参照する
    let mut config_path = if cfg!(debug_assertions) {
        PathBuf::from(Path::new(env!("CARGO_MANIFEST_PATH")).parent().unwrap()) // Cargo.toml の親ディレクトリ取得
    } else {
        PathBuf::from(std::env::current_exe().unwrap().parent().unwrap()) // 実行ファイルの親ディレクトリ取得
    };
    config_path.push(CONFIG_FILE_NAME);
    println!("{:?}", config_path);

    // config.json が存在するか確認
    match config_path.try_exists() {
        Ok(false) => {
            return Err(String::from(
                "config.json が見つかりません。実行ファイルと同じディレクトリに配置してください。",
            ))
        }
        Err(why) => return Err(why.to_string()),
        _ => (),
    }

    // 一時ファイル作成
    let temp_dir_path = match create_temp_files(&user_request_data, config_path) {
        Ok(temp_dir_path) => temp_dir_path,
        Err(why) => return Err(why.to_string()),
    };
    let temp_file_path = temp_dir_path.join("temp_file.txt");

    // 変更行のフィルタリング
    if let Err(why) = filter_lines(
        &PathBuf::from(&user_request_data.source_path),
        &temp_file_path,
    ) {
        return Err(why.to_string());
    }

    // カウントする
    let count_dir_path = temp_dir_path.join("CountFile");
    let total_lines = match count_lines_in_directory(&count_dir_path) {
        Ok(total_lines) => total_lines,
        Err(why) => return Err(why.to_string()),
    };

    // 出力
    println!("総ステップ数: {}", total_lines);
    println!("一時ディレクトリパス: {}", temp_dir_path.display());
    println!("ユーザー名: {}", user_request_data.user_name);

    // 一時ディレクトリを削除
    if let Err(why) = fs::remove_dir_all(&temp_dir_path) {
        return Err(why.to_string());
    }

    Ok(StepCountResultData {
        total_lines,
        temp_dir_path,
    })
}
