use crate::sponge::Absorb;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[cfg(not(feature = "std"))]
use ark_std::vec::Vec;

// Some CRHs output byte arrays.

// Merkle tree Config requires a trait that implements the following functions.
// As far as I can tell (z-tech), this is the thinnest possible trait that satisfies the requirements.

#[derive(Clone, Debug, Eq, PartialEq, Hash, CanonicalSerialize, CanonicalDeserialize)]
pub struct ByteDigest<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for ByteDigest<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> AsRef<[u8]> for ByteDigest<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> From<[u8; N]> for ByteDigest<N> {
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Absorb for ByteDigest<N> {
    fn to_sponge_bytes(&self, dest: &mut Vec<u8>) {
        dest.extend_from_slice(&self.0);
    }

    fn to_sponge_field_elements<F: ark_ff::PrimeField>(&self, dest: &mut Vec<F>) {
        dest.push(F::from_be_bytes_mod_order(&self.0));
    }
}
