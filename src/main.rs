use std::path::Path;

use sirius::{
    ff::Field,
    halo2_proofs::poly::Rotation,
    ivc::{
        step_circuit::{trivial, AssignedCell, ConstraintSystem, Layouter},
        SynthesisError,
    },
    prelude::{
        bn256::{new_default_pp, C1Affine, C1Scalar, C2Affine, C2Scalar},
        CommitmentKey, PrimeField, StepCircuit, IVC,
    },
};

/// Number of folding steps
const FOLD_STEP_COUNT: usize = 5;

// === PRIMARY ===

/// Arity : Input/output size per fold-step for primary step-circuit
const A1: usize = 1;

/// Input to be passed on the zero step to the primary circuit
const PRIMARY_Z_0: [C1Scalar; A1] = [C1Scalar::ONE];

/// Key size for Primary Circuit
///
/// This is the minimum value, for your circuit you may get the output that the key size is
/// insufficient, then increase this constant
const PRIMARY_COMMITMENT_KEY_SIZE: usize = 21;

/// Table size for Primary Circuit
///
/// Requires at least 17, for service purposes, but if the primary requires more, increase the
/// constant
const PRIMARY_CIRCUIT_TABLE_SIZE: usize = 17;

// === SECONDARY ===

/// Arity : Input/output size per fold-step for secondary step-circuit
/// For tivial case it can be any number
const A2: usize = 1;

/// Input to be passed on the zero step to the secondary circuit
const SECONDARY_Z_0: [C2Scalar; A1] = [C2Scalar::ZERO];

/// Table size for Primary Circuit
///
/// Requires at least 17, for service purposes, but if the primary requires more, increase the
/// constant
const SECONDARY_CIRCUIT_TABLE_SIZE: usize = 17;

/// Key size for Secondary Circuit
///
/// This is the minimum value, for your circuit you may get the output that the key size is
/// insufficient, then increase this constant
const SECONDARY_COMMITMENT_KEY_SIZE: usize = 21;

use sirius::halo2_proofs::plonk::{Advice, Column, Selector};
#[derive(Debug, Clone)]
struct MyConfig {
    /// Since we will have one gate
    /// representing the sum, we add
    /// one selector.
    s: Selector,
    /// This column will copy z_in and
    /// represent the sum with itself
    input: Column<Advice>,
    /// This will be the result of the
    /// calculation and cells from this
    /// column will be returned as `z_out`
    output: Column<Advice>,
}

/// This page is a template for your circuit
/// Within this code - it returns the input unchanged
struct MyStepCircuit {}

impl<const A: usize, F: PrimeField> StepCircuit<A, F> for MyStepCircuit {
    /// This is a configuration object that stores things like columns.
    type Config = MyConfig;

    fn configure(cs: &mut ConstraintSystem<F>) -> Self::Config {
        let config = Self::Config {
            s: cs.selector(),
            input: cs.advice_column(),
            output: cs.advice_column(),
        };

        // Allow equality check for `input`
        // for check consistency with `z_in`
        cs.enable_equality(config.input);

        // Creating a gate that reflects the sum
        cs.create_gate("sum", |meta| {
            let s = meta.query_selector(config.s);
            let input = meta.query_advice(config.input, Rotation::cur());
            let output = meta.query_advice(config.output, Rotation::cur());

            vec![s * (input.clone() + input - output)]
        });

        config
    }

    fn synthesize_step(
        &self,
        config: Self::Config,
        layouter: &mut impl Layouter<F>,
        z_i: &[AssignedCell<F, F>; A],
    ) -> Result<[AssignedCell<F, F>; A], SynthesisError> {
        let output = layouter.assign_region(
            || "main",
            |mut region| {
                z_i.iter()
                    .enumerate() // we need an index to use as offset
                    .map(|(i, cell)| {
                        // Enable selector to trigger the gate
                        config.s.enable(&mut region, i)?;

                        let input = region.assign_advice(
                            || "input",
                            config.input,
                            i,
                            || cell.value().copied(),
                        )?;

                        // Check at the constraint system level that the cells are equal
                        region.constrain_equal(cell.cell(), input.cell())?;

                        // Perform the operation and place the result in the cell
                        let output = region.assign_advice(
                            || "output",
                            config.output,
                            i,
                            || input.value().copied() + input.value(),
                        )?;

                        Ok(output)
                    })
                    .collect::<Result<Vec<_>, _>>()
            },
        )?;

        Ok(output
            .try_into() // convert to array
            .expect("safe, because collect from input array"))
    }
}

fn main() {
    let sc1 = MyStepCircuit {};
    let sc2 = trivial::Circuit::<A2, C2Scalar>::default();

    // This folder will store the commitment key so that we don't have to generate it every time.
    //
    // NOTE: since the key files are not serialized, but reflected directly from memory, the
    // functions to load them is `unsafe`
    let key_cache = Path::new(".cache");

    println!("start setup primary commitment key: bn256");

    // Safety: because the cache file is correct
    let primary_commitment_key = unsafe {
        CommitmentKey::<C1Affine>::load_or_setup_cache(
            key_cache,
            "bn256",
            PRIMARY_COMMITMENT_KEY_SIZE,
        )
        .unwrap()
    };

    println!("start setup secondary commitment key: grumpkin");

    // Safety: because the cache file is correct
    let secondary_commitment_key = unsafe {
        CommitmentKey::<C2Affine>::load_or_setup_cache(
            key_cache,
            "grumpkin",
            SECONDARY_COMMITMENT_KEY_SIZE,
        )
        .unwrap()
    };

    let pp = new_default_pp::<A1, _, A2, _>(
        SECONDARY_CIRCUIT_TABLE_SIZE as u32,
        &primary_commitment_key,
        &sc1,
        PRIMARY_CIRCUIT_TABLE_SIZE as u32,
        &secondary_commitment_key,
        &sc2,
    );

    let mut ivc = IVC::new(&pp, &sc1, PRIMARY_Z_0, &sc2, SECONDARY_Z_0, true)
        .expect("failed to create `IVC`");
    println!("ivc created");

    for step in 1..FOLD_STEP_COUNT {
        // you can modify circuit data here
        ivc.fold_step(&pp, &sc1, &sc2)
            .expect("failed to run fold step");

        println!("folding step {step} was successful");
    }

    ivc.verify(&pp).expect("failed to verify ivc");
    println!("verification successful");

    println!("success");
}
