use pangu_application::sslcert::{SSLCertApplicationService, SSLCertRequest};
use pangu_domain::model::DnsProvider;
use pangu_infras::service::sslcert::SSLCertApplicationServiceImpl;
use sqlx::{Pool, Sqlite};
// #[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
    db_pool: std::sync::Mutex<Pool<Sqlite>>,
}
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
