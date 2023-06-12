use pangu_application::sslcert::SSLCertRequest;
use pangu_domain::model::{DnsProvider, SSLCertificate};


// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn list_dns_providers(
    app_service: tauri::State<'_, crate::ApplicationService>,
) -> Result<Vec<DnsProvider>, String> {
    app_service
        .sslcert_app_svc
        .list_dns_providers()
        .await
        .map_err(|err| err.to_string())
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn list_sslcerts(
    app_svc: tauri::State<'_, crate::ApplicationService>,
) -> Result<Vec<SSLCertificate>, String> {
    app_svc
        .sslcert_app_svc
        .list_sslcerts()
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn apply_certificate(
    app_svc: tauri::State<'_, crate::ApplicationService>,
    cert: SSLCertRequest,
) -> Result<(), String> {
    info!("apply_certificate: {:?}", cert);

    app_svc
        .sslcert_app_svc
        .create_cert(cert)
        .await
        .map_err(|err| err.to_string())
}
#[tauri::command]
pub async fn get_sslcert_by_sn(
    app_svc: tauri::State<'_, crate::ApplicationService>,
    sn: String,
) -> Result<SSLCertificate, String> {
    app_svc
        .sslcert_app_svc
        .get_sslcert_by_sn(&sn)
        .await
        .map_err(|err| err.to_string())
}
