use async_trait::async_trait;
use simplelog::info;

use pangu_application::sslcert::{SSLCertApplicationService, SSLCertRequest,DnsProviderCreateReq};
use pangu_domain::errors::Error;
use pangu_domain::model::{DnsProvider, SSLCertificate, SSLCertificateAddition};
use pangu_domain::repository::{DnsProviderRepository, Repository, SSLCertificateRepository};
use pangu_domain::service::sslcert::{DnsProviderService, ResponseData};
use rcgen::{Certificate, CertificateParams, DistinguishedName};

use std::time::Duration;
use tokio::time::sleep;

use instant_acme::{
    Account, AuthorizationStatus, ChallengeType, Identifier, LetsEncrypt, NewAccount, NewOrder,
    OrderStatus,
};

pub struct DnspodServiceImpl {
    dns_provider_repo: Box<dyn DnsProviderRepository + Send + Sync + 'static>,
}

#[async_trait]
impl DnsProviderService for DnspodServiceImpl {
    async fn add_record(
        &self,
        provider_id: i64,
        domain: &str,
        subdomain: &str,
        value: &str,
        record_type: &str,
    ) -> Result<ResponseData, Error> {
        let provider = self.dns_provider_repo.find(provider_id).await?;
        let json: ResponseData = reqwest::Client::new()
            .post("https://dnsapi.cn/Record.Create")
            .form(&[
                (
                    "login_token",
                    (provider.api_key.clone() + "," + provider.api_secret.as_str()).as_str(),
                ),
                ("format", "json"),
                ("domain", domain),
                ("sub_domain", subdomain),
                ("value", value),
                ("record_type", record_type),
                ("ttl", "600"),
                ("record_line", "默认"),
            ])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .or_else(|err| Err(Error::Dnspod(err.to_string())))?;

        println!("Response status {:#?}", json);
        Ok(json)
        // response

        // println!("a");
    }

    async fn remove_record(
        &self,
        provider_id: i64,
        domain: &str,
        record_id: &str,
    ) -> Result<(), Error> {
        let provider = self.dns_provider_repo.find(provider_id).await?;
        let json: ResponseData = reqwest::Client::new()
            .post("https://dnsapi.cn/Record.Remove")
            .form(&[
                (
                    "login_token",
                    (provider.api_key.clone() + "," + provider.api_secret.as_str()).as_str(),
                ),
                ("domain", domain),
                ("record_id", record_id),
            ])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .or_else(|err| Err(Error::Dnspod(err.to_string())))?;

        println!("Response status {:#?}", json);
        Ok(())
    }
}
impl DnspodServiceImpl {
    pub fn new(dns_provider_repo: Box<dyn DnsProviderRepository + Send + Sync>) -> Self {
        Self { dns_provider_repo }
    }
}

//////////=========================SSLCERT GET===========================

// SSLCertApplicationServiceImpl
pub struct SSLCertApplicationServiceImpl {
    dns_provider_svc: Box<dyn DnsProviderService + Send + Sync>,
    dns_provider_repo: Box<dyn DnsProviderRepository + Send + Sync>,
    sslcert_repo: Box<dyn SSLCertificateRepository + Send + Sync>,
}

impl SSLCertApplicationServiceImpl {
    pub fn new(
        dns_provider_svc: Box<dyn DnsProviderService + Send + Sync>,
        dns_provider_repo: Box<dyn DnsProviderRepository + Send + Sync>,
        sslcert_repo: Box<dyn SSLCertificateRepository + Send + Sync>,
        // dns_provider_repo: DnsProviderRepositoryImpl,
    ) -> Self {
        Self {
            dns_provider_svc,
            dns_provider_repo,
            sslcert_repo,
        }
    }
}

