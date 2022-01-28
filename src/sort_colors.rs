pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut p0: i32 = 0;
    let mut curr: i32 = 0;
    let mut p2 = (nums.len() - 1) as i32;
    while curr <= p2 {
        if let Some(&elem) = nums.get(curr as usize) {
            if elem == 0 {
                nums.swap(p0 as usize, curr as usize);
                p0 += 1;
                curr += 1;
            } else if elem == 2 {
                nums.swap(p2 as usize, curr as usize);
                p2 -= 1;
            } else {
                curr += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort_colors;

    #[test]
    fn sort_colors_test_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2])
    }

    #[test]
    fn sort_colors_test_2() {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2])
    }

    #[test]
    fn sort_colors_test_3() {
        let mut nums = vec![2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2])
    }

    #[test]
    fn sort_colors_test_4() {
        let mut nums = vec![2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2])
    }
}
