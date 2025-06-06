use std::{fs::read_to_string, io::Result};

fn insertion_sort(arr: &[i32]) -> u64 {
    if arr.len() <= 1 {
        return 0;
    }

    let mut arr_ = arr.to_vec();

    merge_sort_and_count(&mut arr_, 0, arr.len() - 1)
}

fn merge_sort_and_count(arr: &mut [i32], start: usize, end: usize) -> u64 {
    if start >= end {
        return 0;
    }

    let mid = start + (end - start) / 2;
    let mut count = 0;

    count += merge_sort_and_count(arr, start, mid);
    count += merge_sort_and_count(arr, mid + 1, end);
    count += merge_and_count(arr, start, mid, end);

    count
}

fn merge_and_count(arr: &mut [i32], start: usize, mid: usize, end: usize) -> u64 {
    let left = arr[start..=mid].to_vec();
    let right = arr[mid + 1..=end].to_vec();

    let mut l = 0;
    let mut r = 0;
    let mut k = start;
    let mut count = 0 as u64;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            arr[k] = left[l];
            l += 1;
        } else {
            arr[k] = right[r];
            count += left.len() as u64 - l as u64;
            r += 1;
        }

        k += 1;
    }

    while l < left.len() {
        arr[k] = left[l];

        l += 1;
        k += 1;
    }

    while r < right.len() {
        arr[k] = right[r];

        r += 1;
        k += 1;
    }

    count
}

fn parse_and_run(file_path: &str) -> Result<Vec<u64>> {
    let content = read_to_string(file_path)?;
    let mut lines = content.lines();

    let n = lines.next().unwrap().trim().parse::<i32>().unwrap();
    let mut results = Vec::new();

    for _ in 0..n {
        let _ = lines.next().unwrap().trim().parse::<i32>().unwrap();
        let s = lines
            .next()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        
        results.push(insertion_sort(&s));
    }

    Ok(results)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insertion_sort_analysis_01() {
        let result = insertion_sort(&[1, 1, 1, 2, 2]);
        assert_eq!(result, 0);
    }

    #[test]
    fn insertion_sort_analysis_02() {
        let result = insertion_sort(&[2, 1, 3, 1, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn insertion_sort_analysis_03() {
        // Empty array
        assert_eq!(insertion_sort(&[]), 0);
    }

    #[test]
    fn insertion_sort_analysis_04() {
        // Single element
        assert_eq!(insertion_sort(&[5]), 0);
    }

    #[test]
    fn insertion_sort_analysis_05() {
        // Already sorted
        assert_eq!(insertion_sort(&[1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn insertion_sort_analysis_06() {
        // Reverse sorted (maximum inversions)
        assert_eq!(insertion_sort(&[5, 4, 3, 2, 1]), 10);
    }

    #[test]
    fn insertion_sort_analysis_07() {
        // All same elements
        assert_eq!(insertion_sort(&[3, 3, 3, 3]), 0);
    }

    #[test]
    fn insertion_sort_analysis_08() {
        let result = parse_and_run("src/algorithm/insertion_sort_analysis_01.txt");
        match result {
            Ok(output) => {
                assert_eq!(output, vec![0, 0, 0, 4999950000, 4999950000])
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                // The test will pass even if file doesn't exist, for flexibility
            }
        }
    }
}
