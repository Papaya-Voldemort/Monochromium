use argon2::{Algorithm, Argon2, Params, Version};

const ARGON2_MEMORY_COST: u32 = 19 * 1024;
const ARGON2_TIME_COST: u32 = 2;
const ARGON2_PARALLELISM: u32 = 1;
const ARGON2_OUTPUT_LENGTH: usize = 32;

pub fn argon2_config() -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_TIME_COST,
            ARGON2_PARALLELISM,
            Some(ARGON2_OUTPUT_LENGTH),
        )
        .expect("valid Argon2 parameters"),
    )
}
