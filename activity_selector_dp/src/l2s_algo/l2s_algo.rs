/// The Last-to-Start Algorithm module containing the necessary components to
/// build a schedule.
///

/// A Task containing the id, and start and end times
#[derive(Clone, Copy, Debug)]
pub struct Task {
    // Id
    id: u32,
    // Start Time
    start: u32,
    // End Time
    end: u32,
}

/// The Last to Start algorithm
pub fn l2s_algo(list: Vec<Task>) -> Vec<u32> {

    let mut schedule = Vec::new();

    // Sort
    let sorted = merge_sort(list);

    match sorted {
        // Add the first task to th schedule
        Some(sched) => schedule.push(sched[0].id),

        // Error
        None    => println!("Error: Sort Failed"),
    }

    // Build the schedule
    

    schedule
}

/// Implementation of Merge Sort
fn merge_sort(mut arr: Vec<Task>) -> Option<Vec<Task>> {

    let mid: usize = arr.len() / 2;
    if mid == 0 {
        ()
    }

    merge_sort(arr[..mid].to_vec());
    merge_sort(arr[mid..].to_vec());

    let mut ret = arr.clone();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.copy_from_slice(&ret);

    Some(arr)
    
}

/// Merge for Merge Sort
fn merge(left_arr: &[Task], right_arr: &[Task], ret: &mut [Task]) {

    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while (left < left_arr.len()) && (right < right_arr.len()) {
        // Sort the sub-arrays
        if left_arr[left].end >= right_arr[right].end {
            ret[index] = left_arr[left];
            index += 1;
            left += 1;
        }
        else {
            ret[index] = right_arr[right];
            index += 1;
            right += 1;
        }

    }

    if left < left_arr.len() {
        ret[index..].copy_from_slice(&left_arr[left..]);
    }
    if right < right_arr.len() {
        ret[index..].copy_from_slice(&right_arr[right..]);
    }

}
