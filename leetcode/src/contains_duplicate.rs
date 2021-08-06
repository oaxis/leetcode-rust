// Benchmark : 
// Runtime faster than 73.87%
// Memory usage : 2.6 MB less than 78.38%

// NOT THE OPTIMAL SOLUTION, a for in with hashmap is optimal but I liked the idea of a one liner.
// HashSet guarantee no duplicates so we can compare both len but doing so means we have the worst O(n) solution
// as it will process all nums values even if both duplicates are at the beginning of the vec

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != nums.into_iter().collect::<std::collections::HashSet<i32>>().len()
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::contains_duplicate(vec![2, 7, 11, 15]), false);
    assert_eq!(Solution::contains_duplicate(vec![11, 7, 11, 15]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 7, 11, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![6, 7, 5, 5]), true);
}
