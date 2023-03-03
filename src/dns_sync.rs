use crate::DeviceRecord;
use anyhow::Result;
use async_trait::async_trait;

pub struct SyncReport {
    pub message: String,
}

#[async_trait]
pub trait DNSSync {
    async fn update(&self, records: &Vec<DeviceRecord>) -> Result<SyncReport>;
}
