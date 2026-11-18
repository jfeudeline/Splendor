use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};

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

pub fn below(rng: &mut ChaCha8Rng, n: u64) -> u64 {
    assert!(n > 0, "n must be positive");
    let zone = u64::MAX - (u64::MAX % n);
    loop {
        let next = rng.next_u64();
        if next < zone {
            return next % n;
        }
    }
}

pub fn shuffle<T>(rng: &mut ChaCha8Rng, items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let j = below(rng, (i + 1) as u64) as usize;
        items.swap(i, j);
    }
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

    #[test]
    fn zero_if_n_is_one() {
        let mut rng = rng_from_seed(42u64);
        assert_eq!(below(&mut rng, 1), 0);
    }

    #[test]
    fn all_values_in_0_n() {
        for n in [6, 98, 354, 1546] {
            let mut rng = rng_from_seed(42u64);
            for _i in 0..100 {
                assert!(below(&mut rng, n) < n);
            }
        }
    }

    #[test]
    fn same_seed_generates_same_sample() {
        let mut rng1 = rng_from_seed(42u64);
        let mut rng2 = rng_from_seed(42u64);
        for _ in 0..100 {
            assert_eq!(below(&mut rng1, 6), below(&mut rng2, 6));
        }
    }

    #[test]
    fn below_golden_first_outputs() {
        let expected = [5, 3, 2, 0, 0, 3, 5, 3, 1, 0];
        let mut rng = rng_from_seed(42u64);
        for item in expected {
            assert_eq!(below(&mut rng, 6), item);
        }
    }

    #[test]
    fn shuffle_empty_slice_does_nothing() {
        let mut rng = rng_from_seed(42u64);
        let mut values: [u8; 0] = [];

        shuffle(&mut rng, &mut values);

        assert_eq!(values.len(), 0);
    }

    #[test]
    fn shuffle_single_element_does_nothing() {
        let mut rng = rng_from_seed(42u64);
        let mut values = [42u8];

        shuffle(&mut rng, &mut values);

        assert_eq!(values, [42]);
    }

    #[test]
    fn shuffle_preserves_all_elements() {
        let mut values: Vec<u32> = (0..90).collect();
        let mut rng = rng_from_seed(42u64);

        shuffle(&mut rng, &mut values);

        let mut sorted = values.clone();
        sorted.sort_unstable();

        assert_eq!(sorted, (0..90).collect::<Vec<_>>());
    }

    #[test]
    fn same_seed_gives_same_shuffle() {
        let mut first: Vec<u32> = (0..90).collect();
        let mut second = first.clone();

        let mut rng1 = rng_from_seed(42u64);
        let mut rng2 = rng_from_seed(42u64);

        shuffle(&mut rng1, &mut first);
        shuffle(&mut rng2, &mut second);

        assert_eq!(first, second);
    }

    #[test]
    fn golden_shuffle_0_to_90() {
        let expected: [u32; 90] = [
            58, 56, 27, 72, 73, 43, 8, 14, 86, 22, 84, 51, 0, 68, 28, 19, 88, 65, 60, 32, 85, 33,
            4, 79, 62, 42, 78, 46, 59, 24, 48, 61, 52, 25, 67, 40, 34, 36, 37, 87, 81, 13, 20, 9,
            7, 69, 63, 74, 12, 70, 47, 5, 77, 80, 44, 17, 15, 66, 31, 49, 16, 23, 55, 71, 89, 39,
            41, 18, 57, 10, 1, 35, 21, 53, 76, 3, 30, 83, 2, 82, 75, 45, 26, 11, 50, 54, 6, 38, 64,
            29,
        ];

        let mut values: Vec<u32> = (0..90).collect();
        let mut rng = rng_from_seed(42u64);

        shuffle(&mut rng, &mut values);

        assert_eq!(values.as_slice(), &expected);
    }
}
