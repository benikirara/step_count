use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::config::Config;

// 追加された行を一時ディレクトリに保存する
pub fn filter_and_save(output_file_path: &str, file: &str, added_lines: &[String]) -> Result<()> {
    if !added_lines.is_empty() {
        let destination_file_path = format!(
            "{}/{}",
            output_file_path,
            Path::new(file).file_name().unwrap().to_str().unwrap()
        );
        fs::write(destination_file_path, added_lines.join("\n"))?;
    }
    Ok(())
}

// フィルタリング条件に基づき、対象ファイルをバックアップディレクトリに移動する
pub fn move_files(output_file_path: &str, backup_path: &str, config: &Config) -> Result<()> {
    for entry in fs::read_dir(output_file_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_str = match path.to_str() {
                Some(s) => s,
                None => continue,
            };

            // `should_include`を使用してフィルタリング
            if should_include(file_str, config) {
                let file_name = match path.file_name().and_then(|s| s.to_str()) {
                    Some(name) => name,
                    None => continue,
                };
                let destination_path = format!("{}/{}", backup_path, file_name);
                fs::rename(&path, destination_path)?;
            }
        }
    }
    Ok(())
}

/// 指定されたファイルがフィルタリング条件に合致するかを判断する
pub fn should_include(file: &str, config: &Config) -> bool {
    // ファイルパスからファイル名を取得
    let path = Path::new(file);
    let file_name = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return false,
    };

    // included_extensionsのパターンを作成
    let ext_pattern = format!(r"\.({})$", config.included_extensions.join("|"));
    let ext_regex = match Regex::new(&ext_pattern) {
        Ok(re) => re,
        Err(_) => return false, // 無効な正規表現の場合は除外
    };

    // exclude_filesのパターンを作成 (例: (.Designer\.cs|Reference\.cs)$)
    let excl_pattern = format!(r"\.({})$", config.exclude_files.join("|"));
    let excl_regex = match Regex::new(&excl_pattern) {
        Ok(re) => re,
        Err(_) => return false, // 無効な正規表現の場合は除外
    };

    // included_extensionsに含まれているかチェック
    if !ext_regex.is_match(file_name) {
        return false;
    }

    // exclude_filesに含まれていないかチェック
    !excl_regex.is_match(file_name)
}
