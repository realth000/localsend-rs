use crate::strict_save::{StrictRecvTarget, StrictSendTarget};
use racros::{AutoDebug, AutoStr};
use serde::{Deserialize, Serialize};

/// Device OS types.
#[derive(AutoDebug, AutoStr, Deserialize, Serialize)]
#[autorule = "lowercase"]
pub enum DeviceType {
    /// Linux platform.
    Linux,

    /// MacOS platform.
    Macos,

    /// Windows platform.
    Windows,
}

/// Protocol in file transmission.
#[derive(AutoDebug, AutoStr, Deserialize, Serialize)]
#[autorule = "lowercase"]
pub enum Protocol {
    /// Use HTTP.
    Http,

    /// Use HTTPS.
    Https,
}

/// Definition of the app configuration.
///
/// Used to save in config directory.
#[derive(AutoDebug, Deserialize, Serialize)]
pub struct Config {
    /////////////////// Original Configuration ///////////////////
    /// Device name of the client.
    ///
    /// Rather be generated or manually set.
    device_name: String,

    /// Version of the localsend protocol.
    ///
    /// Format: $major.$minor.
    ///
    /// Current is 2.0.
    version: String,

    /// Device type description.
    ///
    /// Could be null.
    ///
    /// Default to be the hostname or OS type? Not decided yet.
    device_model: String,

    /// OS platform.
    ///
    /// Available values:
    ///
    /// * Linux.
    /// * MacOS.
    /// * Windows.
    device_type: DeviceType,

    /// Use to distinguish device identity.
    ///
    /// According to the official protocol design:
    ///
    /// * Use the SHA-256 hash of the certificate when HTTPS is enabled.
    /// * Use a random string when use bare HTTP.
    ///
    /// This field MUST be generated, never let user modify it.
    fingerprint: String,

    /// The port to use for device discovering and file transmission.
    ///
    /// Both UDP broadcast and TCP connection use the same port.
    port: usize,

    /// Protocol type used in file transmission.
    protocol: Protocol,

    /// Path to save the downloaded files.
    save_path: String,

    /// Flag indicating quick save mode enabled or not.
    ///
    /// * When enabled, save all files without asking the user to confirm.
    ///
    /// # Caution
    ///
    /// Enabling this will be less secure for the device. Must disable by default.
    ///
    /// Default is `false`.
    quick_save: bool,

    /////////////////// CLI specified configuration ///////////////////
    /// Flag indicating strict mode enabled or not.
    ///
    /// Strict mode is a mode that ONLY allows:
    ///
    /// * Send files to configured device [`StrictSendTarget`].
    /// * Receive files from configured device [`StrictRecvTarget`].
    ///
    /// Also, when strict mode is enabled, every file transmission is automatically down without
    /// let the user confirm, which is similar to [`quick_save`] mode.
    ///
    /// Default is `false`.
    strict_mode: bool,

    /// List of devices that allow to send files to when strict mode is enabled.
    ///
    /// Only allow to send files to these devices when [`strict_mode`] is enabled.
    strict_sender_list: Vec<StrictSendTarget>,

    /// List of devices that allow to send files to when strict mode is enabled.
    ///
    /// Only allow to receive files from these devices when [`strict_mode`] is enabled.
    strict_recver_list: Vec<StrictRecvTarget>,
}
