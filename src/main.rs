#![feature(duration_constructors_lite)]

use std::collections::HashSet;

mod api;

const ADDRESSES: [&str; 20] = [
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
    "6fHocfDLvNWqypb4ALddC1Hk1753auYHKxi29dnQZJP9",
    "5h8FSS7aQriPgB4gR5ABnze9niM7LamhnwoMRXYjapuE",
    "4e1beoUeJGEbevcDZJ9wjiTc3aXoftte8YkrG4UAyjFV",
    "Hn4tkCcEUT2hVzYcgW1Xw8rdiSKzoCS9w6hoUjguugZV",
    "BXf3XN8B4Az7m3saGoJx42HuHTSRvuSd551jW322HFfk",
    "ASFd5j4uR8zoHLuWD4HbcPiFtfF9jZkA4PJf77beEWcF",
    "7biLjQc1tDUotGvuzgAVc554wq6ZUwF1AiGLbadgaUX6",
    "EsNsB4PhyNSfE9B6FVMfwzkSCWCcsPRzQw8LS5odVhEB",
    "47Yx8QwGFpCmF8vZiNzrg7tpQ5mbQJ6wBFnQdJQNcxWe",
    "EmdoBAvJhcFc2UgKo2rTQUkxbb7Sj5NZYR9g3Lwz7E2j",
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

async fn logic() {
    let mut ip_set = HashSet::new();
    let mut total_sol = 0f64;

    let clashx_api_client = reqwest::Client::builder().no_proxy().build().unwrap();

    // 当前 api::clashx::switch_mode_to_global(...) 方法设置的全局模式有问题，需要手动设置为全局模式
    let mode = api::clashx::get_mode(&clashx_api_client)
        .await
        .expect("get mode");
    if mode.mode != "global" {
        println!("🙅‍♂️ mode is not global,please switch to global mode manually");
        return;
    }

    // get all proxies of GLOBAL proxy group
    let proxies =
        api::clashx::get_group_proxies(&clashx_api_client, api::clashx::PROXY_GROUP_GLOBAL)
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

        println!("set clashx proxy:{}", proxy);
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
                    let balance = api::solana::get_balance(address).await.unwrap_or(0);
                    let balance = balance as f64 / 1_000_000_000.0;
                    println!("\t [{}/{}] Balance: {:?}", idx, address, balance,);
                    total_sol += balance;

                    let request_airdrop_result = api::solana::request_airdrop(address).await;
                    if let Ok(tx) = request_airdrop_result {
                        println!("\t ✅ Tx: {:?}", tx);

                        idx += 1;
                    } else {
                        println!("\t ❌ ErrCode: {}", request_airdrop_result.err().unwrap());
                    }

                    // save IP ignoring 'requestAirdrop' succes or failed
                    ip_set.insert(ip_info.ip);
                } else {
                    println!("\t ⚠️ IP already used")
                }
            } else {
                println!("\t ❌ query ip failed")
            }
        }
    }

    println!("\n🎉 You have {} SOL 🎉", total_sol);
}

#[tokio::main]
async fn main() {
    logic().await;

    // loop {
    //     logic().await;
    //     tokio::time::sleep(tokio::time::Duration::from_hours(3)).await;
    // }
}
