fn forming_magic_square(s: &[Vec<i32>]) -> i32 {
    // All possible 3x3 magic squares (8 variations through rotations and reflections)
    let magic_squares = vec![
        vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]],
        vec![vec![6, 1, 8], vec![7, 5, 3], vec![2, 9, 4]],
        vec![vec![4, 9, 2], vec![3, 5, 7], vec![8, 1, 6]],
        vec![vec![2, 9, 4], vec![7, 5, 3], vec![6, 1, 8]],
        vec![vec![8, 3, 4], vec![1, 5, 9], vec![6, 7, 2]],
        vec![vec![4, 3, 8], vec![9, 5, 1], vec![2, 7, 6]],
        vec![vec![6, 7, 2], vec![1, 5, 9], vec![8, 3, 4]],
        vec![vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]],
    ];

    let mut min_cost = i32::MAX;

    // Try each possible magic square
    for magic_square in &magic_squares {
        let mut cost = 0;

        // Calculate cost to transform input to this magic square
        for i in 0..3 {
            for j in 0..3 {
                cost += (s[i][j] - magic_square[i][j]).abs();
            }
        }

        min_cost = min_cost.min(cost);
    }

    min_cost
}
