use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0
        }
        
        let nums_set: HashSet<i32> = nums
            .iter()
            .copied()
            .collect();
        
        let mut count = 0;
        let mut longest_streak  = 0;

        for &num in &nums_set {
            if !nums_set.contains(&(num - 1)) {
                count = 1;
                while nums_set.contains(&(num + count)) {
                    count += 1;
                }
                longest_streak = longest_streak.max(count);
            }
        }

        longest_streak
    }
}
