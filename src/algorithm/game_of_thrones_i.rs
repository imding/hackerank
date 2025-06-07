fn game_of_thrones(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();

    chars.sort();

    let (chunks, _) = chars
        .into_iter()
        .fold((Vec::with_capacity(26), '0'), |mut acc, char| {
            if char != acc.1 {
                let chunk = vec![char];

                acc.0.push(chunk);
                acc.1 = char;
            } else {
                match acc.0.last_mut() {
                    Some(chunk) => chunk.push(char),
                    _ => panic!("Expect previous chunk to be valid."),
                };
            }

            acc
        });
    let mut odds = 0;

    for chunk in chunks {
        if odds > 1 {
            break;
        }

        if chunk.len() % 2 > 0 {
            odds += 1;
        }
    }

    match odds {
        0 | 1 => "YES",
        _ => "NO",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_of_thrones_01() {
        let result = game_of_thrones("aaabbbb");
        assert_eq!(result, "YES")
    }

    #[test]
    fn game_of_thrones_02() {
        let result = game_of_thrones("cdefghmnopqrstuvw");
        assert_eq!(result, "NO")
    }

    #[test]
    fn game_of_thrones_03() {
        let result = game_of_thrones("cdcdcdcdeeeef");
        assert_eq!(result, "YES")
    }
}
