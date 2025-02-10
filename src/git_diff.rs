use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

/// 特定のユーザーによる変更ファイルのリストを取得する
pub fn get_changed_files(git_rev: &str, author_name: &str) -> Result<Vec<PathBuf>> {
    let mut args = vec!["log", git_rev, "--name-only", "--pretty=format:"];

    // `author_name`が空の場合、すべてのユーザーの変更を対象とする
    if !author_name.trim().is_empty() {
        args.extend(&["--author", author_name]);
    }

    // 実行するgitコマンドを表示
    let command_str = format!("git {}", args.join(" "));
    println!("\nExecuting Command: {}", command_str);
    println!("しばらくお待ちください...");

    let output = Command::new("git").args(&args).output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow! {
            "Git command failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        });
    }

    let mut changed_path = Vec::new();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    for path_str in stdout.lines() {
        changed_path.push(
            PathBuf::from_str(path_str.trim())
                .unwrap_or_else(|_| panic!("failure {path_str} to PathBuf")),
        );
    }

    Ok(changed_path)
}

/// 整形する(空白行、コメント行をはじき、追加行のみにする)
pub fn get_added_lines<P: AsRef<Path>>(git_rev: &str, file: P) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", git_rev, "--", file.as_ref().to_str().unwrap()])
        .output()?;
    let diff_output_str = String::from_utf8_lossy(&output.stdout);

    let mut added_lines = Vec::new();

    // "{", "}", "};" を追加で除外したい場合はこちらを使う
    // for line in diff_output_str.lines() {
    //     if line.starts_with('+') && !line.starts_with("++") {
    //         let trimmed_line = line[1..].trim();
    //         if !trimmed_line.is_empty() &&
    //            !trimmed_line.starts_with("//") &&
    //            !matches!(trimmed_line, "{" | "}" | "};") {
    //                added_lines.push(trimmed_line.to_string());
    //            }
    //     }
    // }

    // 空白行とコメント行のみを除外する
    for line in diff_output_str.lines() {
        if line.starts_with('+') && !line.starts_with("++") {
            let trimmed_line = line[1..].trim();
            if !trimmed_line.is_empty() && !trimmed_line.starts_with("//") {
                added_lines.push(trimmed_line.to_string());
            }
        }
    }

    Ok(added_lines)
}
