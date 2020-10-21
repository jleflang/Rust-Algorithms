/// This represents a modified form of the Activity Selector algorithm in CLRS
/// chapter 16.
use std::fs::File;
use std::io::{BufRead, BufReader};

mod l2s_algo;

fn main() {

    let file = File::open("act.txt").expect("No File Found");

    let buf = BufReader::new(file);

    let mut tasks = Vec::<l2s_algo::l2s_algo::Task>::new();
    let mut sched_set = false;

    for (i, line) in buf.lines().enumerate() {

        let mut set = 0;

        if !sched_set {
            set = line.unwrap().parse::<u32>().unwrap();
            set -= i as u32;
            sched_set = true;

        } else {
            if set == 0 {
                // Run the algo
                let schedule = l2s_algo::l2s_algo::l2s_algo(tasks);

            } else {
                // Split the string
                let unstruc = line.unwrap().split_whitespace()
                                  .collect::<Vec<&str>>(); 

                // Add the task
                let task = l2s_algo::l2s_algo::Task {
                            id      : unstruc[0].parse::<u32>().unwrap(),
                            start   : unstruc[1].parse::<u32>().unwrap(),
                            end     : unstruc[2].parse::<u32>().unwrap(),
                };

                tasks.push(task);

                // Decrement set
                set -= 1;
            }
        }
    }    
    
}
