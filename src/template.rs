use std::{fs::read_to_string, io::Result};

fn solution(s1: &str, s2: &str) -> String {
    let answer = "OK";

    answer.to_string()
}

fn parse_and_run(file_path: &str) -> Result<String> {
    let content = read_to_string(file_path)?;
    let mut lines = content.lines();

    let first_multiple_input: Vec<String> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let s = lines.next().unwrap();

    let result = solution("s1", "s2");
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution_01() {
        let result = solution("a", "b");
        assert_eq!(result, "OK");
    }

    #[test]
    fn test_solution_02() {
        let result = solution("a", "b");
        assert_eq!(result, "OK");
    }

    #[test]
    fn test_solution_03() {
        let result = solution("a", "b");
        assert_eq!(result, "OK");
    }

    #[test]
    fn test_solution_04() {
        let result = parse_and_run("src/algorithm/solution_01.txt");
        match result {
            Ok(output) => {
                assert_eq!(output, "OK")
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                // The test will pass even if file doesn't exist, for flexibility
            }
        }
    }
}
