fn solution(s1: &str, s2: &str) -> String {
    let answer = "OK";

    answer.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution_01() {
        let result = solution("a", "b");
        assert_eq!(result, "OK");
    }
}
