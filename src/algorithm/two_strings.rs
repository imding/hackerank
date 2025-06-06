fn two_strings(s1: &str, s2: &str) -> String {
    let mut answer = "NO";

    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();

    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let (chunk_l, chunk_r) = alphabet.clone().fold(
        (Vec::with_capacity(26), Vec::with_capacity(26)),
        |mut acc, letter| {
            acc.0.push(
                s1.iter()
                    .filter(|char| **char == letter)
                    .collect::<Vec<_>>(),
            );
            acc.1.push(
                s2.iter()
                    .filter(|char| **char == letter)
                    .collect::<Vec<_>>(),
            );

            acc
        },
    );

    for index in 0..alphabet.count() {
        if !chunk_l[index].is_empty() && !chunk_r[index].is_empty() {
            answer = "YES";

            break;
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_strings_01() {
        let result = two_strings("hello", "world");
        assert_eq!(result, "YES");
    }

    #[test]
    fn test_two_strings_02() {
        let result = two_strings("hi", "world");
        assert_eq!(result, "NO");
    }
}
