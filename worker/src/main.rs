use anyhow::Result;
use clap::Parser;
use common::prover_state::{cli::CliProverStateConfig, TableLoadStrategy};
use dotenvy::dotenv;
use ops::register;
use paladin::runtime::WorkerRuntime;

mod init;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    paladin: paladin::config::Config,
    #[clap(flatten)]
    prover_state_config: CliProverStateConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init::tracing();
    let args = Cli::parse();

    let persistence = args.prover_state_config.persistence;
    args.prover_state_config
        .into_prover_state_manager(persistence.with_load_strategy(TableLoadStrategy::OnDemand))
        .initialize()?;

    let runtime = WorkerRuntime::from_config(&args.paladin, register()).await?;
    runtime.main_loop().await?;

    Ok(())
}
