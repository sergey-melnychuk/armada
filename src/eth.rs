use crate::api::gen::NumAsHex;

#[derive(Clone, Debug)]
pub struct State {
    pub state_root: NumAsHex,
    pub state_block_hash: NumAsHex,
    pub state_block_number: u64,
}

// TODO: add trait bounds? `EthApi: Send + Sync + Clone + 'static`
#[async_trait::async_trait]
pub trait EthApi {
    async fn get_state(&self, address: &str) -> anyhow::Result<State>;
}

#[async_trait::async_trait]
impl EthApi for EthClient {
    async fn get_state(&self, address: &str) -> anyhow::Result<State> {
        let hash = self.get_latest_block_hash().await?;
        Ok(State {
            state_root: self
                .call_starknet_contract(hash.as_ref(), address, "stateRoot()")
                .await?,
            state_block_hash: self
                .call_starknet_contract(hash.as_ref(), address, "stateBlockHash()")
                .await?,
            state_block_number: self
                .call_starknet_contract(hash.as_ref(), address, "stateBlockNumber()")
                .await
                .and_then(parse_hex_as_num)?,
        })
    }
}

#[derive(Clone)]
pub struct EthClient {
    http: reqwest::Client,
    url: String,
}

impl EthClient {
    pub fn new(url: &str) -> Self {
        let http = reqwest::ClientBuilder::new()
            .build()
            .expect("Failed to create HTTP client");
        Self {
            http,
            url: url.to_string(),
        }
    }

    async fn get_latest_block_hash(&self) -> anyhow::Result<NumAsHex> {
        self.call_ethereum(serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [
                "latest",
                false
            ],
            "id": 0
        }))
        .await
        .and_then(|value| parse_num_as_hex(&value["hash"]))
    }

    async fn call_starknet_contract(
        &self,
        block_hash: &str,
        address: &str,
        signature: &str,
    ) -> anyhow::Result<NumAsHex> {
        let data = encode_ethereum_call_data(signature.as_bytes());
        self.call_ethereum(serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [
                {
                    "to": address,
                    "value": "0x0",
                    "data": data
                },
                {"blockHash": block_hash}
            ],
            "id": 0
        }))
        .await
        .and_then(|value| parse_num_as_hex(&value))
    }

    async fn call_ethereum(&self, value: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        let response: serde_json::Value = self
            .http
            .post(&self.url)
            .json(&value)
            .send()
            .await?
            .json()
            .await?;
        // TODO: report error if any
        Ok(response["result"].clone())
    }
}

fn parse_hex_as_num(num: NumAsHex) -> anyhow::Result<u64> {
    let hex = num.as_ref().to_string();
    let hex = hex.strip_prefix("0x").unwrap_or(&hex);
    let num = u64::from_str_radix(hex, 16)?;
    Ok(num)
}

fn parse_num_as_hex(value: &serde_json::Value) -> anyhow::Result<NumAsHex> {
    value
        .as_str()
        .ok_or(anyhow::anyhow!("Failed to parse hex number"))
        .and_then(|hex| {
            if hex == "0x" {
                // input "0x" here causes following runtime error:
                // "thread 'eth::tests::test_goerli' has overflowed its stack"
                NumAsHex::try_new("0x0").map_err(|e| anyhow::anyhow!(e))
            } else {
                NumAsHex::try_new(hex).map_err(|e| anyhow::anyhow!(e))
            }
        })
}

fn encode_ethereum_call_data(signature: &[u8]) -> String {
    let mut output: [u8; 32] = Default::default();
    keccak_hash::keccak_256(signature, &mut output[..]);
    format!("0x{}", hex::encode(&output[0..4]))
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test --package armada --lib -- eth::tests::test_goerli --exact --nocapture
    #[tokio::test]
    #[ignore = "needs valid INFURA_TOKEN env var"]
    async fn test_goerli() -> anyhow::Result<()> {
        let token = std::env::var("INFURA_TOKEN")?;
        let url = format!("https://goerli.infura.io/v3/{token}");
        let client = EthClient::new(&url);

        let hash = client.get_latest_block_hash().await?;
        println!("ethereum block hash: {}\n", hash.as_ref());

        let address = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
        let state = client.get_state(address).await?;
        println!("starknet stateRoot: {}", state.state_root.as_ref());
        println!(
            "starknet stateBlockHash: {}",
            state.state_block_hash.as_ref()
        );
        println!("starknet stateBlockNumber: {}", state.state_block_number);

        Ok(())
    }

    #[test]
    fn test_encode_ethereum_call_data() {
        for (input, expected) in [
            ("stateRoot()", "0x9588eca2"),
            ("stateBlockHash()", "0x382d83e3"),
            ("stateBlockNumber()", "0x35befa5d"),
        ] {
            assert_eq!(encode_ethereum_call_data(input.as_bytes()), expected);
        }
    }

    #[test]
    fn test_parse_hex_as_num() -> anyhow::Result<()> {
        let num = NumAsHex::try_new("0x107d209")?;
        assert_eq!(parse_hex_as_num(num)?, 17289737);
        Ok(())
    }
}
