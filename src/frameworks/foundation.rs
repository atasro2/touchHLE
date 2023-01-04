//! The Foundation framework.
//!
//! A concept that Foundation really likes is "class clusters": abstract classes
//! with private concrete implementations. Apple has their own explanation of it
//! in [Cocoa Core Competencies](https://developer.apple.com/library/archive/documentation/General/Conceptual/DevPedia-CocoaCore/ClassCluster.html).
//! Being aware of this concept will make common types like `NSArray` and
//! `NSString` easier to understand.

pub mod ns_array;
pub mod ns_autorelease_pool;
pub mod ns_coder;
pub mod ns_dictionary;
pub mod ns_keyed_unarchiver;
pub mod ns_object;
pub mod ns_string;
pub mod ns_value;

#[derive(Default)]
pub struct State {
    ns_autorelease_pool: ns_autorelease_pool::State,
    ns_string: ns_string::State,
}

pub type NSInteger = i32;
pub type NSUInteger = u32;

/// Utility to help with implementing the `hash` method, which various classes
/// in Foundation have to do.
fn hash_helper<T: std::hash::Hash>(hashable: &T) -> NSUInteger {
    use std::hash::Hasher;

    // Rust documentation says DefaultHasher::new() should always return the
    // same instance, so this should give consistent hashes.
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hashable.hash(&mut hasher);
    let hash_u64: u64 = hasher.finish();
    (hash_u64 as u32) ^ ((hash_u64 >> 32) as u32)
}
