use thiserror::Error;

pub use target_arch::*;

#[derive(Error, Debug)]
pub enum PoseidonSyscallError {
    #[error("Invalid number of inputs")]
    InvalidNumberOfInputs,
    #[error("Failed to convert a vector of bytes into an array")]
    VecToArray,
    #[error("Failed to convert a the number of inputs to a u8")]
    U64Tou8,
    #[error("Selected width is invalid, select a width between 2 and 16, for 1 to 15 inputs.")]
    InvalidWidthCircom,
    #[error("Unexpected error")]
    Unexpected,
}

impl From<u64> for PoseidonSyscallError {
    fn from(error: u64) -> Self {
        match error {
            1 => PoseidonSyscallError::InvalidNumberOfInputs,
            2 => PoseidonSyscallError::VecToArray,
            3 => PoseidonSyscallError::U64Tou8,
            4 => PoseidonSyscallError::InvalidWidthCircom,
            _ => PoseidonSyscallError::Unexpected,
        }
    }
}

impl From<PoseidonSyscallError> for u64 {
    fn from(error: PoseidonSyscallError) -> Self {
        match error {
            PoseidonSyscallError::InvalidNumberOfInputs => 1,
            PoseidonSyscallError::VecToArray => 2,
            PoseidonSyscallError::U64Tou8 => 3,
            PoseidonSyscallError::InvalidWidthCircom => 4,
            PoseidonSyscallError::Unexpected => 5,
        }
    }
}

#[cfg(not(target_os = "solana"))]
mod target_arch {
    use super::*;

    use ark_bn254::Fr;
    use light_poseidon::{Poseidon, PoseidonBytesHasher, PoseidonError};

    use crate::hash::Hash;

    impl From<PoseidonError> for PoseidonSyscallError {
        fn from(error: PoseidonError) -> Self {
            match error {
                PoseidonError::InvalidNumberOfInputs { .. } => {
                    PoseidonSyscallError::InvalidNumberOfInputs
                }
                PoseidonError::VecToArray => PoseidonSyscallError::VecToArray,
                PoseidonError::U64Tou8 => PoseidonSyscallError::U64Tou8,
                PoseidonError::InvalidWidthCircom { .. } => {
                    PoseidonSyscallError::InvalidWidthCircom
                }
            }
        }
    }

    pub fn hashv(vals: &[&[u8]]) -> Result<Hash, PoseidonSyscallError> {
        let mut poseidon = Poseidon::<Fr>::new_circom(vals.len())?;
        Ok(Hash(poseidon.hash_bytes(vals)?))
    }
}

#[cfg(target_os = "solana")]
mod target_arch {
    use super::*;

    use crate::hash::{Hash, HASH_BYTES};

    pub fn hashv(vals: &[&[u8]]) -> Result<Hash, PoseidonSyscallError> {
        let mut hash_result = [0; HASH_BYTES];
        let result = unsafe {
            crate::syscalls::sol_poseidon(
                vals as *const _ as *const u8,
                vals.len() as u64,
                &mut hash_result as *mut _ as *mut u8,
            )
        };

        match result {
            0 => Ok(Hash::new(&hash_result)),
            e => Err(PoseidonSyscallError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_input_ones_twos() {
        let input1 = [1u8; 32];
        let input2 = [2u8; 32];

        let hash = hashv(&[&input1, &input2]).expect("Failed to compute the Poseidon hash");
        assert_eq!(
            hash.to_bytes(),
            [
                13, 84, 225, 147, 143, 138, 140, 28, 125, 235, 94, 3, 85, 242, 99, 25, 32, 123,
                132, 254, 156, 162, 206, 27, 38, 231, 53, 200, 41, 130, 25, 144
            ]
        );
    }

    #[test]
    fn test_poseidon_input_one_two() {
        let input1 = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ];
        let input2 = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2,
        ];

        let hash = hashv(&[&input1, &input2]).expect("Failed to compute the Poseidon hash");
        assert_eq!(
            hash.to_bytes(),
            [
                17, 92, 192, 245, 231, 214, 144, 65, 61, 246, 76, 107, 150, 98, 233, 207, 42, 54,
                23, 242, 116, 50, 69, 81, 158, 25, 96, 122, 68, 23, 24, 154
            ]
        );
    }

    #[test]
    fn test_poseidon_input_random() {
        let input1 = [
            0x06, 0x9c, 0x63, 0x81, 0xac, 0x0b, 0x96, 0x8e, 0x88, 0x1c, 0x91, 0x3c, 0x17, 0xd8,
            0x36, 0x06, 0x7f, 0xd1, 0x5f, 0x2c, 0xc7, 0x9f, 0x90, 0x2c, 0x80, 0x70, 0xb3, 0x6d,
            0x28, 0x66, 0x17, 0xdd,
        ];
        let input2 = [
            0xc3, 0x3b, 0x60, 0x04, 0x2f, 0x76, 0xc7, 0xfb, 0xd0, 0x5d, 0xb7, 0x76, 0x23, 0xcb,
            0x17, 0xb8, 0x1d, 0x49, 0x41, 0x4b, 0x82, 0xe5, 0x6a, 0x2e, 0xc0, 0x18, 0xf7, 0xa5,
            0x5c, 0x3f, 0x30, 0x0b,
        ];

        let hash = hashv(&[&input1, &input2]).expect("Failed to compute the Poseidon hash");
        assert_eq!(
            hash.to_bytes(),
            [
                10, 19, 173, 132, 77, 52, 135, 173, 61, 186, 243, 135, 103, 96, 235, 151, 18, 131,
                212, 131, 51, 250, 90, 158, 151, 230, 238, 66, 42, 249, 85, 75
            ]
        )
    }
}
