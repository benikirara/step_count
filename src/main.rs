mod config;
mod file_filter;
mod git_diff;
mod line_counter;
mod user_request;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

use crate::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    const CONFIG_PATH: &str = "config.json";
    let config_path = Path::new(CONFIG_PATH);

    // config.jsonを読み込む (実行時のカレントディレクトリに配置されている想定)
    let config = Config::from_file(config_path).unwrap_or_else(|why| {
        use config::ConfigError::*;
        match why {
            IoError(error) => eprintln!("{:?}の読み込みに失敗しました.\n{}", config_path, error),
            SerdeError(error) => eprintln!("コンフィグのパースに失敗しました.\n{}", error),
        };

        eprintln!("エンターキーを押して終了");
        #[allow(clippy::unused_io_amount)]
        std::io::stdin().read(&mut [0u8]).unwrap();
        std::process::exit(1);
    });

    // ユーザーからの入力を取得
    let user_request = user_request::UserRequest::from_user_input()?;

    // Gitリポジトリのディレクトリに移動
    env::set_current_dir(&user_request.source_path)?;

    // 変更されたファイルのリストを取得
    let changed_files =
        git_diff::get_changed_files(&user_request.git_rev, &user_request.author_name)?;

    // ユーザーの一時ディレクトリにCountTempFileを作成
    let mut output_file_path = env::temp_dir();
    output_file_path.push("CountTempFile");

    if !output_file_path.exists() {
        fs::create_dir_all(&output_file_path)?;
    }

    // 各変更ファイルに対して追加された行を取得・保存
    for file_path in changed_files {
        let added_lines = git_diff::get_added_lines(&user_request.git_rev, &file_path)?;
        file_filter::filter_and_save(output_file_path.to_str().unwrap(), file_path, &added_lines)?;
    }

    // バックアップ用のディレクトリを作成
    let backup_path = output_file_path.join("Countfile");
    if !backup_path.exists() {
        fs::create_dir_all(&backup_path)?;
    }

    // 必要なファイルを移動
    file_filter::move_files(&output_file_path, &backup_path, &config)?;

    // 総行数をカウント
    let total_lines = line_counter::count_lines(backup_path)?;
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
