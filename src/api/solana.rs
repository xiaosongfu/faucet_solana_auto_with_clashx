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

pub async fn get_balance(client: &reqwest::Client, address: &str) -> reqwest::Result<u64> {
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

pub async fn request_airdrop(client: &reqwest::Client, address: &str) -> reqwest::Result<()> {
    let r = client
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
        .await?;
    if r.status() == reqwest::StatusCode::OK {
        println!("======> {}", r.text().await.unwrap_or(String::new()));
        Ok(())
    } else {
        Err(r.error_for_status().unwrap_err())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_ok() {
        let solana_api_client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("socks5://127.0.0.1:7890").unwrap())
            .build()
            .unwrap();

        let mut balance = get_balance(
            &solana_api_client,
            "BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5",
        )
        .await
        .unwrap();
        println!("{:?}", balance);
        // request_airdrop(&solana_api_client, "BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5")
        //     .await
        //     .unwrap();
        // balance = get_balance(&solana_api_client, "BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5")
        //     .await
        //     .unwrap();
        // println!("{:?}", balance);
    }
}
