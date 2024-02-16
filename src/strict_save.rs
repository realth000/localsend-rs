use racros::AutoDebug;
use serde::{Deserialize, Serialize};

/// Define a strict sender target device.
///
/// When strict mode is enabled, only allow to send to the device described by this type.
#[derive(AutoDebug, Deserialize, Serialize)]
pub struct StrictSendTarget {}

/// Define a strict receiver target device.
///
/// When strict mode is enabled, only allow to receive from the device described by this type.
#[derive(AutoDebug, Deserialize, Serialize)]
pub struct StrictRecvTarget {}
