struct Solution;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let alice_diff = (alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>()) / 2;
        let bob_box: std::collections::HashSet<_> = bob_sizes.into_iter().collect();

        for box_size in alice_sizes {
            let needed_box = box_size - alice_diff;

            if bob_box.contains(&needed_box) {
                return vec![box_size, needed_box];
            }
        }

        unreachable!();
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
    assert_eq!(Solution::fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
    assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    assert_eq!(Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4]), vec![5, 4]);
}


/* O(n²) bruteforce solution

pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let disparity: i32 = (alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>()) / 2;
        let mut answer: Vec<i32> = vec![0; 2];

        'bruteforce: for (ia, a) in alice_sizes.iter().enumerate() {
            for (ib, b) in bob_sizes.iter().enumerate() {
                if alice_sizes[ia] - bob_sizes[ib] == disparity {
                    answer = vec![*a, *b];
                    break 'bruteforce;
                }
            }
        }
        answer
    }
*/