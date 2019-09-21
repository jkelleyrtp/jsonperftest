/// I wasn't a huge fan of how json is being used as the interchange format instead
/// structs/traits/impls, types etc. but I needed to make sure moving away from serde
/// made performance sense. Therefore, I implemented the same algorithm using both
/// Rust-y methods and the JSON method currently in use.
/// 
/// We're testing with a simple particle moving through a magnetic field
/// 
/// 
/// 
/// The rust-y approach with generics
pub mod rusty;

/// Raw indexing of the serde format
pub mod serdey;









