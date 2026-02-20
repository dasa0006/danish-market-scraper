pub mod address;
pub mod api_response;
pub mod links;
pub mod location;

pub use address::{Address, Building, DaysOnMarket, Registration};
pub use api_response::ApiResponse;
pub use links::{Links, SelfLink};
pub use location::{City, Coordinates, Municipality, Province, Road, Zip};
