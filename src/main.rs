use std::collections::HashSet;

fn main() {
    println!("{:?}", Solution::four_sum(vec![0,0,0,0], 0));
}

struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut temp = nums;
        temp.sort();
        let mut numbers = temp;
        let mut ans: std::collections::HashSet<Vec<i32>> = std::collections::HashSet::new();

        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                let mut k = j + 1;
                let mut l = numbers.len() - 1;
                for k in (j + 1)..numbers.len() {
                    for l in (k + 1)..numbers.len() {
                        // println!("{:?}", vec![numbers[i], numbers[j], numbers[k], numbers[l]]);
                        if (numbers[k] + numbers[l] + numbers[j] + numbers[i]) == target && ans.contains(&vec![numbers[i], numbers[j], numbers[k], numbers[l]]) == false {
                            ans.insert(vec![numbers[i], numbers[j], numbers[k], numbers[l]]);
                        }
                    }
                }
            }
        }
        // println!("{:?}", ans.iter().into_iter());

        return ans.iter().into_iter().map(|x| x.clone()).collect();
    }
}
