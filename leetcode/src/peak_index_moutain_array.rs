// Benchmark : 
// Runtime faster than 100%
// Memory usage : 2.1 MB less than  73.33%

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut highest = arr.len();
        
        let mut lowest = 0;
        while lowest < highest {
            let middle = (lowest + highest) / 2;
            if arr[middle] < arr[middle + 1]{
                lowest = middle + 1;
            }
            else {
                highest = middle;
            }
        }
        lowest as i32
    }
}

#[test]
fn test_solution() {
    // assert_eq!(Solution::peak_index_in_mountain_array(vec![0,1,0]), 1);
    // assert_eq!(Solution::peak_index_in_mountain_array(vec![0,2,1,0]), 1);
    // assert_eq!(Solution::peak_index_in_mountain_array(vec![0,10,5,2]), 1);
    // assert_eq!(Solution::peak_index_in_mountain_array(vec![3,4,5,1]), 2);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![24,69,100,99,79,78,67,36,26,19]), 2);
}