#[async_trait]
impl SSLCertApplicationService for SSLCertApplicationServiceImpl {
    async fn create_cert(&self, cert: SSLCertRequest) -> Result<(), Error> {
        // create account

        let account = Account::create(
            &NewAccount {
                contact: &[format!("mailto:{}", cert.mail).as_str()],
                terms_of_service_agreed: true,
                only_return_existing: false,
            },
            LetsEncrypt::Production.url(),
            // LetsEncrypt::Staging.url(),
            None,
        )
        .await
        .or_else(|err| Err(Error::Acme(err.to_string())))?;

        let cert_domain = format!("{}.{}", cert.subdomain, cert.domain);
        let identifier = Identifier::Dns(cert_domain.clone());
        let mut ids = vec![identifier.clone()];
        if cert.subdomain == "www" {
            ids.push(Identifier::Dns(cert.domain.clone()));
        }

        let mut order = account
            .new_order(&NewOrder { identifiers: &ids })
            .await
            .unwrap();

        let state = order.state();
        info!("order state: {:#?}", state);
        assert!(matches!(state.status, OrderStatus::Pending));

        // Pick the desired challenge type and prepare the response.
        let authorizations = order.authorizations().await.unwrap();
        let mut challenges = Vec::with_capacity(authorizations.len());
        let mut res: ResponseData = Default::default();

        let mut addition: SSLCertificateAddition = Default::default();
        for authz in &authorizations {
            match authz.status {
                AuthorizationStatus::Pending => {}
                AuthorizationStatus::Valid => continue,
                _ => todo!(),
            }

            let challenge = authz
                .challenges
                .iter()
                .find(|c| c.r#type == ChallengeType::Dns01)
                .ok_or_else(|| Error::Acme("no dns01 challenge found".to_string()))?;

            let Identifier::Dns(identifier) = &authz.identifier;

            println!("Please set the following DNS record then press any key:");
            let val = order.key_authorization(challenge).dns_value();
            println!("_acme-challenge.{} IN TXT {}", identifier, val);

            let mut sub_domain = format!("_acme-challenge.{}", cert.subdomain);

            if identifier == &cert.domain && cert.subdomain == "www" {
                sub_domain = "_acme-challenge".to_string();
            }

            addition.identifier = format!("{}.{}", sub_domain, &cert.domain);
            addition.record_type = "TXT".to_string();
            addition.record_value = val.clone();

            res = self
                .dns_provider_svc
                .add_record(
                    cert.provider_id,
                    &cert.domain,
                    &sub_domain,
                    val.as_str(),
                    "TXT",
                )
                .await?;

            challenges.push((identifier, &challenge.url));
        }

        let add_json = serde_json::to_string(&addition).unwrap();
        // Exponentially back off until the order becomes ready or invalid.
        let mut tries = 1u8;
        let model = SSLCertificate::new(
            &cert.sn,
            &cert.mail,
            (format!("{}.{}", &cert.subdomain, &cert.domain)).as_str(),
            "",
            "",
            &add_json,
        );
        let cert_id = self.sslcert_repo.create(model.clone()).await?;

        // Let the server know we're ready to accept the challenges.
        sleep(Duration::from_secs(5)).await;
        for (_, url) in &challenges {
            order.set_challenge_ready(url).await.unwrap();
        }
        let mut delay = Duration::from_secs(20);
        loop {
            debug!(
                "waiting for order to become ready. {}s",
                delay.as_secs_f32()
            );
            sleep(delay).await;
            let state = order.refresh().await.unwrap();
            if let OrderStatus::Ready | OrderStatus::Invalid = state.status {
                // if let OrderStatus::Ready = state.status {
                info!("order state: {:#?}", state);
                break;
            }

            delay *= 2;
            tries += 1;
            match tries < 5 {
                true => info!(
                    "order is not ready, waiting {:?}, {:?}, {:?}",
                    state,
                    tries,
                    delay.as_secs_f32()
                ),
                // true => info!(?state, tries, "order is not ready, waiting {delay:?}"),
                false => {
                    // error!(?state, tries, "order is not ready");
                    error!("order is not ready");
                    self.dns_provider_svc
                        .remove_record(
                            cert.provider_id,
                            &cert.domain,
                            res.record.unwrap().id.unwrap().as_str(),
                        )
                        .await?;
                    return Err(Error::Acme("order is not ready".to_string()));
                }
            }
        }

        let state = order.state();
        //remove dns record
        self.dns_provider_svc
            .remove_record(
                cert.provider_id,
                &cert.domain,
                res.clone().record.unwrap().id.unwrap().as_str(),
            )
            .await?;
        if state.status != OrderStatus::Ready {
            return Err(Error::Acme(format!(
                "order is not ready {:?}",
                state.status
            )));
        }

        let mut names = Vec::with_capacity(challenges.len());
        for (identifier, _) in challenges {
            names.push(identifier.to_owned());
        }

        // If the order is ready, we can provision the certificate.
        // Use the rcgen library to create a Certificate Signing Request.

        let mut params = CertificateParams::new(names.clone());
        params.distinguished_name = DistinguishedName::new();
        let ssl_cert = Certificate::from_params(params).unwrap();
        let csr = ssl_cert
            .serialize_request_der()
            .or_else(|err| Err(Error::Acme(err.to_string())))?;

        // Finalize the order and print certificate chain, private key and account credentials.

        order.finalize(&csr).await.unwrap();
        let cert_chain_pem = loop {
            match order.certificate().await.unwrap() {
                Some(cert_chain_pem) => break cert_chain_pem,
                None => sleep(Duration::from_secs(1)).await,
            }
        };

        // updater certificate status to db
        let mut cert_db = self.sslcert_repo.find(cert_id).await?;

        cert_db.issue(&cert_chain_pem, &ssl_cert.serialize_private_key_pem());
        self.sslcert_repo.update(cert_db).await?;

        info!("certficate chain:\n\n{}", cert_chain_pem);
        info!("private key:\n\n{}", ssl_cert.serialize_private_key_pem());
        info!(
            "account credentials:\n\n{}",
            serde_json::to_string_pretty(&account.credentials()).unwrap()
        );

        Ok(())
    }

    async fn list_dns_providers(&self) -> Result<Vec<DnsProvider>, Error> {
        let providers = self
            .dns_provider_repo
            .list_dns_providers()
            .await
            .or_else(|err| Err(Error::Acme(err.to_string())))?;

        Ok(providers)
    }
    async fn list_sslcerts(&self) -> Result<Vec<SSLCertificate>, Error> {
        let certs = self
            .sslcert_repo
            .find_all()
            .await
            .or_else(|err| Err(Error::Acme(err.to_string())))?;

        Ok(certs)
    }
    async fn get_sslcert_by_sn(&self, sn: &str) -> Result<SSLCertificate, Error> {
        self.sslcert_repo.find_by_sn(sn).await
    }

    async fn remove_sslcert(&self, id: i64) -> Result<(), Error> {
        self.sslcert_repo.remove(id).await
    }

    async fn add_dns_provider(&self, req: DnsProviderCreateReq) -> Result<i64, Error> {
        let provider = DnsProvider::new(&req.name, &req.api_key, &req.api_secret, req.provider_type);
        self.dns_provider_repo.create(provider).await
    }
}
