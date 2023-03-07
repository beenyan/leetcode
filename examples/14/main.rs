struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut header = "".to_string();
        let mut index = 0;

        loop {
            if index >= strs[0].len() {
                break;
            }

            header.push_str(&strs[0].chars().nth(index).unwrap().to_string());

            if let Some(_) = strs.iter().find(|&str| !str.starts_with(&header)) {
                header.pop();
                break;
            }

            index += 1;
        }

        return header.to_string().to_string();
    }
}

fn main() {
    let temp = Solution::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    println!("{:#?}", temp)
}
