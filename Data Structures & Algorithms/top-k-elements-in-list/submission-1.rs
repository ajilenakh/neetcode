use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() +1];

        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        } 
        
        for (&num, &freq) in &freq_map {
            buckets[freq as usize].push(num);
        }


        let mut output: Vec<i32> = Vec::new();
        for bucket in buckets.iter().rev() {
            for &val in bucket {
                output.push(val);

                if output.len() == k as usize {
                    return output;
                }
            }
        }

    output 
    }
}
