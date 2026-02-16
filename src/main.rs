pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // Early Dev

fn main() -> Result<()> {
    Ok(())
}
