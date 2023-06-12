use crate::tests::MyAsyncContext;

use pangu_application::sslcert::SSLCertRequest;
use pangu_domain::model::{DnsProvider, DnsProviderType, SSLCertificate};
use pangu_domain::repository::{DnsProviderRepository, Repository};
use test_context::test_context;

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn create_dns_provider(ctx: &mut MyAsyncContext) {
    let dns_provider = DnsProvider::new("dnspod", "123", "api_secret", DnsProviderType::Dnspod);
    let id = ctx
        .repositories
        .dns_provider_repo
        .create(dns_provider)
        .await
        .unwrap();
    assert_eq!(id > 0, true)
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn list_endpoints_by_name(ctx: &mut MyAsyncContext) {
    let res = ctx
        .repositories
        .dns_provider_repo
        .find_by_name("dns".to_string())
        .await;
    assert!(res.is_ok());
}
#[test_context(MyAsyncContext)]
#[tokio::test]
async fn dnspod_post(ctx: &mut MyAsyncContext) {
    let _r = ctx
        .services
        .dns_provider_svc
        .add_record(1, "ab.com", "test", "1.1.1.1", "A")
        .await
        .unwrap();
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn remove_record(ctx: &mut MyAsyncContext) {
    let _r = ctx
        .services
        .dns_provider_svc
        .remove_record(1, "abcd.co", "1495225757")
        .await
        .unwrap();
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn create_cert_db(ctx: &mut MyAsyncContext) {
    let model = SSLCertificate::new(
        "abcdd",
        "a@b.com",
        "domains.com",
        r#"-----BEGIN CERTIFICATE-----
MIIEXzCCA0egAwIBAgISBJMB/wzkZK8F+ABTOcpcdXxQMA0GCSqGSIb3DQEBCwUA
MDIxCzAJBgNVBAYTAlVTMRYwFAYDVQQKEw1MZXQncyBFbmNyeXB0MQswCQYDVQQD
EwJSMzAeFw0yMjEwMDMwNTMyMDFaFw0yMzAxMDEwNTMyMDBaMB4xHDAaBgNVBAMT
E2Zsb3dtZW0ucGFuZ3UuY2xvdWQwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATF
ch/1jDLkLSMjzGN9CLtRUgiTbNxfgeT1sLjsql5lWeQzDGpqx8igt9iaMxrXuwma
znaHiGh5BVES54vvNjg4o4ICTDCCAkgwDgYDVR0PAQH/BAQDAgeAMB0GA1UdJQQW
MBQGCCsGAQUFBwMBBggrBgEFBQcDAjAMBgNVHRMBAf8EAjAAMB0GA1UdDgQWBBT/
ob5FrdKsnre0yaw3xH93HoJA/jAfBgNVHSMEGDAWgBQULrMXt1hWy65QCUDmH6+d
ixTCxjBVBggrBgEFBQcBAQRJMEcwIQYIKwYBBQUHMAGGFWh0dHA6Ly9yMy5vLmxl
bmNyLm9yZzAiBggrBgEFBQcwAoYWaHR0cDovL3IzLmkubGVuY3Iub3JnLzAeBgNV
HREEFzAVghNmbG93bWVtLnBhbmd1LmNsb3VkMEwGA1UdIARFMEMwCAYGZ4EMAQIB
MDcGCysGAQQBgt8TAQEBMCgwJgYIKwYBBQUHAgEWGmh0dHA6Ly9jcHMubGV0c2Vu
Y3J5cHQub3JnMIIBAgYKKwYBBAHWeQIEAgSB8wSB8ADuAHUArfe++nz/EMiLnT2c
Hj4YarRnKV3PsQwkyoWGNOvcgooAAAGDnItbtwAABAMARjBEAiBmLPNBb5I/Fh7y
yeeOHM6Aeb1GWkftmb9tJ2VmL7SwkAIgUxtTT409q/f+xokPh+c/1Vl05KXdLHIB
1NJNWXK2rTwAdQB6MoxU2LcttiDqOOBSHumEFnAyE4VNO9IrwTpXo1LrUgAAAYOc
i12fAAAEAwBGMEQCICVD+ykCz3gBDsVJZ8X8YUZ7CvUl/pnhJrroFYMApf4BAiAd
aH+XkxeR4H6FA73eSh7MnBn9xqUz4RxbkNZDcuc4NjANBgkqhkiG9w0BAQsFAAOC
AQEAX6xmdPHxf04xxAhTQ5htcoO7yDhbz1GLiUSVnJbe35x4kXmXe4BpsXv85eFl
l/zEksSVXLw1zNe9KUV4RCvTiR+gReVP6MOP5DSKZLrg+dy24VEXwm5GXsjAsgGY
iY0h/c+UIOcmBPD+XVIoQ9gtB2Wn8m7+RgJ18+wS+H8PmtytkBVOhKxUz4N7/eDE
LY3VLnPa+vQejA2xK5IyZyizIlI2mp7SzEd2acnpf4LCccu97fhFeKcyOuvoyKk7
Pj3/SZywzLNJ0+e7SwbJ2yrL57AsLNh0bg4/mMMDmaNC8rj/wgn75xHbbuSA9aqd
s/Y1zBwIxuaJn4H4zxSnVo8RFw==
-----END CERTIFICATE-----

-----BEGIN CERTIFICATE-----
MIIFFjCCAv6gAwIBAgIRAJErCErPDBinU/bWLiWnX1owDQYJKoZIhvcNAQELBQAw
TzELMAkGA1UEBhMCVVMxKTAnBgNVBAoTIEludGVybmV0IFNlY3VyaXR5IFJlc2Vh
cmNoIEdyb3VwMRUwEwYDVQQDEwxJU1JHIFJvb3QgWDEwHhcNMjAwOTA0MDAwMDAw
WhcNMjUwOTE1MTYwMDAwWjAyMQswCQYDVQQGEwJVUzEWMBQGA1UEChMNTGV0J3Mg
RW5jcnlwdDELMAkGA1UEAxMCUjMwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEK
AoIBAQC7AhUozPaglNMPEuyNVZLD+ILxmaZ6QoinXSaqtSu5xUyxr45r+XXIo9cP
R5QUVTVXjJ6oojkZ9YI8QqlObvU7wy7bjcCwXPNZOOftz2nwWgsbvsCUJCWH+jdx
sxPnHKzhm+/b5DtFUkWWqcFTzjTIUu61ru2P3mBw4qVUq7ZtDpelQDRrK9O8Zutm
NHz6a4uPVymZ+DAXXbpyb/uBxa3Shlg9F8fnCbvxK/eG3MHacV3URuPMrSXBiLxg
Z3Vms/EY96Jc5lP/Ooi2R6X/ExjqmAl3P51T+c8B5fWmcBcUr2Ok/5mzk53cU6cG
/kiFHaFpriV1uxPMUgP17VGhi9sVAgMBAAGjggEIMIIBBDAOBgNVHQ8BAf8EBAMC
AYYwHQYDVR0lBBYwFAYIKwYBBQUHAwIGCCsGAQUFBwMBMBIGA1UdEwEB/wQIMAYB
Af8CAQAwHQYDVR0OBBYEFBQusxe3WFbLrlAJQOYfr52LFMLGMB8GA1UdIwQYMBaA
FHm0WeZ7tuXkAXOACIjIGlj26ZtuMDIGCCsGAQUFBwEBBCYwJDAiBggrBgEFBQcw
AoYWaHR0cDovL3gxLmkubGVuY3Iub3JnLzAnBgNVHR8EIDAeMBygGqAYhhZodHRw
Oi8veDEuYy5sZW5jci5vcmcvMCIGA1UdIAQbMBkwCAYGZ4EMAQIBMA0GCysGAQQB
gt8TAQEBMA0GCSqGSIb3DQEBCwUAA4ICAQCFyk5HPqP3hUSFvNVneLKYY611TR6W
PTNlclQtgaDqw+34IL9fzLdwALduO/ZelN7kIJ+m74uyA+eitRY8kc607TkC53wl
ikfmZW4/RvTZ8M6UK+5UzhK8jCdLuMGYL6KvzXGRSgi3yLgjewQtCPkIVz6D2QQz
CkcheAmCJ8MqyJu5zlzyZMjAvnnAT45tRAxekrsu94sQ4egdRCnbWSDtY7kh+BIm
lJNXoB1lBMEKIq4QDUOXoRgffuDghje1WrG9ML+Hbisq/yFOGwXD9RiX8F6sw6W4
avAuvDszue5L3sz85K+EC4Y/wFVDNvZo4TYXao6Z0f+lQKc0t8DQYzk1OXVu8rp2
yJMC6alLbBfODALZvYH7n7do1AZls4I9d1P4jnkDrQoxB3UqQ9hVl3LEKQ73xF1O
yK5GhDDX8oVfGKF5u+decIsH4YaTw7mP3GFxJSqv3+0lUFJoi5Lc5da149p90Ids
hCExroL1+7mryIkXPeFM5TgO9r0rvZaBFOvV2z0gp35Z0+L4WPlbuEjN/lxPFin+
HlUjr8gRsI3qfJOQFy/9rKIJR0Y/8Omwt/8oTWgy1mdeHmmjk7j1nYsvC9JSQ6Zv
MldlTTKB3zhThV1+XWYp6rjd5JW1zbVWEkLNxE7GJThEUG3szgBVGP7pSWTUTsqX
nLRbwHOoq7hHwg==
-----END CERTIFICATE-----

-----BEGIN CERTIFICATE-----
MIIFYDCCBEigAwIBAgIQQAF3ITfU6UK47naqPGQKtzANBgkqhkiG9w0BAQsFADA/
MSQwIgYDVQQKExtEaWdpdGFsIFNpZ25hdHVyZSBUcnVzdCBDby4xFzAVBgNVBAMT
DkRTVCBSb290IENBIFgzMB4XDTIxMDEyMDE5MTQwM1oXDTI0MDkzMDE4MTQwM1ow
TzELMAkGA1UEBhMCVVMxKTAnBgNVBAoTIEludGVybmV0IFNlY3VyaXR5IFJlc2Vh
cmNoIEdyb3VwMRUwEwYDVQQDEwxJU1JHIFJvb3QgWDEwggIiMA0GCSqGSIb3DQEB
AQUAA4ICDwAwggIKAoICAQCt6CRz9BQ385ueK1coHIe+3LffOJCMbjzmV6B493XC
ov71am72AE8o295ohmxEk7axY/0UEmu/H9LqMZshftEzPLpI9d1537O4/xLxIZpL
wYqGcWlKZmZsj348cL+tKSIG8+TA5oCu4kuPt5l+lAOf00eXfJlII1PoOK5PCm+D
LtFJV4yAdLbaL9A4jXsDcCEbdfIwPPqPrt3aY6vrFk/CjhFLfs8L6P+1dy70sntK
4EwSJQxwjQMpoOFTJOwT2e4ZvxCzSow/iaNhUd6shweU9GNx7C7ib1uYgeGJXDR5
bHbvO5BieebbpJovJsXQEOEO3tkQjhb7t/eo98flAgeYjzYIlefiN5YNNnWe+w5y
sR2bvAP5SQXYgd0FtCrWQemsAXaVCg/Y39W9Eh81LygXbNKYwagJZHduRze6zqxZ
Xmidf3LWicUGQSk+WT7dJvUkyRGnWqNMQB9GoZm1pzpRboY7nn1ypxIFeFntPlF4
FQsDj43QLwWyPntKHEtzBRL8xurgUBN8Q5N0s8p0544fAQjQMNRbcTa0B7rBMDBc
SLeCO5imfWCKoqMpgsy6vYMEG6KDA0Gh1gXxG8K28Kh8hjtGqEgqiNx2mna/H2ql
PRmP6zjzZN7IKw0KKP/32+IVQtQi0Cdd4Xn+GOdwiK1O5tmLOsbdJ1Fu/7xk9TND
TwIDAQABo4IBRjCCAUIwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYw
SwYIKwYBBQUHAQEEPzA9MDsGCCsGAQUFBzAChi9odHRwOi8vYXBwcy5pZGVudHJ1
c3QuY29tL3Jvb3RzL2RzdHJvb3RjYXgzLnA3YzAfBgNVHSMEGDAWgBTEp7Gkeyxx
+tvhS5B1/8QVYIWJEDBUBgNVHSAETTBLMAgGBmeBDAECATA/BgsrBgEEAYLfEwEB
ATAwMC4GCCsGAQUFBwIBFiJodHRwOi8vY3BzLnJvb3QteDEubGV0c2VuY3J5cHQu
b3JnMDwGA1UdHwQ1MDMwMaAvoC2GK2h0dHA6Ly9jcmwuaWRlbnRydXN0LmNvbS9E
U1RST09UQ0FYM0NSTC5jcmwwHQYDVR0OBBYEFHm0WeZ7tuXkAXOACIjIGlj26Ztu
MA0GCSqGSIb3DQEBCwUAA4IBAQAKcwBslm7/DlLQrt2M51oGrS+o44+/yQoDFVDC
5WxCu2+b9LRPwkSICHXM6webFGJueN7sJ7o5XPWioW5WlHAQU7G75K/QosMrAdSW
9MUgNTP52GE24HGNtLi1qoJFlcDyqSMo59ahy2cI2qBDLKobkx/J3vWraV0T9VuG
WCLKTVXkcGdtwlfFRjlBz4pYg1htmf5X6DYO8A4jqv2Il9DjXA6USbW1FzXSLr9O
he8Y4IWS6wY7bCkjCWDcRQJMEhg76fsO3txE+FiYruq9RUWhiF1myv4Q6W+CyBFC
Dfvp7OOGAN6dEOM4+qR9sdjoSYKEBpsr6GtPAQw4dy753ec5
-----END CERTIFICATE-----"#,
        r#"-----BEGIN EC PRIVATE KEY-----
MHcCAQEEINAmGlrDQx9JyKuTaWxnA6ev3Q3KAyWGIT+WayW3s+zaoAoGCCqGSM49
AwEHoUQDQgAExXIf9Ywy5C0jI8xjfQi7UVIIk2zcX4Hk9bC47KpeZVnkMwxqasfI
oLfYmjMa17sJms52h4hoeQVREueL7zY4OA==
-----END EC PRIVATE KEY-----"#,
        "{}",
    );
    let res = ctx.repositories.ssl_cert_repo.create(model).await;
    assert_eq!(res.is_ok(), true);
}

#[test_context(MyAsyncContext)]
#[tokio::test]
async fn get_cert(ctx: &mut MyAsyncContext) -> anyhow::Result<()> {
    // let ssl_app_svc = SSLCertApplicationServiceImpl::new(dnspod_svc);

    let sslcert_req = SSLCertRequest {
        sn: "abcd".to_string(),
        provider_id: 1,
        domain: "test.studio".to_string(),
        subdomain: "www".to_string(),
        mail: "a.b@gmail.com".to_string(),
    };

    ctx.app_services
        .sslcert_app_svc
        .create_cert(sslcert_req)
        .await?;
    Ok(())
}
