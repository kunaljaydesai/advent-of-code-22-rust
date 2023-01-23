use std::io::BufRead;

fn get_range(s: &str) -> [i32; 2] {
    let split = s.split('-').collect::<Vec<&str>>();
    [
        split[0].parse::<i32>().unwrap(),
        split[1].parse::<i32>().unwrap(),
    ]
}

fn is_full_overlap(a: [i32; 2], b: [i32; 2]) -> bool {
    return (a[0] <= b[0] && a[1] >= b[1]) || (b[0] <= a[0] && b[1] >= a[1]);
}

fn main() -> Result<(), std::io::Error> {
    let path = std::path::Path::new("day4.txt");
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut num_overlap = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let split = line.split(",").collect::<Vec<&str>>();
        let range = [get_range(split[0]), get_range(split[1])];
        if is_full_overlap(range[0], range[1]) {
            num_overlap += 1;
        }
    }
    println!("Num overlap {}", num_overlap);
    Ok(())
}
