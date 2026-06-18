use anyhow::{Context, anyhow};
use who_cares::WhoCares;

fn main() -> WhoCares<()> {
    let port = parse_port("server.port=8080")?;

    assert_eq!(port, 8080);
    WhoCares(())
}

fn parse_port(line: &str) -> anyhow::Result<u16> {
    let raw_port = line
        .strip_prefix("server.port=")
        .ok_or_else(|| anyhow!("missing server.port prefix"))?;

    raw_port.parse().context("server.port must be a u16")
}
