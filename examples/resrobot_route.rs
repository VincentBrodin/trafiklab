use dotenv::dotenv;
use trafiklab::resrobot::RouteRequest;

#[cfg(feature = "resrobot")]
#[tokio::main]
async fn main() -> Result<(), trafiklab::Error> {
    use trafiklab::{Request, resrobot::Location};

    dotenv().unwrap();
    let key = std::env::var("RESROBOT_API_KEY")
        .expect("Missing  RESROBOT_API_KEY in environment or .env");
    println!("API key: {}", key);

    let res = RouteRequest::new(
        key,
        Location::Id("740000001".into()),
        Location::Id("740000003".into()),
    )
    .with_count_after(2)
    .with_count_before(4)
    .with_pass_list(true)
    .send()
    .await?;

    if let Some(trip) = res.trips.first() {
        println!(
            "Going from {} to {}",
            trip.origin.name, trip.destination.name
        );
        for leg in trip.legs.iter() {
            println!("{}", leg.name);
            for stop in leg.stops.iter() {
                println!("  {}", stop.name)
            }
        }
    }

    Ok(())
}
