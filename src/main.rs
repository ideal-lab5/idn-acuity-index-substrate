fn main() -> sled::Result<()> {
    let path = "db";
    let db = sled::open(path)?;
    Ok(())
}
