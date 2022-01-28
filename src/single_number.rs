pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    for n in nums {
        ans = ans ^ n;
    }
    return ans;
}
