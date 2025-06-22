use serde_json::json;

const HOST: &str = "http://127.0.0.1:9090";

pub const PROXY_GROUP_GLOBAL: &str = "GLOBAL";

// {
//     "all": [
//         "DIRECT",
//         "REJECT",
//         "防失联网址: haita.link",
//         "售前/售后 请联系网站在线客服",
//         "🇭🇰 香港 01",
//         "🇭🇰 香港 02",
//         "🇭🇰 香港 04",
//         "🇬🇧 英国 01",
//         "🇬🇧 英国 02",
//         "Haita Cloud",
//         "自动选择",
//         "故障转移"
//     ],
//     "history": [],
//     "name": "GLOBAL",
//     "now": "🇯🇵 日本 01",
//     "type": "Selector",
//     "udp": true
// }
#[derive(Debug, serde::Deserialize)]
pub struct GroupProxies {
    pub all: Vec<String>,
    pub now: String,
}

pub async fn group_proxies(client: &reqwest::Client, group: &str) -> reqwest::Result<GroupProxies> {
    // let client = reqwest::Client::builder().no_proxy().build().unwrap();
    client
        .get(format!("{}/proxies/{}", HOST, group))
        .send()
        .await?
        .json::<GroupProxies>()
        .await
}

pub async fn set_group_proxy(
    client: &reqwest::Client,
    group: &str,
    proxy: &str,
) -> reqwest::Result<()> {
    // let client = reqwest::Client::builder().no_proxy().build().unwrap();
    client
        .put(format!("{}/proxies/{}", HOST, group))
        .json(&json!({
            "name": proxy
        }))
        .header("Content-Type", "application/json")
        .send()
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_ok() {
        let client = reqwest::Client::builder().no_proxy().build().unwrap();

        // query current config first
        let proxies = group_proxies(&client, PROXY_GROUP_GLOBAL)
            .await
            .expect("query group proxies");
        println!("{:?}", proxies);
        // and then set new proxy and check result
        let new_proxy = proxies
            .all
            .iter()
            .skip(6)
            .find(|p| (**p).ne(&proxies.now))
            .unwrap();
        set_group_proxy(&client, PROXY_GROUP_GLOBAL, new_proxy)
            .await
            .expect("set group proxy");

        let after_proxies = group_proxies(&client, PROXY_GROUP_GLOBAL)
            .await
            .expect("query group proxies after set");
        assert_eq!(after_proxies.now, *new_proxy);
    }
}
