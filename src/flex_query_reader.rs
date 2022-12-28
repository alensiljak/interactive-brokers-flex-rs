/*!
 * Read the required Flex Query report file(s).
 * The logic for choosing a file.
 */

const FILE_SUFFIX: &str = "_cash-tx.xml";

pub fn load_report(path: Option<String>) -> String {
    let report_path = match path {
        Some(file_path) => file_path,
        None => get_latest_report_path(),
    };

    std::fs::read_to_string(report_path).expect("xml file read")
}

pub fn get_latest_report_path() -> String {
    // Load the latest report file.
    let pattern = format!("*{}", FILE_SUFFIX);

    get_latest_filename(&pattern)
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
    use crate::{flex_query_def::FlexQueryResponse, flex_query_reader::load_report};

    use super::get_latest_filename;

    #[test]
    fn test_dir_list() {
        let actual = get_latest_filename("*.xml");

        assert!(!actual.is_empty());
    }

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

        let report = load_report(Some(filename));
        let actual = FlexQueryResponse::from(report);

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
