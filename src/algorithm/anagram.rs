fn anagram(s: &str) -> i32 {
    let mut answer = -1;

    if s.len() % 2 > 0 {
        return answer;
    }

    let half = s.len() / 2;
    let (left, right) = s.split_at(half);
    let mut left = left.chars().collect::<Vec<_>>();
    let mut right = right.chars().collect::<Vec<_>>();

    left.sort();
    right.sort();

    let left_ = left.iter().collect::<String>();
    let right_ = right.iter().collect::<String>();

    answer += 1;

    if left_ == right_ {
        return answer;
    }

    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let (chunk_l, chunk_r) = alphabet.clone().fold(
        (Vec::with_capacity(26), Vec::with_capacity(26)),
        |mut acc, letter| {
            acc.0.push(
                left.iter()
                    .filter(|char| **char == letter)
                    .collect::<Vec<_>>(),
            );
            acc.1.push(
                right
                    .iter()
                    .filter(|char| **char == letter)
                    .collect::<Vec<_>>(),
            );

            acc
        },
    );

    println!("---");
    println!("left: {:?}", chunk_l);
    println!("right: {:?}", chunk_r);

    for index in 0..alphabet.count() {
        let diff = chunk_l[index].len() as i32 - chunk_r[index].len() as i32;

        answer += diff.abs();
    }

    match answer {
        -1 | 0 => answer,
        _ => answer / 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_01() {
        let result = anagram("xyyx");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_02() {
        let result = anagram("hhpddlnnsjfoyxpciioigvjqzfbpllssuj");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_03() {
        let result = anagram("xulkowreuowzxgnhmiqekxhzistdocbnyozmnqthhpievvlj");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_04() {
        let result = anagram("dnqaurlplofnrtmh");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_05() {
        let result = anagram(
            "aujteqimwfkjoqodgqaxbrkrwykpmuimqtgulojjwtukjiqrasqejbvfbixnchzsahpnyayutsgecwvcqngzoehrmeeqlgknnb",
        );
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_06() {
        let result =
            anagram("lbafwuoawkxydlfcbjjtxpzpchzrvbtievqbpedlqbktorypcjkzzkodrpvosqzxmpad");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_07() {
        let result = anagram("drngbjuuhmwqwxrinxccsqxkpwygwcdbtriwaesjsobrntzaqbe");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_anagram_08() {
        let result = anagram("ubulzt");
        assert_eq!(result, -1);
    }
}
