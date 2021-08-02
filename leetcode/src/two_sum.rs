// Benchmark : 
// Runtime faster than 100.00%
// Memory usage : 2.3 MB less than 35.88%

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = std::collections::HashMap::with_capacity(nums.len());
        let mut answer = vec![0, 0];
        // We store each values in hashmap
        // For each number we then check if, when substracted by target, we find a corresponding number
        // in our hashmap, if yes, then this found number and the one we substracted are the two answer index
        for (i, _n) in nums.iter().enumerate() {
            if cache.get(&(target - nums[i])) != None {
                answer = vec![*cache.get(&(target - nums[i])).unwrap(), i as i32];
            } else {
                cache.insert(nums[i], i as i32);
            }
        }
        answer
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
    assert_eq!(Solution::two_sum(vec![10, 8, 12, 6], 22), [0, 2]);
}
