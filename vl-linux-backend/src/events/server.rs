use serde::{Deserialize, Serialize};

pub const METHOD_DEVICE_LIST_UPDATED: &str = "device_list_updated";

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDeviceListUpdated;

/// Generic Response back to server
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseServerEvent;
