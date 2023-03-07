struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .flat_map(|(x_index, &x)| {
                nums.iter()
                    .skip(x_index + 1)
                    .enumerate()
                    .find_map(|(y_index, &y)| {
                        if x + y == target {
                            Some(vec![x_index as i32, (y_index + x_index + 1) as i32])
                        } else {
                            None
                        }
                    })
            })
            .next()
            .unwrap()
    }
}

fn main() {
    let temp = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:#?}", temp)
}
