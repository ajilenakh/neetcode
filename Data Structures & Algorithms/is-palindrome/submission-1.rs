impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Step 1: Normalize the string
        // - Convert all characters to lowercase
        // - Keep only alphanumeric characters (a-z, 0-9)
        // - Store result in a vector for indexed access
        let alphanums: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        // Edge case: if no valid characters remain, it's considered a palindrome
        if alphanums.is_empty() {
            return true;
        }

        // Step 2: Use two-pointer technique
        // l starts from beginning, r starts from end
        let mut l: usize = 0;
        let mut r: usize = alphanums.len() - 1;

        // Step 3: Move pointers inward while characters match
        // If any mismatch happens, loop exits early
        while l < r && alphanums[l] == alphanums[r] {
            l += 1;
            r -= 1;
        }

        // Step 4: Final validation
        // If pointers stopped on a mismatch, it's not a palindrome
        if alphanums[l] != alphanums[r] {
            return false;
        }

        // Otherwise, all characters matched correctly
        return true;

        // TODO:
        // - Reduce space complexity from O(n) by avoiding Vec<char>
        // - Use direct two-pointer traversal on the original string
        // - Avoid full allocation and possibly avoid full lowercase conversion
    }
}