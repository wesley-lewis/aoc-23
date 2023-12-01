use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()>{
    let f = File::open("input.txt").expect("failed to load input");
    let buf = BufReader::new(f);

    let mut sum: u32 = 0;
    for line in buf.lines() {
        let line = line?;
        let ans = find_calibration_values(&line);
        sum += ans;
    }
    println!("ans: {sum}");

    Ok(())
}

fn find_calibration_values(line: &str) -> u32 {
    let mut result: u32 = 0;
    let chars = line.chars();
    let mut rev_chars = line.chars();
    for c in chars {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                result = result + num * 10;
                break;
            }else {
            }
        }
    }

    while let Some(c) = rev_chars.next_back() {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                result = result + num;
                break;
            }else {
            }
        }
    }
    result
}
