pub fn longest_palindrome(s: String) -> String {
    let ca: Vec<char> = s.chars().collect();
    let expand_around_center = |s: String, left: i32, right: i32| -> i32 {
        let mut l = left;
        let mut r = right;
        while l >= 0 && r < s.len() as i32 && ca[l as usize] == ca[r as usize] {
            l -= 1;
            r += 1;
        }
        return r - l - 1;
    };
    if s.is_empty() {
        return String::new();
    }
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    for i in 0..s.len() {
        let len1 = expand_around_center(s.clone(), i as i32, i as i32);
        let len2 = expand_around_center(s.clone(), i as i32, (i + 1) as i32);
        let len = len1.max(len2);
        if len > end - start {
            start = i as i32 - ((len - 1) / 2);
            end = i as i32 + (len / 2);
        }
    }
    return s[(start as usize)..((end + 1) as usize)].to_string();
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn test1() {
        let result = longest_palindrome(String::from("babad"));
        assert_eq!(result, String::from("aba"));
    }

    #[test]
    fn empty() {
        let result = longest_palindrome(String::from(""));
        assert_eq!(result, String::from(""));
    }
}
