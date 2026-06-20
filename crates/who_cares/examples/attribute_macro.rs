use who_cares::who_cares;

#[who_cares]
fn main() {
    let config = "name=Ferris\nscore=42";

    let name = find_value(config, "name")?;
    let score = find_value(config, "score")?.parse::<u8>()?;

    assert_eq!(name, "Ferris");
    assert_eq!(score, 42)
}

fn find_value<'a>(config: &'a str, key: &str) -> Option<&'a str> {
    config
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{key}=")))
}
