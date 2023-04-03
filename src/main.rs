use std::io;

fn main() -> io::Result<()> {
    // cities::processor::to_static_file()?;
    cities::files::verify()?;

    Ok(())
}