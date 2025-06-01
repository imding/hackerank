use std::cmp::{max, min};

fn queens_attack(n: i32, _k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    if n <= 1 {
        return 0;
    }

    let (n, ne, e, se, s, sw, w, nw) = obstacles.iter().fold(
        (
            n - r_q,               // 0 north
            n - max(r_q, c_q),     // 1 north-east
            n - c_q,               // 2 east
            min(r_q - 1, n - c_q), // 3 south-east
            r_q - 1,               // 4 south
            min(r_q - 1, c_q - 1), // 5 south-west
            c_q - 1,               // 6 west
            min(n - r_q, c_q - 1), // 7 north-west
        ),
        |mut acc, obstacle| {
            let r = obstacle[0];
            let c = obstacle[1];
            let same_row = r == r_q;
            let left = c < c_q;
            let right = c > c_q;
            let atop = r > r_q;
            let below = r < r_q;

            println!("obstacle ({}, {})", r, c);

            if same_row {
                if right {
                    // east
                    acc.2 = min(acc.2, c - c_q - 1);
                    println!("east reduced to {}", acc.2);
                } else if left {
                    // west
                    acc.6 = min(acc.6, c_q - c - 1);
                    println!("west reduced to {}", acc.6);
                }
            } else {
                let same_col = c == c_q;
                let is_diag = (r - r_q).abs() == (c - c_q).abs();

                if atop {
                    if same_col {
                        // north
                        acc.0 = min(acc.0, r - r_q - 1);
                        println!("north reduced to {}", acc.0);
                    } else if is_diag {
                        if right {
                            // north-east
                            acc.1 = min(acc.1, r - r_q - 1);
                            println!("northeast reduced to {}", acc.1);
                        } else if left {
                            // north-west
                            acc.7 = min(acc.7, r - r_q - 1);
                            println!("northwest reduced to {}", acc.7);
                        }
                    }
                } else if below {
                    if same_col {
                        // south
                        acc.4 = min(acc.4, r_q - r - 1);
                        println!("south reduced to {}", acc.4);
                    } else if is_diag {
                        if right {
                            // south-east
                            acc.3 = min(acc.3, r_q - r - 1);
                            println!("southeast reduced to {}", acc.3);
                        }
                        if left {
                            // south-west
                            acc.5 = min(acc.5, r_q - r - 1);
                            println!("southwest reduced to {}", acc.5);
                        }
                    }
                }
            }

            acc
        },
    );

    n + ne + e + se + s + sw + w + nw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_no_obstacles() {
        let result = queens_attack(
            100,
            100,
            48,
            81,
            &[
                vec![54, 87],
                vec![64, 97],
                vec![42, 75],
                vec![32, 65],
                vec![42, 87],
                vec![32, 97],
                vec![54, 75],
                vec![64, 65],
                vec![48, 87],
                vec![48, 75],
                vec![54, 81],
                vec![42, 81],
                vec![45, 17],
                vec![14, 24],
                vec![35, 15],
                vec![95, 64],
                vec![63, 87],
                vec![25, 72],
                vec![71, 38],
                vec![96, 97],
                vec![16, 30],
                vec![60, 34],
                vec![31, 67],
                vec![26, 82],
                vec![20, 93],
                vec![81, 38],
                vec![51, 94],
                vec![75, 41],
                vec![79, 84],
                vec![79, 65],
                vec![76, 80],
                vec![52, 87],
                vec![81, 54],
                vec![89, 52],
                vec![20, 31],
                vec![10, 41],
                vec![32, 73],
                vec![83, 98],
                vec![87, 61],
                vec![82, 52],
                vec![80, 64],
                vec![82, 46],
                vec![49, 21],
                vec![73, 86],
                vec![37, 70],
                vec![43, 12],
                vec![94, 28],
                vec![10, 93],
                vec![52, 25],
                vec![50, 61],
                vec![52, 68],
                vec![52, 23],
                vec![60, 91],
                vec![79, 17],
                vec![93, 82],
                vec![12, 18],
                vec![75, 64],
                vec![69, 69],
                vec![94, 74],
                vec![61, 61],
                vec![46, 57],
                vec![67, 45],
                vec![96, 64],
                vec![83, 89],
                vec![58, 87],
                vec![76, 53],
                vec![79, 21],
                vec![94, 70],
                vec![16, 10],
                vec![50, 82],
                vec![92, 20],
                vec![40, 51],
                vec![49, 28],
                vec![51, 82],
                vec![35, 16],
                vec![15, 86],
                vec![78, 89],
                vec![41, 98],
                vec![70, 46],
                vec![79, 79],
                vec![24, 40],
                vec![91, 13],
                vec![59, 73],
                vec![35, 32],
                vec![40, 31],
                vec![14, 31],
                vec![71, 35],
                vec![96, 18],
                vec![27, 39],
                vec![28, 38],
                vec![41, 36],
                vec![31, 63],
                vec![52, 48],
                vec![81, 25],
                vec![49, 90],
                vec![32, 65],
                vec![25, 45],
                vec![63, 94],
                vec![89, 50],
                vec![43, 41],
            ],
        );

        assert_eq!(result, 40);
    }
}
