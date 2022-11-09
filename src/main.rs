mod bm;
mod data;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let mut bm = bm::Bm::new()?;

    bm.load_db()?;

    Ok(())
}
