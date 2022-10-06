pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut num_to_idx: HashMap<i32, i32> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        match num_to_idx.get(&(target - *num)) {
            Some(&idx2) => return vec![idx as i32, idx2],
            None => num_to_idx.insert(*num, idx as i32),
        };
    }
    vec![]
}
