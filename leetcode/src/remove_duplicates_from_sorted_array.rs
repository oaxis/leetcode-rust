// Benchmark : 
// Runtime faster than 100.00%
// Memory usage : 2.2 MB less than 61.16%

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        if len < 2 {
            return len as i32
        } else {

            let mut previous = 0;
            for i in 1..nums.len() {
                if nums[previous] != nums[i] {
                    previous += 1;
                    nums[previous] = nums[i];
                }
            }
            (previous + 1) as i32
            
        }
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
    assert_eq!(Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), 5);
}
