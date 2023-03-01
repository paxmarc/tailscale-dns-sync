use aws_config;
use aws_sdk_route53 as route53;

use anyhow::Result;
use config::Config;
use route53::model::{Change, ChangeBatch, ChangeInfo, ResourceRecord, ResourceRecordSet};
use std::net::IpAddr;
use tailscale_api::*;

#[derive(Debug)]
struct DeviceRecord {
    ip: IpAddr,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("TAILSCALE_DNS"))
        .build()
        .unwrap();

    // Setup AWS client
    let config = aws_config::load_from_env().await;
    let client = route53::Client::new(&config);

    // Setup tailscale client
    let ts_api_key = settings.get_string("api_key")?;
    let ts_api_tailnet = settings.get_string("tailnet")?;
    let r53_domain = settings.get_string("route53_domain")?;
    let tailscale = Tailscale::new(ts_api_key, ts_api_tailnet);

    let devices = get_tailscale_devices(&tailscale).await?;
    let device_records = devices.iter().flat_map(flatten_device_addresses).collect();

    let hosted_zone = get_hosted_zone_by_name(&client, &r53_domain).await?;

    update_dns_records(&client, &hosted_zone, &device_records).await?;

    println!(
        "Synced {} DNS records for {} devices to hosted zone with name {}",
        &device_records.len(),
        &devices.len(),
        &r53_domain
    );

    Ok(())
}

fn map_device_record_to_change(
    device: &DeviceRecord,
    hosted_zone: &route53::model::HostedZone,
) -> Change {
    let rr = ResourceRecord::builder()
        .set_value(Some(device.ip.to_string()))
        .build();

    let record_type = match device.ip {
        IpAddr::V4(_) => route53::model::RrType::A,
        IpAddr::V6(_) => route53::model::RrType::Aaaa,
    };

    let rrs = ResourceRecordSet::builder()
        .set_ttl(Some(300))
        .set_name(Some(format!(
            "{}.{}",
            device.name,
            hosted_zone.name().unwrap()
        )))
        .set_type(Some(record_type))
        .set_resource_records(Some(vec![rr]))
        .build();

    Change::builder()
        .set_action(Some(route53::model::ChangeAction::Upsert))
        .set_resource_record_set(Some(rrs))
        .build()
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

async fn get_tailscale_devices(tailscale: &Tailscale) -> Result<Vec<Device>> {
    let devices = tailscale.list_devices().await?;
    Ok(devices)
}

async fn update_dns_records(
    client: &route53::Client,
    hosted_zone: &route53::model::HostedZone,
    devices: &Vec<DeviceRecord>,
) -> Result<ChangeInfo> {
    let changes = devices
        .iter()
        .map(|device| map_device_record_to_change(device, hosted_zone))
        .collect::<Vec<Change>>();

    let change_batch = ChangeBatch::builder().set_changes(Some(changes)).build();

    let change_action = client
        .change_resource_record_sets()
        .set_change_batch(Some(change_batch))
        .set_hosted_zone_id(Some(String::from(hosted_zone.id().unwrap())))
        .send()
        .await?;

    Ok(change_action.change_info().unwrap().clone())
}

async fn get_hosted_zone_by_name(
    client: &route53::Client,
    domain: &String,
) -> Result<route53::model::HostedZone> {
    let zones = client
        .list_hosted_zones_by_name()
        .set_dns_name(Some(String::from(domain)))
        .send()
        .await?;

    let hosted_zones = zones.hosted_zones().unwrap_or_default();

    let zone = hosted_zones
        .clone()
        .iter()
        .nth(0)
        .expect("Couldn't retrieve hosted zone.");

    Ok(zone.clone())
}
