mod dns_sync;
mod route53;

use anyhow::Result;
use config::Config;
use std::net::IpAddr;
use tailscale_api::*;

use crate::dns_sync::DNSSync;
use crate::route53::Route53Sync;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("TAILSCALE_DNS"))
        .build()
        .unwrap();

    // Setup tailscale client
    let ts_api_key = settings.get_string("api_key")?;
    let ts_api_tailnet = settings.get_string("tailnet")?;
    let r53_domain = settings.get_string("route53_domain")?;
    let r53_hosted_zone_id = settings.get_string("route53_hosted_zone_id")?;
    let devices = Tailscale::new(ts_api_key, ts_api_tailnet)
        .list_devices()
        .await?;

    let device_records: Vec<DeviceRecord> =
        devices.iter().flat_map(flatten_device_addresses).collect();

    let r53sync = Route53Sync::new(r53_hosted_zone_id.clone(), r53_domain.clone());

    let report = r53sync.update(&device_records).await?;

    println!("{}", report.message);
    println!(
        "Synced {} DNS records for {} devices to domain {}",
        &device_records.len(),
        &devices.len(),
        r53_domain.clone()
    );

    Ok(())
}

#[derive(Debug)]
pub struct DeviceRecord {
    ip: IpAddr,
    name: String,
}

fn flatten_device_addresses(device: &Device) -> Vec<DeviceRecord> {
    device
        .addresses
        .iter()
        .map(|address| DeviceRecord {
            ip: address.parse().unwrap(),
            name: String::from(&device.hostname),
        })
        .collect()
}
