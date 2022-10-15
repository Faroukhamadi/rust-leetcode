use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut occurs = HashMap::new();

    for n in nums {
        match occurs.insert(n, n) {
            Some(_) => return true,
            None => (),
        }
    }
    false
}
