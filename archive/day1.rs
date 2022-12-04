use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut running = 0;
    let mut vals = BinaryHeap::new();

    if let Ok(lines) = read_lines("./Input/input.txt"){
        for line in lines {
            let input = line.unwrap_or_default();

            // Phase 1



        }
    }
    let top_3 = [vals.pop().unwrap(), vals.pop().unwrap(), vals.pop().unwrap()];
    println!("The Top Three Vals Have: {}", top_3.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
