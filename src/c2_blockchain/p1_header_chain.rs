//! We want to make the simplest possible blockchain to begin with. Just a hash-linked data structure.
//! We learned from the lecture that it is actually the headers that are hash linked, so let's
//! start with that.
//!

use crate::hash;

// We will use Rust's built-in hashing where the output type is u64. I'll make an alias
// so the code is slightly more readable.
type Hash = u64;

/// The most basic blockchain header possible. We learned its basic structure from lecture.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    // We know from the lecture that we will probably need these, but we don't need them yet.
    extrinsics_root: (),
    state_root: (),
    consensus_digest: (),
}

// Here are the methods for creating a new header and verifying headers.
// It is your job to write them.
impl Header {
    /// Returns a new valid genesis header.
    fn genesis() -> Self {
        Header {
            parent: 0,
            height: 0,
            extrinsics_root: (),
            state_root: (),
            consensus_digest: (),
        }
    }

    /// Create and return a valid child header.
    fn child(&self) -> Self {
        Header {
            parent: hash(&self),
            height: self.height + 1,
            extrinsics_root: (),
            state_root: (),
            consensus_digest: (),
        }
    }

    /// Verify that all the given headers form a valid chain from this header to the tip.
    /// An "entire" chain can be verified by calling this method on a genesis header.
    /// This method may assume that the block on which it is called is valid, but it
    /// must verify all of the blocks in the slice;
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut expected_parent = hash(&self);
        let mut expected_height = self.height + 1;

        let mut is_valid = true;
        for header in chain {
            is_valid &= header.parent == expected_parent && header.height == expected_height;
            expected_parent = hash(&header); // Update the expected hash for the next iteration
            expected_height += 1; // Increment expected height for the next header
        }
        is_valid
    }
}

// And finally a few functions to use the code we just

/// Build and return a valid chain with exactly five blocks including the genesis block.
fn build_valid_chain_length_5() -> Vec<Header> {
    let mut header_chain = Vec::with_capacity(5); // Optimize by setting the initial capacity

    let mut header = Header::genesis();
    for _ in 0..5 {
        header_chain.push(header.clone()); // Clone the header if Header does not derive Copy
        header = header.child();
    }

    header_chain
}

/// Build and return a chain with at least three headers.
/// The chain should start with a proper genesis header,
/// but the entire chain should NOT be valid.
fn build_an_invalid_chain() -> Vec<Header> {
    let genesis = Header::genesis();
    let mut invalid_header = genesis.child();
    invalid_header.height = 5;
    let child_of_invalid_header = invalid_header.child();

    vec![genesis, invalid_header, child_of_invalid_header]
}

// To run these tests: `cargo test bc_1
#[test]
fn bc_1_genesis_block_height() {
    let g = Header::genesis();
    assert!(g.height == 0);
}

#[test]
fn bc_1_genesis_block_parent() {
    let g = Header::genesis();
    assert!(g.parent == 0);
}

#[test]
fn bc_1_child_block_height() {
    let g = Header::genesis();
    let b1 = g.child();
    assert!(b1.height == 1);
}

#[test]
fn bc_1_child_block_parent() {
    let g = Header::genesis();
    let b1 = g.child();
    assert!(b1.parent == hash(&g));
}

#[test]
fn bc_1_verify_genesis_only() {
    let g = Header::genesis();

    assert!(g.verify_sub_chain(&[]));
}

#[test]
fn bc_1_verify_three_blocks() {
    let g = Header::genesis();
    let b1 = g.child();
    let b2 = b1.child();

    assert!(g.verify_sub_chain(&[b1, b2]));
}

#[test]
fn bc_1_cant_verify_invalid_height() {
    // This and following tests use the student's own verify function so as
    // not to give away the solution to writing that function.
    let g = Header::genesis();
    let mut b1 = g.child();
    b1.height = 10;

    assert!(!g.verify_sub_chain(&[b1]))
}

#[test]
fn bc_1_cant_verify_invalid_parent() {
    // This test chooses to use the student's own verify function so as
    // not to give away the solution to writing that function.
    let g = Header::genesis();
    let mut b1 = g.child();
    b1.parent = 10;

    assert!(!g.verify_sub_chain(&[b1]))
}

#[test]
fn bc_1_verify_chain_length_five() {
    // This test chooses to use the student's own verify function.
    // This should be relatively safe given that we have already tested that function.
    let chain = build_valid_chain_length_5();
    assert!(chain[0].verify_sub_chain(&chain[1..]))
}

#[test]
fn bc_1_invalid_chain_is_really_invalid() {
    // This test chooses to use the student's own verify function.
    // This should be relatively safe given that we have already tested that function.
    let invalid_chain = build_an_invalid_chain();
    assert!(!invalid_chain[0].verify_sub_chain(&invalid_chain[1..]))
}
