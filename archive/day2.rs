use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    println!("Enter Input (type q to end): ");

    let mut running = 0;
    let mut strat_running = 0;

    let res_vals: HashMap<&str, i32> = [("A X", 4), ("A Y", 8), ("A Z", 3),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 7), ("C Y", 2), ("C Z", 6)].iter().cloned().collect();
    let strat_vals: HashMap<&str, i32> = [("A X", 3), ("A Y", 4), ("A Z", 8),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 2), ("C Y", 6), ("C Z", 7)].iter().cloned().collect();

    if let Ok(lines) = read_lines("./Input/input2.txt"){
        for line in lines {
            let input = line.unwrap_or_default();

            if input.is_empty(){
                continue;
            }

            // Phase 1
            match res_vals.get(input.trim()) {
                Some(value) => {
                    running += value;
                },
                None => println!("no value"),
            }

            // Phase 2
            match strat_vals.get(input.trim()) {
                Some(value) => {
                    strat_running += value;
                },
                None => println!("no value"),
            }

        }
    }

    println!("Total score: {running}");
    println!("Total strat score: {strat_running}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
