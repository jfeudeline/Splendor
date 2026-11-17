use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;

pub const SEED_BYTES: usize = 32;

pub fn seed_from_u64(seed: u64) -> [u8; SEED_BYTES] {
    let bytes = seed.to_le_bytes();
    let mut joint = [0u8; SEED_BYTES];
    joint[..8].copy_from_slice(&bytes);
    joint
}

pub fn rng_from_seed(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::from_seed(seed_from_u64(seed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::Rng;

    #[test]
    fn seed_is_little_endian_and_padded_with_zeros() {
        assert_eq!(seed_from_u64(0), [0; SEED_BYTES]);

        let bytes = seed_from_u64(0x0102030405060708);
        assert_eq!(&bytes[..8], &[8, 7, 6, 5, 4, 3, 2, 1]);
        assert!(bytes[8..].iter().all(|&byte| byte == 0));
    }
    #[test]
    fn golden_first_outputs() {
        let cases = [
            (
                0u64,
                [
                    15438444565445410878,
                    11647726043916688255,
                    4287315583106450476,
                ],
            ),
            (
                1u64,
                [
                    7037237572835827407,
                    5440448899038119230,
                    14725191807199933458,
                ],
            ),
            (
                42u64,
                [
                    6424161053832095879,
                    5270208426312333099,
                    9102960255288774902,
                ],
            ),
        ];

        for (seed, expected) in cases {
            let mut rng = rng_from_seed(seed);
            for value in expected {
                assert_eq!(rng.next_u64(), value, "seed {seed}");
            }
        }
    }
}
