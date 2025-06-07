fn extra_long_factorials(n: i32) -> String {
    if n == 0 || n == 1 {
        return n.to_string();
    }

    let mut digits = vec![1];

    for n_ in 2..=n {
        let mut carry = 0;

        for digit in digits.iter_mut() {
            let product = *digit * n_ + carry;

            *digit = product % 10;
            carry = product / 10;
        }

        while carry > 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }

    digits
        .iter()
        .rev()
        .map(|n| n.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extra_long_factorials_01() {
        let result = extra_long_factorials(25);
        assert_eq!(result, "15511210043330985984000000");
    }

    #[test]
    fn extra_long_factorials_02() {
        let result = extra_long_factorials(45);
        assert_eq!(
            result,
            "119622220865480194561963161495657715064383733760000000000"
        );
    }

    // #[test]
    // fn extra_long_factorials_03() {
    //     let result = extra_long_factorials("a", "b");
    //     assert_eq!(result, "OK");
    // }
}
