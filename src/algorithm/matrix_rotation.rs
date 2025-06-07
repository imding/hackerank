use crate::algorithm::spiral_data::SpiralData;

fn matrix_rotation(matrix: &[Vec<i32>], r: i32) -> Vec<Vec<i32>> {
    let mut spiral = SpiralData::from_matrix(matrix.to_vec());

    spiral.slide(r as usize);

    let rotated = spiral.to_matrix_unsafe();

    for row in &rotated {
        println!(
            "{}",
            row.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    rotated
}

fn spiral_traversal<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let h = matrix.len();
    let w = matrix[0].len();
    let mut layers = Vec::new();

    let mut t = 0;
    let mut b = h - 1;
    let mut l = 0;
    let mut r = w - 1;

    while t <= b && l <= r {
        let mut layer = Vec::new();

        for x in l..=r {
            layer.push(matrix[t][x].to_owned());
        }

        for y in t + 1..=b {
            layer.push(matrix[y][r].to_owned());
        }

        if b > t {
            for x in (l..r).rev() {
                layer.push(matrix[b][x].to_owned());
            }
        }

        if r > l {
            for y in (t + 1..b).rev() {
                layer.push(matrix[y][l].to_owned());
            }
        }

        layers.push(layer);

        t += 1;
        b = b.saturating_sub(1);
        l += 1;
        r = r.saturating_sub(1);
    }

    layers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matrix_rotation_01() {
        let result = matrix_rotation(
            &[
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            2,
        );
        assert_eq!(
            result,
            vec![
                vec![3, 4, 8, 12],
                vec![2, 11, 10, 16],
                vec![1, 7, 6, 15],
                vec![5, 9, 13, 14],
            ]
        );
    }

    #[test]
    fn matrix_rotation_02() {
        let result = matrix_rotation(
            &[
                vec![1, 2, 3, 4],
                vec![7, 8, 9, 10],
                vec![13, 14, 15, 16],
                vec![19, 20, 21, 22],
                vec![25, 26, 27, 28],
            ],
            7,
        );
        assert_eq!(
            result,
            vec![
                vec![28, 27, 26, 25],
                vec![22, 9, 15, 19],
                vec![16, 8, 21, 13],
                vec![10, 14, 20, 7],
                vec![4, 3, 2, 1],
            ]
        );
    }

    #[test]
    fn matrix_rotation_03() {
        let result = matrix_rotation(&[vec![1, 1], vec![1, 1]], 3);
        assert_eq!(result, vec![vec![1, 1], vec![1, 1]]);
    }
}
