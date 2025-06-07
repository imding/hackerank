#[derive(Debug, Clone)]
pub struct SpiralData<T> {
    layers: Vec<Vec<T>>,
    positions: Vec<Vec<(usize, usize)>>, // (row, col) for each element in each layer
    width: usize,
    height: usize,
}

impl<T: Clone> SpiralData<T> {
    pub fn from_matrix(matrix: Vec<Vec<T>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            return SpiralData {
                layers: vec![],
                positions: vec![],
                width: 0,
                height: 0,
            };
        }

        let height = matrix.len();
        let width = matrix[0].len();
        let mut layers = Vec::new();
        let mut positions = Vec::new();

        let mut top = 0;
        let mut bottom = height - 1;
        let mut left = 0;
        let mut right = width - 1;

        while top <= bottom && left <= right {
            let mut layer = Vec::new();
            let mut layer_positions = Vec::new();

            // Top row: left to right
            for col in left..=right {
                layer.push(matrix[top][col].clone());
                layer_positions.push((top, col));
            }

            // Right column: top to bottom (excluding top corner)
            for row in (top + 1)..=bottom {
                layer.push(matrix[row][right].clone());
                layer_positions.push((row, right));
            }

            // Bottom row: right to left (excluding right corner)
            if bottom > top {
                for col in (left..right).rev() {
                    layer.push(matrix[bottom][col].clone());
                    layer_positions.push((bottom, col));
                }
            }

            // Left column: bottom to top (excluding both corners)
            if right > left {
                for row in ((top + 1)..bottom).rev() {
                    layer.push(matrix[row][left].clone());
                    layer_positions.push((row, left));
                }
            }

            layers.push(layer);
            positions.push(layer_positions);

            // Move to inner layer
            top += 1;
            bottom = bottom.saturating_sub(1);
            left += 1;
            right = right.saturating_sub(1);
        }

        SpiralData {
            layers,
            positions,
            width,
            height,
        }
    }

    pub fn slide(&mut self, r: usize) {
        for index in 0..self.layers.len() {
            let len = self.layers[index].len();

            self.layers[index].rotate_left(r % len);
        }
    }

    pub fn to_matrix(&self) -> Vec<Vec<T>>
    where
        T: Default,
    {
        let mut matrix = vec![vec![T::default(); self.width]; self.height];

        // Direct assignment - no loops or spiral traversal!
        for (layer_idx, layer) in self.layers.iter().enumerate() {
            let layer_positions = &self.positions[layer_idx];

            for (element, &(row, col)) in layer.iter().zip(layer_positions.iter()) {
                matrix[row][col] = element.clone();
            }
        }

        matrix
    }

    // Alternative: even faster with unsafe (if you need maximum performance)
    pub fn to_matrix_unsafe(&self) -> Vec<Vec<T>>
    where
        T: Default + Clone,
    {
        let mut matrix = vec![vec![T::default(); self.width]; self.height];

        for (layer_idx, layer) in self.layers.iter().enumerate() {
            let layer_positions = &self.positions[layer_idx];

            for (element, &(row, col)) in layer.iter().zip(layer_positions.iter()) {
                // Skip bounds checking since we know positions are valid
                unsafe {
                    *matrix.get_unchecked_mut(row).get_unchecked_mut(col) = element.clone();
                }
            }
        }

        matrix
    }
}
