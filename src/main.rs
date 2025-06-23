use std::collections::HashSet;

mod api;

const ADDRESSES: [&str; 10] = [
    "BfnPrCHwe5jGa87nriUTkFNUGZFPWHfE8s6eYgMeF8S5",
    "3dJhG2cSw9Tqhu6y5jUXoMfwz2d1n6gNUXcB23QwiCop",
    "FTddWu4BM7523crdqEcSCBcfnCFqY8cNsiBuodsYVf2X",
    "7yX7j33jabR77AXVyk4q5TsGa5cHXM6wKS2uPVcAdLNK",
    "2To2szMKhPh8CszDbNapDqkKRPq98p7M2dU5VzGRh6vw",
    "DRxLSKXrE2HHXxSbAiygzweko7RWSk8troMoogeFAEnh",
    "HKmharEBwiigKVVQAY4es1n4AWh2tUmZK73xP41aSEvW",
    "H32XrTD7xCgqbc1q5Nw5Rd33FgWGEsoFtKNSrWiSKHan",
    "4kSH34VmCE9ZJLxAzsa3D8Bgwj1K7xheyjGJ5WkEGkg8",
    "Ga7xRstg3QDMgvwscBKpNsffPeFUMUGjECL5xn9XW37z",
];

const SKIP_PROXIES: [&str; 7] = [
    "DIRECT",
    "REJECT",
    "防失联网址: haita.link",
    "售前/售后 请联系网站在线客服",
    "Haita Cloud",
    "自动选择",
    "故障转移",
];

#[tokio::main]
async fn main() {
    let mut ip_set = HashSet::new();

    let clashx_api_client = reqwest::Client::builder().no_proxy().build().unwrap();
    let proxies = api::clashx::group_proxies(&clashx_api_client, api::clashx::PROXY_GROUP_GLOBAL)
        .await
        .expect("query group proxies");

    let mut idx = 0;
    for proxy in proxies.all {
        if SKIP_PROXIES.contains(&proxy.as_str()) {
            continue;
        }

        if idx >= ADDRESSES.len() {
            break;
        }

        println!(
            "set clashx proxy: {}::{}",
            api::clashx::PROXY_GROUP_GLOBAL,
            proxy
        );
        if let Ok(_) = api::clashx::set_group_proxy(
            &clashx_api_client,
            api::clashx::PROXY_GROUP_GLOBAL,
            proxy.as_str(),
        )
        .await
        {
            tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

            if let Ok(ip_info) = api::ip::ip_ifo().await {
                println!("\t ipinfo: {:?}", ip_info);

                if ip_info.country.eq("CN") {
                    continue;
                }

                if !ip_set.contains(&ip_info.ip) {
                    let address = ADDRESSES[idx];
                    let mut balance = api::solana::get_balance(address)
                        .await
                        .unwrap_or(0)
                        .to_string();
                    println!(
                        "\t [{}] before balance: {:?}",
                        address,
                        &balance[..balance.len() - 9]
                    );

                    let request_airdrop_result = api::solana::request_airdrop(address).await;
                    if request_airdrop_result.is_ok() {
                        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

                        balance = api::solana::get_balance(address)
                            .await
                            .unwrap_or(0)
                            .to_string();
                        println!(
                            "\t ✅ [{}] after balance: {:?}",
                            address,
                            &balance[..balance.len() - 9]
                        );

                        idx += 1;
                    } else {
                        println!(
                            "\t ❌ [{}] request airdrop failed: {}",
                            address,
                            request_airdrop_result.err().unwrap()
                        );
                    }

                    // save IP ignoring 'requestAirdrop' succes or failed
                    ip_set.insert(ip_info.ip);
                } else {
                    println!("\t ⚠️ IP already used")
                }
            } else {
                println!("query ip failed")
            }
        }
    }
}
