use serde::{Deserialize, Serialize};

use crate::models::Links;

/// An address entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_new_valuation_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bfe_numbers: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildings: Option<Vec<Building>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<City>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_on_market: Option<DaysOnMarket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub door: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstkvhx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_multiple_cases: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_market: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_valuation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub living_area: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality: Option<Municipality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<Province>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrations: Option<Vec<Registration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub road: Option<Road>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub road_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<Zip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<i64>,
}

/// A building inside an address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Building {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basement_area: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bathroom_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_wall_material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_installation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub housing_area: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kitchen_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bathrooms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_floors: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_rooms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_toilets: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roofing_material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toilet_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_area: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_built: Option<i64>,
}

/// City information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Municipality details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Municipality {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub church_tax_percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub council_tax_percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_value_tax_level_per_thousand: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_schools: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Province information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Province {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Road details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Road {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub road_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub road_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Zip code information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<i64>,
}

/// Geographical coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub coord_type: Option<String>,
}

/// A registration record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>, // Consider using chrono::NaiveDate if needed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
}

/// Days-on-market information (realtors array with unspecified item type).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaysOnMarket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtors: Option<Vec<serde_json::Value>>, // Flexible because item type is not defined
}
