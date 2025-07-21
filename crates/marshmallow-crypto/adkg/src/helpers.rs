use std::fmt::{Display, Formatter};
use std::ops::Index;

/// This structure represents the identifier of a party from 1 to n.
/// When indexing with PartyId in slices, this structures accesses the
/// (party id - 1)th index.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct PartyId(pub usize);

struct PartyIter {
    next: Option<PartyId>,
    n: usize,
}

impl Iterator for PartyIter {
    type Item = PartyId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.next?;
        self.next = if id.0 >= self.n {
            None
        } else {
            Some(PartyId(id.0 + 1))
        };
        Some(id)
    }
}

impl PartyId {
    pub const fn const_from(i: u32) -> Self {
        Self(i as usize)
    }

    pub const fn iter_all(n: usize) -> impl Iterator<Item = PartyId> {
        PartyIter {
            next: Some(Self(1)),
            n,
        }
    }

    pub fn as_index(&self) -> usize {
        if self.0 == 0 {
            panic!("Invalid PartyId, should be greater than zero");
        }

        self.0 - 1
    }
}

impl Display for PartyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

/// Allows to index arrays directly using PartyId.
/// Outputs an element at index (party id - 1).
impl<T> Index<PartyId> for [T] {
    type Output = T;

    fn index(&self, idx: PartyId) -> &Self::Output {
        &self[idx.0 - 1] // may panic
    }
}

/// Allows to index vectors directly using PartyId.
/// Outputs an element at index (party id - 1).
impl<T> Index<PartyId> for Vec<T> {
    type Output = T;

    fn index(&self, idx: PartyId) -> &Self::Output {
        &self[idx.0 - 1] // may panic
    }
}

/// Converts a usize into a PartyId.
/// While the value is not checked, one should never try to convert 0.
impl From<usize> for PartyId {
    fn from(value: usize) -> Self {
        PartyId(value)
    }
}

/// Converts a u32 into a PartyId.
/// While the value is not checked, one should never try to convert 0.
impl From<u32> for PartyId {
    fn from(value: u32) -> Self {
        PartyId(usize_from_u32(value))
    }
}

/// Converts a PartyId into a usize.
impl From<PartyId> for usize {
    fn from(val: PartyId) -> Self {
        val.0
    }
}

impl From<&PartyId> for usize {
    fn from(val: &PartyId) -> Self {
        val.0
    }
}

/// Converts a PartyId into a u64.
impl From<PartyId> for u64 {
    fn from(val: PartyId) -> Self {
        u64_from_usize(val.0)
    }
}

impl From<&PartyId> for u64 {
    fn from(val: &PartyId) -> Self {
        u64_from_usize(val.0)
    }
}

#[allow(unused)]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub const fn usize_from_u32(x: u32) -> usize {
    x as usize
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#[inline(always)]
pub const fn u64_from_usize(x: usize) -> u64 {
    x as u64
}
