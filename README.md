# Trafiklab Rust SDK (Unofficial)
This project aims to provide a clean, async, and well-typed Rust interface to all Trafiklab endpoints.

> [!WARNING]
> This project is in **very early development** and is not yet ready for production use.
> Many endpoints and features are still being designed or implemented.
> If this SDK is something you're interested in using, or if you want to help get it up and running sooner **please consider contributing!**  
> Issues, pull requests, and suggestions are all highly appreciated.

## Usage

### GTFS Svergie 2
> [!NOTE]
> Cooming soon.

### GTFS Regional 
> [!NOTE]
> Cooming soon.

### GTFS Sweden 3 
> [!NOTE]
> Cooming soon.

### SL 
> [!NOTE]
> Cooming soon.


### ResRobot
**Route planner**
```rust
let url = trafiklab::resrobot::RouteRequest::new(
    "API_KEY".into(),
    Location::Id("740000001".to_string()),
    Location::Id("740000003".into()),
)
.build_url()
.unwrap();
```

## Roadmap
**Current Endpoint: ResRobot Route planner**

- [ ] GTFS Svergie 2
- [ ] GTFS Regional
  - [ ] Static
  - [ ] Realtime
- [ ] GTFS Sweden 3
  - [ ] Static
  - [ ] Realtime

- [ ] SL
  - [ ] SL Transport
  - [ ] SL Deviations
  - [ ] SL Journey-planner v2

- [ ] ResRobot v2.1
  - [ ] ResRobot Timetables
  - [ ] ResRobot Route planner
  - [ ] ResRobot Stop lookup
  - [ ] ResRobot Nearby stops
  - [ ] ResRobot Deep Links

## Contributing
Contributions, bug reports, and suggestions are welcome!
The end goal is to make this crate a high-quality, community-driven SDK that Trafiklab may one day adopt officially.
