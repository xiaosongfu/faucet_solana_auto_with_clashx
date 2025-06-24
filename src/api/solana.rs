use serde_json::json;

// {
//     "jsonrpc": "2.0",
//     "result": {
//         "context": {
//             "apiVersion": "2.2.16",
//             "slot": 389327985
//         },
//         "value": 15000000000
//     },
//     "id": "0f45e831-7086-4064-9da6-881b001068e5"
// }
#[derive(Debug, serde::Deserialize)]
struct GetBalance {
    pub result: GetBalanceResult,
}

#[derive(Debug, serde::Deserialize)]
struct GetBalanceResult {
    pub value: u64,
}

pub async fn get_balance(address: &str) -> reqwest::Result<u64> {
    // 不可以复用 Client !!
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(crate::api::LOCAL_SOCKS5_PROXY).unwrap())
        .build()
        .unwrap();
    let balance = client
        .post("https://api.devnet.solana.com")
        .json(&json!(
            {
                "jsonrpc": "2.0",
                "id": "0f45e831-7086-4064-9da6-881b001068e5",
                "method": "getBalance",
                "params": [address]
            }
        ))
        .send()
        .await?
        .json::<GetBalance>()
        .await?;
    Ok(balance.result.value)
}

// {"jsonrpc":"2.0","result":"656xc1h8RCCSB9dYzzfRbLbcQNjaTkR9bsEjKxuLqB9NKmKgSUsfB3se4d4KFb7EZ7PAtJQoMiMUpnKvGD16CFLt","id":"0f45e831-7086-4064-9da6-881b001068e5"}
// {"jsonrpc":"2.0","error":{"code": 429, "message":"You've either reached your airdrop limit today or the airdrop faucet has run dry. Please visit https://faucet.solana.com for alternate sources of test SOL"}, "id": "0f45e831-7086-4064-9da6-881b001068e5" }
#[derive(Debug, serde::Deserialize)]
struct RequestAirdrop {
    pub result: Option<String>,
    pub error: Option<RequestAirdropError>,
}

#[derive(Debug, serde::Deserialize)]
struct RequestAirdropError {
    pub code: u64,
    // pub message: String,
}

pub async fn request_airdrop(address: &str) -> anyhow::Result<String> {
    // 不可以复用 Client !!
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(crate::api::LOCAL_SOCKS5_PROXY).unwrap())
        .build()
        .unwrap();
    let body = client
        .post("https://api.devnet.solana.com")
        .json(&json!(
            {
                "jsonrpc": "2.0",
                "id": "0f45e831-7086-4064-9da6-881b001068e5",
                "method": "requestAirdrop",
                "params": [
                    address,
                    5_000_000_000u64
                ]
            }
        ))
        .send()
        .await?
        .json::<RequestAirdrop>()
        .await?;
    if let Some(tx) = body.result {
        Ok(tx)
    } else {
        Err(anyhow::anyhow!("{:?}", body.error.unwrap().code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_ok() {
        let mut balance = get_balance("BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5")
            .await
            .unwrap();
        println!("{:?}", balance);
        // request_airdrop("BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5")
        //     .await
        //     .unwrap();
        // balance = get_balance("BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5")
        //     .await
        //     .unwrap();
        // println!("{:?}", balance);
    }
}
