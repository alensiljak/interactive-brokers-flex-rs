/*!
 * Parses IB Flex Query transactions.
 */

use crate::flex_query_def::FlexQueryResponse;

const FILE_SUFFIX: &str = "_cash-tx.xml";

/**
 * Parse the latest file. The default behaviour.
 */
pub fn parse() -> FlexQueryResponse {
    // Load the latest report file.
    let pattern = format!("*{}", FILE_SUFFIX);
    let filename = get_latest_filename(&pattern);

    parse_file(&filename)
}

/**
 * Parse the file with the given filename, in the current directory.
 */
pub fn parse_file(filename: &str) -> FlexQueryResponse {
    println!("Reading {}", filename);

    let content = std::fs::read_to_string(filename).expect("xml file read");
    //log::debug!("file content: {:?}", content);

    parse_string(&content)
}

/**
 * Parse the file contents (xml) into the FlexQueryResponse object.
 */
pub fn parse_string(content: &str) -> FlexQueryResponse {
    serde_xml_rs::from_str(content).expect("parsed XML")
}

/// Get the latest of the filest matching the given pattern.
/// Pattern example: *.xml
fn get_latest_filename(file_pattern: &str) -> String {
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

    if filenames.is_empty() {
        panic!("No XML files found in the current directory. Aborting.");
    }

    filenames.sort();

    let result = filenames.last().unwrap().to_owned();

    result
}

#[cfg(test)]
mod tests {
    use std::fs::canonicalize;

    use super::{get_latest_filename, parse_file};

    #[test]
    fn test_dir_list() {
        let actual = get_latest_filename("*.xml");

        assert!(!actual.is_empty());
    }

    // #[test]
    // fn test_filename_composition() {
    //     let file_pattern = format!("*{}", FILE_SUFFIX);
    //     let actual = get_latest_filename(&file_pattern);

    //     assert_ne!(String::default(), actual);
    // }

    #[test]
    fn test_parse_file() -> anyhow::Result<()> {
        let cur_dir = std::env::current_dir()?;
        let filename = format!(
            "{}{}{}{}{}",
            cur_dir.display(),
            std::path::MAIN_SEPARATOR,
            "tests",
            std::path::MAIN_SEPARATOR,
            "report_1.xml"
        );
        // canonicalize(path)

        let actual = parse_file(&filename);

        assert_ne!(
            0,
            actual
                .FlexStatements
                .FlexStatement
                .CashTransactions
                .CashTransaction
                .len()
        );

        Ok(())
    }
}
