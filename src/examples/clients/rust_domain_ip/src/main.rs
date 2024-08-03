use std::str::FromStr;

use icann_rdap_client::{
    create_client, rdap_bootstrapped_request, ClientConfig, MemoryBootstrapStore, QueryType,
    RdapClientError,
};
use icann_rdap_common::response::RdapResponse;

#[tokio::main]
async fn main() -> Result<(), RdapClientError> {
    // the command line args have the domains (except the first arg)
    let domains_names: Vec<String> = std::env::args().skip(1).collect();

    let config = ClientConfig::default();
    let client = create_client(&config)?;
    let store = MemoryBootstrapStore::new();

    for domain_name in domains_names {
        println!("domain: {domain_name}");
        let query = QueryType::from_str(&domain_name)?;
        let response = rdap_bootstrapped_request(&query, &client, &store, |_| {}).await?;
        let RdapResponse::Domain(domain) = response.rdap else {
            panic!("response is not a domain")
        };
        let name_servers = domain.nameservers.unwrap_or_default();
        for ns in name_servers {
            let ns_name = ns.ldh_name.clone().unwrap_or(
                ns.unicode_name
                    .clone()
                    .unwrap_or("No Nameserver Name Given".to_string()),
            );
            if let Some(ip_addresses) = ns.ip_addresses.as_ref() {
                let all_ips = vec![
                    ip_addresses.v6.clone().unwrap_or_default(),
                    ip_addresses.v4.clone().unwrap_or_default(),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>();
                for ip in all_ips {
                    let ip_query = QueryType::from_str(&ip).unwrap();
                    let RdapResponse::Network(ip_response) =
                        rdap_bootstrapped_request(&ip_query, &client, &store, |_| {})
                            .await
                            .unwrap()
                            .rdap
                    else {
                        panic!("response is not IP network")
                    };
                    let start_ip = ip_response
                        .start_address
                        .unwrap_or("NO START IP".to_string());
                    let end_ip = ip_response.end_address.unwrap_or("NO END IP".to_string());
                    println!("{ns_name}({ip}) is in network {start_ip} - {end_ip}");
                }
            }
        }
    }
    Ok(())
}
