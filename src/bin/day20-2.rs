use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("day20.txt");
    let file = std::fs::File::open(path)?;
    let lines = std::io::BufReader::new(file).lines();
    let mut idx = 0;
    let mut nums = lines
        .map(|num_str| {
            idx += 1;
            return (
                num_str.ok().unwrap().parse::<i64>().ok().unwrap() * 811589153,
                idx - 1,
            );
        })
        .collect::<Vec<(i64, i32)>>();

    let len = i64::try_from(nums.len() - 1).unwrap();
    for _ in 0..10 {
        idx = 0;
        while idx < nums.len().try_into().unwrap() {
            let src_idx = nums.iter().position(|node| return node.1 == idx).unwrap();
            let node = nums.remove(src_idx);
            let dest_idx = (i64::try_from(src_idx).unwrap() + node.0 + len).rem_euclid(len);
            nums.insert(usize::try_from(dest_idx).unwrap(), node);
            idx += 1;
        }
    }

    let zero_idx = nums
        .iter()
        .position(|node| {
            return node.0 == 0;
        })
        .unwrap();
    println!(
        "value {}",
        nums.get((zero_idx + 1000) % nums.len()).unwrap().0
            + nums.get((zero_idx + 2000) % nums.len()).unwrap().0
            + nums.get((zero_idx + 3000) % nums.len()).unwrap().0
    );
    Ok(())
}
