/// This performs three different sorts (merge, insert, and stooge) and gets the
/// wall time performance of each and prints the output to file.
/// This is purely a standard academic exercise and thus may not be 
/// optimal/correct for production use.
use std::time::SystemTime;
use rand::{thread_rng, Rng, distributions::Uniform};

fn main() {

    let size_pool = [100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
    
    // Print the header
    println!("{: <10}\t| {: <10}\t| {: <10}\t| {: <10}", 
            "# of elements", "Time", "", "");
    println!("{: <10}\t| {: <10}\t| {: <10}\t| {: <10}", 
            "", "Insert", "Merge", "Stooge");

    for i in size_pool.iter() {

        // Get some RNG
        let rng = thread_rng();
        let range = Uniform::from(0..10001);

        // Create & Fill the array
        let array: Vec<u32> = rng.sample_iter(&range).take(*i).collect();

        // Run Insert Sort
        let mut start_time = SystemTime::now();
        insert_sort(&mut array.clone());
        let since_sort_insert = start_time.elapsed();

        // Run Merge Sort
        start_time = SystemTime::now();
        merge_sort(&mut array.clone());
        let since_sort_merge = start_time.elapsed();

        // Run Stooge Sort
        start_time = SystemTime::now();
        stooge_sort(&mut array.clone());
        let since_sort_stooge = start_time.elapsed();

        // Print the results
        println!("{: <10}\t| {: <10.3?}\t| {: <10.3?}\t| {: <10.3?}", 
                    *i, since_sort_insert.unwrap(), since_sort_merge.unwrap(), 
                        since_sort_stooge.unwrap());

    }

}

/// Implementation of Insert Sort
fn insert_sort(arr: &mut Vec<u32>) {

    for index in 1..arr.len() {
        let mut j = index;

        while (j > 0) && (arr[j - 1] < arr[j]) {
            arr.swap(j, j - 1);

            j -= 1;
        }
    }

}

/// Implementation of Merge Sort
fn merge_sort(arr: &mut Vec<u32>) {

    let mid = f32::floor(arr.len() as f32 / 2.0) as usize;

    if mid == 0 { return; }

    merge_sort(&mut arr[..mid].to_vec());
    merge_sort(&mut arr[mid..].to_vec());

    let mut ret = arr.clone();

    merge(&arr[..mid], &arr[mid..], &mut ret[..]);

    arr.copy_from_slice(&ret);
    
}

/// Merge for Merge Sort
fn merge(left_arr: &[u32], right_arr: &[u32], ret: &mut [u32]) {

    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while (left < left_arr.len()) && (right < right_arr.len()) {
        // Sort the sub-arrays
        if left_arr[left] >= right_arr[right] {
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

/// Implementation of Stooge Sort
fn stooge_sort(arr: &mut Vec<u32>) {

    let len = arr.len();

    if len <= 2 {
        return;
    }

    if arr[0] > arr[len - 1] {
        arr.swap(0, len - 1); 
    }

    if len > 2 {
        let m =  f32::floor(len as f32 / 3.0) as usize;

        stooge_sort(&mut arr[..len-m].to_vec());
        stooge_sort(&mut arr[m..len].to_vec());
        stooge_sort(&mut arr[..len-m].to_vec());
    }
}
