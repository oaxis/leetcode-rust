// Benchmark : 
// Runtime faster than 100.00%
// Memory usage : not stable between 2.6MB 66% and 2.8MB 19%

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::with_capacity(n as usize);
        let is_div = |a, b| a % b == 0;

        for i in 1..n + 1 {
            answer.push(
                if is_div(i, 15) {
                    "FizzBuzz".to_string()
                } else if is_div(i, 3) {
                    "Fizz".to_string()
                } else if is_div(i, 5) {
                    "Buzz".to_string()
                } else {
                    i.to_string()
                })
        }
        answer
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}
