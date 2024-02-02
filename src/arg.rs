use std::collections::HashSet;

const ARMADA_INFURA_TOKEN: &str = "ARMADA_INFURA_TOKEN";

pub struct Args {
    pub data_dir: String,
    pub network: String,
    pub infura_token: String,
    pub flags: HashSet<String>,
}

fn get_pos(
    args: &[String],
    idx: usize,
    name: &'static str,
) -> anyhow::Result<String> {
    if let Some(val) = args.get(idx) {
        return Ok(val.clone());
    }
    anyhow::bail!("Missing required argument: '{name}' at position <{idx}>");
}

pub fn resolve() -> anyhow::Result<Args> {
    let args = std::env::args().collect::<Vec<String>>();

    Ok(Args {
        data_dir: get_pos(&args, 1, "data-directory")?,
        network: get_pos(&args, 2, "network")?,
        infura_token: std::env::var(ARMADA_INFURA_TOKEN)?,
        flags: args
            .into_iter()
            .skip(3)
            .filter_map(|arg| arg.strip_prefix("--").map(|x| x.to_string()))
            .collect(),
    })
}
