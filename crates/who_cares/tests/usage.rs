use std::io::{self, Error, Read};

use who_cares::WhoCares;

#[test]
fn you_can_use_it_on_both_option_and_result() -> WhoCares<()> {
    let config = {
        let mut reader = io::Cursor::new("name=Ferris");
        let mut config = String::new();
        reader.read_to_string(&mut config)?;
        Ok::<String, Error>(config)
    }?;
    let name = config.strip_prefix("name=")?;

    assert_eq!(name, "Ferris");
    WhoCares(())
}

#[test]
fn complex_example() -> WhoCares<()> {
    let report: serde_json::Value = serde_json::from_str(
        r#"{
            "crate": "who_cares",
            "downloads": 42,
            "keywords": ["test", "unwrap", "question-mark"]
        }"#,
    )?;

    let crate_name = report.get("crate")?.as_str()?;
    let downloads = report.get("downloads")?.as_u64()?;
    let first_keyword = report.get("keywords")?.as_array()?.first()?.as_str()?;

    assert_eq!(crate_name, "who_cares");
    assert_eq!(downloads, 42);
    assert_eq!(first_keyword, "test");

    WhoCares(())
}
