fn making_anagrams(s1: &str, s2: &str) -> i32 {
    let mut answer = 0;

    if s1 == s2 {
        return answer;
    }

    let s1 = s1.chars().into_iter().collect::<Vec<_>>();
    let s2 = s2.chars().into_iter().collect::<Vec<_>>();

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

    println!("---");
    println!("left: {:?}", chunk_l);
    println!("right: {:?}", chunk_r);

    for index in 0..alphabet.count() {
        let diff = chunk_l[index].len() as i32 - chunk_r[index].len() as i32;

        answer += diff.abs();
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_making_anagrams_01() {
        let result = making_anagrams("cde", "abc");
        assert_eq!(result, 4);
    }
}
