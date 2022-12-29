/*!
 * Read the required Flex Query report file(s).
 * The logic for choosing a file.
 */

use crate::compare::CompareParams;

const FILE_SUFFIX: &str = "_cash-tx.xml";

/**
 * Loads the Flex report.
 * If the direct path to the report is given, then the report is loaded. This
 * parameter takes precedence over path.
 * If the path to the directory is given, the latest report from that directory
 * will be loaded.
 */
pub fn load_report(params: &CompareParams) -> String {
    log::debug!("load_report with: {:?}", params);
    
    let report_path = match &params.flex_report_path {
        Some(file_path) => file_path.to_owned(),
        None => get_latest_report_path(params.flex_reports_dir.to_owned()),
    };

    std::fs::read_to_string(report_path).expect("xml file read")
}

/**
 * Gets the path to the latest report file in the given directory or the 
 * current directory, if None received.
 */
pub fn get_latest_report_path(report_dir: Option<String>) -> String {
    // Load the latest report file.
    let mut pattern = format!("*{}", FILE_SUFFIX);

    if let Some(dir_path) = report_dir {
        pattern = dir_path + pattern.as_str();
    }

    get_latest_filename(&pattern)
}

/// Get the latest of the filest matching the given pattern.
/// Pattern example: *.xml
fn get_latest_filename(file_pattern: &str) -> String {
    log::debug!("file pattern: {:?}", file_pattern);

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
    use crate::{flex_query_def::FlexQueryResponse, flex_query_reader::load_report, 
        test_fixtures::*, compare::CompareParams};

    use super::get_latest_filename;

    #[test]
    fn test_dir_list() {
        let actual = get_latest_filename("*.xml");

        assert!(!actual.is_empty());
    }

    #[rstest::rstest]
    fn test_parse_file(cmp_params: CompareParams) -> anyhow::Result<()> {
        let report = load_report(&cmp_params);
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
