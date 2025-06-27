use serde_json::json;

const HOST: &str = "http://127.0.0.1:9090";

pub const PROXY_GROUP_GLOBAL: &str = "GLOBAL";

// {
//     "port": 0,
//     "socks-port": 0,
//     "redir-port": 0,
//     "tproxy-port": 0,
//     "mixed-port": 7890,
//     "authentication": [],
//     "allow-lan": true,
//     "bind-address": "*",
//     "mode": "global",
//     "log-level": "info",
//     "ipv6": false
// }
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    // pub port: u16,
    // pub socks_port: u16,
    // pub redir_port: u16,
    // pub tproxy_port: u16,
    // pub mixed_port: u16,
    // pub authentication: Vec<String>,
    // pub allow_lan: bool,
    // pub bind_address: String,
    pub mode: String,
    // pub log_level: String,
    // pub ipv6: bool,
}

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

pub async fn get_mode(client: &reqwest::Client) -> reqwest::Result<Config> {
    client
        .get(format!("{}/configs", HOST))
        .send()
        .await?
        .json::<Config>()
        .await
}

pub async fn switch_mode_to_global(client: &reqwest::Client) -> reqwest::Result<()> {
    client
        .patch(format!("{}/configs", HOST))
        .json(&json!({
            "mode": "Global"
        }))
        .header("Content-Type", "application/json")
        .send()
        .await?;
    Ok(())
}

pub async fn get_group_proxies(
    client: &reqwest::Client,
    group: &str,
) -> reqwest::Result<GroupProxies> {
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
    async fn get_and_switch_mode() {
        let client = reqwest::Client::builder().no_proxy().build().unwrap();

        // query current config first
        let mode = get_mode(&client).await.expect("query mode");
        println!("{:?}", mode);

        // switch mode to global
        switch_mode_to_global(&client)
            .await
            .expect("switch mode to global");

        // query config again and check
        let after_mode = get_mode(&client).await.expect("query mode after set");
        assert_eq!(after_mode.mode, mode.mode);
    }

    #[tokio::test]
    async fn get_and_set_group_proxy() {
        let client = reqwest::Client::builder().no_proxy().build().unwrap();

        // query current config first
        let proxies = get_group_proxies(&client, PROXY_GROUP_GLOBAL)
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

        let after_proxies = get_group_proxies(&client, PROXY_GROUP_GLOBAL)
            .await
            .expect("query group proxies after set");
        assert_eq!(after_proxies.now, *new_proxy);
    }
}
