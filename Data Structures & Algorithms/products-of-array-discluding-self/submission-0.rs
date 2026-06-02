impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = Vec::new();
        
        let mut left = 1;
        for i in 0..nums.len() {
            output.push(left);
            left*=nums[(i)];
        }

        let mut right = 1;
        for i in (0..nums.len()).rev() {
            output[i] = output[i]*right;
            right *= nums[i];
        }

        output
    }
}
