use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();

        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let k = k as usize;
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        for (&num, &freq) in &freq_map {
            min_heap.push(Reverse((freq, num)));
            
            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        let mut output: Vec<i32> = Vec::new();
        while let Some(Reverse((_, num))) = min_heap.pop() {
            output.push(num);
        }
        output   
    }
}
