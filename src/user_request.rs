use anyhow::Result;
use std::io::{self, Write};

/// UserRequest構造体
pub struct UserRequest {
    pub source_path: String,
    pub git_rev: String,
    pub author_name: String,
}

impl UserRequest {
    ///ユーザーからの入力を取得して新しいUserRequestを生成する
    pub fn from_user_input() -> Result<Self> {
        let source_path = UserRequest::prompt("ソースコードのパスを入力してください:")?;
        let git_rev_before = UserRequest::prompt("開始リビジョンを入力してください:")?;
        let git_rev_after = UserRequest::prompt("終了リビジョンを入力してください:")?;
        let git_rev = format!("{}..{}", git_rev_before, git_rev_after);
        let author_name = UserRequest::prompt("ユーザー名を入力してください:")?;

        Ok(UserRequest {
            source_path,
            git_rev,
            author_name,
        })
    }

    /// 標準入力からのプロンプトを表示し、入力を取得
    fn prompt(message: &str) -> Result<String> {
        print!("{}", message);
        io::stdout().flush()?; // すぐに表示するためにフラッシュ
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string()) // 改行を除去して渡す
    }
}
