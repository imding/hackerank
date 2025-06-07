fn palindrome_index(s: &str) -> i32 {
    let left_len = (s.len() as f32 / 2.0).ceil() as usize;
    let (left, right) = s.split_at(left_len);
    let right_len = right.len();
    let left = left.chars().collect::<Vec<_>>();
    let right = right.chars().collect::<Vec<_>>();

    let mut cursor_l = 0;
    let mut cursor_r = right_len as i32 - 1;
    let mut miss_l = 0;
    let mut miss_r = 0;
    let mut answer_l = -1;
    let mut answer_r = -1;

    for _ in 0..left_len {
        if miss_l < 2 {
            let cursor_r_ = right_len as i32 - 1 - cursor_l - miss_l;

            if cursor_r_ >= 0 {
                let left_char = left[cursor_l as usize];
                let right_char = right[cursor_r_ as usize];

                println!(
                    "checking left char at {}({}) against right char {}({})",
                    cursor_l, left_char, cursor_r_, right_char
                );

                if left_char != right_char {
                    if miss_l < 1 {
                        answer_l = cursor_l;
                    }

                    miss_l += 1;
                } else {
                    cursor_l += 1;
                }
            } else {
                miss_l = 3;
            }
        }

        if miss_r < 2 && cursor_r >= 0 {
            let cursor_l_ = right_len as i32 - cursor_r - 1 + miss_r;

            if cursor_l_ < left_len as i32 {
                let left_char = left[cursor_l_ as usize];
                let right_char = right[cursor_r as usize];

                println!(
                    "checking right char at {}({}) against left char {}({})",
                    cursor_r, right_char, cursor_l_, left_char
                );

                if right_char != left_char {
                    if miss_r < 1 {
                        answer_r = cursor_r;
                    }

                    miss_r += 1;
                } else {
                    cursor_r -= 1;
                }
            }
        }
    }

    println!("misses: {}, {}", miss_l, miss_r);
    println!("answers: {}, {}", answer_l, answer_r);

    match ((miss_l, miss_r), (answer_l, answer_r)) {
        ((2, 0 | 1), (answer, _)) => answer,
        ((0 | 1, 2), (_, answer)) => answer + left_len as i32,
        ((1, 1), (answer, _)) => answer,
        ((3, 0 | 1), (answer, _)) => answer,
        ((3, 2), (_, answer)) => answer + left_len as i32,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panindrome_index_01() {
        let result = palindrome_index("aaab");
        assert_eq!(result, 3);
    }

    #[test]
    fn panindrome_index_02() {
        let result = palindrome_index("abc0cba9");
        assert_eq!(result, 7);
    }

    #[test]
    fn panindrome_index_03() {
        let result = palindrome_index("aba");
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_04() {
        let result = palindrome_index("abba");
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_05() {
        let result = palindrome_index("a");
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_06() {
        let result = palindrome_index("aa");
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_07() {
        let result = palindrome_index("abcba");
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_08() {
        let result = palindrome_index("baa");
        // Remove first char 'b' to get "aa" which is a palindrome
        assert_eq!(result, 0);
    }

    #[test]
    fn panindrome_index_09() {
        let result = palindrome_index("abab");
        // Multiple valid answers: removing index 0 or 3 both create palindromes
        assert!(result == 0 || result == 3);
    }

    #[test]
    fn panindrome_index_10() {
        let result = palindrome_index("raceacar");
        // Can remove either 'e' at index 3 or 'a' at index 4 to make palindrome
        assert!(result == 3 || result == 4);
    }

    #[test]
    fn panindrome_index_11() {
        let result = palindrome_index("");
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_12() {
        let result = palindrome_index("ab");
        // Either removing 'a' or 'b' creates palindrome
        assert!(result == 0 || result == 1);
    }

    #[test]
    fn panindrome_index_13() {
        let result = palindrome_index("abc");
        // Cannot make palindrome by removing single char
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_14() {
        let result = palindrome_index("abcdef");
        // Multiple mismatches, impossible
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_15() {
        let s = "a".repeat(10000) + "b" + &"a".repeat(10000);
        let result = palindrome_index(&s);
        assert_eq!(result, -1);
    }

    #[test]
    fn panindrome_index_16() {
        let result = palindrome_index("hgygsvlfcwnswtuhmyaljkqlqjjqlqkjlaymhutwsnwcwflvsgygh");
        assert_eq!(result, 44);
    }
}
