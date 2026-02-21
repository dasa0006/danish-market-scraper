use serde::{Deserialize, Serialize};

use crate::models::{Property, Links};

/// Top-level API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub _links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_full_response() {
        let json = r#"
        {
            "_links": { "self": { "href": "/search/addresses" } },
            "addresses": [
                {
                    "_links": {
                        "self": {
                            "href": "/addresses/0a3f50ba-aa33-32b8-e044-0003ba298018"
                        }
                    },
                    "addressID": "0a3f50ba-aa33-32b8-e044-0003ba298018",
                    "addressType": "condo",
                    "allowNewValuationInfo": true,
                    "bfeNumbers": [327329],
                    "buildings": [
                        {
                            "basementArea": 202,
                            "bathroomCondition": "Badeværelse i enheden",
                            "buildingName": "Etagebolig-bygning, flerfamiliehus eller tofamiliehus",
                            "buildingNumber": "1",
                            "externalWallMaterial": "Mursten",
                            "heatingInstallation": "Fjernvarme/blokvarme",
                            "housingArea": 86,
                            "kitchenCondition": "Eget køkken med afløb",
                            "numberOfBathrooms": 1,
                            "numberOfFloors": 2,
                            "numberOfRooms": 4,
                            "numberOfToilets": 1,
                            "roofingMaterial": "Tegl",
                            "toiletCondition": "Vandskyllende toilet i enheden",
                            "totalArea": 202,
                            "yearBuilt": 1953
                        }
                    ],
                    "city": {
                        "name": "Fredericia",
                        "slug": "fredericia"
                    },
                    "cityName": "Fredericia",
                    "coordinates": {
                        "lat": 55.57248,
                        "lon": 9.762601,
                        "type": "EPSG4326"
                    },
                    "daysOnMarket": {
                        "realtors": []
                    },
                    "door": "th",
                    "entryAddressID": "0a3f508f-0f58-32b8-e044-0003ba298018",
                    "floor": "1",
                    "gstkvhx": "06074360__87__1__th",
                    "hasMultipleCases": false,
                    "houseNumber": "87",
                    "isOnMarket": false,
                    "isPublic": true,
                    "latestValuation": 740000,
                    "livingArea": 86,
                    "municipality": {
                        "churchTaxPercentage": 0.88,
                        "councilTaxPercentage": 25.5,
                        "landValueTaxLevelPerThousand": 13,
                        "municipalityCode": 607,
                        "name": "Fredericia",
                        "numberOfSchools": 21,
                        "population": 52485,
                        "slug": "fredericia"
                    },
                    "propertyNumber": 151679,
                    "province": {
                        "name": "Sydjylland",
                        "provinceCode": "DK032",
                        "regionCode": 1083,
                        "slug": "sydjylland"
                    },
                    "registrations": [
                        {
                            "amount": 850000,
                            "area": 86,
                            "date": "2015-08-27",
                            "municipalityCode": 607,
                            "propertyNumber": 151679,
                            "registrationID": "4804148",
                            "type": "normal"
                        },
                        {
                            "amount": 850000,
                            "date": "2006-09-23",
                            "municipalityCode": 607,
                            "propertyNumber": 151679,
                            "registrationID": "2288618",
                            "type": "normal"
                        },
                        {
                            "amount": 160000,
                            "date": "1995-03-09",
                            "municipalityCode": 607,
                            "propertyNumber": 151679,
                            "registrationID": "310093",
                            "type": "family"
                        }
                    ],
                    "road": {
                        "municipalityCode": 607,
                        "name": "6.Julivej",
                        "roadCode": 4360,
                        "roadID": "4c120663-1180-4d40-a7dc-623509c4fb84",
                        "slug": "6.julivej"
                    },
                    "roadName": "6.Julivej",
                    "slug": "6.julivej-87-1-th-7000-fredericia-06074360__87__1__th",
                    "slugAddress": "6.julivej-87-1-th-7000-fredericia",
                    "zip": {
                        "name": "Fredericia",
                        "slug": "fredericia",
                        "zipCode": 7000
                    },
                    "zipCode": 7000
                }
            ],
            "totalHits": 1
        }

        "#;

        let response: ApiResponse = serde_json::from_str(json).expect("JSON should be valid");

        // Assert top-level fields

        assert!(response._links.is_some());
        assert_eq!(response.total_hits, Some(1));
        assert!(response.addresses.is_some());

        let addresses = response.addresses.unwrap();
        assert_eq!(addresses.len(), 1);
        let addr = &addresses[0];

        // Assert some nested fields
        assert_eq!(
            addr.address_id.as_deref(),
            Some("0a3f50ba-aa33-32b8-e044-0003ba298018")
        );
        assert_eq!(addr.bfe_numbers.as_ref().unwrap(), &[327329]);

        let building = &addr.buildings.as_ref().unwrap()[0];
        assert_eq!(building.basement_area, Some(202));
        assert_eq!(building.year_built, Some(1953));

        // Check renamed fields: "type" in coordinates and registration
        assert_eq!(
            addr.coordinates.as_ref().unwrap().coord_type.as_deref(),
            Some("EPSG4326")
        );
        let reg = &addr.registrations.as_ref().unwrap()[0];
        assert_eq!(reg.registration_type.as_deref(), Some("normal"));

        // DaysOnMarket.realtors should accept any JSON (here strings)
        let realtors = addr
            .days_on_market
            .as_ref()
            .unwrap()
            .realtors
            .as_ref()
            .unwrap();
        assert_eq!(realtors.len(), 0);
    }

    #[test]
    fn deserialize_minimal_address() {
        let json = r#"
        {
            "addressID": "123",
            "cityName": "Springfield"
        }
        "#;
        let addr: Property = serde_json::from_str(json).expect("should parse");
        assert_eq!(addr.address_id, Some("123".to_string()));
        assert_eq!(addr.city_name, Some("Springfield".to_string()));
        assert!(addr.buildings.is_none()); // not present
        assert!(addr.coordinates.is_none()); // not present
    }

    #[test]
    fn deserialize_null_fields() {
        let json = r#"
        {
            "addressID": null,
            "cityName": "Springfield"
        }
        "#;
        let addr: Property = serde_json::from_str(json).unwrap();
        assert_eq!(addr.address_id, None); // null -> None
        assert_eq!(addr.city_name, Some("Springfield".to_string()));
    }

    #[test]
    fn deserialize_empty_arrays() {
        let json = r#"
        {
            "bfeNumbers": [],
            "buildings": []
        }
        "#;
        let addr: Property = serde_json::from_str(json).unwrap();
        assert_eq!(addr.bfe_numbers, Some(vec![]));
        assert_eq!(addr.buildings, Some(vec![]));
    }

    #[test]
    fn deserialize_extra_fields() {
        let json = r#"
        {
            "addressID": "123",
            "unknownField": "should be ignored"
        }
        "#;
        let addr: Property = serde_json::from_str(json).unwrap();
        assert_eq!(addr.address_id, Some("123".to_string()));
        // No error, extra field ignored.
    }
}
