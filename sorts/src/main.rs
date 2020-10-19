/// This performs three different sorts (merge, insert, and stooge) and gets the
/// wall time performance of each and prints the output to file.
/// This is purely a standard academic exercise and thus may not be 
/// optimal/correct for production use.
use std::time::SystemTime;
use rand::{thread_rng, Rng};

fn main() {

    let size_pool = [100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
    
    // Print the header
    println!("{: <10}\t| {: <10}\t| {: <10}\t| {: <10}", 
            "# of elements", "Time", "", "");
    println!("{: <10}\t| {: <10}\t| {: <10}\t| {: <10}", 
            "", "Insert", "Merge", "Stooge");

    for i in size_pool.iter() {

        // Create an empty array
        let mut array: Vec<u32> = Vec::with_capacity(*i);

        // Get some RNG
        let mut rng = thread_rng();

        // Fill the array
        for _ in 0..array.capacity() {
            array.push(rng.gen_range(0, 10000));
        }

        // Run Insert Sort
        let mut start_time = SystemTime::now();
        insert_sort(array.clone());
        let since_sort_insert = start_time.elapsed();

        // Run Merge Sort
        start_time = SystemTime::now();
        merge_sort(array.clone());
        let since_sort_merge = start_time.elapsed();

        // Run Stooge Sort
        start_time = SystemTime::now();
        stooge_sort(array.clone());
        let since_sort_stooge = start_time.elapsed();

        // Print the results
        println!("{: <10}\t| {: <10.3?}\t| {: <10.3?}\t| {: <10.3?}", *i, 
                                            since_sort_insert.unwrap(), 
                                            since_sort_merge.unwrap(), 
                                            since_sort_stooge.unwrap());

    }

}

/// Implementation of Insert Sort
fn insert_sort(mut arr: Vec<u32>) {

    for index in 1..arr.len() {
        let mut j = index;

        while (j > 0) && (arr[j - 1] < arr[j]) {
            let tmp = arr[j];
            arr[j] = arr[j - 1];
            arr[j - 1] = tmp;

            j -= 1;
        }
    }

}

/// Implementation of Merge Sort
fn merge_sort(mut arr: Vec<u32>) {

    let mid: usize = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(arr[..mid].to_vec());
    merge_sort(arr[mid..].to_vec());

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
fn stooge_sort(mut arr: Vec<u32>) {

    let len = arr.len();

    if len <= 2 {
        return;
    }

    if arr[0] > arr[len - 1] {
        let tmp = arr[0];
        arr[0] = arr[len - 1];
        arr[len - 1] = tmp; 
    }

    if len > 2 {
        let m = len / 3 + (len % 3 != 0) as usize;

        stooge_sort(arr[..len-m].to_vec());
        stooge_sort(arr[m..len].to_vec());
        stooge_sort(arr[..len-m].to_vec());
    }
}
