struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let mut r = time.iter().min().unwrap().clone() as i64 * total_trips;
        let mut l = 1 as i64;

        while l < r {
            let m = l + ((r - l) >> 1);
            let trips = time
                .iter()
                .fold(0, |total, &trip_time| total + m / trip_time as i64);

            if trips < total_trips {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l
    }
}

fn main() {
    let temp = Solution::minimum_time(vec![1, 2, 3], 5);
    println!("{:#?}", temp)
}
