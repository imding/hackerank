use std::collections::HashMap;

fn acm_icpc_team(topic: &[String]) -> Vec<i32> {
    let mut map = HashMap::<String, i32>::new();

    for a in topic {
        for b in topic {
            if a == b {
                continue;
            }

            let combined = a
                .chars()
                .zip(b.chars())
                .map(|v| match v {
                    ('1', _) | (_, '1') => '1',
                    _ => '0',
                })
                .collect::<String>();

            let key = format!("{}-{}", a, b);
            let maybe_count = map.get_mut(&key);

            match maybe_count {
                Some(count) => {
                    *count += 1;
                }
                _ => {
                    map.insert(key, 1);
                }
            };
        }
    }

    let mut best = map
        .into_iter()
        .map(|(k, v)| (k.replace("0", "").len() as i32, v))
        .collect::<Vec<_>>();

    best.sort();
    best.reverse();

    [best[0].0, best[0].1].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = acm_icpc_team(&[
            "10101".to_string(),
            "11100".to_string(),
            "11010".to_string(),
            "00101".to_string(),
        ]);

        assert_eq!(result, vec![]);
    }
}
