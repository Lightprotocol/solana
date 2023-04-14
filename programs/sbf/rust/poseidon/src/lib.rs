//! Example SBF program using Poseidon syscall

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    poseidon::{hashv, PoseidonSyscallError},
    program_error::ProgramError,
    pubkey::Pubkey,
};

fn test_poseidon_hash() -> Result<(), PoseidonSyscallError> {
    let input1 = [1u8; 32];
    let input2 = [2u8; 32];

    let hash = hashv(&[&input1, &input2])?;

    assert_eq!(
        hash.to_bytes(),
        [
            13, 84, 225, 147, 143, 138, 140, 28, 125, 235, 94, 3, 85, 242, 99, 25, 32, 123, 132,
            254, 156, 162, 206, 27, 38, 231, 53, 200, 41, 130, 25, 144
        ]
    );

    Ok(())
}

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("poseidon_hash");

    test_poseidon_hash().map_err(|e| ProgramError::from(u64::from(e)))?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_poseidon_hash() {
        super::test_poseidon_hash().unwrap();
    }
}
