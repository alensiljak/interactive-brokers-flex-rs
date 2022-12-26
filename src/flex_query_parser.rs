/*
 * Parse IB Flex Query transactions.
 */

use crate::flex_query_def::FlexQueryResponse;

const FILE_SUFFIX: &str = "_cash-tx.xml";

pub fn parse() -> FlexQueryResponse {
    // Load the latest report file.
    let pattern = format!("*{}", FILE_SUFFIX);
    let filename = get_latest_file(&pattern);

    parse_file(&filename)
}

pub fn parse_file(filename: &String) -> FlexQueryResponse {
    let content = std::fs::read_to_string(filename).expect("xml file read");
    //log::debug!("file content: {:?}", content);

    parse_string(&content)
}

pub fn parse_string(content: &String) -> FlexQueryResponse {
    serde_xml_rs::from_str(content)
        .expect("parsed XML")
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

    if filenames.is_empty() {
        panic!("No XML files found in the current directory. Aborting.");
    }

    filenames.sort();

    let result = filenames.last().unwrap().to_owned();

    result
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
