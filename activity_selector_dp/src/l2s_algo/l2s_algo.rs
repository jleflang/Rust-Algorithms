/// The Last-to-Start Algorithm module containing the necessary components to
/// build a schedule.
//
use std::cmp::Ordering;

/// A Task containing the id, and start and end times
#[derive(Debug, Clone, Copy, Eq)]
pub struct Task {
    // Id
    pub id: u32,
    // Start Time
    pub start: u32,
    // End Time
    pub end: u32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.end)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.end
    }
}

/// The Last to Start algorithm
pub fn l2s_algo(mut list: Vec<Task>) -> Vec<u32> {

    let mut schedule = Vec::new();
    let mut k = 0;

    // Sort
    merge_sort(&mut list);

    // Add the first task to th schedule
    schedule.push(list[k].id);

    // Build the schedule
    for m in 1..list.len() {
        // If the start time of m task is later, take m
        if list[k] >= list[m] {
            // Put into the schedule
            schedule.push(list[m].id);

            k = m;
        }
    } 

    schedule
}

/// Implementation of Merge Sort
fn merge_sort(arr: &mut [Task]) {

    let mid = f32::floor(arr.len() as f32 / 2.0) as usize;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret: Vec<Task> = arr.to_vec();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.copy_from_slice(&ret);
    
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
