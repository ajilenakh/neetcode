use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        // Create a HashSet to keep track of numbers we have already seen.
        // A HashSet automatically stores only unique values.
        let mut seen = HashSet::new(); 
        
        // Loop through each number in the input vector
        for n in nums {
            // Try to insert the number into the set.
            //
            // IMPORTANT:
            // - insert(n) returns:
            //     true  -> if n was NOT already in the set (new value inserted)
            //     false -> if n WAS already in the set (duplicate detected)
            //
            // So:
            // If insert returns false, it means we've seen this number before,
            // which means we found a duplicate → immediately return true.
            if !seen.insert(n) {
                return true;
            }
        }
        
        // If we finish the loop without finding any duplicates,
        // it means all numbers were unique → return false.
        false
    }
}
