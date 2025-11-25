#[test]
#[cfg(all(feature = "stress", feature = "resrobot"))]
fn resrobot_route_build_url_stress_test() {
    use std::time::Instant;

    use trafiklab::{
        Request,
        resrobot::{Location, RouteRequest},
    };

    let request = RouteRequest::new(
        "API_KEY".to_string(),
        Location::Id("740000001".to_string()),
        Location::Id("740000003".into()),
        // Location::Coordinate((59.332709, 18.065601).into()),
    )
    .with_search_for_arrival(true)
    .with_count_after(3)
    .with_count_before(3)
    .with_max_transfers(1);

    const ITERATIONS: u32 = 10_000;
    // warm-up to avoid measuring one-time allocations
    let _ = request.build_url();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = request.build_url().unwrap();
    }
    let total = start.elapsed();

    let avg = total / ITERATIONS;

    println!("Total: {:?} for {ITERATIONS} iterations", total);
    println!("Average: {:?} per call", avg);
    let url = request.build_url().unwrap();
    println!("{}", url.as_str())
    // assert_eq!(url, url::Url::from_str("https://hello.com").unwrap())
}

#[test]
#[cfg(feature = "resrobot")]
fn resrobot_route_build_url_base_test() {
    use trafiklab::{
        Request,
        resrobot::{Location, RouteRequest},
    };

    let url = RouteRequest::new(
        "API_KEY".to_string(),
        Location::Id("740000001".to_string()),
        Location::Id("740000003".into()),
    )
    .build_url()
    .unwrap();
    assert_eq!(
        url.as_str(),
        "https://api.resrobot.se/v2.1/trip?format=json&accessId=API_KEY&lang=sv&originId=740000001&destId=740000003&numF=5&numB=0&passlist=false&searchForArrival=false"
    )
}

#[test]
#[cfg(feature = "resrobot")]
fn resrobot_route_build_url_custom_test() {
    use trafiklab::{
        Request,
        resrobot::{Language, Location, RouteRequest},
    };

    let url = RouteRequest::new(
        "API_KEY".to_string(),
        Location::Id("740000001".to_string()),
        Location::Id("740000003".into()),
    )
    .with_language(Language::English)
    .with_pass_list(true)
    .build_url()
    .unwrap();
    assert_eq!(
        url.as_str(),
        "https://api.resrobot.se/v2.1/trip?format=json&accessId=API_KEY&lang=en&originId=740000001&destId=740000003&numF=5&numB=0&passlist=true&searchForArrival=false"
    )
}
