// Benchmark :
// Runtime faster than 100.00%
// Memory usage : 2 MB less than 92.65%

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        let mut prev = 0;
        for i in 0..len {
            if val != nums[i] {
                nums[prev] = nums[i];
                prev += 1;
            }
        }
        // Edge case to delete val occurence when at the end of array
        // Submit would pass without it but the problem would not be fully respected
        if len > 1 && nums[len - 1] == val {
            nums[len - 1] = 0;
        }
        prev as i32
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(Solution::remove_element(&mut vec![], 2), 0);
    assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
    assert_eq!(Solution::remove_element(&mut vec![1], 2), 1);
    assert_eq!(
        Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
        5
    );
    assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
}
