use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn dna_health(genes: Vec<String>, health: Vec<i32>, strands: Vec<(i32, i32, String)>) -> String {
    let gene_health = genes
        .into_iter()
        .zip(health.into_iter())
        .collect::<Vec<_>>();
    let mut answer = (i32::MAX, i32::MIN);

    for (start, end, mut d) in strands {
        let mut health = 0;
        let healthy_genes = gene_health[start as usize..=end as usize].iter().fold(
            HashMap::<&str, Vec<(String, i32)>>::new(),
            |mut acc, (gene, value)| {
                if let Some(first_char) = gene.get(0..1) {
                    let v = (gene.clone(), *value);

                    if acc.contains_key(first_char) {
                        if let Some(values) = acc.get_mut(first_char) {
                            values.push(v);
                        }
                    } else {
                        acc.insert(first_char, vec![v]);
                    }
                }

                acc
            },
        );

        while d.len() > 0 {
            if let Some(first_char) = d.get(0..1) {
                if let Some(values) = healthy_genes.get(first_char) {
                    for (gene, value) in values {
                        if d.starts_with(gene) {
                            health += value;
                        }
                    }
                }
            }

            d = d[1..d.len()].to_string();
        }

        answer.0 = min(answer.0, health);
        answer.1 = max(answer.1, health);
    }

    let text = format!("{} {}", answer.0, answer.1);

    println!("{}", text);

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_health() {
        let result = dna_health(
            vec![
                "ogwwsenipa".to_string(),
                "obsehkjfcj".to_string(),
                "dhqvptquuu".to_string(),
                "kkdgivnvfc".to_string(),
                "ytdqxmciue".to_string(),
                "rznhvdcnxw".to_string(),
                "kihnqpkdnp".to_string(),
                "hlimdfbfnv".to_string(),
                "mguznrcpfc".to_string(),
                "nrmweeookb".to_string(),
                "drolersfwh".to_string(),
                "ckpykeqotx".to_string(),
                "sioefulviv".to_string(),
                "wnmkasbuzz".to_string(),
                "ddkscwwukr".to_string(),
                "rfzhjgbwbl".to_string(),
                "rzagjaymua".to_string(),
                "mxdyrhunbg".to_string(),
                "eulfdxogtr".to_string(),
                "rnrrtctrpp".to_string(),
                "tdmzbfgxsi".to_string(),
                "fyereiquol".to_string(),
                "liyebrhvly".to_string(),
                "kgbzfeembz".to_string(),
                "wgxazdirzx".to_string(),
                "flfdrgxydi".to_string(),
                "woqzpwdvkg".to_string(),
                "ugpuiqxrix".to_string(),
                "qnkxsbfpcj".to_string(),
                "zazqaqmdly".to_string(),
                "mgebaorzfz".to_string(),
                "yxoiuhmayo".to_string(),
                "lyqkoqacwn".to_string(),
                "aivgjxucxc".to_string(),
                "cxzsgwbuya".to_string(),
                "klyavotxsp".to_string(),
                "muzickfwmc".to_string(),
                "aqccjiakey".to_string(),
                "mojmqgajfu".to_string(),
                "yrozzqjfpw".to_string(),
                "jrmltxvtkz".to_string(),
                "twpejgmlpr".to_string(),
                "gqlwpknbre".to_string(),
                "xdvlqplmkv".to_string(),
                "ngtfmelzsc".to_string(),
                "qyudukojnh".to_string(),
                "nkmjxdairm".to_string(),
                "fgublhhygz".to_string(),
                "byxvcuhsdu".to_string(),
                "btgocgreqk".to_string(),
                "syqnzeuicc".to_string(),
                "ifahdebmwh".to_string(),
                "jaapoexhio".to_string(),
                "rcmjpnnlxq".to_string(),
                "nfvonauqnt".to_string(),
                "xwtznjdlqn".to_string(),
                "bjqnshcgtz".to_string(),
                "yghvwuwrml".to_string(),
                "kmhdlumrhe".to_string(),
                "einwxhebpx".to_string(),
                "bnfilcejts".to_string(),
                "ufebiqxwjh".to_string(),
                "cnprmnysoq".to_string(),
                "rrfwbqahzv".to_string(),
                "atagwkwwif".to_string(),
                "dkvsbjhcby".to_string(),
                "surxqvqter".to_string(),
                "oenpljzjhi".to_string(),
                "rkuofwxoaa".to_string(),
                "osugrmdjfh".to_string(),
                "bwoolbzmkh".to_string(),
                "wdtrrypqpp".to_string(),
                "qdjmlcbomi".to_string(),
                "wpekdpleex".to_string(),
                "nabhtuhinw".to_string(),
                "zfcksnntcb".to_string(),
                "dyqiktzxzd".to_string(),
                "ungxuzubkh".to_string(),
                "almcwgrlbt".to_string(),
                "mftcndxoaw".to_string(),
                "sxjawdzshl".to_string(),
                "zjxonvwegy".to_string(),
                "ysfruuxtiz".to_string(),
                "payzavecpn".to_string(),
                "ppwofjjbop".to_string(),
                "bojghfaeyj".to_string(),
                "golgpodtst".to_string(),
                "hhifwprhqf".to_string(),
                "xuvgacodjm".to_string(),
                "orcbxrpbnj".to_string(),
                "uwtebrtsyl".to_string(),
                "zxfugizuli".to_string(),
                "gzzjawcszp".to_string(),
                "btnwxrnqlm".to_string(),
                "enljjrrile".to_string(),
                "ssdtdgsfar".to_string(),
                "xdlmaidpbp".to_string(),
                "dhepqngkws".to_string(),
                "oomuipccwc".to_string(),
                "ttfeihplxs".to_string(),
            ],
            vec![
                4462805, 1916916, 2870812, 3407597, 5169525, 4087301, 4005965, 1803633, 3357388,
                1112112, 5656776, 4438527, 3841975, 4102090, 2113339, 2977711, 1709727, 1666821,
                4167887, 3742911, 1948785, 3057238, 1940358, 4574138, 4598641, 2922682, 1839758,
                4562812, 1508583, 2531144, 3192788, 4971388, 3448060, 2579952, 4895338, 5133938,
                5667253, 5417655, 3453923, 3024642, 5529768, 3110699, 3979521, 5888095, 3729142,
                2609212, 2865806, 1955221, 3276034, 3550045, 4698132, 1741171, 5607283, 5638490,
                2831662, 1722277, 2561172, 3671420, 5285089, 5586108, 5202564, 4994229, 2073848,
                5166977, 1574181, 5969186, 1817267, 1241435, 2903194, 1787542, 5782429, 4949314,
                1414593, 1278302, 4837409, 4143735, 2887514, 4219567, 5098956, 2679900, 1769612,
                1313440, 5937424, 3893247, 3468283, 2769086, 2131876, 5029455, 2956858, 3933318,
                2131915, 4675774, 2927547, 3205764, 1359103, 1018081, 5691302, 2176370, 3775868,
                2594496,
            ],
            vec![
                (7, 93, "zjxonvwegyeulfdxogtr".to_string()),
                (0, 77, "dkvsbjhcbyrzagjaymua".to_string()),
                (13, 90, "ddkscwwukrrfzhjgbwbl".to_string()),
                (9, 85, "xdvlqplmkvwgxazdirzx".to_string()),
                (7, 77, "jaapoexhiofgublhhygz".to_string()),
                (10, 77, "yghvwuwrmlbjqnshcgtz".to_string()),
                (19, 91, "ifahdebmwhxdvlqplmkv".to_string()),
                (6, 84, "jrmltxvtkzwgxazdirzx".to_string()),
                (12, 85, "oenpljzjhirkuofwxoaa".to_string()),
                (11, 90, "atagwkwwifmxdyrhunbg".to_string()),
                (12, 70, "dkvsbjhcbyfyereiquol".to_string()),
                (3, 87, "drolersfwhxwtznjdlqn".to_string()),
                (6, 80, "wnmkasbuzzrcmjpnnlxq".to_string()),
                (14, 75, "qnkxsbfpcjyxoiuhmayo".to_string()),
                (8, 77, "aivgjxucxczfcksnntcb".to_string()),
                (16, 89, "aivgjxucxcppwofjjbop".to_string()),
                (8, 71, "bjqnshcgtzugpuiqxrix".to_string()),
                (8, 90, "ppwofjjbopalmcwgrlbt".to_string()),
                (2, 80, "woqzpwdvkggqlwpknbre".to_string()),
                (4, 91, "kihnqpkdnpddkscwwukr".to_string()),
                (0, 92, "yrozzqjfpwdyqiktzxzd".to_string()),
                (14, 77, "xwtznjdlqneinwxhebpx".to_string()),
                (8, 80, "aivgjxucxcckpykeqotx".to_string()),
                (19, 81, "zjxonvwegyliyebrhvly".to_string()),
                (0, 81, "flfdrgxydinrmweeookb".to_string()),
                (8, 87, "xwtznjdlqngolgpodtst".to_string()),
                (2, 89, "ugpuiqxrixwpekdpleex".to_string()),
                (14, 75, "rnrrtctrppcxzsgwbuya".to_string()),
                (9, 69, "ddkscwwukrkmhdlumrhe".to_string()),
                (0, 86, "mftcndxoawnkmjxdairm".to_string()),
                (11, 75, "liyebrhvlyaivgjxucxc".to_string()),
                (3, 83, "einwxhebpxzjxonvwegy".to_string()),
                (18, 73, "qdjmlcbomidkvsbjhcby".to_string()),
                (9, 78, "nkmjxdairmwdtrrypqpp".to_string()),
                (10, 92, "rfzhjgbwblbojghfaeyj".to_string()),
                (16, 85, "syqnzeuiccqyudukojnh".to_string()),
                (3, 85, "yxoiuhmayomuzickfwmc".to_string()),
                (11, 85, "byxvcuhsdutwpejgmlpr".to_string()),
                (2, 75, "bnfilcejtsaqccjiakey".to_string()),
                (18, 90, "kgbzfeembzlyqkoqacwn".to_string()),
                (10, 75, "woqzpwdvkgrrfwbqahzv".to_string()),
                (10, 82, "ugpuiqxrixkmhdlumrhe".to_string()),
                (13, 72, "jaapoexhiorzagjaymua".to_string()),
                (4, 76, "jrmltxvtkzjaapoexhio".to_string()),
                (15, 73, "wgxazdirzxqnkxsbfpcj".to_string()),
                (2, 93, "woqzpwdvkgxwtznjdlqn".to_string()),
                (2, 81, "wpekdpleexddkscwwukr".to_string()),
                (14, 86, "fgublhhygzdyqiktzxzd".to_string()),
                (0, 72, "klyavotxsprznhvdcnxw".to_string()),
                (8, 87, "muzickfwmcbtgocgreqk".to_string()),
                (13, 85, "bnfilcejtsjaapoexhio".to_string()),
                (8, 81, "fyereiquoljrmltxvtkz".to_string()),
                (11, 71, "woqzpwdvkgwoqzpwdvkg".to_string()),
                (18, 90, "xdvlqplmkvwpekdpleex".to_string()),
                (11, 93, "zazqaqmdlypayzavecpn".to_string()),
                (3, 75, "ckpykeqotxtwpejgmlpr".to_string()),
                (6, 89, "dkvsbjhcbynabhtuhinw".to_string()),
                (18, 70, "liyebrhvlykmhdlumrhe".to_string()),
                (17, 87, "klyavotxspfyereiquol".to_string()),
                (7, 87, "yrozzqjfpwsioefulviv".to_string()),
                (15, 79, "yghvwuwrmlungxuzubkh".to_string()),
                (17, 71, "lyqkoqacwnyghvwuwrml".to_string()),
                (4, 76, "rkuofwxoaakmhdlumrhe".to_string()),
                (0, 92, "ifahdebmwhatagwkwwif".to_string()),
                (8, 77, "kgbzfeembzmxdyrhunbg".to_string()),
                (6, 92, "byxvcuhsduwnmkasbuzz".to_string()),
                (9, 74, "wdtrrypqppnfvonauqnt".to_string()),
                (18, 89, "nfvonauqntpayzavecpn".to_string()),
                (11, 88, "yxoiuhmayojaapoexhio".to_string()),
                (4, 75, "ddkscwwukrosugrmdjfh".to_string()),
                (18, 92, "wgxazdirzxmgebaorzfz".to_string()),
                (11, 72, "yxoiuhmayobyxvcuhsdu".to_string()),
                (3, 86, "syqnzeuiccdrolersfwh".to_string()),
                (5, 93, "muzickfwmcnfvonauqnt".to_string()),
                (0, 76, "hlimdfbfnvwgxazdirzx".to_string()),
                (13, 86, "aqccjiakeybjqnshcgtz".to_string()),
                (19, 90, "nkmjxdairmxuvgacodjm".to_string()),
                (2, 76, "syqnzeuicccnprmnysoq".to_string()),
                (17, 72, "kgbzfeembzjrmltxvtkz".to_string()),
                (9, 73, "eulfdxogtrliyebrhvly".to_string()),
                (3, 69, "oenpljzjhibyxvcuhsdu".to_string()),
                (19, 76, "ugpuiqxrixzazqaqmdly".to_string()),
                (15, 86, "atagwkwwifbyxvcuhsdu".to_string()),
                (5, 82, "nrmweeookbrkuofwxoaa".to_string()),
                (4, 74, "qdjmlcbomikihnqpkdnp".to_string()),
                (4, 82, "surxqvqterfgublhhygz".to_string()),
                (8, 73, "nkmjxdairmliyebrhvly".to_string()),
                (5, 72, "ckpykeqotxlyqkoqacwn".to_string()),
                (13, 72, "qdjmlcbomirfzhjgbwbl".to_string()),
                (2, 73, "wpekdpleexliyebrhvly".to_string()),
                (12, 90, "nkmjxdairmngtfmelzsc".to_string()),
                (4, 72, "twpejgmlpreulfdxogtr".to_string()),
                (5, 93, "rnrrtctrppzazqaqmdly".to_string()),
                (14, 83, "syqnzeuiccatagwkwwif".to_string()),
                (3, 74, "qdjmlcbomisioefulviv".to_string()),
                (18, 85, "qnkxsbfpcjdkvsbjhcby".to_string()),
                (14, 86, "xwtznjdlqneulfdxogtr".to_string()),
                (1, 93, "ifahdebmwhqnkxsbfpcj".to_string()),
                (5, 79, "ckpykeqotxgqlwpknbre".to_string()),
                (1, 79, "liyebrhvlymojmqgajfu".to_string()),
            ],
        );

        assert_eq!(result, "3218660 11137051");
    }
}
