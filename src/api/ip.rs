// {
//     "ip": "206.237.114.100",
//     "city": "San Jose",
//     "region": "California",
//     "country": "US",
//     "loc": "37.2329,-121.7875",
//     "org": "AS138997 Eons Data Communications Limited",
//     "postal": "95119",
//     "timezone": "America/Los_Angeles",
//     "readme": "https://ipinfo.io/missingauth"
// }
#[derive(Debug, serde::Deserialize)]
pub struct IpInfo {
    pub ip: String,
    pub country: String,
}

pub async fn ip_ifo(client: &reqwest::Client) -> reqwest::Result<IpInfo> {
    client
        .get("https://ipinfo.io/json")
        .send()
        .await?
        .json::<IpInfo>()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_ok() {
        let ipinfo_api_client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("socks5://127.0.0.1:7890").unwrap())
            .build()
            .unwrap();

        let ip_info = ip_ifo(&ipinfo_api_client).await.unwrap();
        println!("IP: {}, Country: {}", ip_info.ip, ip_info.country);
    }
}
