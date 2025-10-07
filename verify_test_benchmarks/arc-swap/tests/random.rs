//! Let it torture the implementation with some randomized operations.

use std::mem;
use std::sync::Arc;

use arc_swap::{ArcSwapAny, DefaultStrategy, IndependentStrategy};
use once_cell::sync::Lazy;
use proptest::prelude::*;

#[derive(Copy, Clone, Debug)]
enum OpsInstruction {
    Store(usize),
    Swap(usize),
    LoadFull,
    Load,
}

impl OpsInstruction {
    fn random() -> impl Strategy<Value = Self> {
        prop_oneof![
            any::<usize>().prop_map(Self::Store),
            any::<usize>().prop_map(Self::Swap),
            Just(Self::LoadFull),
            Just(Self::Load),
        ]
    }
}

proptest! {}

const LIMIT: usize = 5;
#[cfg(not(miri))]
const SIZE: usize = 100;
#[cfg(miri)]
const SIZE: usize = 10;
static ARCS: Lazy<Vec<Arc<usize>>> = Lazy::new(|| (0..LIMIT).map(Arc::new).collect());

#[derive(Copy, Clone, Debug)]
enum SelInstruction {
    Swap(usize),
    Cas(usize, usize),
}

impl SelInstruction {
    fn random() -> impl Strategy<Value = Self> {
        prop_oneof![
            (0..LIMIT).prop_map(Self::Swap),
            (0..LIMIT, 0..LIMIT).prop_map(|(cur, new)| Self::Cas(cur, new)),
        ]
    }
}
