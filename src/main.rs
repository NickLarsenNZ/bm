mod bm;
mod data;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let _ = bm::Bm::new()?;

    Ok(())
}
