// Benchmark : 
// Runtime faster than 100.00%
// Memory usage : 2.2 MB less than 21.65%

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = std::collections::HashMap::with_capacity(7);
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        let iter = s.chars().rev();

        // For
        let mut total = 0;
        let mut last = 0;
        for rn in iter {
            let current = *map.get(&rn).unwrap();
            total += if current >= last { current } else { -current };
            last = current;
        }
        total
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
