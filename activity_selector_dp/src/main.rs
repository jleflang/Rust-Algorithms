/// This represents a modified form of the Activity Selector algorithm in CLRS
/// chapter 16.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod l2s_algo;

fn main() {

    let file = File::open("act.txt").expect("No File Found");

    let mut buf = BufReader::new(file);

    let mut tasks = Vec::<l2s_algo::l2s_algo::Task>::new();
    let mut sets = Vec::new();
    let mut set = 0;
  
    // Loop until the file is empty
    loop {

        let mut cur_str = String::new();

        // Get the header line
        let line_size = buf.read_line(&mut cur_str).expect("Empty String");

        // EOF check
        if line_size == 0 { break; }

        // Set the number of jobs to schedule
        let jobs = cur_str.trim().parse::<u32>().expect("Malformed Job Size");

        // Clear the buffer
        cur_str.clear();

        for _ in 0..jobs {

            // Read the next line
            buf.read_line(&mut cur_str).expect("Empty String");

            // Split the string
            let job = cur_str.trim().split_whitespace().collect::<Vec<&str>>();

            // If the string is not what we expect, panic
            if job.len() != 3 { panic!("Malformed File")};

            // Store the job
            let task = l2s_algo::l2s_algo::Task {
                        id      : job[0].parse::<u32>().unwrap(),
                        start   : job[1].parse::<u32>().unwrap(),
                        end     : job[2].parse::<u32>().unwrap(),
            };

            // Add to the list
            tasks.push(task);

            // Clear
            cur_str.clear();

        }

        // Add to the list of sets
        sets.push(tasks.clone());

        // Clear the task list
        tasks.clear();
    }
    
    // Process each 
    for task in sets {
        // Increment the set number
        set += 1;

        // Start the clock
        let start = Instant::now();

        // Run the algo
        let schedule = l2s_algo::l2s_algo::l2s_algo(task);

        // Stop the clock
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
