fn valid_string(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();

    chars.sort();

    let (chunks, _) = chars.into_iter().fold((Vec::new(), '0'), |mut acc, char| {
        if char != acc.1 {
            let chunk = vec![char];

            acc.0.push(chunk);
        } else {
            let chunk = match acc.0.last_mut() {
                Some(chunk) => chunk,
                _ => panic!("Expect last chunk to be valid."),
            };

            chunk.push(char);
        }

        acc.1 = char;

        acc
    });

    println!("---");
    println!("chunks: {:?}", chunks);

    let lengths =
        chunks
            .iter()
            .map(|chunk| chunk.len())
            .fold(Vec::<Vec<usize>>::new(), |mut acc, length| {
                match acc.iter().position(|chunk| chunk[0] == length) {
                    Some(index) => {
                        acc[index].push(length);
                    }
                    _ => acc.push(vec![length]),
                };

                acc
            });

    println!("lengths: {:?}", lengths);

    match lengths.len() {
        0 | 1 => "YES",
        2 => match lengths.iter().any(|chunk| {
            chunk.len() == 1
                && (chunk[0] == 1 || (lengths[0][0] as i32 - lengths[1][0] as i32).abs() == 1)
        }) {
            true => "YES",
            _ => "NO",
        },
        _ => "NO",
    }
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_string_01() {
        let result = valid_string("aabbcd");
        assert_eq!(result, "NO");
    }

    #[test]
    fn valid_string_02() {
        let result = valid_string("aabbccddeefghi");
        assert_eq!(result, "NO");
    }

    #[test]
    fn valid_string_03() {
        let result = valid_string("abcdefghhgfedecba");
        assert_eq!(result, "YES");
    }

    #[test]
    fn valid_string_04() {
        let result = valid_string("aaaabbcc");
        assert_eq!(result, "NO");
    }

    #[test]
    fn valid_string_05() {
        let result = valid_string(
            "ibfdgaeadiaefgbhbdghhhbgdfgeiccbiehhfcggchgghadhdhagfbahhddgghbdehidbibaeaagaeeigffcebfbaieggabcfbiiedcabfihchdfabifahcbhagccbdfifhghcadfiadeeaheeddddiecaicbgigccageicehfdhdgafaddhffadigfhhcaedcedecafeacbdacgfgfeeibgaiffdehigebhhehiaahfidibccdcdagifgaihacihadecgifihbebffebdfbchbgigeccahgihbcbcaggebaaafgfedbfgagfediddghdgbgehhhifhgcedechahidcbchebheihaadbbbiaiccededchdagfhccfdefigfibifabeiaccghcegfbcghaefifbachebaacbhbfgfddeceababbacgffbagidebeadfihaefefegbghgddbbgddeehgfbhafbccidebgehifafgbghafacgfdccgifdcbbbidfifhdaibgigebigaedeaaiadegfefbhacgddhchgcbgcaeaieiegiffchbgbebgbehbbfcebciiagacaiechdigbgbghefcahgbhfibhedaeeiffebdiabcifgccdefabccdghehfibfiifdaicfedagahhdcbhbicdgibgcedieihcichadgchgbdcdagaihebbabhibcihicadgadfcihdheefbhffiageddhgahaidfdhhdbgciiaciegchiiebfbcbhaeagccfhbfhaddagnfieihghfbaggiffbbfbecgaiiidccdceadbbdfgigibgcgchafccdchgifdeieicbaididhfcfdedbhaadedfageigfdehgcdaecaebebebfcieaecfagfdieaefdiedbcadchabhebgehiidfcgahcdhcdhgchhiiheffiifeegcfdgbdeffhgeghdfhbfbifgidcafbfcd",
        );
        assert_eq!(result, "YES");
    }
}
