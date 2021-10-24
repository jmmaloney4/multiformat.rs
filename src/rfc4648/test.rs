    use super::*;
    use rand::Rng;
    use std::array::IntoIter;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    fn test_octet_ntet_conversion() {
        let cases_6 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([
            (vec![], vec![]),
            (vec![77, 97, 110], vec![19, 22, 5, 46]),
            (vec![77, 97], vec![19, 22, 4]),
            (vec![77], vec![19, 16]),
            (
                // foobar
                vec![102, 111, 111, 98, 97, 114],
                vec![25, 38, 61, 47, 24, 38, 5, 50],
            ),
            (
                // fooba
                vec![102, 111, 111, 98, 97],
                vec![25, 38, 61, 47, 24, 38, 4],
            ),
            (
                // foob
                vec![102, 111, 111, 98],
                vec![25, 38, 61, 47, 24, 32],
            ),
        ]));

        let cases_5 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([
            (vec![], vec![]),
            (vec![102, 111, 111], vec![12, 25, 23, 22, 30]),
        ]));

        let cases_4 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([
            (vec![], vec![]),
            (vec![102], vec![6, 6]),
        ]));

        let cases_3 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([
            (vec![], vec![]),
            (vec![23], vec![0, 5, 6]),
        ]));

        let cases_1 = HashMap::<Vec<u8>, Vec<u8>>::from_iter(IntoIter::new([
            (vec![], vec![]),
            (
                vec![201, 111, 111],
                vec![
                    1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1,
                ],
            ),
        ]));

        let test_n = |cases: HashMap<Vec<u8>, Vec<u8>>, n: u8| {
            for (i, o) in cases {
                assert_eq!(octet_group_to_ntets(&i, n).unwrap(), o);
                assert_eq!(ntet_group_to_octets(&o, n).unwrap(), i);
            }
        };

        test_n(cases_6, 6);
        test_n(cases_5, 5);
        test_n(cases_4, 4);
        test_n(cases_3, 3);
        test_n(cases_1, 1);
    }

    #[test]
    fn test_random_data() {
        const RANDOM_TRIALS: usize = 10;

        for _ in 0..RANDOM_TRIALS {
            let rand_count = rand::random::<u16>();
            let random_bytes: Vec<u8> = (0..rand_count).map(|_| rand::random::<u8>()).collect();

            (1_u8..=7)
                .map(|n| (n, octet_group_to_ntets(&random_bytes, n).unwrap()))
                .map(|(n, ntets)| {
                    println!("{}", ntets.len());
                    (n, ntet_group_to_octets(&ntets, n).unwrap())
                })
                .for_each(|(n, octets)| {
                    assert_eq!(random_bytes, octets, "Equality failed for n={}", n);
                })
        }
    }

