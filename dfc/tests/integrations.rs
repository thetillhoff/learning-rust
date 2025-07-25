#[test]
fn test_run() {
    let args = dfc::cli::Args {
        source: Some(String::from("a.txt")),
        source_format: Some(String::from("")),
        destination: Some(String::from("b.txt")),
        destination_format: Some(String::from("")),
    };
    let result = dfc::run(args);
    assert!(result.is_ok());
}

#[test]
fn test_cli() {
    let format = dfc::cli::source_format(Some(&String::from("a.txt")), Some(&String::from("")));
}
