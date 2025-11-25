use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct RouteResponseRaw {
    #[serde(rename = "Trip")]
    pub trip: Vec<TripRaw>,
    #[serde(rename = "ResultStatus")]
    pub result_status: ResultStatusRaw,
    #[serde(rename = "TechnicalMessages")]
    pub technical_messages: TechnicalMessagesRaw,
    pub server_version: String,
    pub dialect_version: String,
    pub plan_rt_ts: String,
    pub request_id: String,
    pub scr_b: String,
    pub scr_f: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct TripRaw {
    #[serde(rename = "Origin")]
    pub origin: EndpointRaw,
    #[serde(rename = "Destination")]
    pub destination: EndpointRaw,
    #[serde(rename = "ServiceDays")]
    pub service_days: Vec<ServiceDayRaw>,
    #[serde(rename = "LegList")]
    pub leg_list: LegListRaw,
    pub calculation: String,
    #[serde(rename = "TripStatus")]
    pub trip_status: TripStatusRaw,
    #[serde(rename = "Notes")]
    pub notes: Option<NotesRaw>,
    pub idx: i64,
    pub trip_id: String,
    pub ctx_recon: String,
    pub duration: String,
    pub rt_duration: String,
    pub checksum: String,
    pub transfer_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct ServiceDayRaw {
    pub planning_period_begin: String,
    pub planning_period_end: String,
    pub s_days_r: String,
    pub s_days_i: String,
    pub s_days_b: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct LegListRaw {
    #[serde(rename = "Leg")]
    pub leg: Vec<LegRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct LegRaw {
    #[serde(rename = "Origin")]
    pub origin: EndpointRaw,
    #[serde(rename = "Destination")]
    pub destination: EndpointRaw,
    #[serde(rename = "Notes")]
    pub notes: Option<NotesRaw>,
    #[serde(rename = "JourneyDetailRef")]
    pub journey_detail_ref: Option<JourneyDetailRefRaw>,
    #[serde(rename = "JourneyStatus")]
    pub journey_status: Option<String>,
    #[serde(rename = "Product")]
    pub product: Vec<ProductRaw>,
    #[serde(rename = "Stops")]
    pub stops: Option<StopsRaw>,
    #[serde(rename = "JourneyDetail")]
    pub journey_detail: Option<JourneyDetailRaw>,
    pub id: String,
    pub idx: i64,
    pub name: String,
    pub number: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub reachable: Option<bool>,
    pub waiting_state: Option<String>,
    pub direction: Option<String>,
    pub direction_flag: Option<String>,
    pub duration: String,
    #[serde(rename = "GisRef")]
    pub gis_ref: Option<GisRefRaw>,
    #[serde(rename = "GisRoute")]
    pub gis_route: Option<GisRouteRaw>,
    pub dist: Option<i64>,
    pub minimum_change_duration: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct EndpointRaw {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub ext_id: String,
    pub lon: f64,
    pub lat: f64,
    pub route_idx: Option<i64>,
    pub prognosis_type: Option<String>,
    pub time: String,
    pub date: String,
    pub minimum_change_duration: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct NotesRaw {
    #[serde(rename = "Note")]
    pub note: Vec<NoteRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct NoteRaw {
    pub value: String,
    pub key: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub route_idx_from: Option<i64>,
    pub route_idx_to: Option<i64>,
    pub txt_n: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct JourneyDetailRefRaw {
    #[serde(rename = "ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct ProductRaw {
    pub icon: IconRaw,
    pub operator_info: Option<OperatorInfoRaw>,
    pub name: String,
    pub internal_name: String,
    pub display_number: Option<String>,
    pub num: Option<String>,
    pub line: Option<String>,
    pub line_id: Option<String>,
    pub cat_out: Option<String>,
    pub cat_in: Option<String>,
    pub cat_code: Option<String>,
    pub cls: Option<String>,
    pub cat_out_s: Option<String>,
    pub cat_out_l: Option<String>,
    pub operator_code: Option<String>,
    pub operator: Option<String>,
    pub admin: Option<String>,
    pub route_idx_from: Option<i64>,
    pub route_idx_to: Option<i64>,
    pub match_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct IconRaw {
    pub res: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct OperatorInfoRaw {
    pub name: String,
    pub name_s: String,
    pub name_n: String,
    pub name_l: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct StopsRaw {
    #[serde(rename = "Stop")]
    pub stop: Vec<StopRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct StopRaw {
    pub name: String,
    pub id: String,
    pub ext_id: String,
    pub route_idx: i64,
    pub lon: f64,
    pub lat: f64,
    pub dep_time: Option<String>,
    pub dep_date: Option<String>,
    pub dep_dir: Option<String>,
    pub minimum_change_duration: String,
    pub arr_time: Option<String>,
    pub arr_date: Option<String>,
    pub dep_prognosis_type: Option<String>,
    #[serde(rename = "Notes")]
    pub notes: Option<NotesRaw>,
    pub boarding: Option<bool>,
    pub arr_prognosis_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct JourneyDetailRaw {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub day_of_operation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct GisRefRaw {
    #[serde(rename = "ref")]
    pub ref_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct GisRouteRaw {
    pub dist: i64,
    pub dur_s: String,
    pub dir_geo: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct TripStatusRaw {
    pub hint_code: i64,
    pub detour: Option<bool>,
    pub economic: Option<bool>,
    pub convenient: Option<bool>,
    pub unsharp: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct ResultStatusRaw {
    pub time_diff_critical: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct TechnicalMessagesRaw {
    #[serde(rename = "TechnicalMessage")]
    pub technical_message: Vec<TechnicalMessageRaw>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct TechnicalMessageRaw {
    pub value: String,
    pub key: String,
}
