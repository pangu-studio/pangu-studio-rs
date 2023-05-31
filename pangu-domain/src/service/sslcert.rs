pub trait DnsProvider {
    fn add_record(&self, domain: String, subdomain: String, value: String);
}
