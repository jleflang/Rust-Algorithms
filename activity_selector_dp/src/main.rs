/// This represents a modified form of the Activity Selector algorithm in CLRS
/// chapter 16.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod l2s_algo;

fn main() {

    let file = File::open("act.txt").expect("No File Found");

    let mut buf = BufReader::new(file);

    let mut sets = Vec::new();
  
    // Loop until the file is empty
    loop {

        let mut cur_str = String::new();

        // Get the header line
        let line_size = buf.read_line(&mut cur_str).expect("Empty String");

        // EOF check
        if line_size == 0 { break; }

        // Set the number of jobs to schedule
        let jobs = cur_str.trim().parse::<usize>().expect("Malformed Job Size");

        // Clear the buffer
        cur_str.clear();

        // Allocate a vector to store all tasks
        let mut tasks = Vec::<l2s_algo::last2start::Task>::with_capacity(jobs);

        for _ in 0..jobs {

            // Read the next line
            buf.read_line(&mut cur_str).expect("Empty String");

            // Split the string
            let job = cur_str.trim().split_whitespace().collect::<Vec<&str>>();

            // If the string is not what we expect, panic
            if job.len() != 3 { panic!("Malformed File")};

            // Store the job
            let task = l2s_algo::last2start::Task {
                        id      : job[0].parse::<u32>().expect("Bad id"),
                        start   : job[1].parse::<u32>().expect("Bad Start"),
                        end     : job[2].parse::<u32>().expect("Bad End"),
            };

            // Add to the list
            tasks.push(task);

            // Clear
            cur_str.clear();

        }

        // Add to the list of sets
        sets.push(tasks);

    }

    // Process each 
    for (i, tasks) in sets.iter_mut().enumerate() {

        // Start the clock
        let start = Instant::now();

        // Run the algo
        let schedule = l2s_algo::last2start::l2s_algo(tasks);

        // Stop the clock
        let dur = start.elapsed();

        // Print out
        println!("Set {}", i + 1);
        println!("Number of activities selected: {}", schedule.len());
        print!("Activities:");

        schedule.iter().rev().for_each(|id| print!(" {}", id));

        println!();

        println!("Done in {:?}", dur);
    }
    
    
}
