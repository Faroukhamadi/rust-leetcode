pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let matrix = matrix.concat();
    return search(&matrix, target).is_some();
}

fn search(nums: &[i32], target: i32) -> Option<i32> {
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1).try_into().unwrap();
    while left <= right {
        let mid = left + (right - left) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }
    None
}
