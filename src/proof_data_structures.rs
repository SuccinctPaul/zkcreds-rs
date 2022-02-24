use crate::attrs::{Attrs, AttrsVar};

use core::marker::PhantomData;

use ark_crypto_primitives::{
    commitment::{constraints::CommitmentGadget, CommitmentScheme},
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
};
use ark_ec::PairingEngine;
use ark_ff::ToConstraintField;
use ark_groth16::{
    PreparedVerifyingKey as Groth16PreparedVerifyingKey, Proof as Groth16Proof,
    ProvingKey as Groth16ProvingKey,
};

//
// Predicate data structures
//

/// Represents the proving key for a predicate proof
pub struct PredProvingKey<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    AC::Output: ToConstraintField<E::Fr>,
    ACG: CommitmentGadget<AC, E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) pk: Groth16ProvingKey<E>,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

impl<E, A, AV, AC, ACG, H, HG> PredProvingKey<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    AC::Output: ToConstraintField<E::Fr>,
    ACG: CommitmentGadget<AC, E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub fn prepare_verifying_key(&self) -> PredVerifyingKey<E, A, AV, AC, ACG, H, HG> {
        let pvk = ark_groth16::prepare_verifying_key(&self.pk.vk);
        PredVerifyingKey {
            pvk,
            _marker: self._marker,
        }
    }
}

/// Represents the verifying key for a predicate proofs
pub struct PredVerifyingKey<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    AC::Output: ToConstraintField<E::Fr>,
    ACG: CommitmentGadget<AC, E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) pvk: Groth16PreparedVerifyingKey<E>,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

/// Represents the prepared public inputs to a predicate proof
pub struct PredPublicInput<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    AC::Output: ToConstraintField<E::Fr>,
    ACG: CommitmentGadget<AC, E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) pinput: E::G1Projective,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

/// Represents a predicate proof
pub struct PredProof<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    AC::Output: ToConstraintField<E::Fr>,
    ACG: CommitmentGadget<AC, E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) proof: Groth16Proof<E>,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

//
// Merkle tree membership data structures
//

/// Represents the proving key for a Merkle tree membership proof
pub struct TreeProvingKey<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    ACG: CommitmentGadget<AC, E::Fr>,
    AC::Output: ToConstraintField<E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) pk: Groth16ProvingKey<E>,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

/// Represents the verifying key for Merkle tree membership proofs
pub struct TreeVerifyingKey<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    ACG: CommitmentGadget<AC, E::Fr>,
    AC::Output: ToConstraintField<E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) pvk: Groth16PreparedVerifyingKey<E>,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

/// Represents the prepared public inputs to a Merkle tree membership proof
pub struct TreePublicInput<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    ACG: CommitmentGadget<AC, E::Fr>,
    AC::Output: ToConstraintField<E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) pinput: E::G1Projective,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}

/// Represents a Merkle tree membership proof
pub struct TreeProof<E, A, AV, AC, ACG, H, HG>
where
    E: PairingEngine,
    A: Attrs<E::Fr, AC>,
    AV: AttrsVar<E::Fr, A, AC, ACG>,
    AC: CommitmentScheme,
    AC::Output: ToConstraintField<E::Fr>,
    ACG: CommitmentGadget<AC, E::Fr>,
    H: TwoToOneCRH,
    H::Output: ToConstraintField<E::Fr>,
    HG: TwoToOneCRHGadget<H, E::Fr>,
{
    pub(crate) proof: Groth16Proof<E>,
    pub(crate) _marker: PhantomData<(A, AV, AC, ACG, H, HG)>,
}
