use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

use pangu_domain::model::DnsProvider;
use pangu_domain::service::sslcert::DnsProviderService;
pub struct DnspodServiceImpl {
    dns_provider: DnsProvider,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    code: String,
    message: String,
    created_at: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    id: Option<String>,
    name: Option<String>,
    status: Option<String>,
    weight: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    status: Status,
    record: Option<Record>,
}

#[async_trait]
impl DnsProviderService for DnspodServiceImpl {
    async fn add_record(&self, domain: &str, subdomain: &str, value: &str, record_type: &str) {
        let json: ResponseData = reqwest::Client::new()
            .post("https://dnsapi.cn/Record.Create")
            .form(&[
                (
                    "login_token",
                    (self.dns_provider.api_key.clone() + "," + &self.dns_provider.api_secret)
                        .as_str(),
                ),
                ("format", "json"),
                ("domain", domain),
                ("sub_domain", subdomain),
                ("value", value),
                ("record_type", record_type),
                ("record_line", "默认"),
            ])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        println!("Response status {:#?}", json);
        // response

        // println!("a");
    }
}
impl DnspodServiceImpl {
    pub fn new(dns_provider: DnsProvider) -> Self {
        Self { dns_provider }
    }
}
