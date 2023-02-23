use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day24.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    Ok(())
}
