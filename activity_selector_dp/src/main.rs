/// This represents a modified form of the Activity Selector algorithm in CLRS
/// chapter 16.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod l2s_algo;

fn main() {

    let file = File::open("act.txt").expect("No File Found");

    let buf = BufReader::new(file);

    let mut tasks = Vec::<l2s_algo::l2s_algo::Task>::new();
    let mut sets = Vec::new();
    let mut set = 0;
  
    for line in buf.lines().skip(1) {

        // Consume the line
        let l = line.expect("Line should not be empty");

        // Split the string
        let unstruc = l.split_whitespace().collect::<Vec<&str>>();

        match unstruc.len() {
            1   => {
                    // Store and ready for the next set
                    if tasks.len() > 0 {
                        sets.push(tasks.clone());

                        tasks.clear();
                    }
                },
            3   => {  
                    // Add the task
                    let task = l2s_algo::l2s_algo::Task {
                                id      : unstruc[0].parse::<u32>().unwrap(),
                                start   : unstruc[1].parse::<u32>().unwrap(),
                                end     : unstruc[2].parse::<u32>().unwrap(),
                            };

                    tasks.push(task);
                },
            _   => panic!("Malformed Read"), 
        };
                
    }

    // Store the last thing
    sets.push(tasks);
    
    for task in sets {
        // Increment the set number
        set += 1;

        let start = Instant::now();

        // Run the algo
        let schedule = l2s_algo::l2s_algo::l2s_algo(task);

        let dur = start.elapsed();

        // Print out
        println!("Set {}", set);
        println!("Number of activities selected: {}", schedule.len());
        print!("Activities:");

        schedule.iter().rev().for_each(|id| print!(" {}", id));

        print!("\n");

        println!("Done in {:?}", dur);
    }
    
    
}
