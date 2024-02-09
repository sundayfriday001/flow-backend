use flow_lib::command::prelude::*;
use std::time::Duration;
use tokio::time;
use tracing::info;

pub const NAME: &str = "wait";

const DEFINITION: &str = flow_lib::node_definition!("wait.json");

fn build() -> BuildResult {
    static CACHE: BuilderCache =
        BuilderCache::new(|| CmdBuilder::new(DEFINITION)?.check_name(NAME));
    Ok(CACHE.clone()?.build(run))
}

flow_lib::submit!(CommandDescription::new(NAME, |_| { build() }));

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    value: Value,
    wait_for: Value,
    duration_ms: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {}

async fn run(_ctx: Context, input: Input) -> Result<Output, CommandError> {
    info!("duration: {:?}", input.duration_ms);
    match input.duration_ms {
        Some(duration) => {
            time::sleep(Duration::from_millis(duration)).await;
        }
        None => {}
    };

    Ok(Output {})
}
