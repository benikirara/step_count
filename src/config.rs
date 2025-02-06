use serde::Deserialize;
use anyhow::Result;
use std::fs;
use std::io::{self, Write};

///config.json用構造体
#[derive(Deserialize, Debug)]
pub struct Config {
    pub included_extensions: Vec<String>,
    pub exclude_files: Vec<String>,
}

impl Config {
    /// config.jsonを読み込み、Configインスタンスを生成する
    pub fn from_file(path: &str) -> Result<Self> {
        let config_contents = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&config_contents)?;
        Ok(config)
    }

    /// config.jsonを読み込み、存在しない場合にはエラーメッセージを表示して終了する
    pub fn from_file_or_exit(path: &str) -> Self {
        match Self::from_file(path) {
            Ok(cfg) => cfg,
            Err(_) => {
                println!("config.jsonが見つかりません。実行ファイルと同じディレクトリに配置してください。");
                println!("エンターキーを押して終了...");
                let _ = io::stdout().flush(); // メッセージを表示
                let mut input = String::new();
                let _ = io::stdin().read_line(&mut input);
                std::process::exit(1);
            }
        }
    }
}
