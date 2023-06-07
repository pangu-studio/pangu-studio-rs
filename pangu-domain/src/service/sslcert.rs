use async_trait::async_trait;
#[async_trait]
pub trait DnsProviderService {
    async fn add_record(&self, domain: &str, subdomain: &str, value: &str,record_type: &str);
}