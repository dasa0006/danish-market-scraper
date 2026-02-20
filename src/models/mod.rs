pub mod address;
pub mod api_response;
pub mod links;

pub use address::{
    Address, Building, City, Coordinates, DaysOnMarket, Municipality, Province, Registration, Road,
    Zip,
};
pub use api_response::ApiResponse;
pub use links::{Links, SelfLink};
