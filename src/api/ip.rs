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

pub async fn ip_ifo() -> reqwest::Result<IpInfo> {
    reqwest::get("https://ipinfo.io/json")
        .await?
        .json::<IpInfo>()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_ok() {
        let ip_info = ip_ifo().await.unwrap();
        println!("IP: {}, Country: {}", ip_info.ip, ip_info.country);
    }
}
