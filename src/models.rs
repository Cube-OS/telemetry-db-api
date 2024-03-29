//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use super::telemetry;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "telemetry"]
pub struct Entry {
    pub timestamp: f64,
    pub subsystem: String,
    pub parameter: String,
    pub value: Vec<u8>,
}


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
///Result returned from get_telemetry
pub struct GetTelemetryStruct {
    ///
    pub timestamp_ge: Option<f64>,
    ///
    pub timestamp_le: Option<f64>,
    ///
    pub subsystem: Option<String>,
    ///
    pub parameters: Option<Vec<String>>,
    ///
    pub limit: Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
///
pub struct RoutedTelemetryStruct {
    ///
    pub timestamp_ge: Option<f64>,
    ///
    pub timestamp_le: Option<f64>,
    ///
    pub subsystem: Option<String>,
    ///
    pub parameters: Option<Vec<String>>,
    ///
    pub limit: Option<i32>,
    ///
    pub output: String,
    ///
    pub compress: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
///
pub struct DataPoint {
    ///
    pub timestamp: Option<f64>,
    ///
    pub subsystem: String,
    ///
    pub parameter: String,
    ///
    pub value: Vec<u8>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
///
pub struct InsertBulkStruct {
    ///
    pub timestamp: Option<f64>,
    ///
    pub entries: Vec<DataPoint>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
///
pub struct DeleteStruct {
    ///
    pub timestamp_ge: Option<f64>,
    ///
    pub timestamp_le: Option<f64>,
    ///
    pub subsystem: Option<String>,
    ///
    pub parameter: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
///
pub struct GetTelemetryResult {
    ///
    pub result: Vec<Entry>,
}