//! Raw data types used by Serial Studio
//!
//! All objects in this file follow: https://github.com/Serial-Studio/Serial-Studio/wiki/Communication-Protocol

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A SerialStudio widget type
#[derive(Debug, Serialize, Deserialize)]
pub enum WidgetType {
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "gyro")]
    Gyro,
    #[serde(rename = "accelerometer")]
    Accelerometer,
}

/// A subtype of WidgetType
#[derive(Debug, Serialize, Deserialize)]
pub enum WidgetSubType {
    #[serde(rename = "x")]
    GyroX,
    #[serde(rename = "y")]
    GyroY,
    #[serde(rename = "z")]
    GyroZ,
    #[serde(rename = "x")]
    AccelX,
    #[serde(rename = "y")]
    AccelY,
    #[serde(rename = "z")]
    AccelZ,
    #[serde(rename = "lat")]
    Latitude,
    #[serde(rename = "lon")]
    Longitude,
    #[serde(rename = "min")]
    Minimum,
    #[serde(rename = "max")]
    Maximum,
}

/// A SerialStudio dataset object
#[derive(Debug, Serialize, Deserialize)]
pub struct DataSet {
    /// dataset title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "t")]
    pub title: Option<String>,

    /// dataset value
    #[serde(rename = "v")]
    pub value: Value,

    /// dataset unit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "u")]
    pub unit: Option<String>,

    /// dataset graph
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "g")]
    pub graph: Option<bool>,

    /// widget type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "w")]
    pub w_type: Option<WidgetSubType>,
}

/// A SerialStudio group object
#[derive(Debug, Serialize, Deserialize)]
pub struct DataGroup {
    /// group title
    #[serde(rename = "t")]
    pub title: String,

    /// Widget type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "w")]
    pub widget_type: Option<WidgetType>,

    /// group datasets
    #[serde(rename = "d")]
    pub datasets: Vec<DataSet>
}

/// A SerialStudio telemetry object
#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryFrame {
    /// project title
    #[serde(rename = "t")]
    pub title: String,

    /// groups
    #[serde(rename = "g")]
    pub groups: Vec<DataGroup>,
}
