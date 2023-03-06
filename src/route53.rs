use crate::dns_sync::{DNSSync, SyncReport};
use crate::DeviceRecord;

use anyhow::Result;
use async_trait::async_trait;
use aws_sdk_route53::model::{Change, ChangeBatch, ResourceRecord, ResourceRecordSet};
use std::net::IpAddr;

pub struct Route53Sync {
    hosted_zone_id: String,
    domain: String,
}

impl Route53Sync {
    pub fn new(hosted_zone_id: String, domain: String) -> Self {
        Self {
            hosted_zone_id,
            domain,
        }
    }

    fn map_device_record_to_change(&self, device: &DeviceRecord) -> Change {
        let rr = ResourceRecord::builder()
            .set_value(Some(device.ip.to_string()))
            .build();

        let record_type = match device.ip {
            IpAddr::V4(_) => aws_sdk_route53::model::RrType::A,
            IpAddr::V6(_) => aws_sdk_route53::model::RrType::Aaaa,
        };

        let rrs = ResourceRecordSet::builder()
            .set_ttl(Some(300))
            .set_name(Some(format!("{}.{}", device.name, self.domain)))
            .set_type(Some(record_type))
            .set_resource_records(Some(vec![rr]))
            .build();

        Change::builder()
            .set_action(Some(aws_sdk_route53::model::ChangeAction::Upsert))
            .set_resource_record_set(Some(rrs))
            .build()
    }
}

#[async_trait]
impl DNSSync for Route53Sync {
    async fn update(&self, records: &Vec<DeviceRecord>) -> Result<SyncReport> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_route53::Client::new(&config);

        let changes = records
            .iter()
            .map(|device| self.map_device_record_to_change(device))
            .collect();

        let change_batch = ChangeBatch::builder().set_changes(Some(changes)).build();

        let change_action = client
            .change_resource_record_sets()
            .set_change_batch(Some(change_batch))
            .set_hosted_zone_id(Some(self.hosted_zone_id.clone()))
            .send()
            .await?;

        Ok(SyncReport {
            message: format!(
                "Hosted zone updated with change ID {}",
                change_action
                    .change_info()
                    .unwrap()
                    .id()
                    .unwrap_or_default()
            ),
        })
    }
}
