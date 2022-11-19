pub fn find132patteren(nums: Vec<i32>) -> bool {
    let mut res = false;
    let mut min = std::i32::MAX;

    for i in 0..(nums.len() - 1) {
        min = min.min(nums[i]);
        for j in (i + 1)..nums.len() {
            if nums[j] < nums[i] && min < nums[j] {
                res = true
            }
        }
    }
    res
}
