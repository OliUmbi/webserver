mod compliance;
mod stress;
mod util;

fn main() -> anyhow::Result<()> {

    let base_url = "localhost:80";

    compliance::run(&base_url);

    stress::run(&base_url)?;

    Ok(())
}
