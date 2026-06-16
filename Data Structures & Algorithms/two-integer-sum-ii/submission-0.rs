impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let nums: Vec<i32> = numbers;
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        let mut sum: i32;

        while l < r {
            sum = nums[l]+nums[r];

            if sum == target {
                return vec![((l+1) as i32),((r+1) as i32)]
            } else if sum > target {
                    r -= 1;
                } else {
                    l += 1;
                }
        }
        vec![]
    }
}
