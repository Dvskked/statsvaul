use crate::calculator::Statistics;

const HEADER_LEFT: &str = "Metric";
const HEADER_RIGHT: &str = "Value";

/// Formats the statistics as a clean ASCII table, rounded to `decimals`
/// places. A footer explains the two kinds of standard deviation.
pub fn format_table(stats: &Statistics, decimals: usize) -> String {
    let rows = build_rows(stats, decimals);
    let mut table = build_table(HEADER_LEFT, HEADER_RIGHT, &rows);
    table.push_str("\nStd dev: \"sample\" uses denominator n-1, \"population\" uses n.");
    table
}

fn build_rows(stats: &Statistics, decimals: usize) -> Vec<(String, String)> {
    let number = |value: f64| format!("{:.*}", decimals, value);
    vec![
        ("Count".to_string(), stats.count.to_string()),
        ("Sum".to_string(), number(stats.sum)),
        ("Average".to_string(), number(stats.average)),
        ("Minimum".to_string(), number(stats.minimum)),
        ("Maximum".to_string(), number(stats.maximum)),
        (
            "Variance (sample)".to_string(),
            number(stats.variance_sample),
        ),
        ("Std dev (sample)".to_string(), number(stats.std_dev_sample)),
        (
            "Variance (population)".to_string(),
            number(stats.variance_population),
        ),
        (
            "Std dev (population)".to_string(),
            number(stats.std_dev_population),
        ),
    ]
}

fn build_table(left_header: &str, right_header: &str, rows: &[(String, String)]) -> String {
    let mut left_width = left_header.len().max(4);
    let mut right_width = right_header.len().max(5);
    for (left, right) in rows {
        left_width = left_width.max(left.len());
        right_width = right_width.max(right.len());
    }

    let border = format!(
        "+{}+{}+",
        "-".repeat(left_width + 2),
        "-".repeat(right_width + 2)
    );

    let mut out = String::new();
    out.push_str(&border);
    out.push('\n');
    out.push_str(&format!(
        "| {left:<left_width$} | {right:<right_width$} |",
        left = left_header,
        right = right_header
    ));
    out.push('\n');
    out.push_str(&border);
    out.push('\n');
    for (left, right) in rows {
        out.push_str(&format!(
            "| {left:<left_width$} | {right:<right_width$} |",
            left = left,
            right = right
        ));
        out.push('\n');
    }
    out.push_str(&border);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::calculate;

    fn sample() -> Statistics {
        calculate(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).expect("dataset is not empty")
    }

    #[test]
    fn prints_a_bordered_table_with_headers() {
        let table = format_table(&sample(), 2);
        assert!(table.contains("+"));
        assert!(table.contains("Metric"));
        assert!(table.contains("Value"));
        assert!(table.contains("Count"));
    }

    #[test]
    fn honours_the_decimal_count() {
        let three_decimals = format_table(&sample(), 3);
        let one_decimal = format_table(&sample(), 1);

        assert!(three_decimals.contains("5.000"));
        assert!(!three_decimals.contains("5.00 "));
        assert!(one_decimal.contains("5.0"));
        assert!(!one_decimal.contains("5.00"));
    }

    #[test]
    fn explains_the_standard_deviation_kinds() {
        let table = format_table(&sample(), 2);
        assert!(table.contains("population"));
        assert!(table.contains("sample"));
    }
}
