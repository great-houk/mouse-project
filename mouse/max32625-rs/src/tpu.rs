#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PRNG User Entropy Value"]
    pub prng_user_entropy: crate::Reg<prng_user_entropy::PRNG_USER_ENTROPY_SPEC>,
    #[doc = "0x04 - PRNG Random Number Output"]
    pub prng_rnd_num: crate::Reg<prng_rnd_num::PRNG_RND_NUM_SPEC>,
}
#[doc = "PRNG_USER_ENTROPY register accessor: an alias for `Reg<PRNG_USER_ENTROPY_SPEC>`"]
pub type PRNG_USER_ENTROPY = crate::Reg<prng_user_entropy::PRNG_USER_ENTROPY_SPEC>;
#[doc = "PRNG User Entropy Value"]
pub mod prng_user_entropy;
#[doc = "PRNG_RND_NUM register accessor: an alias for `Reg<PRNG_RND_NUM_SPEC>`"]
pub type PRNG_RND_NUM = crate::Reg<prng_rnd_num::PRNG_RND_NUM_SPEC>;
#[doc = "PRNG Random Number Output"]
pub mod prng_rnd_num;
