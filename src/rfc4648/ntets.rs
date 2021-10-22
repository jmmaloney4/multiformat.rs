use anyhow::{ensure, Result};
use num::integer::{div_ceil, div_floor, lcm};

mod rfc4648 {
    /*
     *  Base 64:
     *
     *      +--first octet--+-second octet--+--third octet--+
     *      |7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
     *      +-----------+---+-------+-------+---+-----------+
     *      |5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|
     *      +--1.index--+--2.index--+--3.index--+--4.index--+
     *
     *  Base 32:
     *
     *      01234567 89012345 67890123 45678901 23456789
     *      +--------+--------+--------+--------+--------+
     *      |< 1 >< 2| >< 3 ><|.4 >< 5.|>< 6 ><.|7 >< 8 >|
     *      +--------+--------+--------+--------+--------+
     *
     *  Base 2:
     *      012345678
     *
     */

    fn octet_group_to_ntets(input: Vec<u8>, n: u8) -> Result<Vec<u8>> {
        // N-tets can only be 1-7 in size
        ensure!(0 < n && n < 8, "Invalid N: {}", n);

        println!("{:?}", input);

        // Handle trivial case
        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Octet group size
        let ogs: u8 = lcm(8, n) / 8;
        // N-tet group size
        let ngs: u8 = lcm(8, n) / n;

        // Number of *full* octet groups in the input
        let full_groups: usize = div_floor(input.len(), ogs as usize);
        // Number of octets padded out to a full group.
        let total_groups: usize = div_ceil(input.len(), ogs as usize);

        // Number of octets in input which are not in a full group.
        let residual_octets: u8 = input.len() as u8 % ogs;
        // Number of n-tets needed to fully cover residual octets.
        let residual_ntets: u8 = div_ceil(residual_octets * 8, n);

        // Number of ntets we will output.
        let out_len: usize = (full_groups * ngs as usize) + residual_ntets as usize;

        // Pad input out to a full octet group.
        let mut octets = input;
        octets.resize(total_groups * ogs as usize, 0);
        let mut output: Vec<u8> = Vec::with_capacity(total_groups * ngs as usize);

        // Offset between the start of an octet and the start of the n-tet
        let mut offset: u8 = n;
        // Which octet we are currently translating
        let mut octets_i: usize = 0;

        // Initialize variables used during loop
        let mut octet: u8 = octets[octets_i];
        // Residual from the last octet that belongs in the next n-tet
        let mut carry: u8 = 0;

        loop {
            let q = octet / pow2((8 - offset) % 8);
            let r = octet % pow2((8 - offset) % 8);
            println!("{} {} {} {} {}", offset, carry, octet, q, r);

            output.push(carry * pow2(offset) + q);
            offset += n;

            if offset < 8 {
                // Have not crossed an octet boundary, keep parsing this octet.
                octet = r;
                carry = 0;
            } else {
                // Have reached or crossed an octet boundary.
                octets_i += 1;
                if octets_i < octets.len() {
                    // There are more octets to parse. Fetch the next one, but preserve
                    // the remainder (the lower half) of the current octet, because
                    // it will be the upper segment of the next n-tet.
                    octet = octets[octets_i];

                    if offset == 8 {
                        // Reached the end of a group, exactly at an octet boundary.
                        // The remainder carries an entire n-tet, so just put in in the output.
                        output.push(r);
                        carry = 0;
                        // Bump the offset over, because we have handeled this last n-tet.
                        offset += n;
                    } else {
                        // Otherwise the remainder is the upper part of the next n-tet, so we
                        // need to roll it over.
                        carry = r;
                    }
                } else {
                    // There are no more octets. The last n-tet is just the remainder
                    // of the current octet, but shifted up to the left most significant
                    // side of the n-tet.
                    output.push(r * pow2(offset % 8));
                    break;
                }
            }

            offset %= 8;
            println!("{:?} {} {}", output, octet, carry);
        }

        output.truncate(out_len);
        Ok(output)
    }

    fn pow2(i: u8) -> u8 {
        assert!(i < 8);
        2_u8.pow(i.into())
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_octet_group_to_ntets() {
            assert_eq!(
                super::octet_group_to_ntets(vec![77, 97, 110], 6).unwrap(),
                [19, 22, 5, 46]
            );

            assert_eq!(
                super::octet_group_to_ntets(vec![77, 97], 6).unwrap(),
                [19, 22, 4]
            );

            assert_eq!(super::octet_group_to_ntets(vec![77], 6).unwrap(), [19, 16]);

            assert_eq!(
                super::octet_group_to_ntets(vec![102, 111, 111, 98, 97, 114], 6).unwrap(),
                // Zm9vYmFy
                [25, 38, 61, 47, 24, 38, 5, 50]
            );
        }
    }
}