use anyhow::Result;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// 行数を数える
pub fn count_lines<P: AsRef<Path>>(backup_path: P) -> Result<usize> {
    let mut total_lines = 0;
    for entry in fs::read_dir(backup_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            total_lines += reader.lines().count();
        }
    }
    Ok(total_lines)
}
