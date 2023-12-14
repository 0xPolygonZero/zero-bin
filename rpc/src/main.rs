use std::io::Write;

use anyhow::Result;
use clap::Parser;
use cli::Commands;
use protocol_decoder::types::BlockHeight;
use rpc::fetch_prover_input;

mod cli;
mod init;
mod rpc;

/// The RPC binary doesn't really care for the most part about the checkpoint
/// height, so we'll just hardcode it to `0` for now.
const DUMMY_RPC_CHECKPOINT_HEIGHT: BlockHeight = 0;

#[tokio::main]
async fn main() -> Result<()> {
    init::tracing();
    let args = cli::Cli::parse();

    match args.command {
        Commands::Fetch {
            rpc_url,
            block_number,
        } => {
            let prover_input =
                fetch_prover_input(&rpc_url, block_number, DUMMY_RPC_CHECKPOINT_HEIGHT).await?;
            std::io::stdout().write_all(&serde_json::to_vec(&prover_input)?)?;
        }
    }
    Ok(())
}
