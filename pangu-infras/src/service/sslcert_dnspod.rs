use pangu_domain::service::sslcert::DnsProvider;

pub struct DnspodService {
    pub secret_id: String,
    pub secret_key: String,
}

impl DnsProvider for DnspodService {
    fn add_record(&self, domain: String, subdomain: String, value: String) {
        println!("a");
    }
}
impl DnspodService {
    pub fn new(id: &str, key: &str) -> Self {
        Self {
            secret_id: id.to_string(),
            secret_key: key.to_string(),
        }
    }
}
