use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut running = 0;
    let mut badge_prio = 0;
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut iteration = 0;

    if let Ok(lines) = read_lines("./Input/input3.txt"){
        for line in lines {
            let input = line.unwrap_or_default();
            if input.is_empty(){
                break;
            }

            // Phase 1
            let mut compartment = HashSet::new();

            let compartment1 = &input[0..(input.len()/2)];
            let compartment2 = &input[(input.len()/2)..input.len()];
            compartment1.chars().for_each(|x| {compartment.insert(x);});
            let mut duplicate = char::default();
            for item in compartment2.chars(){
                if compartment.contains(&item){
                    duplicate = item;
                }
            }
            running += find_prio(duplicate);

            // Phase 2
            let mut distinct = HashSet::new();
            input.chars().for_each(|x| {distinct.insert(x);});
            for item in distinct{
                match counts.get(&item) {
                    Some(value) => {
                        counts.insert(item, value + 1);
                    },
                    None => {
                        counts.insert(item, 1);
                    },
                }
            }

            iteration +=1;
            if iteration % 3 == 0{
                iteration = 0;
                let key = counts.iter().find_map(|(key, value)| {
                    if *value == 3 as i32{
                        Some(key.clone())
                    } else {
                        None
                    }
                }).unwrap();
                counts = HashMap::new();
                badge_prio += find_prio(key);
            }

        }
        println!("The Running Total Is: {}", running);
        println!("The Badge Prio Is: {}", badge_prio);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_prio(c: char) -> i32 {
    if c.is_lowercase(){
        return c as i32 - 'a' as i32 + 1;
    } else if c.is_uppercase() {
        return c as i32 - 'A' as i32 + 27;
    }
    return 0;
}
