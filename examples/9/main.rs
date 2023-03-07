struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x < 0 {
            true => false,
            false => x.to_string().chars().rev().eq(x.to_string().chars()),
        }
    }
}

fn main() {
    let temp = Solution::is_palindrome(121);
    println!("{:#?}", temp)
}
