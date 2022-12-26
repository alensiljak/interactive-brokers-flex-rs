/*
 * Parse IB Flex transactions.
 */

use std::fs::read_dir;

const FILE_SUFFIX: &str = "_cash-tx.xml";

pub fn parse() {
    // Load the latest report file.
    let pattern = format!("*{}", FILE_SUFFIX);
    let filename = get_latest_file(&pattern);
}

/// Get the latest of the filest matching the given pattern.
/// Pattern example: *.xml
fn get_latest_file(file_pattern: &str) -> String {
    let mut filenames: Vec<String> = glob::glob(file_pattern)
        .expect("directory list")
        .filter_map(|entry| {
            let path_buf = entry.unwrap();
            if path_buf.is_file() {
                let val = path_buf.to_str().unwrap().to_string();
                Some(val)
            } else {
                None
            }
        })
        .collect();

    filenames.sort();

    let result = filenames.last().unwrap().to_owned();

    result
}

struct IB_Flex_Parser {}

impl IB_Flex_Parser {
    pub fn parse(filename: String) {
        todo!("parse IB Flex report");
    }
}

#[cfg(test)]
mod tests {
    use super::get_latest_file;

    #[test]
    fn test_dir_list() {
        let actual = get_latest_file("*.xml");

        assert!(!actual.is_empty());
    }
}
