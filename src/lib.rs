//! A bounded operational continuity experiment, derived from ESS-MAI/AIGovern-Cyber.
//! It issues no constitutional verdict, knowledge admission or authority token.
mod continuity;
mod fnv;
mod framing;
pub mod pd_memory;
pub mod pd_spine_contract;
pub mod progressive_debatic;
mod projection;
pub mod zero_noi;
pub use continuity::{input, summary, uninterrupted, Store};
pub use fnv::fnv1a64;
