use std::{
    env,
    fs::{self, copy, File},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use crate::UserRequestData;
use serde::Deserialize;

/// diff 出力における追加ファイルのパスプレフィックス
const DIFF_FILE_PATH_PREFIX: &str = "+++ b/"; // diff 出力において、追加されたファイルのパスは "+++ b/" で始まる。
/// diff 出力における追加ファイルのパスプレフィックスの長さ
const DIFF_FILE_PATH_PREFIX_LEN: usize = DIFF_FILE_PATH_PREFIX.len();

/// `config.json`の構造体
#[derive(Deserialize)]
struct AppConfig {
    included_extensions: Vec<String>,
    exclude_files: Vec<String>,
}

/// 差分処理、一時ファイル作成
pub fn create_temp_files(
    user_request_data: &UserRequestData,
    config_path: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // ディレクトリ移動
    env::set_current_dir(&user_request_data.source_path)?;

    // config.json を読み込む
    let config_file = File::open(config_path)?;
    let buf_reader = std::io::BufReader::new(config_file);

    // JSON をパース
    let app_config: AppConfig = serde_json::from_reader(buf_reader)?;

    // git diff コマンドを実行し、差分を取得 (--author オプションでユーザーを絞り込む)
    let output = Command::new("git")
        .args([
            "diff",
            "-U0",
            &user_request_data.start_revision,
            &user_request_data.end_revision,
            "--author",
            &user_request_data.user_name,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "git diff コマンドが失敗しました: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // 差分出力を文字列に変換
    let diff_output = String::from_utf8_lossy(&output.stdout);

    // 一時ディレクトリを作成
    let temp_dir_path = env::temp_dir().join("StepCountTempFile");
    fs::create_dir_all(&temp_dir_path)?;

    // カウント対象ファイルを格納するディレクトリを作成
    let count_dir_path = temp_dir_path.join("CountFile");
    fs::create_dir_all(&count_dir_path)?;

    // 現在処理中のファイルのパスを格納する変数
    let mut current_file: Option<PathBuf> = None;

    // 差分出力を一行ずつ処理
    for line in diff_output.lines() {
        // git diffではファイルの先頭に"+++ b/"がついて出力されるため、そこで判定
        if line.starts_with(DIFF_FILE_PATH_PREFIX) {
            // "+++ b/"を除外し、ファイルパスを取得
            let file_path = &line[DIFF_FILE_PATH_PREFIX_LEN..].trim();
            current_file = Some(PathBuf::from_str(file_path)?);
            continue;
        }

        if let Some(original_path) = &current_file {
            // ファイル名と拡張子を取得
            let file_name_str = original_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            let extension = original_path.extension().and_then(|ext| ext.to_str());

            let mut is_excluded = false;

            // 拡張子が included_extensions に含まれていない場合は除外
            if let Some(ext) = extension {
                if !app_config.included_extensions.contains(&ext.to_string()) {
                    is_excluded = true;
                }
            } else {
                is_excluded = true;
            }

            // ファイル名が exclude_files に含まれている場合は除外
            if app_config
                .exclude_files
                .iter()
                .any(|name| file_name_str.ends_with(name))
            {
                is_excluded = true
            }

            // 除外対象でない場合、一時ファイルにコピーし、CountFile ディレクトリに移動
            if !is_excluded {
                let temp_file_path = temp_dir_path.join(original_path.file_name().unwrap());
                copy(original_path, &temp_file_path)?;
                let count_file_path = count_dir_path.join(original_path.file_name().unwrap());
                fs::rename(&temp_file_path, &count_file_path)?;
            }
        }
    }

    Ok(temp_dir_path)
}
