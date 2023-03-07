struct Solution;

const REMAN_LIST: [(&str, i32); 13] = [
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut s = s;
        while s.len() != 0 {
            for (key, val) in REMAN_LIST.iter() {
                if !s.starts_with(key) {
                    continue;
                }

                s = s[key.len()..].to_string();
                ans += val;

                break;
            }
        }

        return ans;
    }
}

fn main() {
    let temp = Solution::roman_to_int("MCMXCIV".to_string());
    println!("{:#?}", temp)
}
