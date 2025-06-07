use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * MAXIMUM PALINDROMES PROBLEM SOLUTION
 *
 * Problem: Given a string and multiple range queries [l, r], for each query:
 * 1. Consider the substring from index l to r (1-indexed)
 * 2. Find all possible palindromes of MAXIMUM length that can be formed by rearranging characters
 * 3. Return the count of such maximum-length palindromes modulo 10^9 + 7
 *
 * Key Insights:
 * - We can rearrange characters freely, so we only care about character frequencies
 * - Maximum palindrome length = 2 * (total_pairs) + (1 if any odd count exists, 0 otherwise)
 * - Count of palindromes = multinomial coefficient * choices for middle character
 *
 * Algorithm:
 * 1. Use prefix sums for O(1) range frequency queries
 * 2. Precompute factorials and modular inverses for combinatorial calculations
 * 3. For each query: extract frequencies, calculate arrangements using combinatorics
 */

const MOD: i64 = 1000000007; // Required modulo value
// PREFIX_COUNTS[i][j] = count of character i in string[0..j-1]
static mut PREFIX_COUNTS: Vec<Vec<i32>> = Vec::new();
// Precomputed factorials for combinatorial calculations
static mut FACTORIALS: Vec<i64> = Vec::new();
// Precomputed modular inverses of factorials for division in modular arithmetic
static mut INV_FACTORIALS: Vec<i64> = Vec::new();

/*
 * Fast modular exponentiation using binary exponentiation
 * Computes (base^exp) % modulo in O(log exp) time
 * Used for calculating modular inverses
 */
fn mod_pow(base: i64, exp: i64, modulo: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulo;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        exp = exp >> 1;
        base = (base * base) % modulo;
    }
    result
}

/*
 * Calculates modular inverse using Fermat's Little Theorem
 * For prime p: a^(-1) ≡ a^(p-2) (mod p)
 * Since MOD = 10^9 + 7 is prime, we can use this approach
 */
fn mod_inverse(a: i64, modulo: i64) -> i64 {
    mod_pow(a, modulo - 2, modulo)
}

/*
 * Complete the 'initialize' function below.
 *
 * The function accepts STRING s as parameter.
 */

fn initialize(s: &str) {
    /*
     * INITIALIZATION PHASE - Called once before all queries
     *
     * This function sets up two key data structures:
     * 1. Prefix sum arrays for O(1) character frequency queries
     * 2. Precomputed factorials for O(1) combinatorial calculations
     */
    let n = s.len();

    unsafe {
        // Initialize prefix counts for each character (a-z)
        // PREFIX_COUNTS[char][pos] = count of 'char' in s[0..pos-1]
        PREFIX_COUNTS = vec![vec![0; n + 1]; 26];

        // Build prefix sums for character counts
        // For each position, store cumulative count of each character
        for (i, c) in s.chars().enumerate() {
            let char_idx = (c as u8 - b'a') as usize; // Convert 'a'-'z' to 0-25

            // Copy all previous counts to current position
            for j in 0..26 {
                PREFIX_COUNTS[j][i + 1] = PREFIX_COUNTS[j][i];
            }

            // Increment count for current character
            PREFIX_COUNTS[char_idx][i + 1] += 1;
        }

        /*
         * Precompute factorials and their modular inverses
         * This allows O(1) calculation of multinomial coefficients:
         * C(n; k1,k2,...,km) = n! / (k1! * k2! * ... * km!)
         *
         * In modular arithmetic: division by x = multiplication by x^(-1)
         */
        let max_fact = 100001; // Upper bound for factorial calculations
        FACTORIALS = vec![1; max_fact];
        INV_FACTORIALS = vec![1; max_fact];

        // Compute factorials: FACTORIALS[i] = i! mod MOD
        for i in 1..max_fact {
            FACTORIALS[i] = (FACTORIALS[i - 1] * i as i64) % MOD;
        }

        // Compute modular inverses: INV_FACTORIALS[i] = (i!)^(-1) mod MOD
        for i in 1..max_fact {
            INV_FACTORIALS[i] = mod_inverse(FACTORIALS[i], MOD);
        }
    }
}

/*
 * Complete the 'answerQuery' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER l
 *  2. INTEGER r
 */

fn answerQuery(l: i32, r: i32) -> i32 {
    /*
     * QUERY PROCESSING - Answer each query in O(1) time
     *
     * For substring s[l-1..r-1] (converting to 0-indexed):
     * 1. Extract character frequencies using prefix sums
     * 2. Calculate maximum palindrome length and arrangement count
     *
     * Mathematical approach:
     * - Maximum length = 2 * total_pairs + (1 if odd_count > 0)
     * - Arrangement count = multinomial_coefficient * middle_choices
     * - multinomial_coefficient = total_pairs! / (pairs_of_a! * pairs_of_b! * ...)
     * - middle_choices = number of characters with odd counts
     */
    unsafe {
        // Convert to 0-indexed ranges for internal processing
        let left = (l - 1) as usize;
        let right = r as usize;

        // Extract character frequencies in range [l, r] using prefix sums
        // char_counts[i] = frequency of character (i + 'a') in the substring
        let mut char_counts = vec![0; 26];
        for i in 0..26 {
            char_counts[i] = PREFIX_COUNTS[i][right] - PREFIX_COUNTS[i][left];
        }

        // Analyze character frequencies for palindrome construction
        let mut total_pairs = 0; // Total pairs that can be used symmetrically
        let mut odd_count = 0; // Characters with odd counts (candidates for middle)

        for &count in &char_counts {
            total_pairs += count / 2; // Each pair contributes 2 characters to palindrome
            if count % 2 == 1 {
                odd_count += 1; // Track characters that could go in middle
            }
        }

        // Edge case: empty substring
        if total_pairs == 0 && odd_count == 0 {
            return 0;
        }

        /*
         * COMBINATORIAL CALCULATION
         *
         * We need to count arrangements of pairs in the first half of palindrome
         * This is a multinomial coefficient: total_pairs! / (pairs_a! * pairs_b! * ...)
         *
         * Example: "aabbc" has pairs: a=1, b=1, c=0
         * Arrangements of "ab_" = 2!/(1!*1!) = 2 ways: "ab" and "ba"
         * Full palindromes: "abcba" and "bacab" if c goes in middle
         */

        // Start with total_pairs! (numerator of multinomial coefficient)
        let mut result = FACTORIALS[total_pairs as usize];

        // Divide by factorial of each character's pair count (denominator)
        for &count in &char_counts {
            let pairs = count / 2;
            if pairs > 0 {
                // Multiply by modular inverse instead of dividing
                result = (result * INV_FACTORIALS[pairs as usize]) % MOD;
            }
        }

        // Multiply by choices for middle character (if palindrome has odd length)
        if odd_count > 0 {
            // Any character with odd count can be placed in the middle
            result = (result * odd_count as i64) % MOD;
        }

        result as i32
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    initialize(&s);

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let l = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let r = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = answerQuery(l, r);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
