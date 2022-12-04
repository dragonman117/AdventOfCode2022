use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {

    let mut running = 0;
    let mut running2 = 0;

    if let Ok(lines) = read_lines("./Input/input4.txt"){
        for line in lines {
            let input = line.unwrap_or_default();
            let re = Regex::new(r"([\d]+)-([\d]+),([\d]+)-([\d]+)").unwrap();
            if input.is_empty(){
                break;
            }

            // Phase 1

            for cap in re.captures_iter(&input) {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();
                let x = cap[3].parse::<i32>().unwrap();
                let y = cap[4].parse::<i32>().unwrap();
                if (a >= x && b <= y) || (x >= a && y <= b){
                    running += 1;
                }
            }

            // Phase 2
            for cap in re.captures_iter(&input) {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();
                let x = cap[3].parse::<i32>().unwrap();
                let y = cap[4].parse::<i32>().unwrap();
                if (a >= x && a <= y) || (b >= x && b <= y) || (x >= a && x <= b) || (y >= a && y <= b){
                    println!("{} {} {} {}", a, b, x, y);
                    running2 += 1;
                }
            }
        }
        println!("The Running Total Is: {}", running);
        println!("The Running Total2 Is: {}", running2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
