use std::io::{self, Read};

use who_cares::WhoCares;

fn main() -> WhoCares<()> {
    let config = read_fake_config()?;
    let name = config.strip_prefix("name=")?;

    assert_eq!(name, "Ferris");
    WhoCares(())
}

fn read_fake_config() -> io::Result<String> {
    let mut reader = io::Cursor::new("name=Ferris");
    let mut config = String::new();
    reader.read_to_string(&mut config)?;
    Ok(config)
}
