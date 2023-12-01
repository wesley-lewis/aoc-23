use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()>{
    let f = File::open("input.txt").expect("failed to load input");
    let buf = BufReader::new(f);
    let mut counter: u32 = 0;
    for line in buf.lines() {
        if counter == 7 {
            break;
        }
        let line = line?;
        let value = find_calibration_values(&line);
        println!("Ans: {value}");
        counter += 1;
    }

    Ok(())
}

fn find_calibration_values(line: &str) -> String {
    let mut result = String::new(); 
    let chars = line.chars();
    let mut rev_chars = line.chars();
    for c in chars {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                result = result + &num.to_string();
                break;
            }else {
            }
        }
    }

    while let Some(c) = rev_chars.next_back() {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                result = result + &num.to_string();
                break;
            }else {
            }
        }
    }
    result
}
