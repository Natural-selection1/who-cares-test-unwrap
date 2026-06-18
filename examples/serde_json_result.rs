use who_cares::WhoCares;

fn main() -> WhoCares<()> {
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
