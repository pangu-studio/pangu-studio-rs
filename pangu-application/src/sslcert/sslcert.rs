use async_trait::async_trait;
use pangu_domain::errors::Error;
pub struct SSLCertRequest {
    pub domain: String,
    pub email: String,
    pub dns_provider: String,
}
#[async_trait]
trait SSLCertApplicationService {
    async fn create_cert(&self, cert: SSLCertRequest) -> Result<(), Error>;
}
