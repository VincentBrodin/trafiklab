use chrono::Utc;

use crate::Request;
use crate::resrobot::{Language, Location, RouteResponse, RouteResponseRaw};

pub struct RouteRequest {
    access_id: String,
    origin: Location,
    destination: Location,
    language: Language,
    search_for_arrival: bool,
    pass_list: bool,
    date_time: chrono::DateTime<Utc>,
    count_before: u8,
    count_after: u8,
    max_transfer: Option<u8>,
}

impl RouteRequest {
    /// Creates a new [`RouteRequest`] with the required fields.
    ///
    /// Defaults:
    /// - `language`: [`Language::Swedish`]
    /// - `search_for_arrival`: `false`
    /// - `pass_list`: `false`
    /// - `date_time`: current time
    /// - `count_after`: `5`
    /// - `count_before`: `0`
    /// - `max_transfer`: `None`
    pub fn new(access_id: String, origin: Location, destination: Location) -> Self {
        Self {
            access_id,                     // ✓
            origin,                        // ✓
            destination,                   // ✓
            language: Default::default(),  // ✓
            search_for_arrival: false,     // ✓
            pass_list: false,              // ✓
            date_time: chrono::Utc::now(), // ✓
            count_after: 5,                // ✓
            count_before: 0,               // ✓
            max_transfer: None,            // ✓
        }
    }

    /// Sets the language of the API response.
    ///
    /// Defaults to [`Language::Swedish`].
    ///
    /// Affects both data (e.g., transport type names) and error messages.
    ///
    /// Chainable.    
    pub fn with_language(mut self, value: Language) -> Self {
        self.language = value;
        self
    }

    /// Includes a list of intermediate stops in the response.
    ///
    /// Defaults to `false`.
    ///
    /// Chainable.
    pub fn with_pass_list(mut self, value: bool) -> Self {
        self.pass_list = value;
        self
    }

    /// Searches for the *latest arrival time* instead of the default
    /// *earliest departure time*.
    ///
    /// When set to `true`, [`with_count_after`] and [`with_count_before`]
    /// are ignored.
    ///
    /// Chainable.
    pub fn with_search_for_arrival(mut self, value: bool) -> Self {
        self.search_for_arrival = value;
        self
    }

    /// Sets the time to search around.
    ///
    /// Uses Central European Standard Time (GMT+1).
    ///
    /// Combine with [`with_search_for_arrival`] to interpret the time as
    /// an arrival time instead of a departure time.
    ///
    /// Chainable.
    pub fn with_time(mut self, value: chrono::DateTime<Utc>) -> Self {
        self.date_time = value;
        self
    }

    /// Sets how many routes should be returned *after* the given time.
    ///
    /// Defaults to `5`.
    ///
    /// Ignored when [`with_search_for_arrival`] is `true`.
    ///
    /// The sum of `count_after` and `count_before` must not exceed **6**.
    ///
    /// # Panics
    /// Panics if `self.count_before + value > 6`.
    ///
    /// Chainable.
    pub fn with_count_after(mut self, value: u8) -> Self {
        assert!(self.count_before + value <= 6);
        self.count_after = value;
        self
    }

    /// Sets how many routes should be returned *before* the given time.
    ///
    /// Defaults to `0`.
    ///
    /// Ignored when [`with_search_for_arrival`] is `true`.
    ///
    /// The sum of `count_after` and `count_before` must not exceed **6**.
    ///
    /// # Panics
    /// Panics if `value + self.count_after > 6`.
    ///
    /// Chainable.
    pub fn with_count_before(mut self, value: u8) -> Self {
        assert!(value + self.count_after <= 6);
        self.count_before = value;
        self
    }

    /// Limits the maximum number of transfers.
    ///
    /// Valid values are `1` through `3`.  
    /// Use `None` for unlimited transfers.
    ///
    /// Defaults to `None`.
    ///
    /// # Panics
    /// Panics if the value is not in the range `1..=3`.
    ///
    /// Chainable.
    pub fn with_max_transfers(mut self, value: u8) -> Self {
        // has to be between 1 and 3
        assert!((1..4).contains(&value));
        self.max_transfer = Some(value);
        self
    }
}

impl Request for RouteRequest {
    type Output = RouteResponse;

    async fn send(self) -> Result<Self::Output, crate::Error> {
        let url = self.build_url()?;
        let res = reqwest::get(url).await?;
        let raw: RouteResponseRaw = res.json().await?;
        Ok(raw.into())
    }

    fn build_url(&self) -> Result<reqwest::Url, crate::Error> {
        // See https://www.trafiklab.se/api/our-apis/resrobot-v21/route-planner/
        const ORIGIN: &str = "origin";
        const DEST: &str = "dest";
        const PARAM_COUNT: usize = 14;
        let mut params: Vec<(String, String)> = Vec::with_capacity(PARAM_COUNT);
        params.push(("format".into(), "json".into()));
        params.push(("accessId".into(), self.access_id.clone()));
        params.push(("lang".into(), self.language.to_string()));
        params.append(&mut self.origin.as_query_params(ORIGIN));
        params.append(&mut self.destination.as_query_params(DEST));
        params.push(("numF".into(), self.count_after.to_string()));
        params.push(("numB".into(), self.count_before.to_string()));
        params.push(("passlist".into(), self.pass_list.to_string()));
        params.push((
            "searchForArrival".into(),
            self.search_for_arrival.to_string(),
        ));
        if let Some(value) = self.max_transfer {
            params.push(("maxChange".into(), value.to_string()))
        }
        let url = reqwest::Url::parse_with_params("https://api.resrobot.se/v2.1/trip", params)?;
        Ok(url)
    }
}
