impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Convert the string to lowercase, then:
        // 1. Break it into characters
        // 2. Keep only alphanumeric characters (letters + digits)
        // 3. Collect into a vector for indexed access
        let alphanums: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        // If the filtered string is not empty, check palindrome property
        if alphanums.len() != 0 {
            // Initialize two pointers:
            // l -> start of vector
            // r -> end of vector
            let mut l: usize = 0;
            let mut r: usize = alphanums.len() - 1;

            // Move inward while characters match
            while l < r && alphanums[l] == alphanums[r] {
                l += 1;
                r -= 1;
            }

            // If mismatch found, it's not a palindrome
            if alphanums[l] != alphanums[r] {
                return false;
            }
        } else {
            // Empty or non-alphanumeric-only string is considered a palindrome
            return true;
        }

        // If all checks passed, it's a palindrome
        true

        // TODO: Improve time complexity and memory usage
        // Current approach:
        // - Time: O(n) (plus extra overhead from allocations)
        // - Space: O(n) due to Vec<char>
        //
        // Possible improvement:
        // - Use two-pointer technique directly on original string
        // - Avoid allocating Vec<char>
        // - Avoid full lowercase + collect step by processing chars inline
    }
}