use crate::resrobot::{
    Coordinate, EndpointRaw, LegRaw, NoteRaw, ProductRaw, RouteResponseRaw, ServiceDayRaw, StopRaw,
    TripRaw,
};

#[derive(Debug)]
pub struct RouteResponse {
    pub trips: Vec<Trip>,
    pub scr_b: String,
    pub scr_f: String,
}

impl From<RouteResponseRaw> for RouteResponse {
    fn from(value: RouteResponseRaw) -> Self {
        Self {
            trips: value.trip.into_iter().map(|trip| trip.into()).collect(),
            scr_b: value.scr_b,
            scr_f: value.scr_f,
        }
    }
}

#[derive(Debug)]
pub struct Trip {
    pub origin: Endpoint,
    pub destination: Endpoint,
    pub legs: Vec<Leg>,
    pub service_days: Vec<ServiceDay>,
}

impl From<TripRaw> for Trip {
    fn from(value: TripRaw) -> Self {
        Self {
            origin: value.origin.into(),
            destination: value.destination.into(),
            legs: value
                .leg_list
                .leg
                .into_iter()
                .map(|leg| leg.into())
                .collect(),
            service_days: value
                .service_days
                .into_iter()
                .map(|service_day| service_day.into())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct ServiceDay {
    pub planning_period_begin: String,
    pub planning_period_end: String,
    pub s_days_r: String,
    pub s_days_i: String,
    pub s_days_b: String,
}

impl From<ServiceDayRaw> for ServiceDay {
    fn from(value: ServiceDayRaw) -> Self {
        Self {
            planning_period_begin: value.planning_period_begin,
            planning_period_end: value.planning_period_end,
            s_days_r: value.s_days_r,
            s_days_i: value.s_days_i,
            s_days_b: value.s_days_b,
        }
    }
}

#[derive(Debug)]
pub struct Leg {
    pub stops: Vec<Stop>,
    pub notes: Vec<Note>,
    pub products: Vec<Product>,
    pub idx: i64,
    pub name: String,
    pub type_field: String,
    pub reachable: Option<bool>,
    pub direction: Option<String>,
}

impl From<LegRaw> for Leg {
    fn from(value: LegRaw) -> Self {
        Self {
            stops: value
                .stops
                .map(|stops| stops.stop.into_iter().map(|stop| stop.into()).collect())
                .unwrap_or_default(),
            notes: value
                .notes
                .map(|notes| notes.note.into_iter().map(|note| note.into()).collect())
                .unwrap_or_default(),
            products: value
                .product
                .into_iter()
                .map(|product| product.into())
                .collect(),
            idx: value.idx,
            name: value.name,
            type_field: value.type_field,
            reachable: value.reachable,
            direction: value.direction,
        }
    }
}

#[derive(Debug)]
pub enum StopTime {
    Endpoint {
        type_field: String,
        time: String,
        date: String,
    },
    Intermediate {},
}

#[derive(Debug)]
pub struct Stop {
    pub name: String,
    pub id: String,
    pub ext_id: String,
    pub route_idx: Option<i64>,
    pub coordinate: Coordinate,
    pub dep_time: Option<String>,
    pub dep_date: Option<String>,
    pub arr_time: Option<String>,
    pub arr_date: Option<String>,
}

#[derive(Debug)]
pub struct Endpoint {
    pub name: String,
    pub id: String,
    pub ext_id: String,
    pub route_idx: Option<i64>,
    pub coordinate: Coordinate,
    pub type_field: String,
    pub time: String,
    pub date: String,
}

impl From<StopRaw> for Stop {
    fn from(value: StopRaw) -> Self {
        Self {
            name: value.name,
            id: value.id,
            ext_id: value.ext_id,
            route_idx: Some(value.route_idx),
            coordinate: Coordinate {
                latitude: value.lat,
                longitude: value.lon,
            },
            dep_time: value.dep_time,
            dep_date: value.dep_date,
            arr_time: value.arr_time,
            arr_date: value.arr_date,
        }
    }
}

impl From<EndpointRaw> for Endpoint {
    fn from(value: EndpointRaw) -> Self {
        Self {
            name: value.name,
            id: value.id,
            ext_id: value.ext_id,
            route_idx: value.route_idx,
            coordinate: Coordinate {
                latitude: value.lat,
                longitude: value.lon,
            },
            type_field: value.type_field,
            time: value.time,
            date: value.date,
        }
    }
}

#[derive(Debug)]
pub struct Note {
    pub value: String,
    pub key: String,
    pub type_field: String,
    pub route_idx_from: Option<i64>,
    pub route_idx_to: Option<i64>,
}

impl From<NoteRaw> for Note {
    fn from(value: NoteRaw) -> Self {
        Self {
            value: value.value,
            key: value.key,
            type_field: value.type_field,
            route_idx_from: value.route_idx_from,
            route_idx_to: value.route_idx_to,
        }
    }
}

#[derive(Debug)]
pub struct Product {
    pub icon: String,
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

impl From<ProductRaw> for Product {
    fn from(value: ProductRaw) -> Self {
        Self {
            icon: value.icon.res,
            name: value.name,
            num: value.num,
            cat_code: value.cat_code,
            cat_out_s: value.cat_out_s,
            cat_out_l: value.cat_out_l,
            operator_code: value.operator_code,
            operator: value.operator,
            internal_name: value.internal_name,
            display_number: value.display_number,
            line: value.line,
            line_id: value.line_id,
            cat_out: value.cat_out,
            cat_in: value.cat_in,
            cls: value.cls,
            admin: value.admin,
            route_idx_from: value.route_idx_from,
            route_idx_to: value.route_idx_to,
            match_id: value.match_id,
        }
    }
}
