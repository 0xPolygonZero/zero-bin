use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use paladin::runtime::Runtime;
use plonky2_evm::proof::PublicValues;
use plonky_block_proof_gen::types::PlonkyProofIntern;

/// The main function for the jerigon mode.
pub(crate) async fn jerigon_main(
    runtime: Runtime,
    rpc_url: &str,
    block_number: u64,
    previous: Option<PlonkyProofIntern>,
    proof_output_path_opt: Option<PathBuf>,
    checkpoint_height: Option<u64>,
) -> Result<()> {
    let prover_input = rpc::fetch_prover_input(rpc_url, block_number).await?;

    let proof = prover_input.prove(&runtime, previous).await;

    let recovered_pub_vals =
        PublicValues::from_public_inputs(&proof.as_ref().unwrap().intern.public_inputs);

    println!(
        "Public vals of block {}: {:#?}",
        block_number, recovered_pub_vals
    );

    runtime.close().await?;

    let proof = serde_json::to_vec(&proof?.intern)?;
    write_proof(proof, proof_output_path_opt)
}

fn write_proof(proof: Vec<u8>, proof_output_path_opt: Option<PathBuf>) -> Result<()> {
    match proof_output_path_opt {
        Some(p) => {
            if let Some(parent) = p.parent() {
                create_dir_all(parent)?;
            }

            let mut f = File::create(p)?;
            f.write_all(&proof)?;
        }
        None => std::io::stdout().write_all(&proof)?,
    }

    Ok(())
}
