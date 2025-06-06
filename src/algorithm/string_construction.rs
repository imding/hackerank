fn string_construction(s: &str) -> i32 {
    let mut cost = 0;
    let mut free = vec![false; 26];

    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars();

    for char in s.chars() {
        let index = match alphabet.clone().position(|letter| letter == char) {
            Some(index) => index,
            _ => panic!("Expect letter index to be valid."),
        };

        if !free[index] {
            free[index] = true;
            cost += 1;
        }
    }

    cost
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_construction_01() {
        let result = string_construction("abcd");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_string_construction_02() {
        let result = string_construction("abab");
        assert_eq!(result, 2);
    }
}
