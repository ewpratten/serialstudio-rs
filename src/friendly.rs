//! Custom types for creating SerialStudio data frames.
//! *Unfinished*

use crate::data::{DataGroup, DataSet, WidgetSubType, WidgetType};
use serde_json::json;

trait Widget {
    fn get_group(&self) -> DataGroup;
}

/// A Map widget
pub struct MapWidget {
    latitude: f32,
    longitude: f32,
    name: String,
    graph: bool,
}

impl MapWidget {
    pub fn new(name: String, latitude: f32, longitude: f32, graph: bool) -> Self {
        Self {
            latitude,
            longitude,
            name,
            graph,
        }
    }

    pub fn update(&mut self, latitude: f32, longitude: f32) {
        self.latitude = latitude;
        self.longitude = longitude;
    }
}

impl Widget for MapWidget {
    fn get_group(&self) -> DataGroup {
        DataGroup {
            title: self.name.clone(),
            widget_type: Some(WidgetType::Map),
            datasets: vec![
                DataSet {
                    title: None,
                    value: json!(self.latitude),
                    graph: Some(self.graph),
                    unit: None,
                    w_type: Some(WidgetSubType::Latitude),
                },
                DataSet {
                    title: None,
                    value: json!(self.longitude),
                    graph: Some(self.graph),
                    unit: None,
                    w_type: Some(WidgetSubType::Longitude),
                },
            ],
        }
    }
}


/// A Dashboard
pub struct Dashboard {
    name: String,
    widgets: Vec<Box<dyn Widget>>,
}
