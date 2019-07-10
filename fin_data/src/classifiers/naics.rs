use std::collections::HashMap;

use chrono::NaiveDate;

use fin_model::classification::{Code, ClassificationScheme};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct NAICS {
    codes: HashMap<u32, Code<u32>>
}

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl ClassificationScheme<u32> for NAICS {

    fn new() -> Self {
        NAICS { codes: create_data_table() }
    }

    fn name() -> String {
        "North American Industry Classification System".to_string()
    }

    fn acronym() -> String {
        "NAICS".to_string()
    }

    fn source() -> String {
        // https://www.census.gov/eos/www/naics/2017NAICS/2-6%20digit_2017_Codes.xlsx
        "https://www.census.gov/eos/www/naics/".to_string()
    }

    fn governing_body() -> Option<String> {
        Some("US Office of Management and Budget (OMB)".to_string())
    }

    fn last_updated() -> Option<NaiveDate> {
        Some(NaiveDate::from_ymd(2016, 3, 21))
    }

    fn get(&self, code: u32) -> Option<&Code<u32>> {
        self.codes.get(&code)
    }

    fn get_children(&self, _parent: u32) -> Option<Vec<&Code<u32>>> {
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Generated Data Table
// ------------------------------------------------------------------------------------------------

fn create_data_table() -> HashMap<u32, Code<u32>> {
    let table: HashMap<u32, Code<u32>> = 
    [
        (11, Code::<u32> {
            code: 11,
            parent_code: None,
            description: "Agriculture, Forestry, Fishing and Hunting".to_string(),
        }),
        (111, Code::<u32> {
            code: 111,
            parent_code: Some(11),
            description: "Crop Production".to_string(),
        }),
        (1111, Code::<u32> {
            code: 1111,
            parent_code: Some(111),
            description: "Oilseed and Grain Farming".to_string(),
        }),
        (11111, Code::<u32> {
            code: 11111,
            parent_code: Some(1111),
            description: "Soybean Farming".to_string(),
        }),
        (111110, Code::<u32> {
            code: 111110,
            parent_code: Some(11111),
            description: "Soybean Farming".to_string(),
        }),
        (11112, Code::<u32> {
            code: 11112,
            parent_code: Some(1111),
            description: "Oilseed (except Soybean) Farming".to_string(),
        }),
        (111120, Code::<u32> {
            code: 111120,
            parent_code: Some(11112),
            description: "Oilseed (except Soybean) Farming ".to_string(),
        }),
        (11113, Code::<u32> {
            code: 11113,
            parent_code: Some(1111),
            description: "Dry Pea and Bean Farming".to_string(),
        }),
        (111130, Code::<u32> {
            code: 111130,
            parent_code: Some(11113),
            description: "Dry Pea and Bean Farming ".to_string(),
        }),
        (11114, Code::<u32> {
            code: 11114,
            parent_code: Some(1111),
            description: "Wheat Farming".to_string(),
        }),
        (111140, Code::<u32> {
            code: 111140,
            parent_code: Some(11114),
            description: "Wheat Farming".to_string(),
        }),
        (11115, Code::<u32> {
            code: 11115,
            parent_code: Some(1111),
            description: "Corn Farming".to_string(),
        }),
        (111150, Code::<u32> {
            code: 111150,
            parent_code: Some(11115),
            description: "Corn Farming ".to_string(),
        }),
        (11116, Code::<u32> {
            code: 11116,
            parent_code: Some(1111),
            description: "Rice Farming".to_string(),
        }),
        (111160, Code::<u32> {
            code: 111160,
            parent_code: Some(11116),
            description: "Rice Farming".to_string(),
        }),
        (11119, Code::<u32> {
            code: 11119,
            parent_code: Some(1111),
            description: "Other Grain Farming".to_string(),
        }),
        (111191, Code::<u32> {
            code: 111191,
            parent_code: Some(11119),
            description: "Oilseed and Grain Combination Farming ".to_string(),
        }),
        (111199, Code::<u32> {
            code: 111199,
            parent_code: Some(11119),
            description: "All Other Grain Farming ".to_string(),
        }),
        (1112, Code::<u32> {
            code: 1112,
            parent_code: Some(111),
            description: "Vegetable and Melon Farming".to_string(),
        }),
        (11121, Code::<u32> {
            code: 11121,
            parent_code: Some(1112),
            description: "Vegetable and Melon Farming".to_string(),
        }),
        (111211, Code::<u32> {
            code: 111211,
            parent_code: Some(11121),
            description: "Potato Farming ".to_string(),
        }),
        (111219, Code::<u32> {
            code: 111219,
            parent_code: Some(11121),
            description: "Other Vegetable (except Potato) and Melon Farming ".to_string(),
        }),
        (1113, Code::<u32> {
            code: 1113,
            parent_code: Some(111),
            description: "Fruit and Tree Nut Farming".to_string(),
        }),
        (11131, Code::<u32> {
            code: 11131,
            parent_code: Some(1113),
            description: "Orange Groves".to_string(),
        }),
        (111310, Code::<u32> {
            code: 111310,
            parent_code: Some(11131),
            description: "Orange Groves".to_string(),
        }),
        (11132, Code::<u32> {
            code: 11132,
            parent_code: Some(1113),
            description: "Citrus (except Orange) Groves".to_string(),
        }),
        (111320, Code::<u32> {
            code: 111320,
            parent_code: Some(11132),
            description: "Citrus (except Orange) Groves ".to_string(),
        }),
        (11133, Code::<u32> {
            code: 11133,
            parent_code: Some(1113),
            description: "Noncitrus Fruit and Tree Nut Farming".to_string(),
        }),
        (111331, Code::<u32> {
            code: 111331,
            parent_code: Some(11133),
            description: "Apple Orchards ".to_string(),
        }),
        (111332, Code::<u32> {
            code: 111332,
            parent_code: Some(11133),
            description: "Grape Vineyards ".to_string(),
        }),
        (111333, Code::<u32> {
            code: 111333,
            parent_code: Some(11133),
            description: "Strawberry Farming ".to_string(),
        }),
        (111334, Code::<u32> {
            code: 111334,
            parent_code: Some(11133),
            description: "Berry (except Strawberry) Farming ".to_string(),
        }),
        (111335, Code::<u32> {
            code: 111335,
            parent_code: Some(11133),
            description: "Tree Nut Farming ".to_string(),
        }),
        (111336, Code::<u32> {
            code: 111336,
            parent_code: Some(11133),
            description: "Fruit and Tree Nut Combination Farming ".to_string(),
        }),
        (111339, Code::<u32> {
            code: 111339,
            parent_code: Some(11133),
            description: "Other Noncitrus Fruit Farming ".to_string(),
        }),
        (1114, Code::<u32> {
            code: 1114,
            parent_code: Some(111),
            description: "Greenhouse, Nursery, and Floriculture Production".to_string(),
        }),
        (11141, Code::<u32> {
            code: 11141,
            parent_code: Some(1114),
            description: "Food Crops Grown Under Cover".to_string(),
        }),
        (111411, Code::<u32> {
            code: 111411,
            parent_code: Some(11141),
            description: "Mushroom Production ".to_string(),
        }),
        (111419, Code::<u32> {
            code: 111419,
            parent_code: Some(11141),
            description: "Other Food Crops Grown Under Cover ".to_string(),
        }),
        (11142, Code::<u32> {
            code: 11142,
            parent_code: Some(1114),
            description: "Nursery and Floriculture Production".to_string(),
        }),
        (111421, Code::<u32> {
            code: 111421,
            parent_code: Some(11142),
            description: "Nursery and Tree Production ".to_string(),
        }),
        (111422, Code::<u32> {
            code: 111422,
            parent_code: Some(11142),
            description: "Floriculture Production ".to_string(),
        }),
        (1119, Code::<u32> {
            code: 1119,
            parent_code: Some(111),
            description: "Other Crop Farming".to_string(),
        }),
        (11191, Code::<u32> {
            code: 11191,
            parent_code: Some(1119),
            description: "Tobacco Farming".to_string(),
        }),
        (111910, Code::<u32> {
            code: 111910,
            parent_code: Some(11191),
            description: "Tobacco Farming".to_string(),
        }),
        (11192, Code::<u32> {
            code: 11192,
            parent_code: Some(1119),
            description: "Cotton Farming".to_string(),
        }),
        (111920, Code::<u32> {
            code: 111920,
            parent_code: Some(11192),
            description: "Cotton Farming".to_string(),
        }),
        (11193, Code::<u32> {
            code: 11193,
            parent_code: Some(1119),
            description: "Sugarcane Farming".to_string(),
        }),
        (111930, Code::<u32> {
            code: 111930,
            parent_code: Some(11193),
            description: "Sugarcane Farming".to_string(),
        }),
        (11194, Code::<u32> {
            code: 11194,
            parent_code: Some(1119),
            description: "Hay Farming".to_string(),
        }),
        (111940, Code::<u32> {
            code: 111940,
            parent_code: Some(11194),
            description: "Hay Farming ".to_string(),
        }),
        (11199, Code::<u32> {
            code: 11199,
            parent_code: Some(1119),
            description: "All Other Crop Farming".to_string(),
        }),
        (111991, Code::<u32> {
            code: 111991,
            parent_code: Some(11199),
            description: "Sugar Beet Farming ".to_string(),
        }),
        (111992, Code::<u32> {
            code: 111992,
            parent_code: Some(11199),
            description: "Peanut Farming ".to_string(),
        }),
        (111998, Code::<u32> {
            code: 111998,
            parent_code: Some(11199),
            description: "All Other Miscellaneous Crop Farming ".to_string(),
        }),
        (112, Code::<u32> {
            code: 112,
            parent_code: Some(11),
            description: "Animal Production and Aquaculture".to_string(),
        }),
        (1121, Code::<u32> {
            code: 1121,
            parent_code: Some(112),
            description: "Cattle Ranching and Farming".to_string(),
        }),
        (11211, Code::<u32> {
            code: 11211,
            parent_code: Some(1121),
            description: "Beef Cattle Ranching and Farming, including Feedlots".to_string(),
        }),
        (112111, Code::<u32> {
            code: 112111,
            parent_code: Some(11211),
            description: "Beef Cattle Ranching and Farming ".to_string(),
        }),
        (112112, Code::<u32> {
            code: 112112,
            parent_code: Some(11211),
            description: "Cattle Feedlots ".to_string(),
        }),
        (11212, Code::<u32> {
            code: 11212,
            parent_code: Some(1121),
            description: "Dairy Cattle and Milk Production".to_string(),
        }),
        (112120, Code::<u32> {
            code: 112120,
            parent_code: Some(11212),
            description: "Dairy Cattle and Milk Production".to_string(),
        }),
        (11213, Code::<u32> {
            code: 11213,
            parent_code: Some(1121),
            description: "Dual-Purpose Cattle Ranching and Farming".to_string(),
        }),
        (112130, Code::<u32> {
            code: 112130,
            parent_code: Some(11213),
            description: "Dual-Purpose Cattle Ranching and Farming ".to_string(),
        }),
        (1122, Code::<u32> {
            code: 1122,
            parent_code: Some(112),
            description: "Hog and Pig Farming".to_string(),
        }),
        (11221, Code::<u32> {
            code: 11221,
            parent_code: Some(1122),
            description: "Hog and Pig Farming".to_string(),
        }),
        (112210, Code::<u32> {
            code: 112210,
            parent_code: Some(11221),
            description: "Hog and Pig Farming ".to_string(),
        }),
        (1123, Code::<u32> {
            code: 1123,
            parent_code: Some(112),
            description: "Poultry and Egg Production".to_string(),
        }),
        (11231, Code::<u32> {
            code: 11231,
            parent_code: Some(1123),
            description: "Chicken Egg Production".to_string(),
        }),
        (112310, Code::<u32> {
            code: 112310,
            parent_code: Some(11231),
            description: "Chicken Egg Production ".to_string(),
        }),
        (11232, Code::<u32> {
            code: 11232,
            parent_code: Some(1123),
            description: "Broilers and Other Meat Type Chicken Production".to_string(),
        }),
        (112320, Code::<u32> {
            code: 112320,
            parent_code: Some(11232),
            description: "Broilers and Other Meat Type Chicken Production ".to_string(),
        }),
        (11233, Code::<u32> {
            code: 11233,
            parent_code: Some(1123),
            description: "Turkey Production".to_string(),
        }),
        (112330, Code::<u32> {
            code: 112330,
            parent_code: Some(11233),
            description: "Turkey Production".to_string(),
        }),
        (11234, Code::<u32> {
            code: 11234,
            parent_code: Some(1123),
            description: "Poultry Hatcheries".to_string(),
        }),
        (112340, Code::<u32> {
            code: 112340,
            parent_code: Some(11234),
            description: "Poultry Hatcheries".to_string(),
        }),
        (11239, Code::<u32> {
            code: 11239,
            parent_code: Some(1123),
            description: "Other Poultry Production".to_string(),
        }),
        (112390, Code::<u32> {
            code: 112390,
            parent_code: Some(11239),
            description: "Other Poultry Production ".to_string(),
        }),
        (1124, Code::<u32> {
            code: 1124,
            parent_code: Some(112),
            description: "Sheep and Goat Farming".to_string(),
        }),
        (11241, Code::<u32> {
            code: 11241,
            parent_code: Some(1124),
            description: "Sheep Farming".to_string(),
        }),
        (112410, Code::<u32> {
            code: 112410,
            parent_code: Some(11241),
            description: "Sheep Farming".to_string(),
        }),
        (11242, Code::<u32> {
            code: 11242,
            parent_code: Some(1124),
            description: "Goat Farming".to_string(),
        }),
        (112420, Code::<u32> {
            code: 112420,
            parent_code: Some(11242),
            description: "Goat Farming".to_string(),
        }),
        (1125, Code::<u32> {
            code: 1125,
            parent_code: Some(112),
            description: "Aquaculture".to_string(),
        }),
        (11251, Code::<u32> {
            code: 11251,
            parent_code: Some(1125),
            description: "Aquaculture".to_string(),
        }),
        (112511, Code::<u32> {
            code: 112511,
            parent_code: Some(11251),
            description: "Finfish Farming and Fish Hatcheries ".to_string(),
        }),
        (112512, Code::<u32> {
            code: 112512,
            parent_code: Some(11251),
            description: "Shellfish Farming ".to_string(),
        }),
        (112519, Code::<u32> {
            code: 112519,
            parent_code: Some(11251),
            description: "Other Aquaculture ".to_string(),
        }),
        (1129, Code::<u32> {
            code: 1129,
            parent_code: Some(112),
            description: "Other Animal Production".to_string(),
        }),
        (11291, Code::<u32> {
            code: 11291,
            parent_code: Some(1129),
            description: "Apiculture".to_string(),
        }),
        (112910, Code::<u32> {
            code: 112910,
            parent_code: Some(11291),
            description: "Apiculture".to_string(),
        }),
        (11292, Code::<u32> {
            code: 11292,
            parent_code: Some(1129),
            description: "Horses and Other Equine Production".to_string(),
        }),
        (112920, Code::<u32> {
            code: 112920,
            parent_code: Some(11292),
            description: "Horses and Other Equine Production".to_string(),
        }),
        (11293, Code::<u32> {
            code: 11293,
            parent_code: Some(1129),
            description: "Fur-Bearing Animal and Rabbit Production".to_string(),
        }),
        (112930, Code::<u32> {
            code: 112930,
            parent_code: Some(11293),
            description: "Fur-Bearing Animal and Rabbit Production".to_string(),
        }),
        (11299, Code::<u32> {
            code: 11299,
            parent_code: Some(1129),
            description: "All Other Animal Production".to_string(),
        }),
        (112990, Code::<u32> {
            code: 112990,
            parent_code: Some(11299),
            description: "All Other Animal Production ".to_string(),
        }),
        (113, Code::<u32> {
            code: 113,
            parent_code: Some(11),
            description: "Forestry and Logging".to_string(),
        }),
        (1131, Code::<u32> {
            code: 1131,
            parent_code: Some(113),
            description: "Timber Tract Operations".to_string(),
        }),
        (11311, Code::<u32> {
            code: 11311,
            parent_code: Some(1131),
            description: "Timber Tract Operations".to_string(),
        }),
        (113110, Code::<u32> {
            code: 113110,
            parent_code: Some(11311),
            description: "Timber Tract Operations".to_string(),
        }),
        (1132, Code::<u32> {
            code: 1132,
            parent_code: Some(113),
            description: "Forest Nurseries and Gathering of Forest Products".to_string(),
        }),
        (11321, Code::<u32> {
            code: 11321,
            parent_code: Some(1132),
            description: "Forest Nurseries and Gathering of Forest Products".to_string(),
        }),
        (113210, Code::<u32> {
            code: 113210,
            parent_code: Some(11321),
            description: "Forest Nurseries and Gathering of Forest Products ".to_string(),
        }),
        (1133, Code::<u32> {
            code: 1133,
            parent_code: Some(113),
            description: "Logging".to_string(),
        }),
        (11331, Code::<u32> {
            code: 11331,
            parent_code: Some(1133),
            description: "Logging".to_string(),
        }),
        (113310, Code::<u32> {
            code: 113310,
            parent_code: Some(11331),
            description: "Logging ".to_string(),
        }),
        (114, Code::<u32> {
            code: 114,
            parent_code: Some(11),
            description: "Fishing, Hunting and Trapping".to_string(),
        }),
        (1141, Code::<u32> {
            code: 1141,
            parent_code: Some(114),
            description: "Fishing".to_string(),
        }),
        (11411, Code::<u32> {
            code: 11411,
            parent_code: Some(1141),
            description: "Fishing".to_string(),
        }),
        (114111, Code::<u32> {
            code: 114111,
            parent_code: Some(11411),
            description: "Finfish Fishing ".to_string(),
        }),
        (114112, Code::<u32> {
            code: 114112,
            parent_code: Some(11411),
            description: "Shellfish Fishing ".to_string(),
        }),
        (114119, Code::<u32> {
            code: 114119,
            parent_code: Some(11411),
            description: "Other Marine Fishing ".to_string(),
        }),
        (1142, Code::<u32> {
            code: 1142,
            parent_code: Some(114),
            description: "Hunting and Trapping".to_string(),
        }),
        (11421, Code::<u32> {
            code: 11421,
            parent_code: Some(1142),
            description: "Hunting and Trapping".to_string(),
        }),
        (114210, Code::<u32> {
            code: 114210,
            parent_code: Some(11421),
            description: "Hunting and Trapping".to_string(),
        }),
        (115, Code::<u32> {
            code: 115,
            parent_code: Some(11),
            description: "Support Activities for Agriculture and Forestry".to_string(),
        }),
        (1151, Code::<u32> {
            code: 1151,
            parent_code: Some(115),
            description: "Support Activities for Crop Production".to_string(),
        }),
        (11511, Code::<u32> {
            code: 11511,
            parent_code: Some(1151),
            description: "Support Activities for Crop Production".to_string(),
        }),
        (115111, Code::<u32> {
            code: 115111,
            parent_code: Some(11511),
            description: "Cotton Ginning ".to_string(),
        }),
        (115112, Code::<u32> {
            code: 115112,
            parent_code: Some(11511),
            description: "Soil Preparation, Planting, and Cultivating ".to_string(),
        }),
        (115113, Code::<u32> {
            code: 115113,
            parent_code: Some(11511),
            description: "Crop Harvesting, Primarily by Machine ".to_string(),
        }),
        (115114, Code::<u32> {
            code: 115114,
            parent_code: Some(11511),
            description: "Postharvest Crop Activities (except Cotton Ginning) ".to_string(),
        }),
        (115115, Code::<u32> {
            code: 115115,
            parent_code: Some(11511),
            description: "Farm Labor Contractors and Crew Leaders ".to_string(),
        }),
        (115116, Code::<u32> {
            code: 115116,
            parent_code: Some(11511),
            description: "Farm Management Services ".to_string(),
        }),
        (1152, Code::<u32> {
            code: 1152,
            parent_code: Some(115),
            description: "Support Activities for Animal Production".to_string(),
        }),
        (11521, Code::<u32> {
            code: 11521,
            parent_code: Some(1152),
            description: "Support Activities for Animal Production".to_string(),
        }),
        (115210, Code::<u32> {
            code: 115210,
            parent_code: Some(11521),
            description: "Support Activities for Animal Production".to_string(),
        }),
        (1153, Code::<u32> {
            code: 1153,
            parent_code: Some(115),
            description: "Support Activities for Forestry".to_string(),
        }),
        (11531, Code::<u32> {
            code: 11531,
            parent_code: Some(1153),
            description: "Support Activities for Forestry".to_string(),
        }),
        (115310, Code::<u32> {
            code: 115310,
            parent_code: Some(11531),
            description: "Support Activities for Forestry".to_string(),
        }),
        (21, Code::<u32> {
            code: 21,
            parent_code: None,
            description: "Mining, Quarrying, and Oil and Gas Extraction".to_string(),
        }),
        (211, Code::<u32> {
            code: 211,
            parent_code: Some(21),
            description: "Oil and Gas Extraction".to_string(),
        }),
        (2111, Code::<u32> {
            code: 2111,
            parent_code: Some(211),
            description: "Oil and Gas Extraction".to_string(),
        }),
        (21112, Code::<u32> {
            code: 21112,
            parent_code: Some(2111),
            description: "Crude Petroleum Extraction ".to_string(),
        }),
        (211120, Code::<u32> {
            code: 211120,
            parent_code: Some(21112),
            description: "Crude Petroleum Extraction ".to_string(),
        }),
        (21113, Code::<u32> {
            code: 21113,
            parent_code: Some(2111),
            description: "Natural Gas Extraction ".to_string(),
        }),
        (211130, Code::<u32> {
            code: 211130,
            parent_code: Some(21113),
            description: "Natural Gas Extraction ".to_string(),
        }),
        (212, Code::<u32> {
            code: 212,
            parent_code: Some(21),
            description: "Mining (except Oil and Gas)".to_string(),
        }),
        (2121, Code::<u32> {
            code: 2121,
            parent_code: Some(212),
            description: "Coal Mining".to_string(),
        }),
        (21211, Code::<u32> {
            code: 21211,
            parent_code: Some(2121),
            description: "Coal Mining".to_string(),
        }),
        (212111, Code::<u32> {
            code: 212111,
            parent_code: Some(21211),
            description: "Bituminous Coal and Lignite Surface Mining ".to_string(),
        }),
        (212112, Code::<u32> {
            code: 212112,
            parent_code: Some(21211),
            description: "Bituminous Coal Underground Mining ".to_string(),
        }),
        (212113, Code::<u32> {
            code: 212113,
            parent_code: Some(21211),
            description: "Anthracite Mining ".to_string(),
        }),
        (2122, Code::<u32> {
            code: 2122,
            parent_code: Some(212),
            description: "Metal Ore Mining".to_string(),
        }),
        (21221, Code::<u32> {
            code: 21221,
            parent_code: Some(2122),
            description: "Iron Ore Mining".to_string(),
        }),
        (212210, Code::<u32> {
            code: 212210,
            parent_code: Some(21221),
            description: "Iron Ore Mining".to_string(),
        }),
        (21222, Code::<u32> {
            code: 21222,
            parent_code: Some(2122),
            description: "Gold Ore and Silver Ore Mining".to_string(),
        }),
        (212221, Code::<u32> {
            code: 212221,
            parent_code: Some(21222),
            description: "Gold Ore Mining ".to_string(),
        }),
        (212222, Code::<u32> {
            code: 212222,
            parent_code: Some(21222),
            description: "Silver Ore Mining ".to_string(),
        }),
        (21223, Code::<u32> {
            code: 21223,
            parent_code: Some(2122),
            description: "Copper, Nickel, Lead, and Zinc Mining".to_string(),
        }),
        (212230, Code::<u32> {
            code: 212230,
            parent_code: Some(21223),
            description: "Copper, Nickel, Lead, and Zinc Mining ".to_string(),
        }),
        (21229, Code::<u32> {
            code: 21229,
            parent_code: Some(2122),
            description: "Other Metal Ore Mining".to_string(),
        }),
        (212291, Code::<u32> {
            code: 212291,
            parent_code: Some(21229),
            description: "Uranium-Radium-Vanadium Ore Mining ".to_string(),
        }),
        (212299, Code::<u32> {
            code: 212299,
            parent_code: Some(21229),
            description: "All Other Metal Ore Mining ".to_string(),
        }),
        (2123, Code::<u32> {
            code: 2123,
            parent_code: Some(212),
            description: "Nonmetallic Mineral Mining and Quarrying".to_string(),
        }),
        (21231, Code::<u32> {
            code: 21231,
            parent_code: Some(2123),
            description: "Stone Mining and Quarrying".to_string(),
        }),
        (212311, Code::<u32> {
            code: 212311,
            parent_code: Some(21231),
            description: "Dimension Stone Mining and Quarrying ".to_string(),
        }),
        (212312, Code::<u32> {
            code: 212312,
            parent_code: Some(21231),
            description: "Crushed and Broken Limestone Mining and Quarrying ".to_string(),
        }),
        (212313, Code::<u32> {
            code: 212313,
            parent_code: Some(21231),
            description: "Crushed and Broken Granite Mining and Quarrying ".to_string(),
        }),
        (212319, Code::<u32> {
            code: 212319,
            parent_code: Some(21231),
            description: "Other Crushed and Broken Stone Mining and Quarrying ".to_string(),
        }),
        (21232, Code::<u32> {
            code: 21232,
            parent_code: Some(2123),
            description: "Sand, Gravel, Clay, and Ceramic and Refractory Minerals Mining and Quarrying".to_string(),
        }),
        (212321, Code::<u32> {
            code: 212321,
            parent_code: Some(21232),
            description: "Construction Sand and Gravel Mining ".to_string(),
        }),
        (212322, Code::<u32> {
            code: 212322,
            parent_code: Some(21232),
            description: "Industrial Sand Mining ".to_string(),
        }),
        (212324, Code::<u32> {
            code: 212324,
            parent_code: Some(21232),
            description: "Kaolin and Ball Clay Mining ".to_string(),
        }),
        (212325, Code::<u32> {
            code: 212325,
            parent_code: Some(21232),
            description: "Clay and Ceramic and Refractory Minerals Mining ".to_string(),
        }),
        (21239, Code::<u32> {
            code: 21239,
            parent_code: Some(2123),
            description: "Other Nonmetallic Mineral Mining and Quarrying".to_string(),
        }),
        (212391, Code::<u32> {
            code: 212391,
            parent_code: Some(21239),
            description: "Potash, Soda, and Borate Mineral Mining ".to_string(),
        }),
        (212392, Code::<u32> {
            code: 212392,
            parent_code: Some(21239),
            description: "Phosphate Rock Mining ".to_string(),
        }),
        (212393, Code::<u32> {
            code: 212393,
            parent_code: Some(21239),
            description: "Other Chemical and Fertilizer Mineral Mining ".to_string(),
        }),
        (212399, Code::<u32> {
            code: 212399,
            parent_code: Some(21239),
            description: "All Other Nonmetallic Mineral Mining ".to_string(),
        }),
        (213, Code::<u32> {
            code: 213,
            parent_code: Some(21),
            description: "Support Activities for Mining".to_string(),
        }),
        (2131, Code::<u32> {
            code: 2131,
            parent_code: Some(213),
            description: "Support Activities for Mining".to_string(),
        }),
        (21311, Code::<u32> {
            code: 21311,
            parent_code: Some(2131),
            description: "Support Activities for Mining".to_string(),
        }),
        (213111, Code::<u32> {
            code: 213111,
            parent_code: Some(21311),
            description: "Drilling Oil and Gas Wells".to_string(),
        }),
        (213112, Code::<u32> {
            code: 213112,
            parent_code: Some(21311),
            description: "Support Activities for Oil and Gas Operations ".to_string(),
        }),
        (213113, Code::<u32> {
            code: 213113,
            parent_code: Some(21311),
            description: "Support Activities for Coal Mining ".to_string(),
        }),
        (213114, Code::<u32> {
            code: 213114,
            parent_code: Some(21311),
            description: "Support Activities for Metal Mining ".to_string(),
        }),
        (213115, Code::<u32> {
            code: 213115,
            parent_code: Some(21311),
            description: "Support Activities for Nonmetallic Minerals (except Fuels) Mining ".to_string(),
        }),
        (22, Code::<u32> {
            code: 22,
            parent_code: None,
            description: "Utilities".to_string(),
        }),
        (221, Code::<u32> {
            code: 221,
            parent_code: Some(22),
            description: "Utilities ".to_string(),
        }),
        (2211, Code::<u32> {
            code: 2211,
            parent_code: Some(221),
            description: "Electric Power Generation, Transmission and Distribution".to_string(),
        }),
        (22111, Code::<u32> {
            code: 22111,
            parent_code: Some(2211),
            description: "Electric Power Generation ".to_string(),
        }),
        (221111, Code::<u32> {
            code: 221111,
            parent_code: Some(22111),
            description: "Hydroelectric Power Generation ".to_string(),
        }),
        (221112, Code::<u32> {
            code: 221112,
            parent_code: Some(22111),
            description: "Fossil Fuel Electric Power Generation ".to_string(),
        }),
        (221113, Code::<u32> {
            code: 221113,
            parent_code: Some(22111),
            description: "Nuclear Electric Power Generation ".to_string(),
        }),
        (221114, Code::<u32> {
            code: 221114,
            parent_code: Some(22111),
            description: "Solar Electric Power Generation ".to_string(),
        }),
        (221115, Code::<u32> {
            code: 221115,
            parent_code: Some(22111),
            description: "Wind Electric Power Generation ".to_string(),
        }),
        (221116, Code::<u32> {
            code: 221116,
            parent_code: Some(22111),
            description: "Geothermal Electric Power Generation ".to_string(),
        }),
        (221117, Code::<u32> {
            code: 221117,
            parent_code: Some(22111),
            description: "Biomass Electric Power Generation ".to_string(),
        }),
        (221118, Code::<u32> {
            code: 221118,
            parent_code: Some(22111),
            description: "Other Electric Power Generation ".to_string(),
        }),
        (22112, Code::<u32> {
            code: 22112,
            parent_code: Some(2211),
            description: "Electric Power Transmission, Control, and Distribution ".to_string(),
        }),
        (221121, Code::<u32> {
            code: 221121,
            parent_code: Some(22112),
            description: "Electric Bulk Power Transmission and Control ".to_string(),
        }),
        (221122, Code::<u32> {
            code: 221122,
            parent_code: Some(22112),
            description: "Electric Power Distribution ".to_string(),
        }),
        (2212, Code::<u32> {
            code: 2212,
            parent_code: Some(221),
            description: "Natural Gas Distribution ".to_string(),
        }),
        (22121, Code::<u32> {
            code: 22121,
            parent_code: Some(2212),
            description: "Natural Gas Distribution ".to_string(),
        }),
        (221210, Code::<u32> {
            code: 221210,
            parent_code: Some(22121),
            description: "Natural Gas Distribution ".to_string(),
        }),
        (2213, Code::<u32> {
            code: 2213,
            parent_code: Some(221),
            description: "Water, Sewage and Other Systems ".to_string(),
        }),
        (22131, Code::<u32> {
            code: 22131,
            parent_code: Some(2213),
            description: "Water Supply and Irrigation Systems ".to_string(),
        }),
        (221310, Code::<u32> {
            code: 221310,
            parent_code: Some(22131),
            description: "Water Supply and Irrigation Systems ".to_string(),
        }),
        (22132, Code::<u32> {
            code: 22132,
            parent_code: Some(2213),
            description: "Sewage Treatment Facilities ".to_string(),
        }),
        (221320, Code::<u32> {
            code: 221320,
            parent_code: Some(22132),
            description: "Sewage Treatment Facilities ".to_string(),
        }),
        (22133, Code::<u32> {
            code: 22133,
            parent_code: Some(2213),
            description: "Steam and Air-Conditioning Supply ".to_string(),
        }),
        (221330, Code::<u32> {
            code: 221330,
            parent_code: Some(22133),
            description: "Steam and Air-Conditioning Supply ".to_string(),
        }),
        (23, Code::<u32> {
            code: 23,
            parent_code: None,
            description: "Construction".to_string(),
        }),
        (236, Code::<u32> {
            code: 236,
            parent_code: Some(23),
            description: "Construction of Buildings".to_string(),
        }),
        (2361, Code::<u32> {
            code: 2361,
            parent_code: Some(236),
            description: "Residential Building Construction".to_string(),
        }),
        (23611, Code::<u32> {
            code: 23611,
            parent_code: Some(2361),
            description: "Residential Building Construction".to_string(),
        }),
        (236115, Code::<u32> {
            code: 236115,
            parent_code: Some(23611),
            description: "New Single-Family Housing Construction (except For-Sale Builders) ".to_string(),
        }),
        (236116, Code::<u32> {
            code: 236116,
            parent_code: Some(23611),
            description: "New Multifamily Housing Construction (except For-Sale Builders) ".to_string(),
        }),
        (236117, Code::<u32> {
            code: 236117,
            parent_code: Some(23611),
            description: "New Housing For-Sale Builders ".to_string(),
        }),
        (236118, Code::<u32> {
            code: 236118,
            parent_code: Some(23611),
            description: "Residential Remodelers ".to_string(),
        }),
        (2362, Code::<u32> {
            code: 2362,
            parent_code: Some(236),
            description: "Nonresidential Building Construction".to_string(),
        }),
        (23621, Code::<u32> {
            code: 23621,
            parent_code: Some(2362),
            description: "Industrial Building Construction".to_string(),
        }),
        (236210, Code::<u32> {
            code: 236210,
            parent_code: Some(23621),
            description: "Industrial Building Construction ".to_string(),
        }),
        (23622, Code::<u32> {
            code: 23622,
            parent_code: Some(2362),
            description: "Commercial and Institutional Building Construction".to_string(),
        }),
        (236220, Code::<u32> {
            code: 236220,
            parent_code: Some(23622),
            description: "Commercial and Institutional Building Construction ".to_string(),
        }),
        (237, Code::<u32> {
            code: 237,
            parent_code: Some(23),
            description: "Heavy and Civil Engineering Construction".to_string(),
        }),
        (2371, Code::<u32> {
            code: 2371,
            parent_code: Some(237),
            description: "Utility System Construction".to_string(),
        }),
        (23711, Code::<u32> {
            code: 23711,
            parent_code: Some(2371),
            description: "Water and Sewer Line and Related Structures Construction".to_string(),
        }),
        (237110, Code::<u32> {
            code: 237110,
            parent_code: Some(23711),
            description: "Water and Sewer Line and Related Structures Construction ".to_string(),
        }),
        (23712, Code::<u32> {
            code: 23712,
            parent_code: Some(2371),
            description: "Oil and Gas Pipeline and Related Structures Construction".to_string(),
        }),
        (237120, Code::<u32> {
            code: 237120,
            parent_code: Some(23712),
            description: "Oil and Gas Pipeline and Related Structures Construction ".to_string(),
        }),
        (23713, Code::<u32> {
            code: 23713,
            parent_code: Some(2371),
            description: "Power and Communication Line and Related Structures Construction".to_string(),
        }),
        (237130, Code::<u32> {
            code: 237130,
            parent_code: Some(23713),
            description: "Power and Communication Line and Related Structures Construction ".to_string(),
        }),
        (2372, Code::<u32> {
            code: 2372,
            parent_code: Some(237),
            description: "Land Subdivision".to_string(),
        }),
        (23721, Code::<u32> {
            code: 23721,
            parent_code: Some(2372),
            description: "Land Subdivision".to_string(),
        }),
        (237210, Code::<u32> {
            code: 237210,
            parent_code: Some(23721),
            description: "Land Subdivision ".to_string(),
        }),
        (2373, Code::<u32> {
            code: 2373,
            parent_code: Some(237),
            description: "Highway, Street, and Bridge Construction".to_string(),
        }),
        (23731, Code::<u32> {
            code: 23731,
            parent_code: Some(2373),
            description: "Highway, Street, and Bridge Construction".to_string(),
        }),
        (237310, Code::<u32> {
            code: 237310,
            parent_code: Some(23731),
            description: "Highway, Street, and Bridge Construction ".to_string(),
        }),
        (2379, Code::<u32> {
            code: 2379,
            parent_code: Some(237),
            description: "Other Heavy and Civil Engineering Construction".to_string(),
        }),
        (23799, Code::<u32> {
            code: 23799,
            parent_code: Some(2379),
            description: "Other Heavy and Civil Engineering Construction".to_string(),
        }),
        (237990, Code::<u32> {
            code: 237990,
            parent_code: Some(23799),
            description: "Other Heavy and Civil Engineering Construction ".to_string(),
        }),
        (238, Code::<u32> {
            code: 238,
            parent_code: Some(23),
            description: "Specialty Trade Contractors".to_string(),
        }),
        (2381, Code::<u32> {
            code: 2381,
            parent_code: Some(238),
            description: "Foundation, Structure, and Building Exterior Contractors".to_string(),
        }),
        (23811, Code::<u32> {
            code: 23811,
            parent_code: Some(2381),
            description: "Poured Concrete Foundation and Structure Contractors ".to_string(),
        }),
        (238110, Code::<u32> {
            code: 238110,
            parent_code: Some(23811),
            description: "Poured Concrete Foundation and Structure Contractors ".to_string(),
        }),
        (23812, Code::<u32> {
            code: 23812,
            parent_code: Some(2381),
            description: "Structural Steel and Precast Concrete Contractors ".to_string(),
        }),
        (238120, Code::<u32> {
            code: 238120,
            parent_code: Some(23812),
            description: "Structural Steel and Precast Concrete Contractors ".to_string(),
        }),
        (23813, Code::<u32> {
            code: 23813,
            parent_code: Some(2381),
            description: "Framing Contractors ".to_string(),
        }),
        (238130, Code::<u32> {
            code: 238130,
            parent_code: Some(23813),
            description: "Framing Contractors ".to_string(),
        }),
        (23814, Code::<u32> {
            code: 23814,
            parent_code: Some(2381),
            description: "Masonry Contractors ".to_string(),
        }),
        (238140, Code::<u32> {
            code: 238140,
            parent_code: Some(23814),
            description: "Masonry Contractors ".to_string(),
        }),
        (23815, Code::<u32> {
            code: 23815,
            parent_code: Some(2381),
            description: "Glass and Glazing Contractors ".to_string(),
        }),
        (238150, Code::<u32> {
            code: 238150,
            parent_code: Some(23815),
            description: "Glass and Glazing Contractors ".to_string(),
        }),
        (23816, Code::<u32> {
            code: 23816,
            parent_code: Some(2381),
            description: "Roofing Contractors ".to_string(),
        }),
        (238160, Code::<u32> {
            code: 238160,
            parent_code: Some(23816),
            description: "Roofing Contractors ".to_string(),
        }),
        (23817, Code::<u32> {
            code: 23817,
            parent_code: Some(2381),
            description: "Siding Contractors ".to_string(),
        }),
        (238170, Code::<u32> {
            code: 238170,
            parent_code: Some(23817),
            description: "Siding Contractors ".to_string(),
        }),
        (23819, Code::<u32> {
            code: 23819,
            parent_code: Some(2381),
            description: "Other Foundation, Structure, and Building Exterior Contractors ".to_string(),
        }),
        (238190, Code::<u32> {
            code: 238190,
            parent_code: Some(23819),
            description: "Other Foundation, Structure, and Building Exterior Contractors ".to_string(),
        }),
        (2382, Code::<u32> {
            code: 2382,
            parent_code: Some(238),
            description: "Building Equipment Contractors".to_string(),
        }),
        (23821, Code::<u32> {
            code: 23821,
            parent_code: Some(2382),
            description: "Electrical Contractors and Other Wiring Installation Contractors".to_string(),
        }),
        (238210, Code::<u32> {
            code: 238210,
            parent_code: Some(23821),
            description: "Electrical Contractors and Other Wiring Installation Contractors".to_string(),
        }),
        (23822, Code::<u32> {
            code: 23822,
            parent_code: Some(2382),
            description: "Plumbing, Heating, and Air-Conditioning Contractors".to_string(),
        }),
        (238220, Code::<u32> {
            code: 238220,
            parent_code: Some(23822),
            description: "Plumbing, Heating, and Air-Conditioning Contractors ".to_string(),
        }),
        (23829, Code::<u32> {
            code: 23829,
            parent_code: Some(2382),
            description: "Other Building Equipment Contractors".to_string(),
        }),
        (238290, Code::<u32> {
            code: 238290,
            parent_code: Some(23829),
            description: "Other Building Equipment Contractors ".to_string(),
        }),
        (2383, Code::<u32> {
            code: 2383,
            parent_code: Some(238),
            description: "Building Finishing Contractors".to_string(),
        }),
        (23831, Code::<u32> {
            code: 23831,
            parent_code: Some(2383),
            description: "Drywall and Insulation Contractors".to_string(),
        }),
        (238310, Code::<u32> {
            code: 238310,
            parent_code: Some(23831),
            description: "Drywall and Insulation Contractors ".to_string(),
        }),
        (23832, Code::<u32> {
            code: 23832,
            parent_code: Some(2383),
            description: "Painting and Wall Covering Contractors".to_string(),
        }),
        (238320, Code::<u32> {
            code: 238320,
            parent_code: Some(23832),
            description: "Painting and Wall Covering Contractors".to_string(),
        }),
        (23833, Code::<u32> {
            code: 23833,
            parent_code: Some(2383),
            description: "Flooring Contractors".to_string(),
        }),
        (238330, Code::<u32> {
            code: 238330,
            parent_code: Some(23833),
            description: "Flooring Contractors".to_string(),
        }),
        (23834, Code::<u32> {
            code: 23834,
            parent_code: Some(2383),
            description: "Tile and Terrazzo Contractors".to_string(),
        }),
        (238340, Code::<u32> {
            code: 238340,
            parent_code: Some(23834),
            description: "Tile and Terrazzo Contractors".to_string(),
        }),
        (23835, Code::<u32> {
            code: 23835,
            parent_code: Some(2383),
            description: "Finish Carpentry Contractors".to_string(),
        }),
        (238350, Code::<u32> {
            code: 238350,
            parent_code: Some(23835),
            description: "Finish Carpentry Contractors".to_string(),
        }),
        (23839, Code::<u32> {
            code: 23839,
            parent_code: Some(2383),
            description: "Other Building Finishing Contractors".to_string(),
        }),
        (238390, Code::<u32> {
            code: 238390,
            parent_code: Some(23839),
            description: "Other Building Finishing Contractors".to_string(),
        }),
        (2389, Code::<u32> {
            code: 2389,
            parent_code: Some(238),
            description: "Other Specialty Trade Contractors".to_string(),
        }),
        (23891, Code::<u32> {
            code: 23891,
            parent_code: Some(2389),
            description: "Site Preparation Contractors".to_string(),
        }),
        (238910, Code::<u32> {
            code: 238910,
            parent_code: Some(23891),
            description: "Site Preparation Contractors".to_string(),
        }),
        (23899, Code::<u32> {
            code: 23899,
            parent_code: Some(2389),
            description: "All Other Specialty Trade Contractors".to_string(),
        }),
        (238990, Code::<u32> {
            code: 238990,
            parent_code: Some(23899),
            description: "All Other Specialty Trade Contractors".to_string(),
        }),
        (31, Code::<u32> {
            code: 31,
            parent_code: None,
            description: "Manufacturing".to_string(),
        }),
        (32, Code::<u32> {
            code: 32,
            parent_code: None,
            description: "Manufacturing".to_string(),
        }),
        (33, Code::<u32> {
            code: 33,
            parent_code: None,
            description: "Manufacturing".to_string(),
        }),
        (311, Code::<u32> {
            code: 311,
            parent_code: Some(31),
            description: "Food Manufacturing".to_string(),
        }),
        (3111, Code::<u32> {
            code: 3111,
            parent_code: Some(311),
            description: "Animal Food Manufacturing".to_string(),
        }),
        (31111, Code::<u32> {
            code: 31111,
            parent_code: Some(3111),
            description: "Animal Food Manufacturing".to_string(),
        }),
        (311111, Code::<u32> {
            code: 311111,
            parent_code: Some(31111),
            description: "Dog and Cat Food Manufacturing ".to_string(),
        }),
        (311119, Code::<u32> {
            code: 311119,
            parent_code: Some(31111),
            description: "Other Animal Food Manufacturing ".to_string(),
        }),
        (3112, Code::<u32> {
            code: 3112,
            parent_code: Some(311),
            description: "Grain and Oilseed Milling".to_string(),
        }),
        (31121, Code::<u32> {
            code: 31121,
            parent_code: Some(3112),
            description: "Flour Milling and Malt Manufacturing".to_string(),
        }),
        (311211, Code::<u32> {
            code: 311211,
            parent_code: Some(31121),
            description: "Flour Milling ".to_string(),
        }),
        (311212, Code::<u32> {
            code: 311212,
            parent_code: Some(31121),
            description: "Rice Milling ".to_string(),
        }),
        (311213, Code::<u32> {
            code: 311213,
            parent_code: Some(31121),
            description: "Malt Manufacturing ".to_string(),
        }),
        (31122, Code::<u32> {
            code: 31122,
            parent_code: Some(3112),
            description: "Starch and Vegetable Fats and Oils Manufacturing".to_string(),
        }),
        (311221, Code::<u32> {
            code: 311221,
            parent_code: Some(31122),
            description: "Wet Corn Milling ".to_string(),
        }),
        (311224, Code::<u32> {
            code: 311224,
            parent_code: Some(31122),
            description: "Soybean and Other Oilseed Processing ".to_string(),
        }),
        (311225, Code::<u32> {
            code: 311225,
            parent_code: Some(31122),
            description: "Fats and Oils Refining and Blending ".to_string(),
        }),
        (31123, Code::<u32> {
            code: 31123,
            parent_code: Some(3112),
            description: "Breakfast Cereal Manufacturing".to_string(),
        }),
        (311230, Code::<u32> {
            code: 311230,
            parent_code: Some(31123),
            description: "Breakfast Cereal Manufacturing".to_string(),
        }),
        (3113, Code::<u32> {
            code: 3113,
            parent_code: Some(311),
            description: "Sugar and Confectionery Product Manufacturing".to_string(),
        }),
        (31131, Code::<u32> {
            code: 31131,
            parent_code: Some(3113),
            description: "Sugar Manufacturing".to_string(),
        }),
        (311313, Code::<u32> {
            code: 311313,
            parent_code: Some(31131),
            description: "Beet Sugar Manufacturing ".to_string(),
        }),
        (311314, Code::<u32> {
            code: 311314,
            parent_code: Some(31131),
            description: "Cane Sugar Manufacturing ".to_string(),
        }),
        (31134, Code::<u32> {
            code: 31134,
            parent_code: Some(3113),
            description: "Nonchocolate Confectionery Manufacturing".to_string(),
        }),
        (311340, Code::<u32> {
            code: 311340,
            parent_code: Some(31134),
            description: "Nonchocolate Confectionery Manufacturing".to_string(),
        }),
        (31135, Code::<u32> {
            code: 31135,
            parent_code: Some(3113),
            description: "Chocolate and Confectionery Manufacturing".to_string(),
        }),
        (311351, Code::<u32> {
            code: 311351,
            parent_code: Some(31135),
            description: "Chocolate and Confectionery Manufacturing from Cacao Beans ".to_string(),
        }),
        (311352, Code::<u32> {
            code: 311352,
            parent_code: Some(31135),
            description: "Confectionery Manufacturing from Purchased Chocolate ".to_string(),
        }),
        (3114, Code::<u32> {
            code: 3114,
            parent_code: Some(311),
            description: "Fruit and Vegetable Preserving and Specialty Food Manufacturing".to_string(),
        }),
        (31141, Code::<u32> {
            code: 31141,
            parent_code: Some(3114),
            description: "Frozen Food Manufacturing".to_string(),
        }),
        (311411, Code::<u32> {
            code: 311411,
            parent_code: Some(31141),
            description: "Frozen Fruit, Juice, and Vegetable Manufacturing ".to_string(),
        }),
        (311412, Code::<u32> {
            code: 311412,
            parent_code: Some(31141),
            description: "Frozen Specialty Food Manufacturing ".to_string(),
        }),
        (31142, Code::<u32> {
            code: 31142,
            parent_code: Some(3114),
            description: "Fruit and Vegetable Canning, Pickling, and Drying".to_string(),
        }),
        (311421, Code::<u32> {
            code: 311421,
            parent_code: Some(31142),
            description: "Fruit and Vegetable Canning ".to_string(),
        }),
        (311422, Code::<u32> {
            code: 311422,
            parent_code: Some(31142),
            description: "Specialty Canning ".to_string(),
        }),
        (311423, Code::<u32> {
            code: 311423,
            parent_code: Some(31142),
            description: "Dried and Dehydrated Food Manufacturing ".to_string(),
        }),
        (3115, Code::<u32> {
            code: 3115,
            parent_code: Some(311),
            description: "Dairy Product Manufacturing".to_string(),
        }),
        (31151, Code::<u32> {
            code: 31151,
            parent_code: Some(3115),
            description: "Dairy Product (except Frozen) Manufacturing".to_string(),
        }),
        (311511, Code::<u32> {
            code: 311511,
            parent_code: Some(31151),
            description: "Fluid Milk Manufacturing ".to_string(),
        }),
        (311512, Code::<u32> {
            code: 311512,
            parent_code: Some(31151),
            description: "Creamery Butter Manufacturing ".to_string(),
        }),
        (311513, Code::<u32> {
            code: 311513,
            parent_code: Some(31151),
            description: "Cheese Manufacturing ".to_string(),
        }),
        (311514, Code::<u32> {
            code: 311514,
            parent_code: Some(31151),
            description: "Dry, Condensed, and Evaporated Dairy Product Manufacturing ".to_string(),
        }),
        (31152, Code::<u32> {
            code: 31152,
            parent_code: Some(3115),
            description: "Ice Cream and Frozen Dessert Manufacturing".to_string(),
        }),
        (311520, Code::<u32> {
            code: 311520,
            parent_code: Some(31152),
            description: "Ice Cream and Frozen Dessert Manufacturing".to_string(),
        }),
        (3116, Code::<u32> {
            code: 3116,
            parent_code: Some(311),
            description: "Animal Slaughtering and Processing".to_string(),
        }),
        (31161, Code::<u32> {
            code: 31161,
            parent_code: Some(3116),
            description: "Animal Slaughtering and Processing".to_string(),
        }),
        (311611, Code::<u32> {
            code: 311611,
            parent_code: Some(31161),
            description: "Animal (except Poultry) Slaughtering ".to_string(),
        }),
        (311612, Code::<u32> {
            code: 311612,
            parent_code: Some(31161),
            description: "Meat Processed from Carcasses ".to_string(),
        }),
        (311613, Code::<u32> {
            code: 311613,
            parent_code: Some(31161),
            description: "Rendering and Meat Byproduct Processing ".to_string(),
        }),
        (311615, Code::<u32> {
            code: 311615,
            parent_code: Some(31161),
            description: "Poultry Processing ".to_string(),
        }),
        (3117, Code::<u32> {
            code: 3117,
            parent_code: Some(311),
            description: "Seafood Product Preparation and Packaging".to_string(),
        }),
        (31171, Code::<u32> {
            code: 31171,
            parent_code: Some(3117),
            description: "Seafood Product Preparation and Packaging".to_string(),
        }),
        (311710, Code::<u32> {
            code: 311710,
            parent_code: Some(31171),
            description: "Seafood Product Preparation and Packaging".to_string(),
        }),
        (3118, Code::<u32> {
            code: 3118,
            parent_code: Some(311),
            description: "Bakeries and Tortilla Manufacturing".to_string(),
        }),
        (31181, Code::<u32> {
            code: 31181,
            parent_code: Some(3118),
            description: "Bread and Bakery Product Manufacturing".to_string(),
        }),
        (311811, Code::<u32> {
            code: 311811,
            parent_code: Some(31181),
            description: "Retail Bakeries ".to_string(),
        }),
        (311812, Code::<u32> {
            code: 311812,
            parent_code: Some(31181),
            description: "Commercial Bakeries ".to_string(),
        }),
        (311813, Code::<u32> {
            code: 311813,
            parent_code: Some(31181),
            description: "Frozen Cakes, Pies, and Other Pastries Manufacturing ".to_string(),
        }),
        (31182, Code::<u32> {
            code: 31182,
            parent_code: Some(3118),
            description: "Cookie, Cracker, and Pasta Manufacturing".to_string(),
        }),
        (311821, Code::<u32> {
            code: 311821,
            parent_code: Some(31182),
            description: "Cookie and Cracker Manufacturing ".to_string(),
        }),
        (311824, Code::<u32> {
            code: 311824,
            parent_code: Some(31182),
            description: "Dry Pasta, Dough, and Flour Mixes Manufacturing from Purchased Flour ".to_string(),
        }),
        (31183, Code::<u32> {
            code: 31183,
            parent_code: Some(3118),
            description: "Tortilla Manufacturing".to_string(),
        }),
        (311830, Code::<u32> {
            code: 311830,
            parent_code: Some(31183),
            description: "Tortilla Manufacturing".to_string(),
        }),
        (3119, Code::<u32> {
            code: 3119,
            parent_code: Some(311),
            description: "Other Food Manufacturing".to_string(),
        }),
        (31191, Code::<u32> {
            code: 31191,
            parent_code: Some(3119),
            description: "Snack Food Manufacturing".to_string(),
        }),
        (311911, Code::<u32> {
            code: 311911,
            parent_code: Some(31191),
            description: "Roasted Nuts and Peanut Butter Manufacturing ".to_string(),
        }),
        (311919, Code::<u32> {
            code: 311919,
            parent_code: Some(31191),
            description: "Other Snack Food Manufacturing ".to_string(),
        }),
        (31192, Code::<u32> {
            code: 31192,
            parent_code: Some(3119),
            description: "Coffee and Tea Manufacturing".to_string(),
        }),
        (311920, Code::<u32> {
            code: 311920,
            parent_code: Some(31192),
            description: "Coffee and Tea Manufacturing ".to_string(),
        }),
        (31193, Code::<u32> {
            code: 31193,
            parent_code: Some(3119),
            description: "Flavoring Syrup and Concentrate Manufacturing".to_string(),
        }),
        (311930, Code::<u32> {
            code: 311930,
            parent_code: Some(31193),
            description: "Flavoring Syrup and Concentrate Manufacturing".to_string(),
        }),
        (31194, Code::<u32> {
            code: 31194,
            parent_code: Some(3119),
            description: "Seasoning and Dressing Manufacturing".to_string(),
        }),
        (311941, Code::<u32> {
            code: 311941,
            parent_code: Some(31194),
            description: "Mayonnaise, Dressing, and Other Prepared Sauce Manufacturing ".to_string(),
        }),
        (311942, Code::<u32> {
            code: 311942,
            parent_code: Some(31194),
            description: "Spice and Extract Manufacturing ".to_string(),
        }),
        (31199, Code::<u32> {
            code: 31199,
            parent_code: Some(3119),
            description: "All Other Food Manufacturing".to_string(),
        }),
        (311991, Code::<u32> {
            code: 311991,
            parent_code: Some(31199),
            description: "Perishable Prepared Food Manufacturing ".to_string(),
        }),
        (311999, Code::<u32> {
            code: 311999,
            parent_code: Some(31199),
            description: "All Other Miscellaneous Food Manufacturing ".to_string(),
        }),
        (312, Code::<u32> {
            code: 312,
            parent_code: Some(31),
            description: "Beverage and Tobacco Product Manufacturing".to_string(),
        }),
        (3121, Code::<u32> {
            code: 3121,
            parent_code: Some(312),
            description: "Beverage Manufacturing".to_string(),
        }),
        (31211, Code::<u32> {
            code: 31211,
            parent_code: Some(3121),
            description: "Soft Drink and Ice Manufacturing".to_string(),
        }),
        (312111, Code::<u32> {
            code: 312111,
            parent_code: Some(31211),
            description: "Soft Drink Manufacturing ".to_string(),
        }),
        (312112, Code::<u32> {
            code: 312112,
            parent_code: Some(31211),
            description: "Bottled Water Manufacturing ".to_string(),
        }),
        (312113, Code::<u32> {
            code: 312113,
            parent_code: Some(31211),
            description: "Ice Manufacturing ".to_string(),
        }),
        (31212, Code::<u32> {
            code: 31212,
            parent_code: Some(3121),
            description: "Breweries".to_string(),
        }),
        (312120, Code::<u32> {
            code: 312120,
            parent_code: Some(31212),
            description: "Breweries".to_string(),
        }),
        (31213, Code::<u32> {
            code: 31213,
            parent_code: Some(3121),
            description: "Wineries".to_string(),
        }),
        (312130, Code::<u32> {
            code: 312130,
            parent_code: Some(31213),
            description: "Wineries ".to_string(),
        }),
        (31214, Code::<u32> {
            code: 31214,
            parent_code: Some(3121),
            description: "Distilleries".to_string(),
        }),
        (312140, Code::<u32> {
            code: 312140,
            parent_code: Some(31214),
            description: "Distilleries ".to_string(),
        }),
        (3122, Code::<u32> {
            code: 3122,
            parent_code: Some(312),
            description: "Tobacco Manufacturing".to_string(),
        }),
        (31223, Code::<u32> {
            code: 31223,
            parent_code: Some(3122),
            description: "Tobacco Manufacturing".to_string(),
        }),
        (312230, Code::<u32> {
            code: 312230,
            parent_code: Some(31223),
            description: "Tobacco Manufacturing ".to_string(),
        }),
        (313, Code::<u32> {
            code: 313,
            parent_code: Some(31),
            description: "Textile Mills".to_string(),
        }),
        (3131, Code::<u32> {
            code: 3131,
            parent_code: Some(313),
            description: "Fiber, Yarn, and Thread Mills".to_string(),
        }),
        (31311, Code::<u32> {
            code: 31311,
            parent_code: Some(3131),
            description: "Fiber, Yarn, and Thread Mills".to_string(),
        }),
        (313110, Code::<u32> {
            code: 313110,
            parent_code: Some(31311),
            description: "Fiber, Yarn, and Thread Mills ".to_string(),
        }),
        (3132, Code::<u32> {
            code: 3132,
            parent_code: Some(313),
            description: "Fabric Mills".to_string(),
        }),
        (31321, Code::<u32> {
            code: 31321,
            parent_code: Some(3132),
            description: "Broadwoven Fabric Mills".to_string(),
        }),
        (313210, Code::<u32> {
            code: 313210,
            parent_code: Some(31321),
            description: "Broadwoven Fabric Mills".to_string(),
        }),
        (31322, Code::<u32> {
            code: 31322,
            parent_code: Some(3132),
            description: "Narrow Fabric Mills and Schiffli Machine Embroidery".to_string(),
        }),
        (313220, Code::<u32> {
            code: 313220,
            parent_code: Some(31322),
            description: "Narrow Fabric Mills and Schiffli Machine Embroidery".to_string(),
        }),
        (31323, Code::<u32> {
            code: 31323,
            parent_code: Some(3132),
            description: "Nonwoven Fabric Mills".to_string(),
        }),
        (313230, Code::<u32> {
            code: 313230,
            parent_code: Some(31323),
            description: "Nonwoven Fabric Mills".to_string(),
        }),
        (31324, Code::<u32> {
            code: 31324,
            parent_code: Some(3132),
            description: "Knit Fabric Mills".to_string(),
        }),
        (313240, Code::<u32> {
            code: 313240,
            parent_code: Some(31324),
            description: "Knit Fabric Mills".to_string(),
        }),
        (3133, Code::<u32> {
            code: 3133,
            parent_code: Some(313),
            description: "Textile and Fabric Finishing and Fabric Coating Mills".to_string(),
        }),
        (31331, Code::<u32> {
            code: 31331,
            parent_code: Some(3133),
            description: "Textile and Fabric Finishing Mills".to_string(),
        }),
        (313310, Code::<u32> {
            code: 313310,
            parent_code: Some(31331),
            description: "Textile and Fabric Finishing Mills ".to_string(),
        }),
        (31332, Code::<u32> {
            code: 31332,
            parent_code: Some(3133),
            description: "Fabric Coating Mills".to_string(),
        }),
        (313320, Code::<u32> {
            code: 313320,
            parent_code: Some(31332),
            description: "Fabric Coating Mills".to_string(),
        }),
        (314, Code::<u32> {
            code: 314,
            parent_code: Some(31),
            description: "Textile Product Mills".to_string(),
        }),
        (3141, Code::<u32> {
            code: 3141,
            parent_code: Some(314),
            description: "Textile Furnishings Mills".to_string(),
        }),
        (31411, Code::<u32> {
            code: 31411,
            parent_code: Some(3141),
            description: "Carpet and Rug Mills".to_string(),
        }),
        (314110, Code::<u32> {
            code: 314110,
            parent_code: Some(31411),
            description: "Carpet and Rug Mills".to_string(),
        }),
        (31412, Code::<u32> {
            code: 31412,
            parent_code: Some(3141),
            description: "Curtain and Linen Mills".to_string(),
        }),
        (314120, Code::<u32> {
            code: 314120,
            parent_code: Some(31412),
            description: "Curtain and Linen Mills".to_string(),
        }),
        (3149, Code::<u32> {
            code: 3149,
            parent_code: Some(314),
            description: "Other Textile Product Mills".to_string(),
        }),
        (31491, Code::<u32> {
            code: 31491,
            parent_code: Some(3149),
            description: "Textile Bag and Canvas Mills".to_string(),
        }),
        (314910, Code::<u32> {
            code: 314910,
            parent_code: Some(31491),
            description: "Textile Bag and Canvas Mills ".to_string(),
        }),
        (31499, Code::<u32> {
            code: 31499,
            parent_code: Some(3149),
            description: "All Other Textile Product Mills".to_string(),
        }),
        (314994, Code::<u32> {
            code: 314994,
            parent_code: Some(31499),
            description: "Rope, Cordage, Twine, Tire Cord, and Tire Fabric Mills ".to_string(),
        }),
        (314999, Code::<u32> {
            code: 314999,
            parent_code: Some(31499),
            description: "All Other Miscellaneous Textile Product Mills ".to_string(),
        }),
        (315, Code::<u32> {
            code: 315,
            parent_code: Some(31),
            description: "Apparel Manufacturing".to_string(),
        }),
        (3151, Code::<u32> {
            code: 3151,
            parent_code: Some(315),
            description: "Apparel Knitting Mills".to_string(),
        }),
        (31511, Code::<u32> {
            code: 31511,
            parent_code: Some(3151),
            description: "Hosiery and Sock Mills".to_string(),
        }),
        (315110, Code::<u32> {
            code: 315110,
            parent_code: Some(31511),
            description: "Hosiery and Sock Mills".to_string(),
        }),
        (31519, Code::<u32> {
            code: 31519,
            parent_code: Some(3151),
            description: "Other Apparel Knitting Mills".to_string(),
        }),
        (315190, Code::<u32> {
            code: 315190,
            parent_code: Some(31519),
            description: "Other Apparel Knitting Mills ".to_string(),
        }),
        (3152, Code::<u32> {
            code: 3152,
            parent_code: Some(315),
            description: "Cut and Sew Apparel Manufacturing".to_string(),
        }),
        (31521, Code::<u32> {
            code: 31521,
            parent_code: Some(3152),
            description: "Cut and Sew Apparel Contractors ".to_string(),
        }),
        (315210, Code::<u32> {
            code: 315210,
            parent_code: Some(31521),
            description: "Cut and Sew Apparel Contractors ".to_string(),
        }),
        (31522, Code::<u32> {
            code: 31522,
            parent_code: Some(3152),
            description: "Men’s and Boys’ Cut and Sew Apparel Manufacturing ".to_string(),
        }),
        (315220, Code::<u32> {
            code: 315220,
            parent_code: Some(31522),
            description: "Men’s and Boys’ Cut and Sew Apparel Manufacturing ".to_string(),
        }),
        (31524, Code::<u32> {
            code: 31524,
            parent_code: Some(3152),
            description: "Women’s, Girls’, and Infants’ Cut and Sew Apparel Manufacturing".to_string(),
        }),
        (315240, Code::<u32> {
            code: 315240,
            parent_code: Some(31524),
            description: "Women’s, Girls’, and Infants’ Cut and Sew Apparel Manufacturing ".to_string(),
        }),
        (31528, Code::<u32> {
            code: 31528,
            parent_code: Some(3152),
            description: "Other Cut and Sew Apparel Manufacturing ".to_string(),
        }),
        (315280, Code::<u32> {
            code: 315280,
            parent_code: Some(31528),
            description: "Other Cut and Sew Apparel Manufacturing ".to_string(),
        }),
        (3159, Code::<u32> {
            code: 3159,
            parent_code: Some(315),
            description: "Apparel Accessories and Other Apparel Manufacturing".to_string(),
        }),
        (31599, Code::<u32> {
            code: 31599,
            parent_code: Some(3159),
            description: "Apparel Accessories and Other Apparel Manufacturing".to_string(),
        }),
        (315990, Code::<u32> {
            code: 315990,
            parent_code: Some(31599),
            description: "Apparel Accessories and Other Apparel Manufacturing ".to_string(),
        }),
        (316, Code::<u32> {
            code: 316,
            parent_code: Some(31),
            description: "Leather and Allied Product Manufacturing".to_string(),
        }),
        (3161, Code::<u32> {
            code: 3161,
            parent_code: Some(316),
            description: "Leather and Hide Tanning and Finishing".to_string(),
        }),
        (31611, Code::<u32> {
            code: 31611,
            parent_code: Some(3161),
            description: "Leather and Hide Tanning and Finishing".to_string(),
        }),
        (316110, Code::<u32> {
            code: 316110,
            parent_code: Some(31611),
            description: "Leather and Hide Tanning and Finishing".to_string(),
        }),
        (3162, Code::<u32> {
            code: 3162,
            parent_code: Some(316),
            description: "Footwear Manufacturing".to_string(),
        }),
        (31621, Code::<u32> {
            code: 31621,
            parent_code: Some(3162),
            description: "Footwear Manufacturing".to_string(),
        }),
        (316210, Code::<u32> {
            code: 316210,
            parent_code: Some(31621),
            description: "Footwear Manufacturing ".to_string(),
        }),
        (3169, Code::<u32> {
            code: 3169,
            parent_code: Some(316),
            description: "Other Leather and Allied Product Manufacturing".to_string(),
        }),
        (31699, Code::<u32> {
            code: 31699,
            parent_code: Some(3169),
            description: "Other Leather and Allied Product Manufacturing".to_string(),
        }),
        (316992, Code::<u32> {
            code: 316992,
            parent_code: Some(31699),
            description: "Women's Handbag and Purse Manufacturing ".to_string(),
        }),
        (316998, Code::<u32> {
            code: 316998,
            parent_code: Some(31699),
            description: "All Other Leather Good and Allied Product Manufacturing ".to_string(),
        }),
        (321, Code::<u32> {
            code: 321,
            parent_code: Some(32),
            description: "Wood Product Manufacturing".to_string(),
        }),
        (3211, Code::<u32> {
            code: 3211,
            parent_code: Some(321),
            description: "Sawmills and Wood Preservation".to_string(),
        }),
        (32111, Code::<u32> {
            code: 32111,
            parent_code: Some(3211),
            description: "Sawmills and Wood Preservation".to_string(),
        }),
        (321113, Code::<u32> {
            code: 321113,
            parent_code: Some(32111),
            description: "Sawmills ".to_string(),
        }),
        (321114, Code::<u32> {
            code: 321114,
            parent_code: Some(32111),
            description: "Wood Preservation ".to_string(),
        }),
        (3212, Code::<u32> {
            code: 3212,
            parent_code: Some(321),
            description: "Veneer, Plywood, and Engineered Wood Product Manufacturing".to_string(),
        }),
        (32121, Code::<u32> {
            code: 32121,
            parent_code: Some(3212),
            description: "Veneer, Plywood, and Engineered Wood Product Manufacturing".to_string(),
        }),
        (321211, Code::<u32> {
            code: 321211,
            parent_code: Some(32121),
            description: "Hardwood Veneer and Plywood Manufacturing ".to_string(),
        }),
        (321212, Code::<u32> {
            code: 321212,
            parent_code: Some(32121),
            description: "Softwood Veneer and Plywood Manufacturing ".to_string(),
        }),
        (321213, Code::<u32> {
            code: 321213,
            parent_code: Some(32121),
            description: "Engineered Wood Member (except Truss) Manufacturing ".to_string(),
        }),
        (321214, Code::<u32> {
            code: 321214,
            parent_code: Some(32121),
            description: "Truss Manufacturing ".to_string(),
        }),
        (321219, Code::<u32> {
            code: 321219,
            parent_code: Some(32121),
            description: "Reconstituted Wood Product Manufacturing ".to_string(),
        }),
        (3219, Code::<u32> {
            code: 3219,
            parent_code: Some(321),
            description: "Other Wood Product Manufacturing".to_string(),
        }),
        (32191, Code::<u32> {
            code: 32191,
            parent_code: Some(3219),
            description: "Millwork".to_string(),
        }),
        (321911, Code::<u32> {
            code: 321911,
            parent_code: Some(32191),
            description: "Wood Window and Door Manufacturing ".to_string(),
        }),
        (321912, Code::<u32> {
            code: 321912,
            parent_code: Some(32191),
            description: "Cut Stock, Resawing Lumber, and Planing ".to_string(),
        }),
        (321918, Code::<u32> {
            code: 321918,
            parent_code: Some(32191),
            description: "Other Millwork (including Flooring) ".to_string(),
        }),
        (32192, Code::<u32> {
            code: 32192,
            parent_code: Some(3219),
            description: "Wood Container and Pallet Manufacturing".to_string(),
        }),
        (321920, Code::<u32> {
            code: 321920,
            parent_code: Some(32192),
            description: "Wood Container and Pallet Manufacturing".to_string(),
        }),
        (32199, Code::<u32> {
            code: 32199,
            parent_code: Some(3219),
            description: "All Other Wood Product Manufacturing".to_string(),
        }),
        (321991, Code::<u32> {
            code: 321991,
            parent_code: Some(32199),
            description: "Manufactured Home (Mobile Home) Manufacturing ".to_string(),
        }),
        (321992, Code::<u32> {
            code: 321992,
            parent_code: Some(32199),
            description: "Prefabricated Wood Building Manufacturing ".to_string(),
        }),
        (321999, Code::<u32> {
            code: 321999,
            parent_code: Some(32199),
            description: "All Other Miscellaneous Wood Product Manufacturing ".to_string(),
        }),
        (322, Code::<u32> {
            code: 322,
            parent_code: Some(32),
            description: "Paper Manufacturing".to_string(),
        }),
        (3221, Code::<u32> {
            code: 3221,
            parent_code: Some(322),
            description: "Pulp, Paper, and Paperboard Mills".to_string(),
        }),
        (32211, Code::<u32> {
            code: 32211,
            parent_code: Some(3221),
            description: "Pulp Mills".to_string(),
        }),
        (322110, Code::<u32> {
            code: 322110,
            parent_code: Some(32211),
            description: "Pulp Mills ".to_string(),
        }),
        (32212, Code::<u32> {
            code: 32212,
            parent_code: Some(3221),
            description: "Paper Mills".to_string(),
        }),
        (322121, Code::<u32> {
            code: 322121,
            parent_code: Some(32212),
            description: "Paper (except Newsprint) Mills ".to_string(),
        }),
        (322122, Code::<u32> {
            code: 322122,
            parent_code: Some(32212),
            description: "Newsprint Mills ".to_string(),
        }),
        (32213, Code::<u32> {
            code: 32213,
            parent_code: Some(3221),
            description: "Paperboard Mills".to_string(),
        }),
        (322130, Code::<u32> {
            code: 322130,
            parent_code: Some(32213),
            description: "Paperboard Mills ".to_string(),
        }),
        (3222, Code::<u32> {
            code: 3222,
            parent_code: Some(322),
            description: "Converted Paper Product Manufacturing".to_string(),
        }),
        (32221, Code::<u32> {
            code: 32221,
            parent_code: Some(3222),
            description: "Paperboard Container Manufacturing".to_string(),
        }),
        (322211, Code::<u32> {
            code: 322211,
            parent_code: Some(32221),
            description: "Corrugated and Solid Fiber Box Manufacturing ".to_string(),
        }),
        (322212, Code::<u32> {
            code: 322212,
            parent_code: Some(32221),
            description: "Folding Paperboard Box Manufacturing ".to_string(),
        }),
        (322219, Code::<u32> {
            code: 322219,
            parent_code: Some(32221),
            description: "Other Paperboard Container Manufacturing ".to_string(),
        }),
        (32222, Code::<u32> {
            code: 32222,
            parent_code: Some(3222),
            description: "Paper Bag and Coated and Treated Paper Manufacturing".to_string(),
        }),
        (322220, Code::<u32> {
            code: 322220,
            parent_code: Some(32222),
            description: "Paper Bag and Coated and Treated Paper Manufacturing".to_string(),
        }),
        (32223, Code::<u32> {
            code: 32223,
            parent_code: Some(3222),
            description: "Stationery Product Manufacturing".to_string(),
        }),
        (322230, Code::<u32> {
            code: 322230,
            parent_code: Some(32223),
            description: "Stationery Product Manufacturing".to_string(),
        }),
        (32229, Code::<u32> {
            code: 32229,
            parent_code: Some(3222),
            description: "Other Converted Paper Product Manufacturing".to_string(),
        }),
        (322291, Code::<u32> {
            code: 322291,
            parent_code: Some(32229),
            description: "Sanitary Paper Product Manufacturing ".to_string(),
        }),
        (322299, Code::<u32> {
            code: 322299,
            parent_code: Some(32229),
            description: "All Other Converted Paper Product Manufacturing ".to_string(),
        }),
        (323, Code::<u32> {
            code: 323,
            parent_code: Some(32),
            description: "Printing and Related Support Activities".to_string(),
        }),
        (3231, Code::<u32> {
            code: 3231,
            parent_code: Some(323),
            description: "Printing and Related Support Activities".to_string(),
        }),
        (32311, Code::<u32> {
            code: 32311,
            parent_code: Some(3231),
            description: "Printing".to_string(),
        }),
        (323111, Code::<u32> {
            code: 323111,
            parent_code: Some(32311),
            description: "Commercial Printing (except Screen and Books) ".to_string(),
        }),
        (323113, Code::<u32> {
            code: 323113,
            parent_code: Some(32311),
            description: "Commercial Screen Printing ".to_string(),
        }),
        (323117, Code::<u32> {
            code: 323117,
            parent_code: Some(32311),
            description: "Books Printing ".to_string(),
        }),
        (32312, Code::<u32> {
            code: 32312,
            parent_code: Some(3231),
            description: "Support Activities for Printing".to_string(),
        }),
        (323120, Code::<u32> {
            code: 323120,
            parent_code: Some(32312),
            description: "Support Activities for Printing".to_string(),
        }),
        (324, Code::<u32> {
            code: 324,
            parent_code: Some(32),
            description: "Petroleum and Coal Products Manufacturing".to_string(),
        }),
        (3241, Code::<u32> {
            code: 3241,
            parent_code: Some(324),
            description: "Petroleum and Coal Products Manufacturing".to_string(),
        }),
        (32411, Code::<u32> {
            code: 32411,
            parent_code: Some(3241),
            description: "Petroleum Refineries".to_string(),
        }),
        (324110, Code::<u32> {
            code: 324110,
            parent_code: Some(32411),
            description: "Petroleum Refineries".to_string(),
        }),
        (32412, Code::<u32> {
            code: 32412,
            parent_code: Some(3241),
            description: "Asphalt Paving, Roofing, and Saturated Materials Manufacturing".to_string(),
        }),
        (324121, Code::<u32> {
            code: 324121,
            parent_code: Some(32412),
            description: "Asphalt Paving Mixture and Block Manufacturing ".to_string(),
        }),
        (324122, Code::<u32> {
            code: 324122,
            parent_code: Some(32412),
            description: "Asphalt Shingle and Coating Materials Manufacturing ".to_string(),
        }),
        (32419, Code::<u32> {
            code: 32419,
            parent_code: Some(3241),
            description: "Other Petroleum and Coal Products Manufacturing".to_string(),
        }),
        (324191, Code::<u32> {
            code: 324191,
            parent_code: Some(32419),
            description: "Petroleum Lubricating Oil and Grease Manufacturing ".to_string(),
        }),
        (324199, Code::<u32> {
            code: 324199,
            parent_code: Some(32419),
            description: "All Other Petroleum and Coal Products Manufacturing ".to_string(),
        }),
        (325, Code::<u32> {
            code: 325,
            parent_code: Some(32),
            description: "Chemical Manufacturing".to_string(),
        }),
        (3251, Code::<u32> {
            code: 3251,
            parent_code: Some(325),
            description: "Basic Chemical Manufacturing".to_string(),
        }),
        (32511, Code::<u32> {
            code: 32511,
            parent_code: Some(3251),
            description: "Petrochemical Manufacturing".to_string(),
        }),
        (325110, Code::<u32> {
            code: 325110,
            parent_code: Some(32511),
            description: "Petrochemical Manufacturing".to_string(),
        }),
        (32512, Code::<u32> {
            code: 32512,
            parent_code: Some(3251),
            description: "Industrial Gas Manufacturing".to_string(),
        }),
        (325120, Code::<u32> {
            code: 325120,
            parent_code: Some(32512),
            description: "Industrial Gas Manufacturing".to_string(),
        }),
        (32513, Code::<u32> {
            code: 32513,
            parent_code: Some(3251),
            description: "Synthetic Dye and Pigment Manufacturing".to_string(),
        }),
        (325130, Code::<u32> {
            code: 325130,
            parent_code: Some(32513),
            description: "Synthetic Dye and Pigment Manufacturing".to_string(),
        }),
        (32518, Code::<u32> {
            code: 32518,
            parent_code: Some(3251),
            description: "Other Basic Inorganic Chemical Manufacturing".to_string(),
        }),
        (325180, Code::<u32> {
            code: 325180,
            parent_code: Some(32518),
            description: "Other Basic Inorganic Chemical Manufacturing ".to_string(),
        }),
        (32519, Code::<u32> {
            code: 32519,
            parent_code: Some(3251),
            description: "Other Basic Organic Chemical Manufacturing".to_string(),
        }),
        (325193, Code::<u32> {
            code: 325193,
            parent_code: Some(32519),
            description: "Ethyl Alcohol Manufacturing ".to_string(),
        }),
        (325194, Code::<u32> {
            code: 325194,
            parent_code: Some(32519),
            description: "Cyclic Crude, Intermediate, and Gum and Wood Chemical Manufacturing ".to_string(),
        }),
        (325199, Code::<u32> {
            code: 325199,
            parent_code: Some(32519),
            description: "All Other Basic Organic Chemical Manufacturing ".to_string(),
        }),
        (3252, Code::<u32> {
            code: 3252,
            parent_code: Some(325),
            description: "Resin, Synthetic Rubber, and Artificial and Synthetic Fibers and Filaments Manufacturing".to_string(),
        }),
        (32521, Code::<u32> {
            code: 32521,
            parent_code: Some(3252),
            description: "Resin and Synthetic Rubber Manufacturing".to_string(),
        }),
        (325211, Code::<u32> {
            code: 325211,
            parent_code: Some(32521),
            description: "Plastics Material and Resin Manufacturing ".to_string(),
        }),
        (325212, Code::<u32> {
            code: 325212,
            parent_code: Some(32521),
            description: "Synthetic Rubber Manufacturing ".to_string(),
        }),
        (32522, Code::<u32> {
            code: 32522,
            parent_code: Some(3252),
            description: "Artificial and Synthetic Fibers and Filaments Manufacturing".to_string(),
        }),
        (325220, Code::<u32> {
            code: 325220,
            parent_code: Some(32522),
            description: "Artificial and Synthetic Fibers and Filaments Manufacturing".to_string(),
        }),
        (3253, Code::<u32> {
            code: 3253,
            parent_code: Some(325),
            description: "Pesticide, Fertilizer, and Other Agricultural Chemical Manufacturing".to_string(),
        }),
        (32531, Code::<u32> {
            code: 32531,
            parent_code: Some(3253),
            description: "Fertilizer Manufacturing".to_string(),
        }),
        (325311, Code::<u32> {
            code: 325311,
            parent_code: Some(32531),
            description: "Nitrogenous Fertilizer Manufacturing ".to_string(),
        }),
        (325312, Code::<u32> {
            code: 325312,
            parent_code: Some(32531),
            description: "Phosphatic Fertilizer Manufacturing ".to_string(),
        }),
        (325314, Code::<u32> {
            code: 325314,
            parent_code: Some(32531),
            description: "Fertilizer (Mixing Only) Manufacturing ".to_string(),
        }),
        (32532, Code::<u32> {
            code: 32532,
            parent_code: Some(3253),
            description: "Pesticide and Other Agricultural Chemical Manufacturing".to_string(),
        }),
        (325320, Code::<u32> {
            code: 325320,
            parent_code: Some(32532),
            description: "Pesticide and Other Agricultural Chemical Manufacturing".to_string(),
        }),
        (3254, Code::<u32> {
            code: 3254,
            parent_code: Some(325),
            description: "Pharmaceutical and Medicine Manufacturing".to_string(),
        }),
        (32541, Code::<u32> {
            code: 32541,
            parent_code: Some(3254),
            description: "Pharmaceutical and Medicine Manufacturing".to_string(),
        }),
        (325411, Code::<u32> {
            code: 325411,
            parent_code: Some(32541),
            description: "Medicinal and Botanical Manufacturing ".to_string(),
        }),
        (325412, Code::<u32> {
            code: 325412,
            parent_code: Some(32541),
            description: "Pharmaceutical Preparation Manufacturing ".to_string(),
        }),
        (325413, Code::<u32> {
            code: 325413,
            parent_code: Some(32541),
            description: "In-Vitro Diagnostic Substance Manufacturing ".to_string(),
        }),
        (325414, Code::<u32> {
            code: 325414,
            parent_code: Some(32541),
            description: "Biological Product (except Diagnostic) Manufacturing ".to_string(),
        }),
        (3255, Code::<u32> {
            code: 3255,
            parent_code: Some(325),
            description: "Paint, Coating, and Adhesive Manufacturing".to_string(),
        }),
        (32551, Code::<u32> {
            code: 32551,
            parent_code: Some(3255),
            description: "Paint and Coating Manufacturing".to_string(),
        }),
        (325510, Code::<u32> {
            code: 325510,
            parent_code: Some(32551),
            description: "Paint and Coating Manufacturing".to_string(),
        }),
        (32552, Code::<u32> {
            code: 32552,
            parent_code: Some(3255),
            description: "Adhesive Manufacturing".to_string(),
        }),
        (325520, Code::<u32> {
            code: 325520,
            parent_code: Some(32552),
            description: "Adhesive Manufacturing".to_string(),
        }),
        (3256, Code::<u32> {
            code: 3256,
            parent_code: Some(325),
            description: "Soap, Cleaning Compound, and Toilet Preparation Manufacturing".to_string(),
        }),
        (32561, Code::<u32> {
            code: 32561,
            parent_code: Some(3256),
            description: "Soap and Cleaning Compound Manufacturing".to_string(),
        }),
        (325611, Code::<u32> {
            code: 325611,
            parent_code: Some(32561),
            description: "Soap and Other Detergent Manufacturing ".to_string(),
        }),
        (325612, Code::<u32> {
            code: 325612,
            parent_code: Some(32561),
            description: "Polish and Other Sanitation Good Manufacturing ".to_string(),
        }),
        (325613, Code::<u32> {
            code: 325613,
            parent_code: Some(32561),
            description: "Surface Active Agent Manufacturing ".to_string(),
        }),
        (32562, Code::<u32> {
            code: 32562,
            parent_code: Some(3256),
            description: "Toilet Preparation Manufacturing".to_string(),
        }),
        (325620, Code::<u32> {
            code: 325620,
            parent_code: Some(32562),
            description: "Toilet Preparation Manufacturing".to_string(),
        }),
        (3259, Code::<u32> {
            code: 3259,
            parent_code: Some(325),
            description: "Other Chemical Product and Preparation Manufacturing".to_string(),
        }),
        (32591, Code::<u32> {
            code: 32591,
            parent_code: Some(3259),
            description: "Printing Ink Manufacturing".to_string(),
        }),
        (325910, Code::<u32> {
            code: 325910,
            parent_code: Some(32591),
            description: "Printing Ink Manufacturing".to_string(),
        }),
        (32592, Code::<u32> {
            code: 32592,
            parent_code: Some(3259),
            description: "Explosives Manufacturing".to_string(),
        }),
        (325920, Code::<u32> {
            code: 325920,
            parent_code: Some(32592),
            description: "Explosives Manufacturing".to_string(),
        }),
        (32599, Code::<u32> {
            code: 32599,
            parent_code: Some(3259),
            description: "All Other Chemical Product and Preparation Manufacturing".to_string(),
        }),
        (325991, Code::<u32> {
            code: 325991,
            parent_code: Some(32599),
            description: "Custom Compounding of Purchased Resins ".to_string(),
        }),
        (325992, Code::<u32> {
            code: 325992,
            parent_code: Some(32599),
            description: "Photographic Film, Paper, Plate, and Chemical Manufacturing ".to_string(),
        }),
        (325998, Code::<u32> {
            code: 325998,
            parent_code: Some(32599),
            description: "All Other Miscellaneous Chemical Product and Preparation Manufacturing ".to_string(),
        }),
        (326, Code::<u32> {
            code: 326,
            parent_code: Some(32),
            description: "Plastics and Rubber Products Manufacturing".to_string(),
        }),
        (3261, Code::<u32> {
            code: 3261,
            parent_code: Some(326),
            description: "Plastics Product Manufacturing".to_string(),
        }),
        (32611, Code::<u32> {
            code: 32611,
            parent_code: Some(3261),
            description: "Plastics Packaging Materials and Unlaminated Film and Sheet Manufacturing".to_string(),
        }),
        (326111, Code::<u32> {
            code: 326111,
            parent_code: Some(32611),
            description: "Plastics Bag and Pouch Manufacturing ".to_string(),
        }),
        (326112, Code::<u32> {
            code: 326112,
            parent_code: Some(32611),
            description: "Plastics Packaging Film and Sheet (including Laminated) Manufacturing ".to_string(),
        }),
        (326113, Code::<u32> {
            code: 326113,
            parent_code: Some(32611),
            description: "Unlaminated Plastics Film and Sheet (except Packaging) Manufacturing ".to_string(),
        }),
        (32612, Code::<u32> {
            code: 32612,
            parent_code: Some(3261),
            description: "Plastics Pipe, Pipe Fitting, and Unlaminated Profile Shape Manufacturing".to_string(),
        }),
        (326121, Code::<u32> {
            code: 326121,
            parent_code: Some(32612),
            description: "Unlaminated Plastics Profile Shape Manufacturing ".to_string(),
        }),
        (326122, Code::<u32> {
            code: 326122,
            parent_code: Some(32612),
            description: "Plastics Pipe and Pipe Fitting Manufacturing ".to_string(),
        }),
        (32613, Code::<u32> {
            code: 32613,
            parent_code: Some(3261),
            description: "Laminated Plastics Plate, Sheet (except Packaging), and Shape Manufacturing".to_string(),
        }),
        (326130, Code::<u32> {
            code: 326130,
            parent_code: Some(32613),
            description: "Laminated Plastics Plate, Sheet (except Packaging), and Shape Manufacturing".to_string(),
        }),
        (32614, Code::<u32> {
            code: 32614,
            parent_code: Some(3261),
            description: "Polystyrene Foam Product Manufacturing".to_string(),
        }),
        (326140, Code::<u32> {
            code: 326140,
            parent_code: Some(32614),
            description: "Polystyrene Foam Product Manufacturing".to_string(),
        }),
        (32615, Code::<u32> {
            code: 32615,
            parent_code: Some(3261),
            description: "Urethane and Other Foam Product (except Polystyrene) Manufacturing".to_string(),
        }),
        (326150, Code::<u32> {
            code: 326150,
            parent_code: Some(32615),
            description: "Urethane and Other Foam Product (except Polystyrene) Manufacturing".to_string(),
        }),
        (32616, Code::<u32> {
            code: 32616,
            parent_code: Some(3261),
            description: "Plastics Bottle Manufacturing".to_string(),
        }),
        (326160, Code::<u32> {
            code: 326160,
            parent_code: Some(32616),
            description: "Plastics Bottle Manufacturing".to_string(),
        }),
        (32619, Code::<u32> {
            code: 32619,
            parent_code: Some(3261),
            description: "Other Plastics Product Manufacturing".to_string(),
        }),
        (326191, Code::<u32> {
            code: 326191,
            parent_code: Some(32619),
            description: "Plastics Plumbing Fixture Manufacturing ".to_string(),
        }),
        (326199, Code::<u32> {
            code: 326199,
            parent_code: Some(32619),
            description: "All Other Plastics Product Manufacturing ".to_string(),
        }),
        (3262, Code::<u32> {
            code: 3262,
            parent_code: Some(326),
            description: "Rubber Product Manufacturing".to_string(),
        }),
        (32621, Code::<u32> {
            code: 32621,
            parent_code: Some(3262),
            description: "Tire Manufacturing".to_string(),
        }),
        (326211, Code::<u32> {
            code: 326211,
            parent_code: Some(32621),
            description: "Tire Manufacturing (except Retreading) ".to_string(),
        }),
        (326212, Code::<u32> {
            code: 326212,
            parent_code: Some(32621),
            description: "Tire Retreading ".to_string(),
        }),
        (32622, Code::<u32> {
            code: 32622,
            parent_code: Some(3262),
            description: "Rubber and Plastics Hoses and Belting Manufacturing".to_string(),
        }),
        (326220, Code::<u32> {
            code: 326220,
            parent_code: Some(32622),
            description: "Rubber and Plastics Hoses and Belting Manufacturing".to_string(),
        }),
        (32629, Code::<u32> {
            code: 32629,
            parent_code: Some(3262),
            description: "Other Rubber Product Manufacturing".to_string(),
        }),
        (326291, Code::<u32> {
            code: 326291,
            parent_code: Some(32629),
            description: "Rubber Product Manufacturing for Mechanical Use ".to_string(),
        }),
        (326299, Code::<u32> {
            code: 326299,
            parent_code: Some(32629),
            description: "All Other Rubber Product Manufacturing ".to_string(),
        }),
        (327, Code::<u32> {
            code: 327,
            parent_code: Some(32),
            description: "Nonmetallic Mineral Product Manufacturing".to_string(),
        }),
        (3271, Code::<u32> {
            code: 3271,
            parent_code: Some(327),
            description: "Clay Product and Refractory Manufacturing".to_string(),
        }),
        (32711, Code::<u32> {
            code: 32711,
            parent_code: Some(3271),
            description: "Pottery, Ceramics, and Plumbing Fixture Manufacturing".to_string(),
        }),
        (327110, Code::<u32> {
            code: 327110,
            parent_code: Some(32711),
            description: "Pottery, Ceramics, and Plumbing Fixture Manufacturing ".to_string(),
        }),
        (32712, Code::<u32> {
            code: 32712,
            parent_code: Some(3271),
            description: "Clay Building Material and Refractories Manufacturing".to_string(),
        }),
        (327120, Code::<u32> {
            code: 327120,
            parent_code: Some(32712),
            description: "Clay Building Material and Refractories Manufacturing ".to_string(),
        }),
        (3272, Code::<u32> {
            code: 3272,
            parent_code: Some(327),
            description: "Glass and Glass Product Manufacturing".to_string(),
        }),
        (32721, Code::<u32> {
            code: 32721,
            parent_code: Some(3272),
            description: "Glass and Glass Product Manufacturing".to_string(),
        }),
        (327211, Code::<u32> {
            code: 327211,
            parent_code: Some(32721),
            description: "Flat Glass Manufacturing ".to_string(),
        }),
        (327212, Code::<u32> {
            code: 327212,
            parent_code: Some(32721),
            description: "Other Pressed and Blown Glass and Glassware Manufacturing ".to_string(),
        }),
        (327213, Code::<u32> {
            code: 327213,
            parent_code: Some(32721),
            description: "Glass Container Manufacturing ".to_string(),
        }),
        (327215, Code::<u32> {
            code: 327215,
            parent_code: Some(32721),
            description: "Glass Product Manufacturing Made of Purchased Glass ".to_string(),
        }),
        (3273, Code::<u32> {
            code: 3273,
            parent_code: Some(327),
            description: "Cement and Concrete Product Manufacturing".to_string(),
        }),
        (32731, Code::<u32> {
            code: 32731,
            parent_code: Some(3273),
            description: "Cement Manufacturing".to_string(),
        }),
        (327310, Code::<u32> {
            code: 327310,
            parent_code: Some(32731),
            description: "Cement Manufacturing".to_string(),
        }),
        (32732, Code::<u32> {
            code: 32732,
            parent_code: Some(3273),
            description: "Ready-Mix Concrete Manufacturing".to_string(),
        }),
        (327320, Code::<u32> {
            code: 327320,
            parent_code: Some(32732),
            description: "Ready-Mix Concrete Manufacturing".to_string(),
        }),
        (32733, Code::<u32> {
            code: 32733,
            parent_code: Some(3273),
            description: "Concrete Pipe, Brick, and Block Manufacturing".to_string(),
        }),
        (327331, Code::<u32> {
            code: 327331,
            parent_code: Some(32733),
            description: "Concrete Block and Brick Manufacturing ".to_string(),
        }),
        (327332, Code::<u32> {
            code: 327332,
            parent_code: Some(32733),
            description: "Concrete Pipe Manufacturing ".to_string(),
        }),
        (32739, Code::<u32> {
            code: 32739,
            parent_code: Some(3273),
            description: "Other Concrete Product Manufacturing".to_string(),
        }),
        (327390, Code::<u32> {
            code: 327390,
            parent_code: Some(32739),
            description: "Other Concrete Product Manufacturing ".to_string(),
        }),
        (3274, Code::<u32> {
            code: 3274,
            parent_code: Some(327),
            description: "Lime and Gypsum Product Manufacturing".to_string(),
        }),
        (32741, Code::<u32> {
            code: 32741,
            parent_code: Some(3274),
            description: "Lime Manufacturing".to_string(),
        }),
        (327410, Code::<u32> {
            code: 327410,
            parent_code: Some(32741),
            description: "Lime Manufacturing".to_string(),
        }),
        (32742, Code::<u32> {
            code: 32742,
            parent_code: Some(3274),
            description: "Gypsum Product Manufacturing".to_string(),
        }),
        (327420, Code::<u32> {
            code: 327420,
            parent_code: Some(32742),
            description: "Gypsum Product Manufacturing".to_string(),
        }),
        (3279, Code::<u32> {
            code: 3279,
            parent_code: Some(327),
            description: "Other Nonmetallic Mineral Product Manufacturing".to_string(),
        }),
        (32791, Code::<u32> {
            code: 32791,
            parent_code: Some(3279),
            description: "Abrasive Product Manufacturing".to_string(),
        }),
        (327910, Code::<u32> {
            code: 327910,
            parent_code: Some(32791),
            description: "Abrasive Product Manufacturing".to_string(),
        }),
        (32799, Code::<u32> {
            code: 32799,
            parent_code: Some(3279),
            description: "All Other Nonmetallic Mineral Product Manufacturing".to_string(),
        }),
        (327991, Code::<u32> {
            code: 327991,
            parent_code: Some(32799),
            description: "Cut Stone and Stone Product Manufacturing ".to_string(),
        }),
        (327992, Code::<u32> {
            code: 327992,
            parent_code: Some(32799),
            description: "Ground or Treated Mineral and Earth Manufacturing ".to_string(),
        }),
        (327993, Code::<u32> {
            code: 327993,
            parent_code: Some(32799),
            description: "Mineral Wool Manufacturing ".to_string(),
        }),
        (327999, Code::<u32> {
            code: 327999,
            parent_code: Some(32799),
            description: "All Other Miscellaneous Nonmetallic Mineral Product Manufacturing ".to_string(),
        }),
        (331, Code::<u32> {
            code: 331,
            parent_code: Some(33),
            description: "Primary Metal Manufacturing".to_string(),
        }),
        (3311, Code::<u32> {
            code: 3311,
            parent_code: Some(331),
            description: "Iron and Steel Mills and Ferroalloy Manufacturing".to_string(),
        }),
        (33111, Code::<u32> {
            code: 33111,
            parent_code: Some(3311),
            description: "Iron and Steel Mills and Ferroalloy Manufacturing".to_string(),
        }),
        (331110, Code::<u32> {
            code: 331110,
            parent_code: Some(33111),
            description: "Iron and Steel Mills and Ferroalloy Manufacturing ".to_string(),
        }),
        (3312, Code::<u32> {
            code: 3312,
            parent_code: Some(331),
            description: "Steel Product Manufacturing from Purchased Steel".to_string(),
        }),
        (33121, Code::<u32> {
            code: 33121,
            parent_code: Some(3312),
            description: "Iron and Steel Pipe and Tube Manufacturing from Purchased Steel".to_string(),
        }),
        (331210, Code::<u32> {
            code: 331210,
            parent_code: Some(33121),
            description: "Iron and Steel Pipe and Tube Manufacturing from Purchased Steel".to_string(),
        }),
        (33122, Code::<u32> {
            code: 33122,
            parent_code: Some(3312),
            description: "Rolling and Drawing of Purchased Steel".to_string(),
        }),
        (331221, Code::<u32> {
            code: 331221,
            parent_code: Some(33122),
            description: "Rolled Steel Shape Manufacturing ".to_string(),
        }),
        (331222, Code::<u32> {
            code: 331222,
            parent_code: Some(33122),
            description: "Steel Wire Drawing ".to_string(),
        }),
        (3313, Code::<u32> {
            code: 3313,
            parent_code: Some(331),
            description: "Alumina and Aluminum Production and Processing".to_string(),
        }),
        (33131, Code::<u32> {
            code: 33131,
            parent_code: Some(3313),
            description: "Alumina and Aluminum Production and Processing".to_string(),
        }),
        (331313, Code::<u32> {
            code: 331313,
            parent_code: Some(33131),
            description: "Alumina Refining and Primary Aluminum Production ".to_string(),
        }),
        (331314, Code::<u32> {
            code: 331314,
            parent_code: Some(33131),
            description: "Secondary Smelting and Alloying of Aluminum ".to_string(),
        }),
        (331315, Code::<u32> {
            code: 331315,
            parent_code: Some(33131),
            description: "Aluminum Sheet, Plate, and Foil Manufacturing ".to_string(),
        }),
        (331318, Code::<u32> {
            code: 331318,
            parent_code: Some(33131),
            description: "Other Aluminum Rolling, Drawing, and Extruding ".to_string(),
        }),
        (3314, Code::<u32> {
            code: 3314,
            parent_code: Some(331),
            description: "Nonferrous Metal (except Aluminum) Production and Processing".to_string(),
        }),
        (33141, Code::<u32> {
            code: 33141,
            parent_code: Some(3314),
            description: "Nonferrous Metal (except Aluminum) Smelting and Refining".to_string(),
        }),
        (331410, Code::<u32> {
            code: 331410,
            parent_code: Some(33141),
            description: "Nonferrous Metal (except Aluminum) Smelting and Refining ".to_string(),
        }),
        (33142, Code::<u32> {
            code: 33142,
            parent_code: Some(3314),
            description: "Copper Rolling, Drawing, Extruding, and Alloying".to_string(),
        }),
        (331420, Code::<u32> {
            code: 331420,
            parent_code: Some(33142),
            description: "Copper Rolling, Drawing, Extruding, and Alloying".to_string(),
        }),
        (33149, Code::<u32> {
            code: 33149,
            parent_code: Some(3314),
            description: "Nonferrous Metal (except Copper and Aluminum) Rolling, Drawing, Extruding, and Alloying".to_string(),
        }),
        (331491, Code::<u32> {
            code: 331491,
            parent_code: Some(33149),
            description: "Nonferrous Metal (except Copper and Aluminum) Rolling, Drawing, and Extruding ".to_string(),
        }),
        (331492, Code::<u32> {
            code: 331492,
            parent_code: Some(33149),
            description: "Secondary Smelting, Refining, and Alloying of Nonferrous Metal (except Copper and Aluminum) ".to_string(),
        }),
        (3315, Code::<u32> {
            code: 3315,
            parent_code: Some(331),
            description: "Foundries".to_string(),
        }),
        (33151, Code::<u32> {
            code: 33151,
            parent_code: Some(3315),
            description: "Ferrous Metal Foundries".to_string(),
        }),
        (331511, Code::<u32> {
            code: 331511,
            parent_code: Some(33151),
            description: "Iron Foundries ".to_string(),
        }),
        (331512, Code::<u32> {
            code: 331512,
            parent_code: Some(33151),
            description: "Steel Investment Foundries ".to_string(),
        }),
        (331513, Code::<u32> {
            code: 331513,
            parent_code: Some(33151),
            description: "Steel Foundries (except Investment) ".to_string(),
        }),
        (33152, Code::<u32> {
            code: 33152,
            parent_code: Some(3315),
            description: "Nonferrous Metal Foundries".to_string(),
        }),
        (331523, Code::<u32> {
            code: 331523,
            parent_code: Some(33152),
            description: "Nonferrous Metal Die-Casting Foundries ".to_string(),
        }),
        (331524, Code::<u32> {
            code: 331524,
            parent_code: Some(33152),
            description: "Aluminum Foundries (except Die-Casting) ".to_string(),
        }),
        (331529, Code::<u32> {
            code: 331529,
            parent_code: Some(33152),
            description: "Other Nonferrous Metal Foundries (except Die-Casting) ".to_string(),
        }),
        (332, Code::<u32> {
            code: 332,
            parent_code: Some(33),
            description: "Fabricated Metal Product Manufacturing".to_string(),
        }),
        (3321, Code::<u32> {
            code: 3321,
            parent_code: Some(332),
            description: "Forging and Stamping".to_string(),
        }),
        (33211, Code::<u32> {
            code: 33211,
            parent_code: Some(3321),
            description: "Forging and Stamping".to_string(),
        }),
        (332111, Code::<u32> {
            code: 332111,
            parent_code: Some(33211),
            description: "Iron and Steel Forging ".to_string(),
        }),
        (332112, Code::<u32> {
            code: 332112,
            parent_code: Some(33211),
            description: "Nonferrous Forging ".to_string(),
        }),
        (332114, Code::<u32> {
            code: 332114,
            parent_code: Some(33211),
            description: "Custom Roll Forming ".to_string(),
        }),
        (332117, Code::<u32> {
            code: 332117,
            parent_code: Some(33211),
            description: "Powder Metallurgy Part Manufacturing ".to_string(),
        }),
        (332119, Code::<u32> {
            code: 332119,
            parent_code: Some(33211),
            description: "Metal Crown, Closure, and Other Metal Stamping (except Automotive) ".to_string(),
        }),
        (3322, Code::<u32> {
            code: 3322,
            parent_code: Some(332),
            description: "Cutlery and Handtool Manufacturing".to_string(),
        }),
        (33221, Code::<u32> {
            code: 33221,
            parent_code: Some(3322),
            description: "Cutlery and Handtool Manufacturing".to_string(),
        }),
        (332215, Code::<u32> {
            code: 332215,
            parent_code: Some(33221),
            description: "Metal Kitchen Cookware, Utensil, Cutlery, and Flatware (except Precious) Manufacturing ".to_string(),
        }),
        (332216, Code::<u32> {
            code: 332216,
            parent_code: Some(33221),
            description: "Saw Blade and Handtool Manufacturing ".to_string(),
        }),
        (3323, Code::<u32> {
            code: 3323,
            parent_code: Some(332),
            description: "Architectural and Structural Metals Manufacturing".to_string(),
        }),
        (33231, Code::<u32> {
            code: 33231,
            parent_code: Some(3323),
            description: "Plate Work and Fabricated Structural Product Manufacturing".to_string(),
        }),
        (332311, Code::<u32> {
            code: 332311,
            parent_code: Some(33231),
            description: "Prefabricated Metal Building and Component Manufacturing ".to_string(),
        }),
        (332312, Code::<u32> {
            code: 332312,
            parent_code: Some(33231),
            description: "Fabricated Structural Metal Manufacturing ".to_string(),
        }),
        (332313, Code::<u32> {
            code: 332313,
            parent_code: Some(33231),
            description: "Plate Work Manufacturing ".to_string(),
        }),
        (33232, Code::<u32> {
            code: 33232,
            parent_code: Some(3323),
            description: "Ornamental and Architectural Metal Products Manufacturing".to_string(),
        }),
        (332321, Code::<u32> {
            code: 332321,
            parent_code: Some(33232),
            description: "Metal Window and Door Manufacturing ".to_string(),
        }),
        (332322, Code::<u32> {
            code: 332322,
            parent_code: Some(33232),
            description: "Sheet Metal Work Manufacturing ".to_string(),
        }),
        (332323, Code::<u32> {
            code: 332323,
            parent_code: Some(33232),
            description: "Ornamental and Architectural Metal Work Manufacturing ".to_string(),
        }),
        (3324, Code::<u32> {
            code: 3324,
            parent_code: Some(332),
            description: "Boiler, Tank, and Shipping Container Manufacturing".to_string(),
        }),
        (33241, Code::<u32> {
            code: 33241,
            parent_code: Some(3324),
            description: "Power Boiler and Heat Exchanger Manufacturing".to_string(),
        }),
        (332410, Code::<u32> {
            code: 332410,
            parent_code: Some(33241),
            description: "Power Boiler and Heat Exchanger Manufacturing".to_string(),
        }),
        (33242, Code::<u32> {
            code: 33242,
            parent_code: Some(3324),
            description: "Metal Tank (Heavy Gauge) Manufacturing".to_string(),
        }),
        (332420, Code::<u32> {
            code: 332420,
            parent_code: Some(33242),
            description: "Metal Tank (Heavy Gauge) Manufacturing".to_string(),
        }),
        (33243, Code::<u32> {
            code: 33243,
            parent_code: Some(3324),
            description: "Metal Can, Box, and Other Metal Container (Light Gauge) Manufacturing".to_string(),
        }),
        (332431, Code::<u32> {
            code: 332431,
            parent_code: Some(33243),
            description: "Metal Can Manufacturing ".to_string(),
        }),
        (332439, Code::<u32> {
            code: 332439,
            parent_code: Some(33243),
            description: "Other Metal Container Manufacturing ".to_string(),
        }),
        (3325, Code::<u32> {
            code: 3325,
            parent_code: Some(332),
            description: "Hardware Manufacturing".to_string(),
        }),
        (33251, Code::<u32> {
            code: 33251,
            parent_code: Some(3325),
            description: "Hardware Manufacturing".to_string(),
        }),
        (332510, Code::<u32> {
            code: 332510,
            parent_code: Some(33251),
            description: "Hardware Manufacturing".to_string(),
        }),
        (3326, Code::<u32> {
            code: 3326,
            parent_code: Some(332),
            description: "Spring and Wire Product Manufacturing".to_string(),
        }),
        (33261, Code::<u32> {
            code: 33261,
            parent_code: Some(3326),
            description: "Spring and Wire Product Manufacturing".to_string(),
        }),
        (332613, Code::<u32> {
            code: 332613,
            parent_code: Some(33261),
            description: "Spring Manufacturing ".to_string(),
        }),
        (332618, Code::<u32> {
            code: 332618,
            parent_code: Some(33261),
            description: "Other Fabricated Wire Product Manufacturing ".to_string(),
        }),
        (3327, Code::<u32> {
            code: 3327,
            parent_code: Some(332),
            description: "Machine Shops; Turned Product; and Screw, Nut, and Bolt Manufacturing".to_string(),
        }),
        (33271, Code::<u32> {
            code: 33271,
            parent_code: Some(3327),
            description: "Machine Shops".to_string(),
        }),
        (332710, Code::<u32> {
            code: 332710,
            parent_code: Some(33271),
            description: "Machine Shops".to_string(),
        }),
        (33272, Code::<u32> {
            code: 33272,
            parent_code: Some(3327),
            description: "Turned Product and Screw, Nut, and Bolt Manufacturing".to_string(),
        }),
        (332721, Code::<u32> {
            code: 332721,
            parent_code: Some(33272),
            description: "Precision Turned Product Manufacturing ".to_string(),
        }),
        (332722, Code::<u32> {
            code: 332722,
            parent_code: Some(33272),
            description: "Bolt, Nut, Screw, Rivet, and Washer Manufacturing ".to_string(),
        }),
        (3328, Code::<u32> {
            code: 3328,
            parent_code: Some(332),
            description: "Coating, Engraving, Heat Treating, and Allied Activities".to_string(),
        }),
        (33281, Code::<u32> {
            code: 33281,
            parent_code: Some(3328),
            description: "Coating, Engraving, Heat Treating, and Allied Activities".to_string(),
        }),
        (332811, Code::<u32> {
            code: 332811,
            parent_code: Some(33281),
            description: "Metal Heat Treating ".to_string(),
        }),
        (332812, Code::<u32> {
            code: 332812,
            parent_code: Some(33281),
            description: "Metal Coating, Engraving (except Jewelry and Silverware), and Allied Services to Manufacturers ".to_string(),
        }),
        (332813, Code::<u32> {
            code: 332813,
            parent_code: Some(33281),
            description: "Electroplating, Plating, Polishing, Anodizing, and Coloring ".to_string(),
        }),
        (3329, Code::<u32> {
            code: 3329,
            parent_code: Some(332),
            description: "Other Fabricated Metal Product Manufacturing".to_string(),
        }),
        (33291, Code::<u32> {
            code: 33291,
            parent_code: Some(3329),
            description: "Metal Valve Manufacturing".to_string(),
        }),
        (332911, Code::<u32> {
            code: 332911,
            parent_code: Some(33291),
            description: "Industrial Valve Manufacturing ".to_string(),
        }),
        (332912, Code::<u32> {
            code: 332912,
            parent_code: Some(33291),
            description: "Fluid Power Valve and Hose Fitting Manufacturing ".to_string(),
        }),
        (332913, Code::<u32> {
            code: 332913,
            parent_code: Some(33291),
            description: "Plumbing Fixture Fitting and Trim Manufacturing ".to_string(),
        }),
        (332919, Code::<u32> {
            code: 332919,
            parent_code: Some(33291),
            description: "Other Metal Valve and Pipe Fitting Manufacturing ".to_string(),
        }),
        (33299, Code::<u32> {
            code: 33299,
            parent_code: Some(3329),
            description: "All Other Fabricated Metal Product Manufacturing".to_string(),
        }),
        (332991, Code::<u32> {
            code: 332991,
            parent_code: Some(33299),
            description: "Ball and Roller Bearing Manufacturing".to_string(),
        }),
        (332992, Code::<u32> {
            code: 332992,
            parent_code: Some(33299),
            description: "Small Arms Ammunition Manufacturing ".to_string(),
        }),
        (332993, Code::<u32> {
            code: 332993,
            parent_code: Some(33299),
            description: "Ammunition (except Small Arms) Manufacturing ".to_string(),
        }),
        (332994, Code::<u32> {
            code: 332994,
            parent_code: Some(33299),
            description: "Small Arms, Ordnance, and Ordnance Accessories Manufacturing ".to_string(),
        }),
        (332996, Code::<u32> {
            code: 332996,
            parent_code: Some(33299),
            description: "Fabricated Pipe and Pipe Fitting Manufacturing ".to_string(),
        }),
        (332999, Code::<u32> {
            code: 332999,
            parent_code: Some(33299),
            description: "All Other Miscellaneous Fabricated Metal Product Manufacturing ".to_string(),
        }),
        (333, Code::<u32> {
            code: 333,
            parent_code: Some(33),
            description: "Machinery Manufacturing".to_string(),
        }),
        (3331, Code::<u32> {
            code: 3331,
            parent_code: Some(333),
            description: "Agriculture, Construction, and Mining Machinery Manufacturing".to_string(),
        }),
        (33311, Code::<u32> {
            code: 33311,
            parent_code: Some(3331),
            description: "Agricultural Implement Manufacturing".to_string(),
        }),
        (333111, Code::<u32> {
            code: 333111,
            parent_code: Some(33311),
            description: "Farm Machinery and Equipment Manufacturing ".to_string(),
        }),
        (333112, Code::<u32> {
            code: 333112,
            parent_code: Some(33311),
            description: "Lawn and Garden Tractor and Home Lawn and Garden Equipment Manufacturing ".to_string(),
        }),
        (33312, Code::<u32> {
            code: 33312,
            parent_code: Some(3331),
            description: "Construction Machinery Manufacturing".to_string(),
        }),
        (333120, Code::<u32> {
            code: 333120,
            parent_code: Some(33312),
            description: "Construction Machinery Manufacturing".to_string(),
        }),
        (33313, Code::<u32> {
            code: 33313,
            parent_code: Some(3331),
            description: "Mining and Oil and Gas Field Machinery Manufacturing".to_string(),
        }),
        (333131, Code::<u32> {
            code: 333131,
            parent_code: Some(33313),
            description: "Mining Machinery and Equipment Manufacturing ".to_string(),
        }),
        (333132, Code::<u32> {
            code: 333132,
            parent_code: Some(33313),
            description: "Oil and Gas Field Machinery and Equipment Manufacturing ".to_string(),
        }),
        (3332, Code::<u32> {
            code: 3332,
            parent_code: Some(333),
            description: "Industrial Machinery Manufacturing".to_string(),
        }),
        (33324, Code::<u32> {
            code: 33324,
            parent_code: Some(3332),
            description: "Industrial Machinery Manufacturing".to_string(),
        }),
        (333241, Code::<u32> {
            code: 333241,
            parent_code: Some(33324),
            description: "Food Product Machinery Manufacturing ".to_string(),
        }),
        (333242, Code::<u32> {
            code: 333242,
            parent_code: Some(33324),
            description: "Semiconductor Machinery Manufacturing ".to_string(),
        }),
        (333243, Code::<u32> {
            code: 333243,
            parent_code: Some(33324),
            description: "Sawmill, Woodworking, and Paper Machinery Manufacturing ".to_string(),
        }),
        (333244, Code::<u32> {
            code: 333244,
            parent_code: Some(33324),
            description: "Printing Machinery and Equipment Manufacturing ".to_string(),
        }),
        (333249, Code::<u32> {
            code: 333249,
            parent_code: Some(33324),
            description: "Other Industrial Machinery Manufacturing ".to_string(),
        }),
        (3333, Code::<u32> {
            code: 3333,
            parent_code: Some(333),
            description: "Commercial and Service Industry Machinery Manufacturing".to_string(),
        }),
        (33331, Code::<u32> {
            code: 33331,
            parent_code: Some(3333),
            description: "Commercial and Service Industry Machinery Manufacturing".to_string(),
        }),
        (333314, Code::<u32> {
            code: 333314,
            parent_code: Some(33331),
            description: "Optical Instrument and Lens Manufacturing ".to_string(),
        }),
        (333316, Code::<u32> {
            code: 333316,
            parent_code: Some(33331),
            description: "Photographic and Photocopying Equipment Manufacturing ".to_string(),
        }),
        (333318, Code::<u32> {
            code: 333318,
            parent_code: Some(33331),
            description: "Other Commercial and Service Industry Machinery Manufacturing ".to_string(),
        }),
        (3334, Code::<u32> {
            code: 3334,
            parent_code: Some(333),
            description: "Ventilation, Heating, Air-Conditioning, and Commercial Refrigeration Equipment Manufacturing".to_string(),
        }),
        (33341, Code::<u32> {
            code: 33341,
            parent_code: Some(3334),
            description: "Ventilation, Heating, Air-Conditioning, and Commercial Refrigeration Equipment Manufacturing".to_string(),
        }),
        (333413, Code::<u32> {
            code: 333413,
            parent_code: Some(33341),
            description: "Industrial and Commercial Fan and Blower and Air Purification Equipment Manufacturing ".to_string(),
        }),
        (333414, Code::<u32> {
            code: 333414,
            parent_code: Some(33341),
            description: "Heating Equipment (except Warm Air Furnaces) Manufacturing ".to_string(),
        }),
        (333415, Code::<u32> {
            code: 333415,
            parent_code: Some(33341),
            description: "Air-Conditioning and Warm Air Heating Equipment and Commercial and Industrial Refrigeration Equipment Manufacturing ".to_string(),
        }),
        (3335, Code::<u32> {
            code: 3335,
            parent_code: Some(333),
            description: "Metalworking Machinery Manufacturing".to_string(),
        }),
        (33351, Code::<u32> {
            code: 33351,
            parent_code: Some(3335),
            description: "Metalworking Machinery Manufacturing".to_string(),
        }),
        (333511, Code::<u32> {
            code: 333511,
            parent_code: Some(33351),
            description: "Industrial Mold Manufacturing ".to_string(),
        }),
        (333514, Code::<u32> {
            code: 333514,
            parent_code: Some(33351),
            description: "Special Die and Tool, Die Set, Jig, and Fixture Manufacturing ".to_string(),
        }),
        (333515, Code::<u32> {
            code: 333515,
            parent_code: Some(33351),
            description: "Cutting Tool and Machine Tool Accessory Manufacturing ".to_string(),
        }),
        (333517, Code::<u32> {
            code: 333517,
            parent_code: Some(33351),
            description: "Machine Tool Manufacturing ".to_string(),
        }),
        (333519, Code::<u32> {
            code: 333519,
            parent_code: Some(33351),
            description: "Rolling Mill and Other Metalworking Machinery Manufacturing ".to_string(),
        }),
        (3336, Code::<u32> {
            code: 3336,
            parent_code: Some(333),
            description: "Engine, Turbine, and Power Transmission Equipment Manufacturing".to_string(),
        }),
        (33361, Code::<u32> {
            code: 33361,
            parent_code: Some(3336),
            description: "Engine, Turbine, and Power Transmission Equipment Manufacturing".to_string(),
        }),
        (333611, Code::<u32> {
            code: 333611,
            parent_code: Some(33361),
            description: "Turbine and Turbine Generator Set Units Manufacturing ".to_string(),
        }),
        (333612, Code::<u32> {
            code: 333612,
            parent_code: Some(33361),
            description: "Speed Changer, Industrial High-Speed Drive, and Gear Manufacturing ".to_string(),
        }),
        (333613, Code::<u32> {
            code: 333613,
            parent_code: Some(33361),
            description: "Mechanical Power Transmission Equipment Manufacturing ".to_string(),
        }),
        (333618, Code::<u32> {
            code: 333618,
            parent_code: Some(33361),
            description: "Other Engine Equipment Manufacturing ".to_string(),
        }),
        (3339, Code::<u32> {
            code: 3339,
            parent_code: Some(333),
            description: "Other General Purpose Machinery Manufacturing".to_string(),
        }),
        (33391, Code::<u32> {
            code: 33391,
            parent_code: Some(3339),
            description: "Pump and Compressor Manufacturing".to_string(),
        }),
        (333912, Code::<u32> {
            code: 333912,
            parent_code: Some(33391),
            description: "Air and Gas Compressor Manufacturing ".to_string(),
        }),
        (333914, Code::<u32> {
            code: 333914,
            parent_code: Some(33391),
            description: "Measuring, Dispensing, and Other Pumping Equipment Manufacturing ".to_string(),
        }),
        (33392, Code::<u32> {
            code: 33392,
            parent_code: Some(3339),
            description: "Material Handling Equipment Manufacturing".to_string(),
        }),
        (333921, Code::<u32> {
            code: 333921,
            parent_code: Some(33392),
            description: "Elevator and Moving Stairway Manufacturing ".to_string(),
        }),
        (333922, Code::<u32> {
            code: 333922,
            parent_code: Some(33392),
            description: "Conveyor and Conveying Equipment Manufacturing ".to_string(),
        }),
        (333923, Code::<u32> {
            code: 333923,
            parent_code: Some(33392),
            description: "Overhead Traveling Crane, Hoist, and Monorail System Manufacturing ".to_string(),
        }),
        (333924, Code::<u32> {
            code: 333924,
            parent_code: Some(33392),
            description: "Industrial Truck, Tractor, Trailer, and Stacker Machinery Manufacturing ".to_string(),
        }),
        (33399, Code::<u32> {
            code: 33399,
            parent_code: Some(3339),
            description: "All Other General Purpose Machinery Manufacturing".to_string(),
        }),
        (333991, Code::<u32> {
            code: 333991,
            parent_code: Some(33399),
            description: "Power-Driven Handtool Manufacturing ".to_string(),
        }),
        (333992, Code::<u32> {
            code: 333992,
            parent_code: Some(33399),
            description: "Welding and Soldering Equipment Manufacturing ".to_string(),
        }),
        (333993, Code::<u32> {
            code: 333993,
            parent_code: Some(33399),
            description: "Packaging Machinery Manufacturing ".to_string(),
        }),
        (333994, Code::<u32> {
            code: 333994,
            parent_code: Some(33399),
            description: "Industrial Process Furnace and Oven Manufacturing ".to_string(),
        }),
        (333995, Code::<u32> {
            code: 333995,
            parent_code: Some(33399),
            description: "Fluid Power Cylinder and Actuator Manufacturing ".to_string(),
        }),
        (333996, Code::<u32> {
            code: 333996,
            parent_code: Some(33399),
            description: "Fluid Power Pump and Motor Manufacturing ".to_string(),
        }),
        (333997, Code::<u32> {
            code: 333997,
            parent_code: Some(33399),
            description: "Scale and Balance Manufacturing ".to_string(),
        }),
        (333999, Code::<u32> {
            code: 333999,
            parent_code: Some(33399),
            description: "All Other Miscellaneous General Purpose Machinery Manufacturing ".to_string(),
        }),
        (334, Code::<u32> {
            code: 334,
            parent_code: Some(33),
            description: "Computer and Electronic Product Manufacturing".to_string(),
        }),
        (3341, Code::<u32> {
            code: 3341,
            parent_code: Some(334),
            description: "Computer and Peripheral Equipment Manufacturing".to_string(),
        }),
        (33411, Code::<u32> {
            code: 33411,
            parent_code: Some(3341),
            description: "Computer and Peripheral Equipment Manufacturing".to_string(),
        }),
        (334111, Code::<u32> {
            code: 334111,
            parent_code: Some(33411),
            description: "Electronic Computer Manufacturing ".to_string(),
        }),
        (334112, Code::<u32> {
            code: 334112,
            parent_code: Some(33411),
            description: "Computer Storage Device Manufacturing ".to_string(),
        }),
        (334118, Code::<u32> {
            code: 334118,
            parent_code: Some(33411),
            description: "Computer Terminal and Other Computer Peripheral Equipment Manufacturing ".to_string(),
        }),
        (3342, Code::<u32> {
            code: 3342,
            parent_code: Some(334),
            description: "Communications Equipment Manufacturing".to_string(),
        }),
        (33421, Code::<u32> {
            code: 33421,
            parent_code: Some(3342),
            description: "Telephone Apparatus Manufacturing".to_string(),
        }),
        (334210, Code::<u32> {
            code: 334210,
            parent_code: Some(33421),
            description: "Telephone Apparatus Manufacturing".to_string(),
        }),
        (33422, Code::<u32> {
            code: 33422,
            parent_code: Some(3342),
            description: "Radio and Television Broadcasting and Wireless Communications Equipment Manufacturing".to_string(),
        }),
        (334220, Code::<u32> {
            code: 334220,
            parent_code: Some(33422),
            description: "Radio and Television Broadcasting and Wireless Communications Equipment Manufacturing".to_string(),
        }),
        (33429, Code::<u32> {
            code: 33429,
            parent_code: Some(3342),
            description: "Other Communications Equipment Manufacturing".to_string(),
        }),
        (334290, Code::<u32> {
            code: 334290,
            parent_code: Some(33429),
            description: "Other Communications Equipment Manufacturing".to_string(),
        }),
        (3343, Code::<u32> {
            code: 3343,
            parent_code: Some(334),
            description: "Audio and Video Equipment Manufacturing".to_string(),
        }),
        (33431, Code::<u32> {
            code: 33431,
            parent_code: Some(3343),
            description: "Audio and Video Equipment Manufacturing".to_string(),
        }),
        (334310, Code::<u32> {
            code: 334310,
            parent_code: Some(33431),
            description: "Audio and Video Equipment Manufacturing".to_string(),
        }),
        (3344, Code::<u32> {
            code: 3344,
            parent_code: Some(334),
            description: "Semiconductor and Other Electronic Component Manufacturing".to_string(),
        }),
        (33441, Code::<u32> {
            code: 33441,
            parent_code: Some(3344),
            description: "Semiconductor and Other Electronic Component Manufacturing".to_string(),
        }),
        (334412, Code::<u32> {
            code: 334412,
            parent_code: Some(33441),
            description: "Bare Printed Circuit Board Manufacturing  ".to_string(),
        }),
        (334413, Code::<u32> {
            code: 334413,
            parent_code: Some(33441),
            description: "Semiconductor and Related Device Manufacturing ".to_string(),
        }),
        (334416, Code::<u32> {
            code: 334416,
            parent_code: Some(33441),
            description: "Capacitor, Resistor, Coil, Transformer, and Other Inductor Manufacturing ".to_string(),
        }),
        (334417, Code::<u32> {
            code: 334417,
            parent_code: Some(33441),
            description: "Electronic Connector Manufacturing ".to_string(),
        }),
        (334418, Code::<u32> {
            code: 334418,
            parent_code: Some(33441),
            description: "Printed Circuit Assembly (Electronic Assembly) Manufacturing ".to_string(),
        }),
        (334419, Code::<u32> {
            code: 334419,
            parent_code: Some(33441),
            description: "Other Electronic Component Manufacturing ".to_string(),
        }),
        (3345, Code::<u32> {
            code: 3345,
            parent_code: Some(334),
            description: "Navigational, Measuring, Electromedical, and Control Instruments Manufacturing".to_string(),
        }),
        (33451, Code::<u32> {
            code: 33451,
            parent_code: Some(3345),
            description: "Navigational, Measuring, Electromedical, and Control Instruments Manufacturing".to_string(),
        }),
        (334510, Code::<u32> {
            code: 334510,
            parent_code: Some(33451),
            description: "Electromedical and Electrotherapeutic Apparatus Manufacturing ".to_string(),
        }),
        (334511, Code::<u32> {
            code: 334511,
            parent_code: Some(33451),
            description: "Search, Detection, Navigation, Guidance, Aeronautical, and Nautical System and Instrument Manufacturing ".to_string(),
        }),
        (334512, Code::<u32> {
            code: 334512,
            parent_code: Some(33451),
            description: "Automatic Environmental Control Manufacturing for Residential, Commercial, and Appliance Use ".to_string(),
        }),
        (334513, Code::<u32> {
            code: 334513,
            parent_code: Some(33451),
            description: "Instruments and Related Products Manufacturing for Measuring, Displaying, and Controlling Industrial Process Variables ".to_string(),
        }),
        (334514, Code::<u32> {
            code: 334514,
            parent_code: Some(33451),
            description: "Totalizing Fluid Meter and Counting Device Manufacturing ".to_string(),
        }),
        (334515, Code::<u32> {
            code: 334515,
            parent_code: Some(33451),
            description: "Instrument Manufacturing for Measuring and Testing Electricity and Electrical Signals ".to_string(),
        }),
        (334516, Code::<u32> {
            code: 334516,
            parent_code: Some(33451),
            description: "Analytical Laboratory Instrument Manufacturing ".to_string(),
        }),
        (334517, Code::<u32> {
            code: 334517,
            parent_code: Some(33451),
            description: "Irradiation Apparatus Manufacturing ".to_string(),
        }),
        (334519, Code::<u32> {
            code: 334519,
            parent_code: Some(33451),
            description: "Other Measuring and Controlling Device Manufacturing ".to_string(),
        }),
        (3346, Code::<u32> {
            code: 3346,
            parent_code: Some(334),
            description: "Manufacturing and Reproducing Magnetic and Optical Media".to_string(),
        }),
        (33461, Code::<u32> {
            code: 33461,
            parent_code: Some(3346),
            description: "Manufacturing and Reproducing Magnetic and Optical Media".to_string(),
        }),
        (334613, Code::<u32> {
            code: 334613,
            parent_code: Some(33461),
            description: "Blank Magnetic and Optical Recording Media Manufacturing ".to_string(),
        }),
        (334614, Code::<u32> {
            code: 334614,
            parent_code: Some(33461),
            description: "Software and Other Prerecorded Compact Disc, Tape, and Record Reproducing ".to_string(),
        }),
        (335, Code::<u32> {
            code: 335,
            parent_code: Some(33),
            description: "Electrical Equipment, Appliance, and Component Manufacturing".to_string(),
        }),
        (3351, Code::<u32> {
            code: 3351,
            parent_code: Some(335),
            description: "Electric Lighting Equipment Manufacturing".to_string(),
        }),
        (33511, Code::<u32> {
            code: 33511,
            parent_code: Some(3351),
            description: "Electric Lamp Bulb and Part Manufacturing".to_string(),
        }),
        (335110, Code::<u32> {
            code: 335110,
            parent_code: Some(33511),
            description: "Electric Lamp Bulb and Part Manufacturing".to_string(),
        }),
        (33512, Code::<u32> {
            code: 33512,
            parent_code: Some(3351),
            description: "Lighting Fixture Manufacturing".to_string(),
        }),
        (335121, Code::<u32> {
            code: 335121,
            parent_code: Some(33512),
            description: "Residential Electric Lighting Fixture Manufacturing ".to_string(),
        }),
        (335122, Code::<u32> {
            code: 335122,
            parent_code: Some(33512),
            description: "Commercial, Industrial, and Institutional Electric Lighting Fixture Manufacturing ".to_string(),
        }),
        (335129, Code::<u32> {
            code: 335129,
            parent_code: Some(33512),
            description: "Other Lighting Equipment Manufacturing ".to_string(),
        }),
        (3352, Code::<u32> {
            code: 3352,
            parent_code: Some(335),
            description: "Household Appliance Manufacturing".to_string(),
        }),
        (33521, Code::<u32> {
            code: 33521,
            parent_code: Some(3352),
            description: "Small Electrical Appliance Manufacturing".to_string(),
        }),
        (335210, Code::<u32> {
            code: 335210,
            parent_code: Some(33521),
            description: "Small Electrical Appliance Manufacturing".to_string(),
        }),
        (33522, Code::<u32> {
            code: 33522,
            parent_code: Some(3352),
            description: "Major Household Appliance Manufacturing ".to_string(),
        }),
        (335220, Code::<u32> {
            code: 335220,
            parent_code: Some(33522),
            description: "Major Household Appliance Manufacturing ".to_string(),
        }),
        (3353, Code::<u32> {
            code: 3353,
            parent_code: Some(335),
            description: "Electrical Equipment Manufacturing".to_string(),
        }),
        (33531, Code::<u32> {
            code: 33531,
            parent_code: Some(3353),
            description: "Electrical Equipment Manufacturing".to_string(),
        }),
        (335311, Code::<u32> {
            code: 335311,
            parent_code: Some(33531),
            description: "Power, Distribution, and Specialty Transformer Manufacturing ".to_string(),
        }),
        (335312, Code::<u32> {
            code: 335312,
            parent_code: Some(33531),
            description: "Motor and Generator Manufacturing ".to_string(),
        }),
        (335313, Code::<u32> {
            code: 335313,
            parent_code: Some(33531),
            description: "Switchgear and Switchboard Apparatus Manufacturing ".to_string(),
        }),
        (335314, Code::<u32> {
            code: 335314,
            parent_code: Some(33531),
            description: "Relay and Industrial Control Manufacturing ".to_string(),
        }),
        (3359, Code::<u32> {
            code: 3359,
            parent_code: Some(335),
            description: "Other Electrical Equipment and Component Manufacturing".to_string(),
        }),
        (33591, Code::<u32> {
            code: 33591,
            parent_code: Some(3359),
            description: "Battery Manufacturing".to_string(),
        }),
        (335911, Code::<u32> {
            code: 335911,
            parent_code: Some(33591),
            description: "Storage Battery Manufacturing ".to_string(),
        }),
        (335912, Code::<u32> {
            code: 335912,
            parent_code: Some(33591),
            description: "Primary Battery Manufacturing ".to_string(),
        }),
        (33592, Code::<u32> {
            code: 33592,
            parent_code: Some(3359),
            description: "Communication and Energy Wire and Cable Manufacturing".to_string(),
        }),
        (335921, Code::<u32> {
            code: 335921,
            parent_code: Some(33592),
            description: "Fiber Optic Cable Manufacturing ".to_string(),
        }),
        (335929, Code::<u32> {
            code: 335929,
            parent_code: Some(33592),
            description: "Other Communication and Energy Wire Manufacturing ".to_string(),
        }),
        (33593, Code::<u32> {
            code: 33593,
            parent_code: Some(3359),
            description: "Wiring Device Manufacturing".to_string(),
        }),
        (335931, Code::<u32> {
            code: 335931,
            parent_code: Some(33593),
            description: "Current-Carrying Wiring Device Manufacturing ".to_string(),
        }),
        (335932, Code::<u32> {
            code: 335932,
            parent_code: Some(33593),
            description: "Noncurrent-Carrying Wiring Device Manufacturing ".to_string(),
        }),
        (33599, Code::<u32> {
            code: 33599,
            parent_code: Some(3359),
            description: "All Other Electrical Equipment and Component Manufacturing".to_string(),
        }),
        (335991, Code::<u32> {
            code: 335991,
            parent_code: Some(33599),
            description: "Carbon and Graphite Product Manufacturing ".to_string(),
        }),
        (335999, Code::<u32> {
            code: 335999,
            parent_code: Some(33599),
            description: "All Other Miscellaneous Electrical Equipment and Component Manufacturing ".to_string(),
        }),
        (336, Code::<u32> {
            code: 336,
            parent_code: Some(33),
            description: "Transportation Equipment Manufacturing".to_string(),
        }),
        (3361, Code::<u32> {
            code: 3361,
            parent_code: Some(336),
            description: "Motor Vehicle Manufacturing".to_string(),
        }),
        (33611, Code::<u32> {
            code: 33611,
            parent_code: Some(3361),
            description: "Automobile and Light Duty Motor Vehicle Manufacturing".to_string(),
        }),
        (336111, Code::<u32> {
            code: 336111,
            parent_code: Some(33611),
            description: "Automobile Manufacturing ".to_string(),
        }),
        (336112, Code::<u32> {
            code: 336112,
            parent_code: Some(33611),
            description: "Light Truck and Utility Vehicle Manufacturing ".to_string(),
        }),
        (33612, Code::<u32> {
            code: 33612,
            parent_code: Some(3361),
            description: "Heavy Duty Truck Manufacturing".to_string(),
        }),
        (336120, Code::<u32> {
            code: 336120,
            parent_code: Some(33612),
            description: "Heavy Duty Truck Manufacturing".to_string(),
        }),
        (3362, Code::<u32> {
            code: 3362,
            parent_code: Some(336),
            description: "Motor Vehicle Body and Trailer Manufacturing".to_string(),
        }),
        (33621, Code::<u32> {
            code: 33621,
            parent_code: Some(3362),
            description: "Motor Vehicle Body and Trailer Manufacturing".to_string(),
        }),
        (336211, Code::<u32> {
            code: 336211,
            parent_code: Some(33621),
            description: "Motor Vehicle Body Manufacturing ".to_string(),
        }),
        (336212, Code::<u32> {
            code: 336212,
            parent_code: Some(33621),
            description: "Truck Trailer Manufacturing ".to_string(),
        }),
        (336213, Code::<u32> {
            code: 336213,
            parent_code: Some(33621),
            description: "Motor Home Manufacturing ".to_string(),
        }),
        (336214, Code::<u32> {
            code: 336214,
            parent_code: Some(33621),
            description: "Travel Trailer and Camper Manufacturing ".to_string(),
        }),
        (3363, Code::<u32> {
            code: 3363,
            parent_code: Some(336),
            description: "Motor Vehicle Parts Manufacturing".to_string(),
        }),
        (33631, Code::<u32> {
            code: 33631,
            parent_code: Some(3363),
            description: "Motor Vehicle Gasoline Engine and Engine Parts Manufacturing".to_string(),
        }),
        (336310, Code::<u32> {
            code: 336310,
            parent_code: Some(33631),
            description: "Motor Vehicle Gasoline Engine and Engine Parts Manufacturing".to_string(),
        }),
        (33632, Code::<u32> {
            code: 33632,
            parent_code: Some(3363),
            description: "Motor Vehicle Electrical and Electronic Equipment Manufacturing".to_string(),
        }),
        (336320, Code::<u32> {
            code: 336320,
            parent_code: Some(33632),
            description: "Motor Vehicle Electrical and Electronic Equipment Manufacturing".to_string(),
        }),
        (33633, Code::<u32> {
            code: 33633,
            parent_code: Some(3363),
            description: "Motor Vehicle Steering and Suspension Components (except Spring) Manufacturing".to_string(),
        }),
        (336330, Code::<u32> {
            code: 336330,
            parent_code: Some(33633),
            description: "Motor Vehicle Steering and Suspension Components (except Spring) Manufacturing".to_string(),
        }),
        (33634, Code::<u32> {
            code: 33634,
            parent_code: Some(3363),
            description: "Motor Vehicle Brake System Manufacturing".to_string(),
        }),
        (336340, Code::<u32> {
            code: 336340,
            parent_code: Some(33634),
            description: "Motor Vehicle Brake System Manufacturing".to_string(),
        }),
        (33635, Code::<u32> {
            code: 33635,
            parent_code: Some(3363),
            description: "Motor Vehicle Transmission and Power Train Parts Manufacturing".to_string(),
        }),
        (336350, Code::<u32> {
            code: 336350,
            parent_code: Some(33635),
            description: "Motor Vehicle Transmission and Power Train Parts Manufacturing".to_string(),
        }),
        (33636, Code::<u32> {
            code: 33636,
            parent_code: Some(3363),
            description: "Motor Vehicle Seating and Interior Trim Manufacturing".to_string(),
        }),
        (336360, Code::<u32> {
            code: 336360,
            parent_code: Some(33636),
            description: "Motor Vehicle Seating and Interior Trim Manufacturing".to_string(),
        }),
        (33637, Code::<u32> {
            code: 33637,
            parent_code: Some(3363),
            description: "Motor Vehicle Metal Stamping".to_string(),
        }),
        (336370, Code::<u32> {
            code: 336370,
            parent_code: Some(33637),
            description: "Motor Vehicle Metal Stamping".to_string(),
        }),
        (33639, Code::<u32> {
            code: 33639,
            parent_code: Some(3363),
            description: "Other Motor Vehicle Parts Manufacturing".to_string(),
        }),
        (336390, Code::<u32> {
            code: 336390,
            parent_code: Some(33639),
            description: "Other Motor Vehicle Parts Manufacturing".to_string(),
        }),
        (3364, Code::<u32> {
            code: 3364,
            parent_code: Some(336),
            description: "Aerospace Product and Parts Manufacturing".to_string(),
        }),
        (33641, Code::<u32> {
            code: 33641,
            parent_code: Some(3364),
            description: "Aerospace Product and Parts Manufacturing".to_string(),
        }),
        (336411, Code::<u32> {
            code: 336411,
            parent_code: Some(33641),
            description: "Aircraft Manufacturing ".to_string(),
        }),
        (336412, Code::<u32> {
            code: 336412,
            parent_code: Some(33641),
            description: "Aircraft Engine and Engine Parts Manufacturing ".to_string(),
        }),
        (336413, Code::<u32> {
            code: 336413,
            parent_code: Some(33641),
            description: "Other Aircraft Parts and Auxiliary Equipment Manufacturing ".to_string(),
        }),
        (336414, Code::<u32> {
            code: 336414,
            parent_code: Some(33641),
            description: "Guided Missile and Space Vehicle Manufacturing ".to_string(),
        }),
        (336415, Code::<u32> {
            code: 336415,
            parent_code: Some(33641),
            description: "Guided Missile and Space Vehicle Propulsion Unit and Propulsion Unit Parts Manufacturing ".to_string(),
        }),
        (336419, Code::<u32> {
            code: 336419,
            parent_code: Some(33641),
            description: "Other Guided Missile and Space Vehicle Parts and Auxiliary Equipment Manufacturing ".to_string(),
        }),
        (3365, Code::<u32> {
            code: 3365,
            parent_code: Some(336),
            description: "Railroad Rolling Stock Manufacturing".to_string(),
        }),
        (33651, Code::<u32> {
            code: 33651,
            parent_code: Some(3365),
            description: "Railroad Rolling Stock Manufacturing".to_string(),
        }),
        (336510, Code::<u32> {
            code: 336510,
            parent_code: Some(33651),
            description: "Railroad Rolling Stock Manufacturing".to_string(),
        }),
        (3366, Code::<u32> {
            code: 3366,
            parent_code: Some(336),
            description: "Ship and Boat Building".to_string(),
        }),
        (33661, Code::<u32> {
            code: 33661,
            parent_code: Some(3366),
            description: "Ship and Boat Building".to_string(),
        }),
        (336611, Code::<u32> {
            code: 336611,
            parent_code: Some(33661),
            description: "Ship Building and Repairing ".to_string(),
        }),
        (336612, Code::<u32> {
            code: 336612,
            parent_code: Some(33661),
            description: "Boat Building ".to_string(),
        }),
        (3369, Code::<u32> {
            code: 3369,
            parent_code: Some(336),
            description: "Other Transportation Equipment Manufacturing".to_string(),
        }),
        (33699, Code::<u32> {
            code: 33699,
            parent_code: Some(3369),
            description: "Other Transportation Equipment Manufacturing".to_string(),
        }),
        (336991, Code::<u32> {
            code: 336991,
            parent_code: Some(33699),
            description: "Motorcycle, Bicycle, and Parts Manufacturing ".to_string(),
        }),
        (336992, Code::<u32> {
            code: 336992,
            parent_code: Some(33699),
            description: "Military Armored Vehicle, Tank, and Tank Component Manufacturing ".to_string(),
        }),
        (336999, Code::<u32> {
            code: 336999,
            parent_code: Some(33699),
            description: "All Other Transportation Equipment Manufacturing ".to_string(),
        }),
        (337, Code::<u32> {
            code: 337,
            parent_code: Some(33),
            description: "Furniture and Related Product Manufacturing".to_string(),
        }),
        (3371, Code::<u32> {
            code: 3371,
            parent_code: Some(337),
            description: "Household and Institutional Furniture and Kitchen Cabinet Manufacturing".to_string(),
        }),
        (33711, Code::<u32> {
            code: 33711,
            parent_code: Some(3371),
            description: "Wood Kitchen Cabinet and Countertop Manufacturing".to_string(),
        }),
        (337110, Code::<u32> {
            code: 337110,
            parent_code: Some(33711),
            description: "Wood Kitchen Cabinet and Countertop Manufacturing".to_string(),
        }),
        (33712, Code::<u32> {
            code: 33712,
            parent_code: Some(3371),
            description: "Household and Institutional Furniture Manufacturing".to_string(),
        }),
        (337121, Code::<u32> {
            code: 337121,
            parent_code: Some(33712),
            description: "Upholstered Household Furniture Manufacturing ".to_string(),
        }),
        (337122, Code::<u32> {
            code: 337122,
            parent_code: Some(33712),
            description: "Nonupholstered Wood Household Furniture Manufacturing ".to_string(),
        }),
        (337124, Code::<u32> {
            code: 337124,
            parent_code: Some(33712),
            description: "Metal Household Furniture Manufacturing ".to_string(),
        }),
        (337125, Code::<u32> {
            code: 337125,
            parent_code: Some(33712),
            description: "Household Furniture (except Wood and Metal) Manufacturing ".to_string(),
        }),
        (337127, Code::<u32> {
            code: 337127,
            parent_code: Some(33712),
            description: "Institutional Furniture Manufacturing ".to_string(),
        }),
        (3372, Code::<u32> {
            code: 3372,
            parent_code: Some(337),
            description: "Office Furniture (including Fixtures) Manufacturing".to_string(),
        }),
        (33721, Code::<u32> {
            code: 33721,
            parent_code: Some(3372),
            description: "Office Furniture (including Fixtures) Manufacturing".to_string(),
        }),
        (337211, Code::<u32> {
            code: 337211,
            parent_code: Some(33721),
            description: "Wood Office Furniture Manufacturing ".to_string(),
        }),
        (337212, Code::<u32> {
            code: 337212,
            parent_code: Some(33721),
            description: "Custom Architectural Woodwork and Millwork Manufacturing ".to_string(),
        }),
        (337214, Code::<u32> {
            code: 337214,
            parent_code: Some(33721),
            description: "Office Furniture (except Wood) Manufacturing ".to_string(),
        }),
        (337215, Code::<u32> {
            code: 337215,
            parent_code: Some(33721),
            description: "Showcase, Partition, Shelving, and Locker Manufacturing ".to_string(),
        }),
        (3379, Code::<u32> {
            code: 3379,
            parent_code: Some(337),
            description: "Other Furniture Related Product Manufacturing".to_string(),
        }),
        (33791, Code::<u32> {
            code: 33791,
            parent_code: Some(3379),
            description: "Mattress Manufacturing".to_string(),
        }),
        (337910, Code::<u32> {
            code: 337910,
            parent_code: Some(33791),
            description: "Mattress Manufacturing".to_string(),
        }),
        (33792, Code::<u32> {
            code: 33792,
            parent_code: Some(3379),
            description: "Blind and Shade Manufacturing".to_string(),
        }),
        (337920, Code::<u32> {
            code: 337920,
            parent_code: Some(33792),
            description: "Blind and Shade Manufacturing".to_string(),
        }),
        (339, Code::<u32> {
            code: 339,
            parent_code: Some(33),
            description: "Miscellaneous Manufacturing".to_string(),
        }),
        (3391, Code::<u32> {
            code: 3391,
            parent_code: Some(339),
            description: "Medical Equipment and Supplies Manufacturing".to_string(),
        }),
        (33911, Code::<u32> {
            code: 33911,
            parent_code: Some(3391),
            description: "Medical Equipment and Supplies Manufacturing".to_string(),
        }),
        (339112, Code::<u32> {
            code: 339112,
            parent_code: Some(33911),
            description: "Surgical and Medical Instrument Manufacturing ".to_string(),
        }),
        (339113, Code::<u32> {
            code: 339113,
            parent_code: Some(33911),
            description: "Surgical Appliance and Supplies Manufacturing ".to_string(),
        }),
        (339114, Code::<u32> {
            code: 339114,
            parent_code: Some(33911),
            description: "Dental Equipment and Supplies Manufacturing ".to_string(),
        }),
        (339115, Code::<u32> {
            code: 339115,
            parent_code: Some(33911),
            description: "Ophthalmic Goods Manufacturing ".to_string(),
        }),
        (339116, Code::<u32> {
            code: 339116,
            parent_code: Some(33911),
            description: "Dental Laboratories ".to_string(),
        }),
        (3399, Code::<u32> {
            code: 3399,
            parent_code: Some(339),
            description: "Other Miscellaneous Manufacturing".to_string(),
        }),
        (33991, Code::<u32> {
            code: 33991,
            parent_code: Some(3399),
            description: "Jewelry and Silverware Manufacturing".to_string(),
        }),
        (339910, Code::<u32> {
            code: 339910,
            parent_code: Some(33991),
            description: "Jewelry and Silverware Manufacturing ".to_string(),
        }),
        (33992, Code::<u32> {
            code: 33992,
            parent_code: Some(3399),
            description: "Sporting and Athletic Goods Manufacturing".to_string(),
        }),
        (339920, Code::<u32> {
            code: 339920,
            parent_code: Some(33992),
            description: "Sporting and Athletic Goods Manufacturing".to_string(),
        }),
        (33993, Code::<u32> {
            code: 33993,
            parent_code: Some(3399),
            description: "Doll, Toy, and Game Manufacturing".to_string(),
        }),
        (339930, Code::<u32> {
            code: 339930,
            parent_code: Some(33993),
            description: "Doll, Toy, and Game Manufacturing".to_string(),
        }),
        (33994, Code::<u32> {
            code: 33994,
            parent_code: Some(3399),
            description: "Office Supplies (except Paper) Manufacturing".to_string(),
        }),
        (339940, Code::<u32> {
            code: 339940,
            parent_code: Some(33994),
            description: "Office Supplies (except Paper) Manufacturing".to_string(),
        }),
        (33995, Code::<u32> {
            code: 33995,
            parent_code: Some(3399),
            description: "Sign Manufacturing".to_string(),
        }),
        (339950, Code::<u32> {
            code: 339950,
            parent_code: Some(33995),
            description: "Sign Manufacturing".to_string(),
        }),
        (33999, Code::<u32> {
            code: 33999,
            parent_code: Some(3399),
            description: "All Other Miscellaneous Manufacturing".to_string(),
        }),
        (339991, Code::<u32> {
            code: 339991,
            parent_code: Some(33999),
            description: "Gasket, Packing, and Sealing Device Manufacturing ".to_string(),
        }),
        (339992, Code::<u32> {
            code: 339992,
            parent_code: Some(33999),
            description: "Musical Instrument Manufacturing ".to_string(),
        }),
        (339993, Code::<u32> {
            code: 339993,
            parent_code: Some(33999),
            description: "Fastener, Button, Needle, and Pin Manufacturing ".to_string(),
        }),
        (339994, Code::<u32> {
            code: 339994,
            parent_code: Some(33999),
            description: "Broom, Brush, and Mop Manufacturing ".to_string(),
        }),
        (339995, Code::<u32> {
            code: 339995,
            parent_code: Some(33999),
            description: "Burial Casket Manufacturing ".to_string(),
        }),
        (339999, Code::<u32> {
            code: 339999,
            parent_code: Some(33999),
            description: "All Other Miscellaneous Manufacturing ".to_string(),
        }),
        (42, Code::<u32> {
            code: 42,
            parent_code: None,
            description: "Wholesale Trade".to_string(),
        }),
        (423, Code::<u32> {
            code: 423,
            parent_code: Some(42),
            description: "Merchant Wholesalers, Durable Goods ".to_string(),
        }),
        (4231, Code::<u32> {
            code: 4231,
            parent_code: Some(423),
            description: "Motor Vehicle and Motor Vehicle Parts and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42311, Code::<u32> {
            code: 42311,
            parent_code: Some(4231),
            description: "Automobile and Other Motor Vehicle Merchant Wholesalers ".to_string(),
        }),
        (423110, Code::<u32> {
            code: 423110,
            parent_code: Some(42311),
            description: "Automobile and Other Motor Vehicle Merchant Wholesalers ".to_string(),
        }),
        (42312, Code::<u32> {
            code: 42312,
            parent_code: Some(4231),
            description: "Motor Vehicle Supplies and New Parts Merchant Wholesalers ".to_string(),
        }),
        (423120, Code::<u32> {
            code: 423120,
            parent_code: Some(42312),
            description: "Motor Vehicle Supplies and New Parts Merchant Wholesalers ".to_string(),
        }),
        (42313, Code::<u32> {
            code: 42313,
            parent_code: Some(4231),
            description: "Tire and Tube Merchant Wholesalers ".to_string(),
        }),
        (423130, Code::<u32> {
            code: 423130,
            parent_code: Some(42313),
            description: "Tire and Tube Merchant Wholesalers ".to_string(),
        }),
        (42314, Code::<u32> {
            code: 42314,
            parent_code: Some(4231),
            description: "Motor Vehicle Parts (Used) Merchant Wholesalers ".to_string(),
        }),
        (423140, Code::<u32> {
            code: 423140,
            parent_code: Some(42314),
            description: "Motor Vehicle Parts (Used) Merchant Wholesalers ".to_string(),
        }),
        (4232, Code::<u32> {
            code: 4232,
            parent_code: Some(423),
            description: "Furniture and Home Furnishing Merchant Wholesalers ".to_string(),
        }),
        (42321, Code::<u32> {
            code: 42321,
            parent_code: Some(4232),
            description: "Furniture Merchant Wholesalers ".to_string(),
        }),
        (423210, Code::<u32> {
            code: 423210,
            parent_code: Some(42321),
            description: "Furniture Merchant Wholesalers ".to_string(),
        }),
        (42322, Code::<u32> {
            code: 42322,
            parent_code: Some(4232),
            description: "Home Furnishing Merchant Wholesalers ".to_string(),
        }),
        (423220, Code::<u32> {
            code: 423220,
            parent_code: Some(42322),
            description: "Home Furnishing Merchant Wholesalers ".to_string(),
        }),
        (4233, Code::<u32> {
            code: 4233,
            parent_code: Some(423),
            description: "Lumber and Other Construction Materials Merchant Wholesalers ".to_string(),
        }),
        (42331, Code::<u32> {
            code: 42331,
            parent_code: Some(4233),
            description: "Lumber, Plywood, Millwork, and Wood Panel Merchant Wholesalers ".to_string(),
        }),
        (423310, Code::<u32> {
            code: 423310,
            parent_code: Some(42331),
            description: "Lumber, Plywood, Millwork, and Wood Panel Merchant Wholesalers ".to_string(),
        }),
        (42332, Code::<u32> {
            code: 42332,
            parent_code: Some(4233),
            description: "Brick, Stone, and Related Construction Material Merchant Wholesalers ".to_string(),
        }),
        (423320, Code::<u32> {
            code: 423320,
            parent_code: Some(42332),
            description: "Brick, Stone, and Related Construction Material Merchant Wholesalers ".to_string(),
        }),
        (42333, Code::<u32> {
            code: 42333,
            parent_code: Some(4233),
            description: "Roofing, Siding, and Insulation Material Merchant Wholesalers ".to_string(),
        }),
        (423330, Code::<u32> {
            code: 423330,
            parent_code: Some(42333),
            description: "Roofing, Siding, and Insulation Material Merchant Wholesalers ".to_string(),
        }),
        (42339, Code::<u32> {
            code: 42339,
            parent_code: Some(4233),
            description: "Other Construction Material Merchant Wholesalers ".to_string(),
        }),
        (423390, Code::<u32> {
            code: 423390,
            parent_code: Some(42339),
            description: "Other Construction Material Merchant Wholesalers ".to_string(),
        }),
        (4234, Code::<u32> {
            code: 4234,
            parent_code: Some(423),
            description: "Professional and Commercial Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42341, Code::<u32> {
            code: 42341,
            parent_code: Some(4234),
            description: "Photographic Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423410, Code::<u32> {
            code: 423410,
            parent_code: Some(42341),
            description: "Photographic Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42342, Code::<u32> {
            code: 42342,
            parent_code: Some(4234),
            description: "Office Equipment Merchant Wholesalers ".to_string(),
        }),
        (423420, Code::<u32> {
            code: 423420,
            parent_code: Some(42342),
            description: "Office Equipment Merchant Wholesalers ".to_string(),
        }),
        (42343, Code::<u32> {
            code: 42343,
            parent_code: Some(4234),
            description: "Computer and Computer Peripheral Equipment and Software Merchant Wholesalers ".to_string(),
        }),
        (423430, Code::<u32> {
            code: 423430,
            parent_code: Some(42343),
            description: "Computer and Computer Peripheral Equipment and Software Merchant Wholesalers ".to_string(),
        }),
        (42344, Code::<u32> {
            code: 42344,
            parent_code: Some(4234),
            description: "Other Commercial Equipment Merchant Wholesalers ".to_string(),
        }),
        (423440, Code::<u32> {
            code: 423440,
            parent_code: Some(42344),
            description: "Other Commercial Equipment Merchant Wholesalers ".to_string(),
        }),
        (42345, Code::<u32> {
            code: 42345,
            parent_code: Some(4234),
            description: "Medical, Dental, and Hospital Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423450, Code::<u32> {
            code: 423450,
            parent_code: Some(42345),
            description: "Medical, Dental, and Hospital Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42346, Code::<u32> {
            code: 42346,
            parent_code: Some(4234),
            description: "Ophthalmic Goods Merchant Wholesalers ".to_string(),
        }),
        (423460, Code::<u32> {
            code: 423460,
            parent_code: Some(42346),
            description: "Ophthalmic Goods Merchant Wholesalers ".to_string(),
        }),
        (42349, Code::<u32> {
            code: 42349,
            parent_code: Some(4234),
            description: "Other Professional Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423490, Code::<u32> {
            code: 423490,
            parent_code: Some(42349),
            description: "Other Professional Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (4235, Code::<u32> {
            code: 4235,
            parent_code: Some(423),
            description: "Metal and Mineral (except Petroleum) Merchant Wholesalers ".to_string(),
        }),
        (42351, Code::<u32> {
            code: 42351,
            parent_code: Some(4235),
            description: "Metal Service Centers and Other Metal Merchant Wholesalers ".to_string(),
        }),
        (423510, Code::<u32> {
            code: 423510,
            parent_code: Some(42351),
            description: "Metal Service Centers and Other Metal Merchant Wholesalers ".to_string(),
        }),
        (42352, Code::<u32> {
            code: 42352,
            parent_code: Some(4235),
            description: "Coal and Other Mineral and Ore Merchant Wholesalers ".to_string(),
        }),
        (423520, Code::<u32> {
            code: 423520,
            parent_code: Some(42352),
            description: "Coal and Other Mineral and Ore Merchant Wholesalers ".to_string(),
        }),
        (4236, Code::<u32> {
            code: 4236,
            parent_code: Some(423),
            description: "Household Appliances and Electrical and Electronic Goods Merchant Wholesalers ".to_string(),
        }),
        (42361, Code::<u32> {
            code: 42361,
            parent_code: Some(4236),
            description: "Electrical Apparatus and Equipment, Wiring Supplies, and Related Equipment Merchant Wholesalers ".to_string(),
        }),
        (423610, Code::<u32> {
            code: 423610,
            parent_code: Some(42361),
            description: "Electrical Apparatus and Equipment, Wiring Supplies, and Related Equipment Merchant Wholesalers ".to_string(),
        }),
        (42362, Code::<u32> {
            code: 42362,
            parent_code: Some(4236),
            description: "Household Appliances, Electric Housewares, and Consumer Electronics Merchant Wholesalers ".to_string(),
        }),
        (423620, Code::<u32> {
            code: 423620,
            parent_code: Some(42362),
            description: "Household Appliances, Electric Housewares, and Consumer Electronics Merchant Wholesalers ".to_string(),
        }),
        (42369, Code::<u32> {
            code: 42369,
            parent_code: Some(4236),
            description: "Other Electronic Parts and Equipment Merchant Wholesalers ".to_string(),
        }),
        (423690, Code::<u32> {
            code: 423690,
            parent_code: Some(42369),
            description: "Other Electronic Parts and Equipment Merchant Wholesalers ".to_string(),
        }),
        (4237, Code::<u32> {
            code: 4237,
            parent_code: Some(423),
            description: "Hardware, and Plumbing and Heating Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42371, Code::<u32> {
            code: 42371,
            parent_code: Some(4237),
            description: "Hardware Merchant Wholesalers ".to_string(),
        }),
        (423710, Code::<u32> {
            code: 423710,
            parent_code: Some(42371),
            description: "Hardware Merchant Wholesalers ".to_string(),
        }),
        (42372, Code::<u32> {
            code: 42372,
            parent_code: Some(4237),
            description: "Plumbing and Heating Equipment and Supplies (Hydronics) Merchant Wholesalers ".to_string(),
        }),
        (423720, Code::<u32> {
            code: 423720,
            parent_code: Some(42372),
            description: "Plumbing and Heating Equipment and Supplies (Hydronics) Merchant Wholesalers ".to_string(),
        }),
        (42373, Code::<u32> {
            code: 42373,
            parent_code: Some(4237),
            description: "Warm Air Heating and Air-Conditioning Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423730, Code::<u32> {
            code: 423730,
            parent_code: Some(42373),
            description: "Warm Air Heating and Air-Conditioning Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42374, Code::<u32> {
            code: 42374,
            parent_code: Some(4237),
            description: "Refrigeration Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423740, Code::<u32> {
            code: 423740,
            parent_code: Some(42374),
            description: "Refrigeration Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (4238, Code::<u32> {
            code: 4238,
            parent_code: Some(423),
            description: "Machinery, Equipment, and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42381, Code::<u32> {
            code: 42381,
            parent_code: Some(4238),
            description: "Construction and Mining (except Oil Well) Machinery and Equipment Merchant Wholesalers ".to_string(),
        }),
        (423810, Code::<u32> {
            code: 423810,
            parent_code: Some(42381),
            description: "Construction and Mining (except Oil Well) Machinery and Equipment Merchant Wholesalers ".to_string(),
        }),
        (42382, Code::<u32> {
            code: 42382,
            parent_code: Some(4238),
            description: "Farm and Garden Machinery and Equipment Merchant Wholesalers ".to_string(),
        }),
        (423820, Code::<u32> {
            code: 423820,
            parent_code: Some(42382),
            description: "Farm and Garden Machinery and Equipment Merchant Wholesalers ".to_string(),
        }),
        (42383, Code::<u32> {
            code: 42383,
            parent_code: Some(4238),
            description: "Industrial Machinery and Equipment Merchant Wholesalers ".to_string(),
        }),
        (423830, Code::<u32> {
            code: 423830,
            parent_code: Some(42383),
            description: "Industrial Machinery and Equipment Merchant Wholesalers ".to_string(),
        }),
        (42384, Code::<u32> {
            code: 42384,
            parent_code: Some(4238),
            description: "Industrial Supplies Merchant Wholesalers ".to_string(),
        }),
        (423840, Code::<u32> {
            code: 423840,
            parent_code: Some(42384),
            description: "Industrial Supplies Merchant Wholesalers".to_string(),
        }),
        (42385, Code::<u32> {
            code: 42385,
            parent_code: Some(4238),
            description: "Service Establishment Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423850, Code::<u32> {
            code: 423850,
            parent_code: Some(42385),
            description: "Service Establishment Equipment and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42386, Code::<u32> {
            code: 42386,
            parent_code: Some(4238),
            description: "Transportation Equipment and Supplies (except Motor Vehicle) Merchant Wholesalers ".to_string(),
        }),
        (423860, Code::<u32> {
            code: 423860,
            parent_code: Some(42386),
            description: "Transportation Equipment and Supplies (except Motor Vehicle) Merchant Wholesalers ".to_string(),
        }),
        (4239, Code::<u32> {
            code: 4239,
            parent_code: Some(423),
            description: "Miscellaneous Durable Goods Merchant Wholesalers ".to_string(),
        }),
        (42391, Code::<u32> {
            code: 42391,
            parent_code: Some(4239),
            description: "Sporting and Recreational Goods and Supplies Merchant Wholesalers".to_string(),
        }),
        (423910, Code::<u32> {
            code: 423910,
            parent_code: Some(42391),
            description: "Sporting and Recreational Goods and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42392, Code::<u32> {
            code: 42392,
            parent_code: Some(4239),
            description: "Toy and Hobby Goods and Supplies Merchant Wholesalers ".to_string(),
        }),
        (423920, Code::<u32> {
            code: 423920,
            parent_code: Some(42392),
            description: "Toy and Hobby Goods and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42393, Code::<u32> {
            code: 42393,
            parent_code: Some(4239),
            description: "Recyclable Material Merchant Wholesalers ".to_string(),
        }),
        (423930, Code::<u32> {
            code: 423930,
            parent_code: Some(42393),
            description: "Recyclable Material Merchant Wholesalers ".to_string(),
        }),
        (42394, Code::<u32> {
            code: 42394,
            parent_code: Some(4239),
            description: "Jewelry, Watch, Precious Stone, and Precious Metal Merchant Wholesalers ".to_string(),
        }),
        (423940, Code::<u32> {
            code: 423940,
            parent_code: Some(42394),
            description: "Jewelry, Watch, Precious Stone, and Precious Metal Merchant Wholesalers ".to_string(),
        }),
        (42399, Code::<u32> {
            code: 42399,
            parent_code: Some(4239),
            description: "Other Miscellaneous Durable Goods Merchant Wholesalers ".to_string(),
        }),
        (423990, Code::<u32> {
            code: 423990,
            parent_code: Some(42399),
            description: "Other Miscellaneous Durable Goods Merchant Wholesalers ".to_string(),
        }),
        (424, Code::<u32> {
            code: 424,
            parent_code: Some(42),
            description: "Merchant Wholesalers, Nondurable Goods ".to_string(),
        }),
        (4241, Code::<u32> {
            code: 4241,
            parent_code: Some(424),
            description: "Paper and Paper Product Merchant Wholesalers ".to_string(),
        }),
        (42411, Code::<u32> {
            code: 42411,
            parent_code: Some(4241),
            description: "Printing and Writing Paper Merchant Wholesalers ".to_string(),
        }),
        (424110, Code::<u32> {
            code: 424110,
            parent_code: Some(42411),
            description: "Printing and Writing Paper Merchant Wholesalers ".to_string(),
        }),
        (42412, Code::<u32> {
            code: 42412,
            parent_code: Some(4241),
            description: "Stationery and Office Supplies Merchant Wholesalers ".to_string(),
        }),
        (424120, Code::<u32> {
            code: 424120,
            parent_code: Some(42412),
            description: "Stationery and Office Supplies Merchant Wholesalers ".to_string(),
        }),
        (42413, Code::<u32> {
            code: 42413,
            parent_code: Some(4241),
            description: "Industrial and Personal Service Paper Merchant Wholesalers ".to_string(),
        }),
        (424130, Code::<u32> {
            code: 424130,
            parent_code: Some(42413),
            description: "Industrial and Personal Service Paper Merchant Wholesalers ".to_string(),
        }),
        (4242, Code::<u32> {
            code: 4242,
            parent_code: Some(424),
            description: "Drugs and Druggists' Sundries Merchant Wholesalers ".to_string(),
        }),
        (42421, Code::<u32> {
            code: 42421,
            parent_code: Some(4242),
            description: "Drugs and Druggists' Sundries Merchant Wholesalers ".to_string(),
        }),
        (424210, Code::<u32> {
            code: 424210,
            parent_code: Some(42421),
            description: "Drugs and Druggists' Sundries Merchant Wholesalers ".to_string(),
        }),
        (4243, Code::<u32> {
            code: 4243,
            parent_code: Some(424),
            description: "Apparel, Piece Goods, and Notions Merchant Wholesalers ".to_string(),
        }),
        (42431, Code::<u32> {
            code: 42431,
            parent_code: Some(4243),
            description: "Piece Goods, Notions, and Other Dry Goods Merchant Wholesalers ".to_string(),
        }),
        (424310, Code::<u32> {
            code: 424310,
            parent_code: Some(42431),
            description: "Piece Goods, Notions, and Other Dry Goods Merchant Wholesalers ".to_string(),
        }),
        (42432, Code::<u32> {
            code: 42432,
            parent_code: Some(4243),
            description: "Men's and Boys' Clothing and Furnishings Merchant Wholesalers ".to_string(),
        }),
        (424320, Code::<u32> {
            code: 424320,
            parent_code: Some(42432),
            description: "Men's and Boys' Clothing and Furnishings Merchant Wholesalers ".to_string(),
        }),
        (42433, Code::<u32> {
            code: 42433,
            parent_code: Some(4243),
            description: "Women's, Children's, and Infants' Clothing and Accessories Merchant Wholesalers ".to_string(),
        }),
        (424330, Code::<u32> {
            code: 424330,
            parent_code: Some(42433),
            description: "Women's, Children's, and Infants' Clothing and Accessories Merchant Wholesalers ".to_string(),
        }),
        (42434, Code::<u32> {
            code: 42434,
            parent_code: Some(4243),
            description: "Footwear Merchant Wholesalers ".to_string(),
        }),
        (424340, Code::<u32> {
            code: 424340,
            parent_code: Some(42434),
            description: "Footwear Merchant Wholesalers ".to_string(),
        }),
        (4244, Code::<u32> {
            code: 4244,
            parent_code: Some(424),
            description: "Grocery and Related Product Merchant Wholesalers ".to_string(),
        }),
        (42441, Code::<u32> {
            code: 42441,
            parent_code: Some(4244),
            description: "General Line Grocery Merchant Wholesalers ".to_string(),
        }),
        (424410, Code::<u32> {
            code: 424410,
            parent_code: Some(42441),
            description: "General Line Grocery Merchant Wholesalers ".to_string(),
        }),
        (42442, Code::<u32> {
            code: 42442,
            parent_code: Some(4244),
            description: "Packaged Frozen Food Merchant Wholesalers ".to_string(),
        }),
        (424420, Code::<u32> {
            code: 424420,
            parent_code: Some(42442),
            description: "Packaged Frozen Food Merchant Wholesalers ".to_string(),
        }),
        (42443, Code::<u32> {
            code: 42443,
            parent_code: Some(4244),
            description: "Dairy Product (except Dried or Canned) Merchant Wholesalers ".to_string(),
        }),
        (424430, Code::<u32> {
            code: 424430,
            parent_code: Some(42443),
            description: "Dairy Product (except Dried or Canned) Merchant Wholesalers ".to_string(),
        }),
        (42444, Code::<u32> {
            code: 42444,
            parent_code: Some(4244),
            description: "Poultry and Poultry Product Merchant Wholesalers ".to_string(),
        }),
        (424440, Code::<u32> {
            code: 424440,
            parent_code: Some(42444),
            description: "Poultry and Poultry Product Merchant Wholesalers ".to_string(),
        }),
        (42445, Code::<u32> {
            code: 42445,
            parent_code: Some(4244),
            description: "Confectionery Merchant Wholesalers ".to_string(),
        }),
        (424450, Code::<u32> {
            code: 424450,
            parent_code: Some(42445),
            description: "Confectionery Merchant Wholesalers ".to_string(),
        }),
        (42446, Code::<u32> {
            code: 42446,
            parent_code: Some(4244),
            description: "Fish and Seafood Merchant Wholesalers ".to_string(),
        }),
        (424460, Code::<u32> {
            code: 424460,
            parent_code: Some(42446),
            description: "Fish and Seafood Merchant Wholesalers ".to_string(),
        }),
        (42447, Code::<u32> {
            code: 42447,
            parent_code: Some(4244),
            description: "Meat and Meat Product Merchant Wholesalers ".to_string(),
        }),
        (424470, Code::<u32> {
            code: 424470,
            parent_code: Some(42447),
            description: "Meat and Meat Product Merchant Wholesalers ".to_string(),
        }),
        (42448, Code::<u32> {
            code: 42448,
            parent_code: Some(4244),
            description: "Fresh Fruit and Vegetable Merchant Wholesalers ".to_string(),
        }),
        (424480, Code::<u32> {
            code: 424480,
            parent_code: Some(42448),
            description: "Fresh Fruit and Vegetable Merchant Wholesalers ".to_string(),
        }),
        (42449, Code::<u32> {
            code: 42449,
            parent_code: Some(4244),
            description: "Other Grocery and Related Products Merchant Wholesalers ".to_string(),
        }),
        (424490, Code::<u32> {
            code: 424490,
            parent_code: Some(42449),
            description: "Other Grocery and Related Products Merchant Wholesalers ".to_string(),
        }),
        (4245, Code::<u32> {
            code: 4245,
            parent_code: Some(424),
            description: "Farm Product Raw Material Merchant Wholesalers ".to_string(),
        }),
        (42451, Code::<u32> {
            code: 42451,
            parent_code: Some(4245),
            description: "Grain and Field Bean Merchant Wholesalers ".to_string(),
        }),
        (424510, Code::<u32> {
            code: 424510,
            parent_code: Some(42451),
            description: "Grain and Field Bean Merchant Wholesalers ".to_string(),
        }),
        (42452, Code::<u32> {
            code: 42452,
            parent_code: Some(4245),
            description: "Livestock Merchant Wholesalers ".to_string(),
        }),
        (424520, Code::<u32> {
            code: 424520,
            parent_code: Some(42452),
            description: "Livestock Merchant Wholesalers ".to_string(),
        }),
        (42459, Code::<u32> {
            code: 42459,
            parent_code: Some(4245),
            description: "Other Farm Product Raw Material Merchant Wholesalers ".to_string(),
        }),
        (424590, Code::<u32> {
            code: 424590,
            parent_code: Some(42459),
            description: "Other Farm Product Raw Material Merchant Wholesalers ".to_string(),
        }),
        (4246, Code::<u32> {
            code: 4246,
            parent_code: Some(424),
            description: "Chemical and Allied Products Merchant Wholesalers ".to_string(),
        }),
        (42461, Code::<u32> {
            code: 42461,
            parent_code: Some(4246),
            description: "Plastics Materials and Basic Forms and Shapes Merchant Wholesalers ".to_string(),
        }),
        (424610, Code::<u32> {
            code: 424610,
            parent_code: Some(42461),
            description: "Plastics Materials and Basic Forms and Shapes Merchant Wholesalers ".to_string(),
        }),
        (42469, Code::<u32> {
            code: 42469,
            parent_code: Some(4246),
            description: "Other Chemical and Allied Products Merchant Wholesalers ".to_string(),
        }),
        (424690, Code::<u32> {
            code: 424690,
            parent_code: Some(42469),
            description: "Other Chemical and Allied Products Merchant Wholesalers ".to_string(),
        }),
        (4247, Code::<u32> {
            code: 4247,
            parent_code: Some(424),
            description: "Petroleum and Petroleum Products Merchant Wholesalers ".to_string(),
        }),
        (42471, Code::<u32> {
            code: 42471,
            parent_code: Some(4247),
            description: "Petroleum Bulk Stations and Terminals ".to_string(),
        }),
        (424710, Code::<u32> {
            code: 424710,
            parent_code: Some(42471),
            description: "Petroleum Bulk Stations and Terminals ".to_string(),
        }),
        (42472, Code::<u32> {
            code: 42472,
            parent_code: Some(4247),
            description: "Petroleum and Petroleum Products Merchant Wholesalers (except Bulk Stations and Terminals) ".to_string(),
        }),
        (424720, Code::<u32> {
            code: 424720,
            parent_code: Some(42472),
            description: "Petroleum and Petroleum Products Merchant Wholesalers (except Bulk Stations and Terminals) ".to_string(),
        }),
        (4248, Code::<u32> {
            code: 4248,
            parent_code: Some(424),
            description: "Beer, Wine, and Distilled Alcoholic Beverage Merchant Wholesalers ".to_string(),
        }),
        (42481, Code::<u32> {
            code: 42481,
            parent_code: Some(4248),
            description: "Beer and Ale Merchant Wholesalers ".to_string(),
        }),
        (424810, Code::<u32> {
            code: 424810,
            parent_code: Some(42481),
            description: "Beer and Ale Merchant Wholesalers ".to_string(),
        }),
        (42482, Code::<u32> {
            code: 42482,
            parent_code: Some(4248),
            description: "Wine and Distilled Alcoholic Beverage Merchant Wholesalers ".to_string(),
        }),
        (424820, Code::<u32> {
            code: 424820,
            parent_code: Some(42482),
            description: "Wine and Distilled Alcoholic Beverage Merchant Wholesalers ".to_string(),
        }),
        (4249, Code::<u32> {
            code: 4249,
            parent_code: Some(424),
            description: "Miscellaneous Nondurable Goods Merchant Wholesalers ".to_string(),
        }),
        (42491, Code::<u32> {
            code: 42491,
            parent_code: Some(4249),
            description: "Farm Supplies Merchant Wholesalers ".to_string(),
        }),
        (424910, Code::<u32> {
            code: 424910,
            parent_code: Some(42491),
            description: "Farm Supplies Merchant Wholesalers ".to_string(),
        }),
        (42492, Code::<u32> {
            code: 42492,
            parent_code: Some(4249),
            description: "Book, Periodical, and Newspaper Merchant Wholesalers ".to_string(),
        }),
        (424920, Code::<u32> {
            code: 424920,
            parent_code: Some(42492),
            description: "Book, Periodical, and Newspaper Merchant Wholesalers ".to_string(),
        }),
        (42493, Code::<u32> {
            code: 42493,
            parent_code: Some(4249),
            description: "Flower, Nursery Stock, and Florists' Supplies Merchant Wholesalers ".to_string(),
        }),
        (424930, Code::<u32> {
            code: 424930,
            parent_code: Some(42493),
            description: "Flower, Nursery Stock, and Florists' Supplies Merchant Wholesalers ".to_string(),
        }),
        (42494, Code::<u32> {
            code: 42494,
            parent_code: Some(4249),
            description: "Tobacco and Tobacco Product Merchant Wholesalers ".to_string(),
        }),
        (424940, Code::<u32> {
            code: 424940,
            parent_code: Some(42494),
            description: "Tobacco and Tobacco Product Merchant Wholesalers ".to_string(),
        }),
        (42495, Code::<u32> {
            code: 42495,
            parent_code: Some(4249),
            description: "Paint, Varnish, and Supplies Merchant Wholesalers ".to_string(),
        }),
        (424950, Code::<u32> {
            code: 424950,
            parent_code: Some(42495),
            description: "Paint, Varnish, and Supplies Merchant Wholesalers ".to_string(),
        }),
        (42499, Code::<u32> {
            code: 42499,
            parent_code: Some(4249),
            description: "Other Miscellaneous Nondurable Goods Merchant Wholesalers ".to_string(),
        }),
        (424990, Code::<u32> {
            code: 424990,
            parent_code: Some(42499),
            description: "Other Miscellaneous Nondurable Goods Merchant Wholesalers ".to_string(),
        }),
        (425, Code::<u32> {
            code: 425,
            parent_code: Some(42),
            description: "Wholesale Electronic Markets and Agents and Brokers ".to_string(),
        }),
        (4251, Code::<u32> {
            code: 4251,
            parent_code: Some(425),
            description: "Wholesale Electronic Markets and Agents and Brokers ".to_string(),
        }),
        (42511, Code::<u32> {
            code: 42511,
            parent_code: Some(4251),
            description: "Business to Business Electronic Markets ".to_string(),
        }),
        (425110, Code::<u32> {
            code: 425110,
            parent_code: Some(42511),
            description: "Business to Business Electronic Markets ".to_string(),
        }),
        (42512, Code::<u32> {
            code: 42512,
            parent_code: Some(4251),
            description: "Wholesale Trade Agents and Brokers ".to_string(),
        }),
        (425120, Code::<u32> {
            code: 425120,
            parent_code: Some(42512),
            description: "Wholesale Trade Agents and Brokers ".to_string(),
        }),
        (44, Code::<u32> {
            code: 44,
            parent_code: None,
            description: "Retail Trade".to_string(),
        }),
        (45, Code::<u32> {
            code: 45,
            parent_code: None,
            description: "Retail Trade".to_string(),
        }),
        (441, Code::<u32> {
            code: 441,
            parent_code: Some(44),
            description: "Motor Vehicle and Parts Dealers ".to_string(),
        }),
        (4411, Code::<u32> {
            code: 4411,
            parent_code: Some(441),
            description: "Automobile Dealers ".to_string(),
        }),
        (44111, Code::<u32> {
            code: 44111,
            parent_code: Some(4411),
            description: "New Car Dealers ".to_string(),
        }),
        (441110, Code::<u32> {
            code: 441110,
            parent_code: Some(44111),
            description: "New Car Dealers ".to_string(),
        }),
        (44112, Code::<u32> {
            code: 44112,
            parent_code: Some(4411),
            description: "Used Car Dealers ".to_string(),
        }),
        (441120, Code::<u32> {
            code: 441120,
            parent_code: Some(44112),
            description: "Used Car Dealers ".to_string(),
        }),
        (4412, Code::<u32> {
            code: 4412,
            parent_code: Some(441),
            description: "Other Motor Vehicle Dealers ".to_string(),
        }),
        (44121, Code::<u32> {
            code: 44121,
            parent_code: Some(4412),
            description: "Recreational Vehicle Dealers ".to_string(),
        }),
        (441210, Code::<u32> {
            code: 441210,
            parent_code: Some(44121),
            description: "Recreational Vehicle Dealers ".to_string(),
        }),
        (44122, Code::<u32> {
            code: 44122,
            parent_code: Some(4412),
            description: "Motorcycle, Boat, and Other Motor Vehicle Dealers ".to_string(),
        }),
        (441222, Code::<u32> {
            code: 441222,
            parent_code: Some(44122),
            description: "Boat Dealers ".to_string(),
        }),
        (441228, Code::<u32> {
            code: 441228,
            parent_code: Some(44122),
            description: "Motorcycle, ATV, and All Other Motor Vehicle Dealers ".to_string(),
        }),
        (4413, Code::<u32> {
            code: 4413,
            parent_code: Some(441),
            description: "Automotive Parts, Accessories, and Tire Stores ".to_string(),
        }),
        (44131, Code::<u32> {
            code: 44131,
            parent_code: Some(4413),
            description: "Automotive Parts and Accessories Stores ".to_string(),
        }),
        (441310, Code::<u32> {
            code: 441310,
            parent_code: Some(44131),
            description: "Automotive Parts and Accessories Stores ".to_string(),
        }),
        (44132, Code::<u32> {
            code: 44132,
            parent_code: Some(4413),
            description: "Tire Dealers ".to_string(),
        }),
        (441320, Code::<u32> {
            code: 441320,
            parent_code: Some(44132),
            description: "Tire Dealers ".to_string(),
        }),
        (442, Code::<u32> {
            code: 442,
            parent_code: Some(44),
            description: "Furniture and Home Furnishings Stores ".to_string(),
        }),
        (4421, Code::<u32> {
            code: 4421,
            parent_code: Some(442),
            description: "Furniture Stores ".to_string(),
        }),
        (44211, Code::<u32> {
            code: 44211,
            parent_code: Some(4421),
            description: "Furniture Stores ".to_string(),
        }),
        (442110, Code::<u32> {
            code: 442110,
            parent_code: Some(44211),
            description: "Furniture Stores ".to_string(),
        }),
        (4422, Code::<u32> {
            code: 4422,
            parent_code: Some(442),
            description: "Home Furnishings Stores ".to_string(),
        }),
        (44221, Code::<u32> {
            code: 44221,
            parent_code: Some(4422),
            description: "Floor Covering Stores ".to_string(),
        }),
        (442210, Code::<u32> {
            code: 442210,
            parent_code: Some(44221),
            description: "Floor Covering Stores ".to_string(),
        }),
        (44229, Code::<u32> {
            code: 44229,
            parent_code: Some(4422),
            description: "Other Home Furnishings Stores ".to_string(),
        }),
        (442291, Code::<u32> {
            code: 442291,
            parent_code: Some(44229),
            description: "Window Treatment Stores ".to_string(),
        }),
        (442299, Code::<u32> {
            code: 442299,
            parent_code: Some(44229),
            description: "All Other Home Furnishings Stores ".to_string(),
        }),
        (443, Code::<u32> {
            code: 443,
            parent_code: Some(44),
            description: "Electronics and Appliance Stores ".to_string(),
        }),
        (4431, Code::<u32> {
            code: 4431,
            parent_code: Some(443),
            description: "Electronics and Appliance Stores ".to_string(),
        }),
        (44314, Code::<u32> {
            code: 44314,
            parent_code: Some(4431),
            description: "Electronics and Appliance Stores ".to_string(),
        }),
        (443141, Code::<u32> {
            code: 443141,
            parent_code: Some(44314),
            description: "Household Appliance Stores ".to_string(),
        }),
        (443142, Code::<u32> {
            code: 443142,
            parent_code: Some(44314),
            description: "Electronics Stores ".to_string(),
        }),
        (444, Code::<u32> {
            code: 444,
            parent_code: Some(44),
            description: "Building Material and Garden Equipment and Supplies Dealers ".to_string(),
        }),
        (4441, Code::<u32> {
            code: 4441,
            parent_code: Some(444),
            description: "Building Material and Supplies Dealers ".to_string(),
        }),
        (44411, Code::<u32> {
            code: 44411,
            parent_code: Some(4441),
            description: "Home Centers ".to_string(),
        }),
        (444110, Code::<u32> {
            code: 444110,
            parent_code: Some(44411),
            description: "Home Centers ".to_string(),
        }),
        (44412, Code::<u32> {
            code: 44412,
            parent_code: Some(4441),
            description: "Paint and Wallpaper Stores ".to_string(),
        }),
        (444120, Code::<u32> {
            code: 444120,
            parent_code: Some(44412),
            description: "Paint and Wallpaper Stores ".to_string(),
        }),
        (44413, Code::<u32> {
            code: 44413,
            parent_code: Some(4441),
            description: "Hardware Stores ".to_string(),
        }),
        (444130, Code::<u32> {
            code: 444130,
            parent_code: Some(44413),
            description: "Hardware Stores ".to_string(),
        }),
        (44419, Code::<u32> {
            code: 44419,
            parent_code: Some(4441),
            description: "Other Building Material Dealers ".to_string(),
        }),
        (444190, Code::<u32> {
            code: 444190,
            parent_code: Some(44419),
            description: "Other Building Material Dealers ".to_string(),
        }),
        (4442, Code::<u32> {
            code: 4442,
            parent_code: Some(444),
            description: "Lawn and Garden Equipment and Supplies Stores ".to_string(),
        }),
        (44421, Code::<u32> {
            code: 44421,
            parent_code: Some(4442),
            description: "Outdoor Power Equipment Stores ".to_string(),
        }),
        (444210, Code::<u32> {
            code: 444210,
            parent_code: Some(44421),
            description: "Outdoor Power Equipment Stores ".to_string(),
        }),
        (44422, Code::<u32> {
            code: 44422,
            parent_code: Some(4442),
            description: "Nursery, Garden Center, and Farm Supply Stores ".to_string(),
        }),
        (444220, Code::<u32> {
            code: 444220,
            parent_code: Some(44422),
            description: "Nursery, Garden Center, and Farm Supply Stores ".to_string(),
        }),
        (445, Code::<u32> {
            code: 445,
            parent_code: Some(44),
            description: "Food and Beverage Stores ".to_string(),
        }),
        (4451, Code::<u32> {
            code: 4451,
            parent_code: Some(445),
            description: "Grocery Stores ".to_string(),
        }),
        (44511, Code::<u32> {
            code: 44511,
            parent_code: Some(4451),
            description: "Supermarkets and Other Grocery (except Convenience) Stores ".to_string(),
        }),
        (445110, Code::<u32> {
            code: 445110,
            parent_code: Some(44511),
            description: "Supermarkets and Other Grocery (except Convenience) Stores ".to_string(),
        }),
        (44512, Code::<u32> {
            code: 44512,
            parent_code: Some(4451),
            description: "Convenience Stores ".to_string(),
        }),
        (445120, Code::<u32> {
            code: 445120,
            parent_code: Some(44512),
            description: "Convenience Stores ".to_string(),
        }),
        (4452, Code::<u32> {
            code: 4452,
            parent_code: Some(445),
            description: "Specialty Food Stores ".to_string(),
        }),
        (44521, Code::<u32> {
            code: 44521,
            parent_code: Some(4452),
            description: "Meat Markets ".to_string(),
        }),
        (445210, Code::<u32> {
            code: 445210,
            parent_code: Some(44521),
            description: "Meat Markets ".to_string(),
        }),
        (44522, Code::<u32> {
            code: 44522,
            parent_code: Some(4452),
            description: "Fish and Seafood Markets ".to_string(),
        }),
        (445220, Code::<u32> {
            code: 445220,
            parent_code: Some(44522),
            description: "Fish and Seafood Markets ".to_string(),
        }),
        (44523, Code::<u32> {
            code: 44523,
            parent_code: Some(4452),
            description: "Fruit and Vegetable Markets ".to_string(),
        }),
        (445230, Code::<u32> {
            code: 445230,
            parent_code: Some(44523),
            description: "Fruit and Vegetable Markets ".to_string(),
        }),
        (44529, Code::<u32> {
            code: 44529,
            parent_code: Some(4452),
            description: "Other Specialty Food Stores ".to_string(),
        }),
        (445291, Code::<u32> {
            code: 445291,
            parent_code: Some(44529),
            description: "Baked Goods Stores ".to_string(),
        }),
        (445292, Code::<u32> {
            code: 445292,
            parent_code: Some(44529),
            description: "Confectionery and Nut Stores ".to_string(),
        }),
        (445299, Code::<u32> {
            code: 445299,
            parent_code: Some(44529),
            description: "All Other Specialty Food Stores ".to_string(),
        }),
        (4453, Code::<u32> {
            code: 4453,
            parent_code: Some(445),
            description: "Beer, Wine, and Liquor Stores ".to_string(),
        }),
        (44531, Code::<u32> {
            code: 44531,
            parent_code: Some(4453),
            description: "Beer, Wine, and Liquor Stores ".to_string(),
        }),
        (445310, Code::<u32> {
            code: 445310,
            parent_code: Some(44531),
            description: "Beer, Wine, and Liquor Stores ".to_string(),
        }),
        (446, Code::<u32> {
            code: 446,
            parent_code: Some(44),
            description: "Health and Personal Care Stores ".to_string(),
        }),
        (4461, Code::<u32> {
            code: 4461,
            parent_code: Some(446),
            description: "Health and Personal Care Stores ".to_string(),
        }),
        (44611, Code::<u32> {
            code: 44611,
            parent_code: Some(4461),
            description: "Pharmacies and Drug Stores ".to_string(),
        }),
        (446110, Code::<u32> {
            code: 446110,
            parent_code: Some(44611),
            description: "Pharmacies and Drug Stores ".to_string(),
        }),
        (44612, Code::<u32> {
            code: 44612,
            parent_code: Some(4461),
            description: "Cosmetics, Beauty Supplies, and Perfume Stores ".to_string(),
        }),
        (446120, Code::<u32> {
            code: 446120,
            parent_code: Some(44612),
            description: "Cosmetics, Beauty Supplies, and Perfume Stores ".to_string(),
        }),
        (44613, Code::<u32> {
            code: 44613,
            parent_code: Some(4461),
            description: "Optical Goods Stores ".to_string(),
        }),
        (446130, Code::<u32> {
            code: 446130,
            parent_code: Some(44613),
            description: "Optical Goods Stores ".to_string(),
        }),
        (44619, Code::<u32> {
            code: 44619,
            parent_code: Some(4461),
            description: "Other Health and Personal Care Stores ".to_string(),
        }),
        (446191, Code::<u32> {
            code: 446191,
            parent_code: Some(44619),
            description: "Food (Health) Supplement Stores ".to_string(),
        }),
        (446199, Code::<u32> {
            code: 446199,
            parent_code: Some(44619),
            description: "All Other Health and Personal Care Stores ".to_string(),
        }),
        (447, Code::<u32> {
            code: 447,
            parent_code: Some(44),
            description: "Gasoline Stations ".to_string(),
        }),
        (4471, Code::<u32> {
            code: 4471,
            parent_code: Some(447),
            description: "Gasoline Stations ".to_string(),
        }),
        (44711, Code::<u32> {
            code: 44711,
            parent_code: Some(4471),
            description: "Gasoline Stations with Convenience Stores ".to_string(),
        }),
        (447110, Code::<u32> {
            code: 447110,
            parent_code: Some(44711),
            description: "Gasoline Stations with Convenience Stores ".to_string(),
        }),
        (44719, Code::<u32> {
            code: 44719,
            parent_code: Some(4471),
            description: "Other Gasoline Stations ".to_string(),
        }),
        (447190, Code::<u32> {
            code: 447190,
            parent_code: Some(44719),
            description: "Other Gasoline Stations ".to_string(),
        }),
        (448, Code::<u32> {
            code: 448,
            parent_code: Some(44),
            description: "Clothing and Clothing Accessories Stores ".to_string(),
        }),
        (4481, Code::<u32> {
            code: 4481,
            parent_code: Some(448),
            description: "Clothing Stores ".to_string(),
        }),
        (44811, Code::<u32> {
            code: 44811,
            parent_code: Some(4481),
            description: "Men's Clothing Stores ".to_string(),
        }),
        (448110, Code::<u32> {
            code: 448110,
            parent_code: Some(44811),
            description: "Men's Clothing Stores ".to_string(),
        }),
        (44812, Code::<u32> {
            code: 44812,
            parent_code: Some(4481),
            description: "Women's Clothing Stores ".to_string(),
        }),
        (448120, Code::<u32> {
            code: 448120,
            parent_code: Some(44812),
            description: "Women's Clothing Stores ".to_string(),
        }),
        (44813, Code::<u32> {
            code: 44813,
            parent_code: Some(4481),
            description: "Children's and Infants' Clothing Stores ".to_string(),
        }),
        (448130, Code::<u32> {
            code: 448130,
            parent_code: Some(44813),
            description: "Children's and Infants' Clothing Stores ".to_string(),
        }),
        (44814, Code::<u32> {
            code: 44814,
            parent_code: Some(4481),
            description: "Family Clothing Stores ".to_string(),
        }),
        (448140, Code::<u32> {
            code: 448140,
            parent_code: Some(44814),
            description: "Family Clothing Stores ".to_string(),
        }),
        (44815, Code::<u32> {
            code: 44815,
            parent_code: Some(4481),
            description: "Clothing Accessories Stores ".to_string(),
        }),
        (448150, Code::<u32> {
            code: 448150,
            parent_code: Some(44815),
            description: "Clothing Accessories Stores ".to_string(),
        }),
        (44819, Code::<u32> {
            code: 44819,
            parent_code: Some(4481),
            description: "Other Clothing Stores ".to_string(),
        }),
        (448190, Code::<u32> {
            code: 448190,
            parent_code: Some(44819),
            description: "Other Clothing Stores ".to_string(),
        }),
        (4482, Code::<u32> {
            code: 4482,
            parent_code: Some(448),
            description: "Shoe Stores ".to_string(),
        }),
        (44821, Code::<u32> {
            code: 44821,
            parent_code: Some(4482),
            description: "Shoe Stores ".to_string(),
        }),
        (448210, Code::<u32> {
            code: 448210,
            parent_code: Some(44821),
            description: "Shoe Stores ".to_string(),
        }),
        (4483, Code::<u32> {
            code: 4483,
            parent_code: Some(448),
            description: "Jewelry, Luggage, and Leather Goods Stores ".to_string(),
        }),
        (44831, Code::<u32> {
            code: 44831,
            parent_code: Some(4483),
            description: "Jewelry Stores ".to_string(),
        }),
        (448310, Code::<u32> {
            code: 448310,
            parent_code: Some(44831),
            description: "Jewelry Stores ".to_string(),
        }),
        (44832, Code::<u32> {
            code: 44832,
            parent_code: Some(4483),
            description: "Luggage and Leather Goods Stores ".to_string(),
        }),
        (448320, Code::<u32> {
            code: 448320,
            parent_code: Some(44832),
            description: "Luggage and Leather Goods Stores ".to_string(),
        }),
        (451, Code::<u32> {
            code: 451,
            parent_code: Some(45),
            description: "Sporting Goods, Hobby, Musical Instrument, and Book Stores ".to_string(),
        }),
        (4511, Code::<u32> {
            code: 4511,
            parent_code: Some(451),
            description: "Sporting Goods, Hobby, and Musical Instrument Stores ".to_string(),
        }),
        (45111, Code::<u32> {
            code: 45111,
            parent_code: Some(4511),
            description: "Sporting Goods Stores ".to_string(),
        }),
        (451110, Code::<u32> {
            code: 451110,
            parent_code: Some(45111),
            description: "Sporting Goods Stores ".to_string(),
        }),
        (45112, Code::<u32> {
            code: 45112,
            parent_code: Some(4511),
            description: "Hobby, Toy, and Game Stores ".to_string(),
        }),
        (451120, Code::<u32> {
            code: 451120,
            parent_code: Some(45112),
            description: "Hobby, Toy, and Game Stores ".to_string(),
        }),
        (45113, Code::<u32> {
            code: 45113,
            parent_code: Some(4511),
            description: "Sewing, Needlework, and Piece Goods Stores ".to_string(),
        }),
        (451130, Code::<u32> {
            code: 451130,
            parent_code: Some(45113),
            description: "Sewing, Needlework, and Piece Goods Stores ".to_string(),
        }),
        (45114, Code::<u32> {
            code: 45114,
            parent_code: Some(4511),
            description: "Musical Instrument and Supplies Stores ".to_string(),
        }),
        (451140, Code::<u32> {
            code: 451140,
            parent_code: Some(45114),
            description: "Musical Instrument and Supplies Stores ".to_string(),
        }),
        (4512, Code::<u32> {
            code: 4512,
            parent_code: Some(451),
            description: "Book Stores and News Dealers ".to_string(),
        }),
        (45121, Code::<u32> {
            code: 45121,
            parent_code: Some(4512),
            description: "Book Stores and News Dealers ".to_string(),
        }),
        (451211, Code::<u32> {
            code: 451211,
            parent_code: Some(45121),
            description: "Book Stores ".to_string(),
        }),
        (451212, Code::<u32> {
            code: 451212,
            parent_code: Some(45121),
            description: "News Dealers and Newsstands ".to_string(),
        }),
        (452, Code::<u32> {
            code: 452,
            parent_code: Some(45),
            description: "General Merchandise Stores ".to_string(),
        }),
        (4522, Code::<u32> {
            code: 4522,
            parent_code: Some(452),
            description: "Department Stores ".to_string(),
        }),
        (45221, Code::<u32> {
            code: 45221,
            parent_code: Some(4522),
            description: "Department Stores ".to_string(),
        }),
        (452210, Code::<u32> {
            code: 452210,
            parent_code: Some(45221),
            description: "Department Stores ".to_string(),
        }),
        (4523, Code::<u32> {
            code: 4523,
            parent_code: Some(452),
            description: "General Merchandise Stores, including Warehouse Clubs and Supercenters ".to_string(),
        }),
        (45231, Code::<u32> {
            code: 45231,
            parent_code: Some(4523),
            description: "General Merchandise Stores, including Warehouse Clubs and Supercenters ".to_string(),
        }),
        (452311, Code::<u32> {
            code: 452311,
            parent_code: Some(45231),
            description: "Warehouse Clubs and Supercenters ".to_string(),
        }),
        (452319, Code::<u32> {
            code: 452319,
            parent_code: Some(45231),
            description: "All Other General Merchandise Stores ".to_string(),
        }),
        (453, Code::<u32> {
            code: 453,
            parent_code: Some(45),
            description: "Miscellaneous Store Retailers ".to_string(),
        }),
        (4531, Code::<u32> {
            code: 4531,
            parent_code: Some(453),
            description: "Florists ".to_string(),
        }),
        (45311, Code::<u32> {
            code: 45311,
            parent_code: Some(4531),
            description: "Florists ".to_string(),
        }),
        (453110, Code::<u32> {
            code: 453110,
            parent_code: Some(45311),
            description: "Florists ".to_string(),
        }),
        (4532, Code::<u32> {
            code: 4532,
            parent_code: Some(453),
            description: "Office Supplies, Stationery, and Gift Stores ".to_string(),
        }),
        (45321, Code::<u32> {
            code: 45321,
            parent_code: Some(4532),
            description: "Office Supplies and Stationery Stores ".to_string(),
        }),
        (453210, Code::<u32> {
            code: 453210,
            parent_code: Some(45321),
            description: "Office Supplies and Stationery Stores ".to_string(),
        }),
        (45322, Code::<u32> {
            code: 45322,
            parent_code: Some(4532),
            description: "Gift, Novelty, and Souvenir Stores ".to_string(),
        }),
        (453220, Code::<u32> {
            code: 453220,
            parent_code: Some(45322),
            description: "Gift, Novelty, and Souvenir Stores ".to_string(),
        }),
        (4533, Code::<u32> {
            code: 4533,
            parent_code: Some(453),
            description: "Used Merchandise Stores ".to_string(),
        }),
        (45331, Code::<u32> {
            code: 45331,
            parent_code: Some(4533),
            description: "Used Merchandise Stores ".to_string(),
        }),
        (453310, Code::<u32> {
            code: 453310,
            parent_code: Some(45331),
            description: "Used Merchandise Stores ".to_string(),
        }),
        (4539, Code::<u32> {
            code: 4539,
            parent_code: Some(453),
            description: "Other Miscellaneous Store Retailers ".to_string(),
        }),
        (45391, Code::<u32> {
            code: 45391,
            parent_code: Some(4539),
            description: "Pet and Pet Supplies Stores ".to_string(),
        }),
        (453910, Code::<u32> {
            code: 453910,
            parent_code: Some(45391),
            description: "Pet and Pet Supplies Stores ".to_string(),
        }),
        (45392, Code::<u32> {
            code: 45392,
            parent_code: Some(4539),
            description: "Art Dealers ".to_string(),
        }),
        (453920, Code::<u32> {
            code: 453920,
            parent_code: Some(45392),
            description: "Art Dealers ".to_string(),
        }),
        (45393, Code::<u32> {
            code: 45393,
            parent_code: Some(4539),
            description: "Manufactured (Mobile) Home Dealers ".to_string(),
        }),
        (453930, Code::<u32> {
            code: 453930,
            parent_code: Some(45393),
            description: "Manufactured (Mobile) Home Dealers ".to_string(),
        }),
        (45399, Code::<u32> {
            code: 45399,
            parent_code: Some(4539),
            description: "All Other Miscellaneous Store Retailers ".to_string(),
        }),
        (453991, Code::<u32> {
            code: 453991,
            parent_code: Some(45399),
            description: "Tobacco Stores ".to_string(),
        }),
        (453998, Code::<u32> {
            code: 453998,
            parent_code: Some(45399),
            description: "All Other Miscellaneous Store Retailers (except Tobacco Stores) ".to_string(),
        }),
        (454, Code::<u32> {
            code: 454,
            parent_code: Some(45),
            description: "Nonstore Retailers ".to_string(),
        }),
        (4541, Code::<u32> {
            code: 4541,
            parent_code: Some(454),
            description: "Electronic Shopping and Mail-Order Houses ".to_string(),
        }),
        (45411, Code::<u32> {
            code: 45411,
            parent_code: Some(4541),
            description: "Electronic Shopping and Mail-Order Houses ".to_string(),
        }),
        (454110, Code::<u32> {
            code: 454110,
            parent_code: Some(45411),
            description: "Electronic Shopping and Mail-Order Houses ".to_string(),
        }),
        (4542, Code::<u32> {
            code: 4542,
            parent_code: Some(454),
            description: "Vending Machine Operators ".to_string(),
        }),
        (45421, Code::<u32> {
            code: 45421,
            parent_code: Some(4542),
            description: "Vending Machine Operators ".to_string(),
        }),
        (454210, Code::<u32> {
            code: 454210,
            parent_code: Some(45421),
            description: "Vending Machine Operators ".to_string(),
        }),
        (4543, Code::<u32> {
            code: 4543,
            parent_code: Some(454),
            description: "Direct Selling Establishments ".to_string(),
        }),
        (45431, Code::<u32> {
            code: 45431,
            parent_code: Some(4543),
            description: "Fuel Dealers ".to_string(),
        }),
        (454310, Code::<u32> {
            code: 454310,
            parent_code: Some(45431),
            description: "Fuel Dealers ".to_string(),
        }),
        (45439, Code::<u32> {
            code: 45439,
            parent_code: Some(4543),
            description: "Other Direct Selling Establishments ".to_string(),
        }),
        (454390, Code::<u32> {
            code: 454390,
            parent_code: Some(45439),
            description: "Other Direct Selling Establishments ".to_string(),
        }),
        (48, Code::<u32> {
            code: 48,
            parent_code: None,
            description: "Transportation and Warehousing".to_string(),
        }),
        (49, Code::<u32> {
            code: 49,
            parent_code: None,
            description: "Transportation and Warehousing".to_string(),
        }),
        (481, Code::<u32> {
            code: 481,
            parent_code: Some(48),
            description: "Air Transportation".to_string(),
        }),
        (4811, Code::<u32> {
            code: 4811,
            parent_code: Some(481),
            description: "Scheduled Air Transportation".to_string(),
        }),
        (48111, Code::<u32> {
            code: 48111,
            parent_code: Some(4811),
            description: "Scheduled Air Transportation".to_string(),
        }),
        (481111, Code::<u32> {
            code: 481111,
            parent_code: Some(48111),
            description: "Scheduled Passenger Air Transportation ".to_string(),
        }),
        (481112, Code::<u32> {
            code: 481112,
            parent_code: Some(48111),
            description: "Scheduled Freight Air Transportation ".to_string(),
        }),
        (4812, Code::<u32> {
            code: 4812,
            parent_code: Some(481),
            description: "Nonscheduled Air Transportation".to_string(),
        }),
        (48121, Code::<u32> {
            code: 48121,
            parent_code: Some(4812),
            description: "Nonscheduled Air Transportation".to_string(),
        }),
        (481211, Code::<u32> {
            code: 481211,
            parent_code: Some(48121),
            description: "Nonscheduled Chartered Passenger Air Transportation ".to_string(),
        }),
        (481212, Code::<u32> {
            code: 481212,
            parent_code: Some(48121),
            description: "Nonscheduled Chartered Freight Air Transportation ".to_string(),
        }),
        (481219, Code::<u32> {
            code: 481219,
            parent_code: Some(48121),
            description: "Other Nonscheduled Air Transportation ".to_string(),
        }),
        (482, Code::<u32> {
            code: 482,
            parent_code: Some(48),
            description: "Rail Transportation".to_string(),
        }),
        (4821, Code::<u32> {
            code: 4821,
            parent_code: Some(482),
            description: "Rail Transportation".to_string(),
        }),
        (48211, Code::<u32> {
            code: 48211,
            parent_code: Some(4821),
            description: "Rail Transportation".to_string(),
        }),
        (482111, Code::<u32> {
            code: 482111,
            parent_code: Some(48211),
            description: "Line-Haul Railroads ".to_string(),
        }),
        (482112, Code::<u32> {
            code: 482112,
            parent_code: Some(48211),
            description: "Short Line Railroads ".to_string(),
        }),
        (483, Code::<u32> {
            code: 483,
            parent_code: Some(48),
            description: "Water Transportation".to_string(),
        }),
        (4831, Code::<u32> {
            code: 4831,
            parent_code: Some(483),
            description: "Deep Sea, Coastal, and Great Lakes Water Transportation".to_string(),
        }),
        (48311, Code::<u32> {
            code: 48311,
            parent_code: Some(4831),
            description: "Deep Sea, Coastal, and Great Lakes Water Transportation".to_string(),
        }),
        (483111, Code::<u32> {
            code: 483111,
            parent_code: Some(48311),
            description: "Deep Sea Freight Transportation ".to_string(),
        }),
        (483112, Code::<u32> {
            code: 483112,
            parent_code: Some(48311),
            description: "Deep Sea Passenger Transportation ".to_string(),
        }),
        (483113, Code::<u32> {
            code: 483113,
            parent_code: Some(48311),
            description: "Coastal and Great Lakes Freight Transportation ".to_string(),
        }),
        (483114, Code::<u32> {
            code: 483114,
            parent_code: Some(48311),
            description: "Coastal and Great Lakes Passenger Transportation ".to_string(),
        }),
        (4832, Code::<u32> {
            code: 4832,
            parent_code: Some(483),
            description: "Inland Water Transportation".to_string(),
        }),
        (48321, Code::<u32> {
            code: 48321,
            parent_code: Some(4832),
            description: "Inland Water Transportation".to_string(),
        }),
        (483211, Code::<u32> {
            code: 483211,
            parent_code: Some(48321),
            description: "Inland Water Freight Transportation ".to_string(),
        }),
        (483212, Code::<u32> {
            code: 483212,
            parent_code: Some(48321),
            description: "Inland Water Passenger Transportation ".to_string(),
        }),
        (484, Code::<u32> {
            code: 484,
            parent_code: Some(48),
            description: "Truck Transportation".to_string(),
        }),
        (4841, Code::<u32> {
            code: 4841,
            parent_code: Some(484),
            description: "General Freight Trucking".to_string(),
        }),
        (48411, Code::<u32> {
            code: 48411,
            parent_code: Some(4841),
            description: "General Freight Trucking, Local".to_string(),
        }),
        (484110, Code::<u32> {
            code: 484110,
            parent_code: Some(48411),
            description: "General Freight Trucking, Local ".to_string(),
        }),
        (48412, Code::<u32> {
            code: 48412,
            parent_code: Some(4841),
            description: "General Freight Trucking, Long-Distance".to_string(),
        }),
        (484121, Code::<u32> {
            code: 484121,
            parent_code: Some(48412),
            description: "General Freight Trucking, Long-Distance, Truckload ".to_string(),
        }),
        (484122, Code::<u32> {
            code: 484122,
            parent_code: Some(48412),
            description: "General Freight Trucking, Long-Distance, Less Than Truckload ".to_string(),
        }),
        (4842, Code::<u32> {
            code: 4842,
            parent_code: Some(484),
            description: "Specialized Freight Trucking".to_string(),
        }),
        (48421, Code::<u32> {
            code: 48421,
            parent_code: Some(4842),
            description: "Used Household and Office Goods Moving".to_string(),
        }),
        (484210, Code::<u32> {
            code: 484210,
            parent_code: Some(48421),
            description: "Used Household and Office Goods Moving".to_string(),
        }),
        (48422, Code::<u32> {
            code: 48422,
            parent_code: Some(4842),
            description: "Specialized Freight (except Used Goods) Trucking, Local".to_string(),
        }),
        (484220, Code::<u32> {
            code: 484220,
            parent_code: Some(48422),
            description: "Specialized Freight (except Used Goods) Trucking, Local ".to_string(),
        }),
        (48423, Code::<u32> {
            code: 48423,
            parent_code: Some(4842),
            description: "Specialized Freight (except Used Goods) Trucking, Long-Distance".to_string(),
        }),
        (484230, Code::<u32> {
            code: 484230,
            parent_code: Some(48423),
            description: "Specialized Freight (except Used Goods) Trucking, Long-Distance ".to_string(),
        }),
        (485, Code::<u32> {
            code: 485,
            parent_code: Some(48),
            description: "Transit and Ground Passenger Transportation".to_string(),
        }),
        (4851, Code::<u32> {
            code: 4851,
            parent_code: Some(485),
            description: "Urban Transit Systems".to_string(),
        }),
        (48511, Code::<u32> {
            code: 48511,
            parent_code: Some(4851),
            description: "Urban Transit Systems".to_string(),
        }),
        (485111, Code::<u32> {
            code: 485111,
            parent_code: Some(48511),
            description: "Mixed Mode Transit Systems ".to_string(),
        }),
        (485112, Code::<u32> {
            code: 485112,
            parent_code: Some(48511),
            description: "Commuter Rail Systems ".to_string(),
        }),
        (485113, Code::<u32> {
            code: 485113,
            parent_code: Some(48511),
            description: "Bus and Other Motor Vehicle Transit Systems ".to_string(),
        }),
        (485119, Code::<u32> {
            code: 485119,
            parent_code: Some(48511),
            description: "Other Urban Transit Systems ".to_string(),
        }),
        (4852, Code::<u32> {
            code: 4852,
            parent_code: Some(485),
            description: "Interurban and Rural Bus Transportation".to_string(),
        }),
        (48521, Code::<u32> {
            code: 48521,
            parent_code: Some(4852),
            description: "Interurban and Rural Bus Transportation".to_string(),
        }),
        (485210, Code::<u32> {
            code: 485210,
            parent_code: Some(48521),
            description: "Interurban and Rural Bus Transportation".to_string(),
        }),
        (4853, Code::<u32> {
            code: 4853,
            parent_code: Some(485),
            description: "Taxi and Limousine Service".to_string(),
        }),
        (48531, Code::<u32> {
            code: 48531,
            parent_code: Some(4853),
            description: "Taxi Service".to_string(),
        }),
        (485310, Code::<u32> {
            code: 485310,
            parent_code: Some(48531),
            description: "Taxi Service ".to_string(),
        }),
        (48532, Code::<u32> {
            code: 48532,
            parent_code: Some(4853),
            description: "Limousine Service".to_string(),
        }),
        (485320, Code::<u32> {
            code: 485320,
            parent_code: Some(48532),
            description: "Limousine Service".to_string(),
        }),
        (4854, Code::<u32> {
            code: 4854,
            parent_code: Some(485),
            description: "School and Employee Bus Transportation".to_string(),
        }),
        (48541, Code::<u32> {
            code: 48541,
            parent_code: Some(4854),
            description: "School and Employee Bus Transportation".to_string(),
        }),
        (485410, Code::<u32> {
            code: 485410,
            parent_code: Some(48541),
            description: "School and Employee Bus Transportation".to_string(),
        }),
        (4855, Code::<u32> {
            code: 4855,
            parent_code: Some(485),
            description: "Charter Bus Industry".to_string(),
        }),
        (48551, Code::<u32> {
            code: 48551,
            parent_code: Some(4855),
            description: "Charter Bus Industry".to_string(),
        }),
        (485510, Code::<u32> {
            code: 485510,
            parent_code: Some(48551),
            description: "Charter Bus Industry".to_string(),
        }),
        (4859, Code::<u32> {
            code: 4859,
            parent_code: Some(485),
            description: "Other Transit and Ground Passenger Transportation".to_string(),
        }),
        (48599, Code::<u32> {
            code: 48599,
            parent_code: Some(4859),
            description: "Other Transit and Ground Passenger Transportation".to_string(),
        }),
        (485991, Code::<u32> {
            code: 485991,
            parent_code: Some(48599),
            description: "Special Needs Transportation ".to_string(),
        }),
        (485999, Code::<u32> {
            code: 485999,
            parent_code: Some(48599),
            description: "All Other Transit and Ground Passenger Transportation ".to_string(),
        }),
        (486, Code::<u32> {
            code: 486,
            parent_code: Some(48),
            description: "Pipeline Transportation".to_string(),
        }),
        (4861, Code::<u32> {
            code: 4861,
            parent_code: Some(486),
            description: "Pipeline Transportation of Crude Oil".to_string(),
        }),
        (48611, Code::<u32> {
            code: 48611,
            parent_code: Some(4861),
            description: "Pipeline Transportation of Crude Oil".to_string(),
        }),
        (486110, Code::<u32> {
            code: 486110,
            parent_code: Some(48611),
            description: "Pipeline Transportation of Crude Oil".to_string(),
        }),
        (4862, Code::<u32> {
            code: 4862,
            parent_code: Some(486),
            description: "Pipeline Transportation of Natural Gas".to_string(),
        }),
        (48621, Code::<u32> {
            code: 48621,
            parent_code: Some(4862),
            description: "Pipeline Transportation of Natural Gas".to_string(),
        }),
        (486210, Code::<u32> {
            code: 486210,
            parent_code: Some(48621),
            description: "Pipeline Transportation of Natural Gas".to_string(),
        }),
        (4869, Code::<u32> {
            code: 4869,
            parent_code: Some(486),
            description: "Other Pipeline Transportation".to_string(),
        }),
        (48691, Code::<u32> {
            code: 48691,
            parent_code: Some(4869),
            description: "Pipeline Transportation of Refined Petroleum Products".to_string(),
        }),
        (486910, Code::<u32> {
            code: 486910,
            parent_code: Some(48691),
            description: "Pipeline Transportation of Refined Petroleum Products".to_string(),
        }),
        (48699, Code::<u32> {
            code: 48699,
            parent_code: Some(4869),
            description: "All Other Pipeline Transportation".to_string(),
        }),
        (486990, Code::<u32> {
            code: 486990,
            parent_code: Some(48699),
            description: "All Other Pipeline Transportation".to_string(),
        }),
        (487, Code::<u32> {
            code: 487,
            parent_code: Some(48),
            description: "Scenic and Sightseeing Transportation".to_string(),
        }),
        (4871, Code::<u32> {
            code: 4871,
            parent_code: Some(487),
            description: "Scenic and Sightseeing Transportation, Land".to_string(),
        }),
        (48711, Code::<u32> {
            code: 48711,
            parent_code: Some(4871),
            description: "Scenic and Sightseeing Transportation, Land".to_string(),
        }),
        (487110, Code::<u32> {
            code: 487110,
            parent_code: Some(48711),
            description: "Scenic and Sightseeing Transportation, Land".to_string(),
        }),
        (4872, Code::<u32> {
            code: 4872,
            parent_code: Some(487),
            description: "Scenic and Sightseeing Transportation, Water".to_string(),
        }),
        (48721, Code::<u32> {
            code: 48721,
            parent_code: Some(4872),
            description: "Scenic and Sightseeing Transportation, Water".to_string(),
        }),
        (487210, Code::<u32> {
            code: 487210,
            parent_code: Some(48721),
            description: "Scenic and Sightseeing Transportation, Water".to_string(),
        }),
        (4879, Code::<u32> {
            code: 4879,
            parent_code: Some(487),
            description: "Scenic and Sightseeing Transportation, Other".to_string(),
        }),
        (48799, Code::<u32> {
            code: 48799,
            parent_code: Some(4879),
            description: "Scenic and Sightseeing Transportation, Other".to_string(),
        }),
        (487990, Code::<u32> {
            code: 487990,
            parent_code: Some(48799),
            description: "Scenic and Sightseeing Transportation, Other".to_string(),
        }),
        (488, Code::<u32> {
            code: 488,
            parent_code: Some(48),
            description: "Support Activities for Transportation".to_string(),
        }),
        (4881, Code::<u32> {
            code: 4881,
            parent_code: Some(488),
            description: "Support Activities for Air Transportation".to_string(),
        }),
        (48811, Code::<u32> {
            code: 48811,
            parent_code: Some(4881),
            description: "Airport Operations".to_string(),
        }),
        (488111, Code::<u32> {
            code: 488111,
            parent_code: Some(48811),
            description: "Air Traffic Control".to_string(),
        }),
        (488119, Code::<u32> {
            code: 488119,
            parent_code: Some(48811),
            description: "Other Airport Operations ".to_string(),
        }),
        (48819, Code::<u32> {
            code: 48819,
            parent_code: Some(4881),
            description: "Other Support Activities for Air Transportation".to_string(),
        }),
        (488190, Code::<u32> {
            code: 488190,
            parent_code: Some(48819),
            description: "Other Support Activities for Air Transportation".to_string(),
        }),
        (4882, Code::<u32> {
            code: 4882,
            parent_code: Some(488),
            description: "Support Activities for Rail Transportation".to_string(),
        }),
        (48821, Code::<u32> {
            code: 48821,
            parent_code: Some(4882),
            description: "Support Activities for Rail Transportation".to_string(),
        }),
        (488210, Code::<u32> {
            code: 488210,
            parent_code: Some(48821),
            description: "Support Activities for Rail Transportation".to_string(),
        }),
        (4883, Code::<u32> {
            code: 4883,
            parent_code: Some(488),
            description: "Support Activities for Water Transportation".to_string(),
        }),
        (48831, Code::<u32> {
            code: 48831,
            parent_code: Some(4883),
            description: "Port and Harbor Operations".to_string(),
        }),
        (488310, Code::<u32> {
            code: 488310,
            parent_code: Some(48831),
            description: "Port and Harbor Operations".to_string(),
        }),
        (48832, Code::<u32> {
            code: 48832,
            parent_code: Some(4883),
            description: "Marine Cargo Handling".to_string(),
        }),
        (488320, Code::<u32> {
            code: 488320,
            parent_code: Some(48832),
            description: "Marine Cargo Handling".to_string(),
        }),
        (48833, Code::<u32> {
            code: 48833,
            parent_code: Some(4883),
            description: "Navigational Services to Shipping".to_string(),
        }),
        (488330, Code::<u32> {
            code: 488330,
            parent_code: Some(48833),
            description: "Navigational Services to Shipping ".to_string(),
        }),
        (48839, Code::<u32> {
            code: 48839,
            parent_code: Some(4883),
            description: "Other Support Activities for Water Transportation".to_string(),
        }),
        (488390, Code::<u32> {
            code: 488390,
            parent_code: Some(48839),
            description: "Other Support Activities for Water Transportation".to_string(),
        }),
        (4884, Code::<u32> {
            code: 4884,
            parent_code: Some(488),
            description: "Support Activities for Road Transportation".to_string(),
        }),
        (48841, Code::<u32> {
            code: 48841,
            parent_code: Some(4884),
            description: "Motor Vehicle Towing".to_string(),
        }),
        (488410, Code::<u32> {
            code: 488410,
            parent_code: Some(48841),
            description: "Motor Vehicle Towing".to_string(),
        }),
        (48849, Code::<u32> {
            code: 48849,
            parent_code: Some(4884),
            description: "Other Support Activities for Road Transportation".to_string(),
        }),
        (488490, Code::<u32> {
            code: 488490,
            parent_code: Some(48849),
            description: "Other Support Activities for Road Transportation ".to_string(),
        }),
        (4885, Code::<u32> {
            code: 4885,
            parent_code: Some(488),
            description: "Freight Transportation Arrangement".to_string(),
        }),
        (48851, Code::<u32> {
            code: 48851,
            parent_code: Some(4885),
            description: "Freight Transportation Arrangement".to_string(),
        }),
        (488510, Code::<u32> {
            code: 488510,
            parent_code: Some(48851),
            description: "Freight Transportation Arrangement ".to_string(),
        }),
        (4889, Code::<u32> {
            code: 4889,
            parent_code: Some(488),
            description: "Other Support Activities for Transportation".to_string(),
        }),
        (48899, Code::<u32> {
            code: 48899,
            parent_code: Some(4889),
            description: "Other Support Activities for Transportation".to_string(),
        }),
        (488991, Code::<u32> {
            code: 488991,
            parent_code: Some(48899),
            description: "Packing and Crating ".to_string(),
        }),
        (488999, Code::<u32> {
            code: 488999,
            parent_code: Some(48899),
            description: "All Other Support Activities for Transportation ".to_string(),
        }),
        (491, Code::<u32> {
            code: 491,
            parent_code: Some(49),
            description: "Postal Service".to_string(),
        }),
        (4911, Code::<u32> {
            code: 4911,
            parent_code: Some(491),
            description: "Postal Service".to_string(),
        }),
        (49111, Code::<u32> {
            code: 49111,
            parent_code: Some(4911),
            description: "Postal Service".to_string(),
        }),
        (491110, Code::<u32> {
            code: 491110,
            parent_code: Some(49111),
            description: "Postal Service".to_string(),
        }),
        (492, Code::<u32> {
            code: 492,
            parent_code: Some(49),
            description: "Couriers and Messengers".to_string(),
        }),
        (4921, Code::<u32> {
            code: 4921,
            parent_code: Some(492),
            description: "Couriers and Express Delivery Services".to_string(),
        }),
        (49211, Code::<u32> {
            code: 49211,
            parent_code: Some(4921),
            description: "Couriers and Express Delivery Services".to_string(),
        }),
        (492110, Code::<u32> {
            code: 492110,
            parent_code: Some(49211),
            description: "Couriers and Express Delivery Services".to_string(),
        }),
        (4922, Code::<u32> {
            code: 4922,
            parent_code: Some(492),
            description: "Local Messengers and Local Delivery".to_string(),
        }),
        (49221, Code::<u32> {
            code: 49221,
            parent_code: Some(4922),
            description: "Local Messengers and Local Delivery".to_string(),
        }),
        (492210, Code::<u32> {
            code: 492210,
            parent_code: Some(49221),
            description: "Local Messengers and Local Delivery".to_string(),
        }),
        (493, Code::<u32> {
            code: 493,
            parent_code: Some(49),
            description: "Warehousing and Storage".to_string(),
        }),
        (4931, Code::<u32> {
            code: 4931,
            parent_code: Some(493),
            description: "Warehousing and Storage".to_string(),
        }),
        (49311, Code::<u32> {
            code: 49311,
            parent_code: Some(4931),
            description: "General Warehousing and Storage".to_string(),
        }),
        (493110, Code::<u32> {
            code: 493110,
            parent_code: Some(49311),
            description: "General Warehousing and Storage ".to_string(),
        }),
        (49312, Code::<u32> {
            code: 49312,
            parent_code: Some(4931),
            description: "Refrigerated Warehousing and Storage".to_string(),
        }),
        (493120, Code::<u32> {
            code: 493120,
            parent_code: Some(49312),
            description: "Refrigerated Warehousing and Storage".to_string(),
        }),
        (49313, Code::<u32> {
            code: 49313,
            parent_code: Some(4931),
            description: "Farm Product Warehousing and Storage".to_string(),
        }),
        (493130, Code::<u32> {
            code: 493130,
            parent_code: Some(49313),
            description: "Farm Product Warehousing and Storage".to_string(),
        }),
        (49319, Code::<u32> {
            code: 49319,
            parent_code: Some(4931),
            description: "Other Warehousing and Storage".to_string(),
        }),
        (493190, Code::<u32> {
            code: 493190,
            parent_code: Some(49319),
            description: "Other Warehousing and Storage".to_string(),
        }),
        (51, Code::<u32> {
            code: 51,
            parent_code: None,
            description: "Information".to_string(),
        }),
        (511, Code::<u32> {
            code: 511,
            parent_code: Some(51),
            description: "Publishing Industries (except Internet)".to_string(),
        }),
        (5111, Code::<u32> {
            code: 5111,
            parent_code: Some(511),
            description: "Newspaper, Periodical, Book, and Directory Publishers".to_string(),
        }),
        (51111, Code::<u32> {
            code: 51111,
            parent_code: Some(5111),
            description: "Newspaper Publishers".to_string(),
        }),
        (511110, Code::<u32> {
            code: 511110,
            parent_code: Some(51111),
            description: "Newspaper Publishers ".to_string(),
        }),
        (51112, Code::<u32> {
            code: 51112,
            parent_code: Some(5111),
            description: "Periodical Publishers".to_string(),
        }),
        (511120, Code::<u32> {
            code: 511120,
            parent_code: Some(51112),
            description: "Periodical Publishers ".to_string(),
        }),
        (51113, Code::<u32> {
            code: 51113,
            parent_code: Some(5111),
            description: "Book Publishers".to_string(),
        }),
        (511130, Code::<u32> {
            code: 511130,
            parent_code: Some(51113),
            description: "Book Publishers ".to_string(),
        }),
        (51114, Code::<u32> {
            code: 51114,
            parent_code: Some(5111),
            description: "Directory and Mailing List Publishers".to_string(),
        }),
        (511140, Code::<u32> {
            code: 511140,
            parent_code: Some(51114),
            description: "Directory and Mailing List Publishers ".to_string(),
        }),
        (51119, Code::<u32> {
            code: 51119,
            parent_code: Some(5111),
            description: "Other Publishers".to_string(),
        }),
        (511191, Code::<u32> {
            code: 511191,
            parent_code: Some(51119),
            description: "Greeting Card Publishers ".to_string(),
        }),
        (511199, Code::<u32> {
            code: 511199,
            parent_code: Some(51119),
            description: "All Other Publishers ".to_string(),
        }),
        (5112, Code::<u32> {
            code: 5112,
            parent_code: Some(511),
            description: "Software Publishers".to_string(),
        }),
        (51121, Code::<u32> {
            code: 51121,
            parent_code: Some(5112),
            description: "Software Publishers".to_string(),
        }),
        (511210, Code::<u32> {
            code: 511210,
            parent_code: Some(51121),
            description: "Software Publishers".to_string(),
        }),
        (512, Code::<u32> {
            code: 512,
            parent_code: Some(51),
            description: "Motion Picture and Sound Recording Industries".to_string(),
        }),
        (5121, Code::<u32> {
            code: 5121,
            parent_code: Some(512),
            description: "Motion Picture and Video Industries".to_string(),
        }),
        (51211, Code::<u32> {
            code: 51211,
            parent_code: Some(5121),
            description: "Motion Picture and Video Production".to_string(),
        }),
        (512110, Code::<u32> {
            code: 512110,
            parent_code: Some(51211),
            description: "Motion Picture and Video Production ".to_string(),
        }),
        (51212, Code::<u32> {
            code: 51212,
            parent_code: Some(5121),
            description: "Motion Picture and Video Distribution".to_string(),
        }),
        (512120, Code::<u32> {
            code: 512120,
            parent_code: Some(51212),
            description: "Motion Picture and Video Distribution".to_string(),
        }),
        (51213, Code::<u32> {
            code: 51213,
            parent_code: Some(5121),
            description: "Motion Picture and Video Exhibition".to_string(),
        }),
        (512131, Code::<u32> {
            code: 512131,
            parent_code: Some(51213),
            description: "Motion Picture Theaters (except Drive-Ins) ".to_string(),
        }),
        (512132, Code::<u32> {
            code: 512132,
            parent_code: Some(51213),
            description: "Drive-In Motion Picture Theaters ".to_string(),
        }),
        (51219, Code::<u32> {
            code: 51219,
            parent_code: Some(5121),
            description: "Postproduction Services and Other Motion Picture and Video Industries".to_string(),
        }),
        (512191, Code::<u32> {
            code: 512191,
            parent_code: Some(51219),
            description: "Teleproduction and Other Postproduction Services ".to_string(),
        }),
        (512199, Code::<u32> {
            code: 512199,
            parent_code: Some(51219),
            description: "Other Motion Picture and Video Industries ".to_string(),
        }),
        (5122, Code::<u32> {
            code: 5122,
            parent_code: Some(512),
            description: "Sound Recording Industries".to_string(),
        }),
        (51223, Code::<u32> {
            code: 51223,
            parent_code: Some(5122),
            description: "Music Publishers".to_string(),
        }),
        (512230, Code::<u32> {
            code: 512230,
            parent_code: Some(51223),
            description: "Music Publishers".to_string(),
        }),
        (51224, Code::<u32> {
            code: 51224,
            parent_code: Some(5122),
            description: "Sound Recording Studios".to_string(),
        }),
        (512240, Code::<u32> {
            code: 512240,
            parent_code: Some(51224),
            description: "Sound Recording Studios".to_string(),
        }),
        (51225, Code::<u32> {
            code: 51225,
            parent_code: Some(5122),
            description: "Record Production and Distribution".to_string(),
        }),
        (512250, Code::<u32> {
            code: 512250,
            parent_code: Some(51225),
            description: "Record Production and Distribution".to_string(),
        }),
        (51229, Code::<u32> {
            code: 51229,
            parent_code: Some(5122),
            description: "Other Sound Recording Industries".to_string(),
        }),
        (512290, Code::<u32> {
            code: 512290,
            parent_code: Some(51229),
            description: "Other Sound Recording Industries".to_string(),
        }),
        (515, Code::<u32> {
            code: 515,
            parent_code: Some(51),
            description: "Broadcasting (except Internet)".to_string(),
        }),
        (5151, Code::<u32> {
            code: 5151,
            parent_code: Some(515),
            description: "Radio and Television Broadcasting".to_string(),
        }),
        (51511, Code::<u32> {
            code: 51511,
            parent_code: Some(5151),
            description: "Radio Broadcasting".to_string(),
        }),
        (515111, Code::<u32> {
            code: 515111,
            parent_code: Some(51511),
            description: "Radio Networks ".to_string(),
        }),
        (515112, Code::<u32> {
            code: 515112,
            parent_code: Some(51511),
            description: "Radio Stations ".to_string(),
        }),
        (51512, Code::<u32> {
            code: 51512,
            parent_code: Some(5151),
            description: "Television Broadcasting".to_string(),
        }),
        (515120, Code::<u32> {
            code: 515120,
            parent_code: Some(51512),
            description: "Television Broadcasting".to_string(),
        }),
        (5152, Code::<u32> {
            code: 5152,
            parent_code: Some(515),
            description: "Cable and Other Subscription Programming".to_string(),
        }),
        (51521, Code::<u32> {
            code: 51521,
            parent_code: Some(5152),
            description: "Cable and Other Subscription Programming".to_string(),
        }),
        (515210, Code::<u32> {
            code: 515210,
            parent_code: Some(51521),
            description: "Cable and Other Subscription Programming".to_string(),
        }),
        (517, Code::<u32> {
            code: 517,
            parent_code: Some(51),
            description: "Telecommunications".to_string(),
        }),
        (5173, Code::<u32> {
            code: 5173,
            parent_code: Some(517),
            description: "Wired and Wireless Telecommunications Carriers".to_string(),
        }),
        (51731, Code::<u32> {
            code: 51731,
            parent_code: Some(5173),
            description: "Wired and Wireless Telecommunications Carriers".to_string(),
        }),
        (517311, Code::<u32> {
            code: 517311,
            parent_code: Some(51731),
            description: "Wired Telecommunications Carriers ".to_string(),
        }),
        (517312, Code::<u32> {
            code: 517312,
            parent_code: Some(51731),
            description: "Wireless Telecommunications Carriers (except Satellite)".to_string(),
        }),
        (5174, Code::<u32> {
            code: 5174,
            parent_code: Some(517),
            description: "Satellite Telecommunications".to_string(),
        }),
        (51741, Code::<u32> {
            code: 51741,
            parent_code: Some(5174),
            description: "Satellite Telecommunications".to_string(),
        }),
        (517410, Code::<u32> {
            code: 517410,
            parent_code: Some(51741),
            description: "Satellite Telecommunications".to_string(),
        }),
        (5179, Code::<u32> {
            code: 5179,
            parent_code: Some(517),
            description: "Other Telecommunications".to_string(),
        }),
        (51791, Code::<u32> {
            code: 51791,
            parent_code: Some(5179),
            description: "Other Telecommunications".to_string(),
        }),
        (517911, Code::<u32> {
            code: 517911,
            parent_code: Some(51791),
            description: "Telecommunications Resellers ".to_string(),
        }),
        (517919, Code::<u32> {
            code: 517919,
            parent_code: Some(51791),
            description: "All Other Telecommunications ".to_string(),
        }),
        (518, Code::<u32> {
            code: 518,
            parent_code: Some(51),
            description: "Data Processing, Hosting, and Related Services".to_string(),
        }),
        (5182, Code::<u32> {
            code: 5182,
            parent_code: Some(518),
            description: "Data Processing, Hosting, and Related Services".to_string(),
        }),
        (51821, Code::<u32> {
            code: 51821,
            parent_code: Some(5182),
            description: "Data Processing, Hosting, and Related Services".to_string(),
        }),
        (518210, Code::<u32> {
            code: 518210,
            parent_code: Some(51821),
            description: "Data Processing, Hosting, and Related Services".to_string(),
        }),
        (519, Code::<u32> {
            code: 519,
            parent_code: Some(51),
            description: "Other Information Services".to_string(),
        }),
        (5191, Code::<u32> {
            code: 5191,
            parent_code: Some(519),
            description: "Other Information Services".to_string(),
        }),
        (51911, Code::<u32> {
            code: 51911,
            parent_code: Some(5191),
            description: "News Syndicates".to_string(),
        }),
        (519110, Code::<u32> {
            code: 519110,
            parent_code: Some(51911),
            description: "News Syndicates".to_string(),
        }),
        (51912, Code::<u32> {
            code: 51912,
            parent_code: Some(5191),
            description: "Libraries and Archives".to_string(),
        }),
        (519120, Code::<u32> {
            code: 519120,
            parent_code: Some(51912),
            description: "Libraries and Archives ".to_string(),
        }),
        (51913, Code::<u32> {
            code: 51913,
            parent_code: Some(5191),
            description: "Internet Publishing and Broadcasting and Web Search Portals".to_string(),
        }),
        (519130, Code::<u32> {
            code: 519130,
            parent_code: Some(51913),
            description: "Internet Publishing and Broadcasting and Web Search Portals".to_string(),
        }),
        (51919, Code::<u32> {
            code: 51919,
            parent_code: Some(5191),
            description: "All Other Information Services".to_string(),
        }),
        (519190, Code::<u32> {
            code: 519190,
            parent_code: Some(51919),
            description: "All Other Information Services".to_string(),
        }),
        (52, Code::<u32> {
            code: 52,
            parent_code: None,
            description: "Finance and Insurance".to_string(),
        }),
        (521, Code::<u32> {
            code: 521,
            parent_code: Some(52),
            description: "Monetary Authorities-Central Bank".to_string(),
        }),
        (5211, Code::<u32> {
            code: 5211,
            parent_code: Some(521),
            description: "Monetary Authorities-Central Bank".to_string(),
        }),
        (52111, Code::<u32> {
            code: 52111,
            parent_code: Some(5211),
            description: "Monetary Authorities-Central Bank".to_string(),
        }),
        (521110, Code::<u32> {
            code: 521110,
            parent_code: Some(52111),
            description: "Monetary Authorities-Central Bank".to_string(),
        }),
        (522, Code::<u32> {
            code: 522,
            parent_code: Some(52),
            description: "Credit Intermediation and Related Activities".to_string(),
        }),
        (5221, Code::<u32> {
            code: 5221,
            parent_code: Some(522),
            description: "Depository Credit Intermediation ".to_string(),
        }),
        (52211, Code::<u32> {
            code: 52211,
            parent_code: Some(5221),
            description: "Commercial Banking ".to_string(),
        }),
        (522110, Code::<u32> {
            code: 522110,
            parent_code: Some(52211),
            description: "Commercial Banking ".to_string(),
        }),
        (52212, Code::<u32> {
            code: 52212,
            parent_code: Some(5221),
            description: "Savings Institutions ".to_string(),
        }),
        (522120, Code::<u32> {
            code: 522120,
            parent_code: Some(52212),
            description: "Savings Institutions ".to_string(),
        }),
        (52213, Code::<u32> {
            code: 52213,
            parent_code: Some(5221),
            description: "Credit Unions ".to_string(),
        }),
        (522130, Code::<u32> {
            code: 522130,
            parent_code: Some(52213),
            description: "Credit Unions ".to_string(),
        }),
        (52219, Code::<u32> {
            code: 52219,
            parent_code: Some(5221),
            description: "Other Depository Credit Intermediation ".to_string(),
        }),
        (522190, Code::<u32> {
            code: 522190,
            parent_code: Some(52219),
            description: "Other Depository Credit Intermediation ".to_string(),
        }),
        (5222, Code::<u32> {
            code: 5222,
            parent_code: Some(522),
            description: "Nondepository Credit Intermediation ".to_string(),
        }),
        (52221, Code::<u32> {
            code: 52221,
            parent_code: Some(5222),
            description: "Credit Card Issuing ".to_string(),
        }),
        (522210, Code::<u32> {
            code: 522210,
            parent_code: Some(52221),
            description: "Credit Card Issuing ".to_string(),
        }),
        (52222, Code::<u32> {
            code: 52222,
            parent_code: Some(5222),
            description: "Sales Financing ".to_string(),
        }),
        (522220, Code::<u32> {
            code: 522220,
            parent_code: Some(52222),
            description: "Sales Financing ".to_string(),
        }),
        (52229, Code::<u32> {
            code: 52229,
            parent_code: Some(5222),
            description: "Other Nondepository Credit Intermediation ".to_string(),
        }),
        (522291, Code::<u32> {
            code: 522291,
            parent_code: Some(52229),
            description: "Consumer Lending ".to_string(),
        }),
        (522292, Code::<u32> {
            code: 522292,
            parent_code: Some(52229),
            description: "Real Estate Credit ".to_string(),
        }),
        (522293, Code::<u32> {
            code: 522293,
            parent_code: Some(52229),
            description: "International Trade Financing ".to_string(),
        }),
        (522294, Code::<u32> {
            code: 522294,
            parent_code: Some(52229),
            description: "Secondary Market Financing ".to_string(),
        }),
        (522298, Code::<u32> {
            code: 522298,
            parent_code: Some(52229),
            description: "All Other Nondepository Credit Intermediation ".to_string(),
        }),
        (5223, Code::<u32> {
            code: 5223,
            parent_code: Some(522),
            description: "Activities Related to Credit Intermediation ".to_string(),
        }),
        (52231, Code::<u32> {
            code: 52231,
            parent_code: Some(5223),
            description: "Mortgage and Nonmortgage Loan Brokers ".to_string(),
        }),
        (522310, Code::<u32> {
            code: 522310,
            parent_code: Some(52231),
            description: "Mortgage and Nonmortgage Loan Brokers ".to_string(),
        }),
        (52232, Code::<u32> {
            code: 52232,
            parent_code: Some(5223),
            description: "Financial Transactions Processing, Reserve, and Clearinghouse Activities ".to_string(),
        }),
        (522320, Code::<u32> {
            code: 522320,
            parent_code: Some(52232),
            description: "Financial Transactions Processing, Reserve, and Clearinghouse Activities ".to_string(),
        }),
        (52239, Code::<u32> {
            code: 52239,
            parent_code: Some(5223),
            description: "Other Activities Related to Credit Intermediation ".to_string(),
        }),
        (522390, Code::<u32> {
            code: 522390,
            parent_code: Some(52239),
            description: "Other Activities Related to Credit Intermediation ".to_string(),
        }),
        (523, Code::<u32> {
            code: 523,
            parent_code: Some(52),
            description: "Securities, Commodity Contracts, and Other Financial Investments and Related Activities".to_string(),
        }),
        (5231, Code::<u32> {
            code: 5231,
            parent_code: Some(523),
            description: "Securities and Commodity Contracts Intermediation and Brokerage".to_string(),
        }),
        (52311, Code::<u32> {
            code: 52311,
            parent_code: Some(5231),
            description: "Investment Banking and Securities Dealing ".to_string(),
        }),
        (523110, Code::<u32> {
            code: 523110,
            parent_code: Some(52311),
            description: "Investment Banking and Securities Dealing ".to_string(),
        }),
        (52312, Code::<u32> {
            code: 52312,
            parent_code: Some(5231),
            description: "Securities Brokerage ".to_string(),
        }),
        (523120, Code::<u32> {
            code: 523120,
            parent_code: Some(52312),
            description: "Securities Brokerage ".to_string(),
        }),
        (52313, Code::<u32> {
            code: 52313,
            parent_code: Some(5231),
            description: "Commodity Contracts Dealing ".to_string(),
        }),
        (523130, Code::<u32> {
            code: 523130,
            parent_code: Some(52313),
            description: "Commodity Contracts Dealing ".to_string(),
        }),
        (52314, Code::<u32> {
            code: 52314,
            parent_code: Some(5231),
            description: "Commodity Contracts Brokerage ".to_string(),
        }),
        (523140, Code::<u32> {
            code: 523140,
            parent_code: Some(52314),
            description: "Commodity Contracts Brokerage ".to_string(),
        }),
        (5232, Code::<u32> {
            code: 5232,
            parent_code: Some(523),
            description: "Securities and Commodity Exchanges".to_string(),
        }),
        (52321, Code::<u32> {
            code: 52321,
            parent_code: Some(5232),
            description: "Securities and Commodity Exchanges".to_string(),
        }),
        (523210, Code::<u32> {
            code: 523210,
            parent_code: Some(52321),
            description: "Securities and Commodity Exchanges".to_string(),
        }),
        (5239, Code::<u32> {
            code: 5239,
            parent_code: Some(523),
            description: "Other Financial Investment Activities".to_string(),
        }),
        (52391, Code::<u32> {
            code: 52391,
            parent_code: Some(5239),
            description: "Miscellaneous Intermediation ".to_string(),
        }),
        (523910, Code::<u32> {
            code: 523910,
            parent_code: Some(52391),
            description: "Miscellaneous Intermediation ".to_string(),
        }),
        (52392, Code::<u32> {
            code: 52392,
            parent_code: Some(5239),
            description: "Portfolio Management ".to_string(),
        }),
        (523920, Code::<u32> {
            code: 523920,
            parent_code: Some(52392),
            description: "Portfolio Management ".to_string(),
        }),
        (52393, Code::<u32> {
            code: 52393,
            parent_code: Some(5239),
            description: "Investment Advice ".to_string(),
        }),
        (523930, Code::<u32> {
            code: 523930,
            parent_code: Some(52393),
            description: "Investment Advice ".to_string(),
        }),
        (52399, Code::<u32> {
            code: 52399,
            parent_code: Some(5239),
            description: "All Other Financial Investment Activities ".to_string(),
        }),
        (523991, Code::<u32> {
            code: 523991,
            parent_code: Some(52399),
            description: "Trust, Fiduciary, and Custody Activities ".to_string(),
        }),
        (523999, Code::<u32> {
            code: 523999,
            parent_code: Some(52399),
            description: "Miscellaneous Financial Investment Activities ".to_string(),
        }),
        (524, Code::<u32> {
            code: 524,
            parent_code: Some(52),
            description: "Insurance Carriers and Related Activities".to_string(),
        }),
        (5241, Code::<u32> {
            code: 5241,
            parent_code: Some(524),
            description: "Insurance Carriers".to_string(),
        }),
        (52411, Code::<u32> {
            code: 52411,
            parent_code: Some(5241),
            description: "Direct Life, Health, and Medical Insurance Carriers ".to_string(),
        }),
        (524113, Code::<u32> {
            code: 524113,
            parent_code: Some(52411),
            description: "Direct Life Insurance Carriers ".to_string(),
        }),
        (524114, Code::<u32> {
            code: 524114,
            parent_code: Some(52411),
            description: "Direct Health and Medical Insurance Carriers ".to_string(),
        }),
        (52412, Code::<u32> {
            code: 52412,
            parent_code: Some(5241),
            description: "Direct Insurance (except Life, Health, and Medical) Carriers ".to_string(),
        }),
        (524126, Code::<u32> {
            code: 524126,
            parent_code: Some(52412),
            description: "Direct Property and Casualty Insurance Carriers ".to_string(),
        }),
        (524127, Code::<u32> {
            code: 524127,
            parent_code: Some(52412),
            description: "Direct Title Insurance Carriers ".to_string(),
        }),
        (524128, Code::<u32> {
            code: 524128,
            parent_code: Some(52412),
            description: "Other Direct Insurance (except Life, Health, and Medical) Carriers ".to_string(),
        }),
        (52413, Code::<u32> {
            code: 52413,
            parent_code: Some(5241),
            description: "Reinsurance Carriers ".to_string(),
        }),
        (524130, Code::<u32> {
            code: 524130,
            parent_code: Some(52413),
            description: "Reinsurance Carriers ".to_string(),
        }),
        (5242, Code::<u32> {
            code: 5242,
            parent_code: Some(524),
            description: "Agencies, Brokerages, and Other Insurance Related Activities".to_string(),
        }),
        (52421, Code::<u32> {
            code: 52421,
            parent_code: Some(5242),
            description: "Insurance Agencies and Brokerages ".to_string(),
        }),
        (524210, Code::<u32> {
            code: 524210,
            parent_code: Some(52421),
            description: "Insurance Agencies and Brokerages ".to_string(),
        }),
        (52429, Code::<u32> {
            code: 52429,
            parent_code: Some(5242),
            description: "Other Insurance Related Activities ".to_string(),
        }),
        (524291, Code::<u32> {
            code: 524291,
            parent_code: Some(52429),
            description: "Claims Adjusting ".to_string(),
        }),
        (524292, Code::<u32> {
            code: 524292,
            parent_code: Some(52429),
            description: "Third Party Administration of Insurance and Pension Funds ".to_string(),
        }),
        (524298, Code::<u32> {
            code: 524298,
            parent_code: Some(52429),
            description: "All Other Insurance Related Activities ".to_string(),
        }),
        (525, Code::<u32> {
            code: 525,
            parent_code: Some(52),
            description: "Funds, Trusts, and Other Financial Vehicles ".to_string(),
        }),
        (5251, Code::<u32> {
            code: 5251,
            parent_code: Some(525),
            description: "Insurance and Employee Benefit Funds ".to_string(),
        }),
        (52511, Code::<u32> {
            code: 52511,
            parent_code: Some(5251),
            description: "Pension Funds ".to_string(),
        }),
        (525110, Code::<u32> {
            code: 525110,
            parent_code: Some(52511),
            description: "Pension Funds ".to_string(),
        }),
        (52512, Code::<u32> {
            code: 52512,
            parent_code: Some(5251),
            description: "Health and Welfare Funds ".to_string(),
        }),
        (525120, Code::<u32> {
            code: 525120,
            parent_code: Some(52512),
            description: "Health and Welfare Funds ".to_string(),
        }),
        (52519, Code::<u32> {
            code: 52519,
            parent_code: Some(5251),
            description: "Other Insurance Funds ".to_string(),
        }),
        (525190, Code::<u32> {
            code: 525190,
            parent_code: Some(52519),
            description: "Other Insurance Funds ".to_string(),
        }),
        (5259, Code::<u32> {
            code: 5259,
            parent_code: Some(525),
            description: "Other Investment Pools and Funds".to_string(),
        }),
        (52591, Code::<u32> {
            code: 52591,
            parent_code: Some(5259),
            description: "Open-End Investment Funds ".to_string(),
        }),
        (525910, Code::<u32> {
            code: 525910,
            parent_code: Some(52591),
            description: "Open-End Investment Funds ".to_string(),
        }),
        (52592, Code::<u32> {
            code: 52592,
            parent_code: Some(5259),
            description: "Trusts, Estates, and Agency Accounts ".to_string(),
        }),
        (525920, Code::<u32> {
            code: 525920,
            parent_code: Some(52592),
            description: "Trusts, Estates, and Agency Accounts ".to_string(),
        }),
        (52599, Code::<u32> {
            code: 52599,
            parent_code: Some(5259),
            description: "Other Financial Vehicles ".to_string(),
        }),
        (525990, Code::<u32> {
            code: 525990,
            parent_code: Some(52599),
            description: "Other Financial Vehicles ".to_string(),
        }),
        (53, Code::<u32> {
            code: 53,
            parent_code: None,
            description: "Real Estate and Rental and Leasing".to_string(),
        }),
        (531, Code::<u32> {
            code: 531,
            parent_code: Some(53),
            description: "Real Estate".to_string(),
        }),
        (5311, Code::<u32> {
            code: 5311,
            parent_code: Some(531),
            description: "Lessors of Real Estate".to_string(),
        }),
        (53111, Code::<u32> {
            code: 53111,
            parent_code: Some(5311),
            description: "Lessors of Residential Buildings and Dwellings ".to_string(),
        }),
        (531110, Code::<u32> {
            code: 531110,
            parent_code: Some(53111),
            description: "Lessors of Residential Buildings and Dwellings ".to_string(),
        }),
        (53112, Code::<u32> {
            code: 53112,
            parent_code: Some(5311),
            description: "Lessors of Nonresidential Buildings (except Miniwarehouses) ".to_string(),
        }),
        (531120, Code::<u32> {
            code: 531120,
            parent_code: Some(53112),
            description: "Lessors of Nonresidential Buildings (except Miniwarehouses) ".to_string(),
        }),
        (53113, Code::<u32> {
            code: 53113,
            parent_code: Some(5311),
            description: "Lessors of Miniwarehouses and Self-Storage Units ".to_string(),
        }),
        (531130, Code::<u32> {
            code: 531130,
            parent_code: Some(53113),
            description: "Lessors of Miniwarehouses and Self-Storage Units ".to_string(),
        }),
        (53119, Code::<u32> {
            code: 53119,
            parent_code: Some(5311),
            description: "Lessors of Other Real Estate Property ".to_string(),
        }),
        (531190, Code::<u32> {
            code: 531190,
            parent_code: Some(53119),
            description: "Lessors of Other Real Estate Property ".to_string(),
        }),
        (5312, Code::<u32> {
            code: 5312,
            parent_code: Some(531),
            description: "Offices of Real Estate Agents and Brokers".to_string(),
        }),
        (53121, Code::<u32> {
            code: 53121,
            parent_code: Some(5312),
            description: "Offices of Real Estate Agents and Brokers".to_string(),
        }),
        (531210, Code::<u32> {
            code: 531210,
            parent_code: Some(53121),
            description: "Offices of Real Estate Agents and Brokers".to_string(),
        }),
        (5313, Code::<u32> {
            code: 5313,
            parent_code: Some(531),
            description: "Activities Related to Real Estate".to_string(),
        }),
        (53131, Code::<u32> {
            code: 53131,
            parent_code: Some(5313),
            description: "Real Estate Property Managers ".to_string(),
        }),
        (531311, Code::<u32> {
            code: 531311,
            parent_code: Some(53131),
            description: "Residential Property Managers ".to_string(),
        }),
        (531312, Code::<u32> {
            code: 531312,
            parent_code: Some(53131),
            description: "Nonresidential Property Managers ".to_string(),
        }),
        (53132, Code::<u32> {
            code: 53132,
            parent_code: Some(5313),
            description: "Offices of Real Estate Appraisers ".to_string(),
        }),
        (531320, Code::<u32> {
            code: 531320,
            parent_code: Some(53132),
            description: "Offices of Real Estate Appraisers ".to_string(),
        }),
        (53139, Code::<u32> {
            code: 53139,
            parent_code: Some(5313),
            description: "Other Activities Related to Real Estate ".to_string(),
        }),
        (531390, Code::<u32> {
            code: 531390,
            parent_code: Some(53139),
            description: "Other Activities Related to Real Estate ".to_string(),
        }),
        (532, Code::<u32> {
            code: 532,
            parent_code: Some(53),
            description: "Rental and Leasing Services".to_string(),
        }),
        (5321, Code::<u32> {
            code: 5321,
            parent_code: Some(532),
            description: "Automotive Equipment Rental and Leasing".to_string(),
        }),
        (53211, Code::<u32> {
            code: 53211,
            parent_code: Some(5321),
            description: "Passenger Car Rental and Leasing".to_string(),
        }),
        (532111, Code::<u32> {
            code: 532111,
            parent_code: Some(53211),
            description: "Passenger Car Rental ".to_string(),
        }),
        (532112, Code::<u32> {
            code: 532112,
            parent_code: Some(53211),
            description: "Passenger Car Leasing ".to_string(),
        }),
        (53212, Code::<u32> {
            code: 53212,
            parent_code: Some(5321),
            description: "Truck, Utility Trailer, and RV (Recreational Vehicle) Rental and Leasing".to_string(),
        }),
        (532120, Code::<u32> {
            code: 532120,
            parent_code: Some(53212),
            description: "Truck, Utility Trailer, and RV (Recreational Vehicle) Rental and Leasing ".to_string(),
        }),
        (5322, Code::<u32> {
            code: 5322,
            parent_code: Some(532),
            description: "Consumer Goods Rental".to_string(),
        }),
        (53221, Code::<u32> {
            code: 53221,
            parent_code: Some(5322),
            description: "Consumer Electronics and Appliances Rental".to_string(),
        }),
        (532210, Code::<u32> {
            code: 532210,
            parent_code: Some(53221),
            description: "Consumer Electronics and Appliances Rental".to_string(),
        }),
        (53228, Code::<u32> {
            code: 53228,
            parent_code: Some(5322),
            description: "Other Consumer Goods Rental ".to_string(),
        }),
        (532281, Code::<u32> {
            code: 532281,
            parent_code: Some(53228),
            description: "Formal Wear and Costume Rental".to_string(),
        }),
        (532282, Code::<u32> {
            code: 532282,
            parent_code: Some(53228),
            description: "Video Tape and Disc Rental".to_string(),
        }),
        (532283, Code::<u32> {
            code: 532283,
            parent_code: Some(53228),
            description: "Home Health Equipment Rental ".to_string(),
        }),
        (532284, Code::<u32> {
            code: 532284,
            parent_code: Some(53228),
            description: "Recreational Goods Rental ".to_string(),
        }),
        (532289, Code::<u32> {
            code: 532289,
            parent_code: Some(53228),
            description: "All Other Consumer Goods Rental ".to_string(),
        }),
        (5323, Code::<u32> {
            code: 5323,
            parent_code: Some(532),
            description: "General Rental Centers".to_string(),
        }),
        (53231, Code::<u32> {
            code: 53231,
            parent_code: Some(5323),
            description: "General Rental Centers".to_string(),
        }),
        (532310, Code::<u32> {
            code: 532310,
            parent_code: Some(53231),
            description: "General Rental Centers".to_string(),
        }),
        (5324, Code::<u32> {
            code: 5324,
            parent_code: Some(532),
            description: "Commercial and Industrial Machinery and Equipment Rental and Leasing".to_string(),
        }),
        (53241, Code::<u32> {
            code: 53241,
            parent_code: Some(5324),
            description: "Construction, Transportation, Mining, and Forestry Machinery and Equipment Rental and Leasing".to_string(),
        }),
        (532411, Code::<u32> {
            code: 532411,
            parent_code: Some(53241),
            description: "Commercial Air, Rail, and Water Transportation Equipment Rental and Leasing ".to_string(),
        }),
        (532412, Code::<u32> {
            code: 532412,
            parent_code: Some(53241),
            description: "Construction, Mining, and Forestry Machinery and Equipment Rental and Leasing ".to_string(),
        }),
        (53242, Code::<u32> {
            code: 53242,
            parent_code: Some(5324),
            description: "Office Machinery and Equipment Rental and Leasing".to_string(),
        }),
        (532420, Code::<u32> {
            code: 532420,
            parent_code: Some(53242),
            description: "Office Machinery and Equipment Rental and Leasing".to_string(),
        }),
        (53249, Code::<u32> {
            code: 53249,
            parent_code: Some(5324),
            description: "Other Commercial and Industrial Machinery and Equipment Rental and Leasing".to_string(),
        }),
        (532490, Code::<u32> {
            code: 532490,
            parent_code: Some(53249),
            description: "Other Commercial and Industrial Machinery and Equipment Rental and Leasing ".to_string(),
        }),
        (533, Code::<u32> {
            code: 533,
            parent_code: Some(53),
            description: "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)".to_string(),
        }),
        (5331, Code::<u32> {
            code: 5331,
            parent_code: Some(533),
            description: "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)".to_string(),
        }),
        (53311, Code::<u32> {
            code: 53311,
            parent_code: Some(5331),
            description: "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)".to_string(),
        }),
        (533110, Code::<u32> {
            code: 533110,
            parent_code: Some(53311),
            description: "Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)".to_string(),
        }),
        (54, Code::<u32> {
            code: 54,
            parent_code: None,
            description: "Professional, Scientific, and Technical Services".to_string(),
        }),
        (541, Code::<u32> {
            code: 541,
            parent_code: Some(54),
            description: "Professional, Scientific, and Technical Services".to_string(),
        }),
        (5411, Code::<u32> {
            code: 5411,
            parent_code: Some(541),
            description: "Legal Services".to_string(),
        }),
        (54111, Code::<u32> {
            code: 54111,
            parent_code: Some(5411),
            description: "Offices of Lawyers".to_string(),
        }),
        (541110, Code::<u32> {
            code: 541110,
            parent_code: Some(54111),
            description: "Offices of Lawyers".to_string(),
        }),
        (54112, Code::<u32> {
            code: 54112,
            parent_code: Some(5411),
            description: "Offices of Notaries".to_string(),
        }),
        (541120, Code::<u32> {
            code: 541120,
            parent_code: Some(54112),
            description: "Offices of Notaries".to_string(),
        }),
        (54119, Code::<u32> {
            code: 54119,
            parent_code: Some(5411),
            description: "Other Legal Services".to_string(),
        }),
        (541191, Code::<u32> {
            code: 541191,
            parent_code: Some(54119),
            description: "Title Abstract and Settlement Offices ".to_string(),
        }),
        (541199, Code::<u32> {
            code: 541199,
            parent_code: Some(54119),
            description: "All Other Legal Services ".to_string(),
        }),
        (5412, Code::<u32> {
            code: 5412,
            parent_code: Some(541),
            description: "Accounting, Tax Preparation, Bookkeeping, and Payroll Services".to_string(),
        }),
        (54121, Code::<u32> {
            code: 54121,
            parent_code: Some(5412),
            description: "Accounting, Tax Preparation, Bookkeeping, and Payroll Services".to_string(),
        }),
        (541211, Code::<u32> {
            code: 541211,
            parent_code: Some(54121),
            description: "Offices of Certified Public Accountants ".to_string(),
        }),
        (541213, Code::<u32> {
            code: 541213,
            parent_code: Some(54121),
            description: "Tax Preparation Services ".to_string(),
        }),
        (541214, Code::<u32> {
            code: 541214,
            parent_code: Some(54121),
            description: "Payroll Services ".to_string(),
        }),
        (541219, Code::<u32> {
            code: 541219,
            parent_code: Some(54121),
            description: "Other Accounting Services ".to_string(),
        }),
        (5413, Code::<u32> {
            code: 5413,
            parent_code: Some(541),
            description: "Architectural, Engineering, and Related Services".to_string(),
        }),
        (54131, Code::<u32> {
            code: 54131,
            parent_code: Some(5413),
            description: "Architectural Services".to_string(),
        }),
        (541310, Code::<u32> {
            code: 541310,
            parent_code: Some(54131),
            description: "Architectural Services".to_string(),
        }),
        (54132, Code::<u32> {
            code: 54132,
            parent_code: Some(5413),
            description: "Landscape Architectural Services".to_string(),
        }),
        (541320, Code::<u32> {
            code: 541320,
            parent_code: Some(54132),
            description: "Landscape Architectural Services".to_string(),
        }),
        (54133, Code::<u32> {
            code: 54133,
            parent_code: Some(5413),
            description: "Engineering Services".to_string(),
        }),
        (541330, Code::<u32> {
            code: 541330,
            parent_code: Some(54133),
            description: "Engineering Services".to_string(),
        }),
        (54134, Code::<u32> {
            code: 54134,
            parent_code: Some(5413),
            description: "Drafting Services".to_string(),
        }),
        (541340, Code::<u32> {
            code: 541340,
            parent_code: Some(54134),
            description: "Drafting Services".to_string(),
        }),
        (54135, Code::<u32> {
            code: 54135,
            parent_code: Some(5413),
            description: "Building Inspection Services".to_string(),
        }),
        (541350, Code::<u32> {
            code: 541350,
            parent_code: Some(54135),
            description: "Building Inspection Services".to_string(),
        }),
        (54136, Code::<u32> {
            code: 54136,
            parent_code: Some(5413),
            description: "Geophysical Surveying and Mapping Services".to_string(),
        }),
        (541360, Code::<u32> {
            code: 541360,
            parent_code: Some(54136),
            description: "Geophysical Surveying and Mapping Services".to_string(),
        }),
        (54137, Code::<u32> {
            code: 54137,
            parent_code: Some(5413),
            description: "Surveying and Mapping (except Geophysical) Services".to_string(),
        }),
        (541370, Code::<u32> {
            code: 541370,
            parent_code: Some(54137),
            description: "Surveying and Mapping (except Geophysical) Services".to_string(),
        }),
        (54138, Code::<u32> {
            code: 54138,
            parent_code: Some(5413),
            description: "Testing Laboratories".to_string(),
        }),
        (541380, Code::<u32> {
            code: 541380,
            parent_code: Some(54138),
            description: "Testing Laboratories".to_string(),
        }),
        (5414, Code::<u32> {
            code: 5414,
            parent_code: Some(541),
            description: "Specialized Design Services".to_string(),
        }),
        (54141, Code::<u32> {
            code: 54141,
            parent_code: Some(5414),
            description: "Interior Design Services".to_string(),
        }),
        (541410, Code::<u32> {
            code: 541410,
            parent_code: Some(54141),
            description: "Interior Design Services".to_string(),
        }),
        (54142, Code::<u32> {
            code: 54142,
            parent_code: Some(5414),
            description: "Industrial Design Services".to_string(),
        }),
        (541420, Code::<u32> {
            code: 541420,
            parent_code: Some(54142),
            description: "Industrial Design Services".to_string(),
        }),
        (54143, Code::<u32> {
            code: 54143,
            parent_code: Some(5414),
            description: "Graphic Design Services".to_string(),
        }),
        (541430, Code::<u32> {
            code: 541430,
            parent_code: Some(54143),
            description: "Graphic Design Services".to_string(),
        }),
        (54149, Code::<u32> {
            code: 54149,
            parent_code: Some(5414),
            description: "Other Specialized Design Services".to_string(),
        }),
        (541490, Code::<u32> {
            code: 541490,
            parent_code: Some(54149),
            description: "Other Specialized Design Services".to_string(),
        }),
        (5415, Code::<u32> {
            code: 5415,
            parent_code: Some(541),
            description: "Computer Systems Design and Related Services".to_string(),
        }),
        (54151, Code::<u32> {
            code: 54151,
            parent_code: Some(5415),
            description: "Computer Systems Design and Related Services".to_string(),
        }),
        (541511, Code::<u32> {
            code: 541511,
            parent_code: Some(54151),
            description: "Custom Computer Programming Services ".to_string(),
        }),
        (541512, Code::<u32> {
            code: 541512,
            parent_code: Some(54151),
            description: "Computer Systems Design Services ".to_string(),
        }),
        (541513, Code::<u32> {
            code: 541513,
            parent_code: Some(54151),
            description: "Computer Facilities Management Services ".to_string(),
        }),
        (541519, Code::<u32> {
            code: 541519,
            parent_code: Some(54151),
            description: "Other Computer Related Services".to_string(),
        }),
        (5416, Code::<u32> {
            code: 5416,
            parent_code: Some(541),
            description: "Management, Scientific, and Technical Consulting Services".to_string(),
        }),
        (54161, Code::<u32> {
            code: 54161,
            parent_code: Some(5416),
            description: "Management Consulting Services".to_string(),
        }),
        (541611, Code::<u32> {
            code: 541611,
            parent_code: Some(54161),
            description: "Administrative Management and General Management Consulting Services ".to_string(),
        }),
        (541612, Code::<u32> {
            code: 541612,
            parent_code: Some(54161),
            description: "Human Resources Consulting Services ".to_string(),
        }),
        (541613, Code::<u32> {
            code: 541613,
            parent_code: Some(54161),
            description: "Marketing Consulting Services ".to_string(),
        }),
        (541614, Code::<u32> {
            code: 541614,
            parent_code: Some(54161),
            description: "Process, Physical Distribution, and Logistics Consulting Services ".to_string(),
        }),
        (541618, Code::<u32> {
            code: 541618,
            parent_code: Some(54161),
            description: "Other Management Consulting Services ".to_string(),
        }),
        (54162, Code::<u32> {
            code: 54162,
            parent_code: Some(5416),
            description: "Environmental Consulting Services".to_string(),
        }),
        (541620, Code::<u32> {
            code: 541620,
            parent_code: Some(54162),
            description: "Environmental Consulting Services".to_string(),
        }),
        (54169, Code::<u32> {
            code: 54169,
            parent_code: Some(5416),
            description: "Other Scientific and Technical Consulting Services".to_string(),
        }),
        (541690, Code::<u32> {
            code: 541690,
            parent_code: Some(54169),
            description: "Other Scientific and Technical Consulting Services".to_string(),
        }),
        (5417, Code::<u32> {
            code: 5417,
            parent_code: Some(541),
            description: "Scientific Research and Development Services".to_string(),
        }),
        (54171, Code::<u32> {
            code: 54171,
            parent_code: Some(5417),
            description: "Research and Development in the Physical, Engineering, and Life Sciences".to_string(),
        }),
        (541713, Code::<u32> {
            code: 541713,
            parent_code: Some(54171),
            description: "Research and Development in Nanotechnology ".to_string(),
        }),
        (541714, Code::<u32> {
            code: 541714,
            parent_code: Some(54171),
            description: "Research and Development in Biotechnology (except Nanobiotechnology)".to_string(),
        }),
        (541715, Code::<u32> {
            code: 541715,
            parent_code: Some(54171),
            description: "Research and Development in the Physical, Engineering, and Life Sciences (except Nanotechnology and Biotechnology) ".to_string(),
        }),
        (54172, Code::<u32> {
            code: 54172,
            parent_code: Some(5417),
            description: "Research and Development in the Social Sciences and Humanities".to_string(),
        }),
        (541720, Code::<u32> {
            code: 541720,
            parent_code: Some(54172),
            description: "Research and Development in the Social Sciences and Humanities ".to_string(),
        }),
        (5418, Code::<u32> {
            code: 5418,
            parent_code: Some(541),
            description: "Advertising, Public Relations, and Related Services".to_string(),
        }),
        (54181, Code::<u32> {
            code: 54181,
            parent_code: Some(5418),
            description: "Advertising Agencies".to_string(),
        }),
        (541810, Code::<u32> {
            code: 541810,
            parent_code: Some(54181),
            description: "Advertising Agencies".to_string(),
        }),
        (54182, Code::<u32> {
            code: 54182,
            parent_code: Some(5418),
            description: "Public Relations Agencies".to_string(),
        }),
        (541820, Code::<u32> {
            code: 541820,
            parent_code: Some(54182),
            description: "Public Relations Agencies".to_string(),
        }),
        (54183, Code::<u32> {
            code: 54183,
            parent_code: Some(5418),
            description: "Media Buying Agencies".to_string(),
        }),
        (541830, Code::<u32> {
            code: 541830,
            parent_code: Some(54183),
            description: "Media Buying Agencies".to_string(),
        }),
        (54184, Code::<u32> {
            code: 54184,
            parent_code: Some(5418),
            description: "Media Representatives".to_string(),
        }),
        (541840, Code::<u32> {
            code: 541840,
            parent_code: Some(54184),
            description: "Media Representatives".to_string(),
        }),
        (54185, Code::<u32> {
            code: 54185,
            parent_code: Some(5418),
            description: "Outdoor Advertising".to_string(),
        }),
        (541850, Code::<u32> {
            code: 541850,
            parent_code: Some(54185),
            description: "Outdoor Advertising".to_string(),
        }),
        (54186, Code::<u32> {
            code: 54186,
            parent_code: Some(5418),
            description: "Direct Mail Advertising".to_string(),
        }),
        (541860, Code::<u32> {
            code: 541860,
            parent_code: Some(54186),
            description: "Direct Mail Advertising".to_string(),
        }),
        (54187, Code::<u32> {
            code: 54187,
            parent_code: Some(5418),
            description: "Advertising Material Distribution Services".to_string(),
        }),
        (541870, Code::<u32> {
            code: 541870,
            parent_code: Some(54187),
            description: "Advertising Material Distribution Services".to_string(),
        }),
        (54189, Code::<u32> {
            code: 54189,
            parent_code: Some(5418),
            description: "Other Services Related to Advertising".to_string(),
        }),
        (541890, Code::<u32> {
            code: 541890,
            parent_code: Some(54189),
            description: "Other Services Related to Advertising ".to_string(),
        }),
        (5419, Code::<u32> {
            code: 5419,
            parent_code: Some(541),
            description: "Other Professional, Scientific, and Technical Services".to_string(),
        }),
        (54191, Code::<u32> {
            code: 54191,
            parent_code: Some(5419),
            description: "Marketing Research and Public Opinion Polling".to_string(),
        }),
        (541910, Code::<u32> {
            code: 541910,
            parent_code: Some(54191),
            description: "Marketing Research and Public Opinion Polling".to_string(),
        }),
        (54192, Code::<u32> {
            code: 54192,
            parent_code: Some(5419),
            description: "Photographic Services".to_string(),
        }),
        (541921, Code::<u32> {
            code: 541921,
            parent_code: Some(54192),
            description: "Photography Studios, Portrait ".to_string(),
        }),
        (541922, Code::<u32> {
            code: 541922,
            parent_code: Some(54192),
            description: "Commercial Photography ".to_string(),
        }),
        (54193, Code::<u32> {
            code: 54193,
            parent_code: Some(5419),
            description: "Translation and Interpretation Services".to_string(),
        }),
        (541930, Code::<u32> {
            code: 541930,
            parent_code: Some(54193),
            description: "Translation and Interpretation Services".to_string(),
        }),
        (54194, Code::<u32> {
            code: 54194,
            parent_code: Some(5419),
            description: "Veterinary Services".to_string(),
        }),
        (541940, Code::<u32> {
            code: 541940,
            parent_code: Some(54194),
            description: "Veterinary Services ".to_string(),
        }),
        (54199, Code::<u32> {
            code: 54199,
            parent_code: Some(5419),
            description: "All Other Professional, Scientific, and Technical Services".to_string(),
        }),
        (541990, Code::<u32> {
            code: 541990,
            parent_code: Some(54199),
            description: "All Other Professional, Scientific, and Technical Services".to_string(),
        }),
        (55, Code::<u32> {
            code: 55,
            parent_code: None,
            description: "Management of Companies and Enterprises".to_string(),
        }),
        (551, Code::<u32> {
            code: 551,
            parent_code: Some(55),
            description: "Management of Companies and Enterprises".to_string(),
        }),
        (5511, Code::<u32> {
            code: 5511,
            parent_code: Some(551),
            description: "Management of Companies and Enterprises".to_string(),
        }),
        (55111, Code::<u32> {
            code: 55111,
            parent_code: Some(5511),
            description: "Management of Companies and Enterprises".to_string(),
        }),
        (551111, Code::<u32> {
            code: 551111,
            parent_code: Some(55111),
            description: "Offices of Bank Holding Companies ".to_string(),
        }),
        (551112, Code::<u32> {
            code: 551112,
            parent_code: Some(55111),
            description: "Offices of Other Holding Companies ".to_string(),
        }),
        (551114, Code::<u32> {
            code: 551114,
            parent_code: Some(55111),
            description: "Corporate, Subsidiary, and Regional Managing Offices ".to_string(),
        }),
        (56, Code::<u32> {
            code: 56,
            parent_code: None,
            description: "Administrative and Support and Waste Management and Remediation Services".to_string(),
        }),
        (561, Code::<u32> {
            code: 561,
            parent_code: Some(56),
            description: "Administrative and Support Services".to_string(),
        }),
        (5611, Code::<u32> {
            code: 5611,
            parent_code: Some(561),
            description: "Office Administrative Services".to_string(),
        }),
        (56111, Code::<u32> {
            code: 56111,
            parent_code: Some(5611),
            description: "Office Administrative Services".to_string(),
        }),
        (561110, Code::<u32> {
            code: 561110,
            parent_code: Some(56111),
            description: "Office Administrative Services".to_string(),
        }),
        (5612, Code::<u32> {
            code: 5612,
            parent_code: Some(561),
            description: "Facilities Support Services".to_string(),
        }),
        (56121, Code::<u32> {
            code: 56121,
            parent_code: Some(5612),
            description: "Facilities Support Services".to_string(),
        }),
        (561210, Code::<u32> {
            code: 561210,
            parent_code: Some(56121),
            description: "Facilities Support Services".to_string(),
        }),
        (5613, Code::<u32> {
            code: 5613,
            parent_code: Some(561),
            description: "Employment Services".to_string(),
        }),
        (56131, Code::<u32> {
            code: 56131,
            parent_code: Some(5613),
            description: "Employment Placement Agencies and Executive Search Services".to_string(),
        }),
        (561311, Code::<u32> {
            code: 561311,
            parent_code: Some(56131),
            description: "Employment Placement Agencies ".to_string(),
        }),
        (561312, Code::<u32> {
            code: 561312,
            parent_code: Some(56131),
            description: "Executive Search Services ".to_string(),
        }),
        (56132, Code::<u32> {
            code: 56132,
            parent_code: Some(5613),
            description: "Temporary Help Services".to_string(),
        }),
        (561320, Code::<u32> {
            code: 561320,
            parent_code: Some(56132),
            description: "Temporary Help Services".to_string(),
        }),
        (56133, Code::<u32> {
            code: 56133,
            parent_code: Some(5613),
            description: "Professional Employer Organizations".to_string(),
        }),
        (561330, Code::<u32> {
            code: 561330,
            parent_code: Some(56133),
            description: "Professional Employer Organizations".to_string(),
        }),
        (5614, Code::<u32> {
            code: 5614,
            parent_code: Some(561),
            description: "Business Support Services".to_string(),
        }),
        (56141, Code::<u32> {
            code: 56141,
            parent_code: Some(5614),
            description: "Document Preparation Services".to_string(),
        }),
        (561410, Code::<u32> {
            code: 561410,
            parent_code: Some(56141),
            description: "Document Preparation Services".to_string(),
        }),
        (56142, Code::<u32> {
            code: 56142,
            parent_code: Some(5614),
            description: "Telephone Call Centers".to_string(),
        }),
        (561421, Code::<u32> {
            code: 561421,
            parent_code: Some(56142),
            description: "Telephone Answering Services ".to_string(),
        }),
        (561422, Code::<u32> {
            code: 561422,
            parent_code: Some(56142),
            description: "Telemarketing Bureaus and Other Contact Centers ".to_string(),
        }),
        (56143, Code::<u32> {
            code: 56143,
            parent_code: Some(5614),
            description: "Business Service Centers".to_string(),
        }),
        (561431, Code::<u32> {
            code: 561431,
            parent_code: Some(56143),
            description: "Private Mail Centers ".to_string(),
        }),
        (561439, Code::<u32> {
            code: 561439,
            parent_code: Some(56143),
            description: "Other Business Service Centers (including Copy Shops) ".to_string(),
        }),
        (56144, Code::<u32> {
            code: 56144,
            parent_code: Some(5614),
            description: "Collection Agencies".to_string(),
        }),
        (561440, Code::<u32> {
            code: 561440,
            parent_code: Some(56144),
            description: "Collection Agencies".to_string(),
        }),
        (56145, Code::<u32> {
            code: 56145,
            parent_code: Some(5614),
            description: "Credit Bureaus".to_string(),
        }),
        (561450, Code::<u32> {
            code: 561450,
            parent_code: Some(56145),
            description: "Credit Bureaus".to_string(),
        }),
        (56149, Code::<u32> {
            code: 56149,
            parent_code: Some(5614),
            description: "Other Business Support Services".to_string(),
        }),
        (561491, Code::<u32> {
            code: 561491,
            parent_code: Some(56149),
            description: "Repossession Services ".to_string(),
        }),
        (561492, Code::<u32> {
            code: 561492,
            parent_code: Some(56149),
            description: "Court Reporting and Stenotype Services ".to_string(),
        }),
        (561499, Code::<u32> {
            code: 561499,
            parent_code: Some(56149),
            description: "All Other Business Support Services ".to_string(),
        }),
        (5615, Code::<u32> {
            code: 5615,
            parent_code: Some(561),
            description: "Travel Arrangement and Reservation Services".to_string(),
        }),
        (56151, Code::<u32> {
            code: 56151,
            parent_code: Some(5615),
            description: "Travel Agencies".to_string(),
        }),
        (561510, Code::<u32> {
            code: 561510,
            parent_code: Some(56151),
            description: "Travel Agencies".to_string(),
        }),
        (56152, Code::<u32> {
            code: 56152,
            parent_code: Some(5615),
            description: "Tour Operators".to_string(),
        }),
        (561520, Code::<u32> {
            code: 561520,
            parent_code: Some(56152),
            description: "Tour Operators".to_string(),
        }),
        (56159, Code::<u32> {
            code: 56159,
            parent_code: Some(5615),
            description: "Other Travel Arrangement and Reservation Services".to_string(),
        }),
        (561591, Code::<u32> {
            code: 561591,
            parent_code: Some(56159),
            description: "Convention and Visitors Bureaus ".to_string(),
        }),
        (561599, Code::<u32> {
            code: 561599,
            parent_code: Some(56159),
            description: "All Other Travel Arrangement and Reservation Services ".to_string(),
        }),
        (5616, Code::<u32> {
            code: 5616,
            parent_code: Some(561),
            description: "Investigation and Security Services".to_string(),
        }),
        (56161, Code::<u32> {
            code: 56161,
            parent_code: Some(5616),
            description: "Investigation, Guard, and Armored Car Services".to_string(),
        }),
        (561611, Code::<u32> {
            code: 561611,
            parent_code: Some(56161),
            description: "Investigation Services ".to_string(),
        }),
        (561612, Code::<u32> {
            code: 561612,
            parent_code: Some(56161),
            description: "Security Guards and Patrol Services ".to_string(),
        }),
        (561613, Code::<u32> {
            code: 561613,
            parent_code: Some(56161),
            description: "Armored Car Services ".to_string(),
        }),
        (56162, Code::<u32> {
            code: 56162,
            parent_code: Some(5616),
            description: "Security Systems Services".to_string(),
        }),
        (561621, Code::<u32> {
            code: 561621,
            parent_code: Some(56162),
            description: "Security Systems Services (except Locksmiths) ".to_string(),
        }),
        (561622, Code::<u32> {
            code: 561622,
            parent_code: Some(56162),
            description: "Locksmiths ".to_string(),
        }),
        (5617, Code::<u32> {
            code: 5617,
            parent_code: Some(561),
            description: "Services to Buildings and Dwellings".to_string(),
        }),
        (56171, Code::<u32> {
            code: 56171,
            parent_code: Some(5617),
            description: "Exterminating and Pest Control Services".to_string(),
        }),
        (561710, Code::<u32> {
            code: 561710,
            parent_code: Some(56171),
            description: "Exterminating and Pest Control Services".to_string(),
        }),
        (56172, Code::<u32> {
            code: 56172,
            parent_code: Some(5617),
            description: "Janitorial Services".to_string(),
        }),
        (561720, Code::<u32> {
            code: 561720,
            parent_code: Some(56172),
            description: "Janitorial Services ".to_string(),
        }),
        (56173, Code::<u32> {
            code: 56173,
            parent_code: Some(5617),
            description: "Landscaping Services".to_string(),
        }),
        (561730, Code::<u32> {
            code: 561730,
            parent_code: Some(56173),
            description: "Landscaping Services".to_string(),
        }),
        (56174, Code::<u32> {
            code: 56174,
            parent_code: Some(5617),
            description: "Carpet and Upholstery Cleaning Services".to_string(),
        }),
        (561740, Code::<u32> {
            code: 561740,
            parent_code: Some(56174),
            description: "Carpet and Upholstery Cleaning Services".to_string(),
        }),
        (56179, Code::<u32> {
            code: 56179,
            parent_code: Some(5617),
            description: "Other Services to Buildings and Dwellings".to_string(),
        }),
        (561790, Code::<u32> {
            code: 561790,
            parent_code: Some(56179),
            description: "Other Services to Buildings and Dwellings ".to_string(),
        }),
        (5619, Code::<u32> {
            code: 5619,
            parent_code: Some(561),
            description: "Other Support Services".to_string(),
        }),
        (56191, Code::<u32> {
            code: 56191,
            parent_code: Some(5619),
            description: "Packaging and Labeling Services".to_string(),
        }),
        (561910, Code::<u32> {
            code: 561910,
            parent_code: Some(56191),
            description: "Packaging and Labeling Services".to_string(),
        }),
        (56192, Code::<u32> {
            code: 56192,
            parent_code: Some(5619),
            description: "Convention and Trade Show Organizers".to_string(),
        }),
        (561920, Code::<u32> {
            code: 561920,
            parent_code: Some(56192),
            description: "Convention and Trade Show Organizers".to_string(),
        }),
        (56199, Code::<u32> {
            code: 56199,
            parent_code: Some(5619),
            description: "All Other Support Services".to_string(),
        }),
        (561990, Code::<u32> {
            code: 561990,
            parent_code: Some(56199),
            description: "All Other Support Services".to_string(),
        }),
        (562, Code::<u32> {
            code: 562,
            parent_code: Some(56),
            description: "Waste Management and Remediation Services".to_string(),
        }),
        (5621, Code::<u32> {
            code: 5621,
            parent_code: Some(562),
            description: "Waste Collection ".to_string(),
        }),
        (56211, Code::<u32> {
            code: 56211,
            parent_code: Some(5621),
            description: "Waste Collection ".to_string(),
        }),
        (562111, Code::<u32> {
            code: 562111,
            parent_code: Some(56211),
            description: "Solid Waste Collection ".to_string(),
        }),
        (562112, Code::<u32> {
            code: 562112,
            parent_code: Some(56211),
            description: "Hazardous Waste Collection ".to_string(),
        }),
        (562119, Code::<u32> {
            code: 562119,
            parent_code: Some(56211),
            description: "Other Waste Collection ".to_string(),
        }),
        (5622, Code::<u32> {
            code: 5622,
            parent_code: Some(562),
            description: "Waste Treatment and Disposal ".to_string(),
        }),
        (56221, Code::<u32> {
            code: 56221,
            parent_code: Some(5622),
            description: "Waste Treatment and Disposal ".to_string(),
        }),
        (562211, Code::<u32> {
            code: 562211,
            parent_code: Some(56221),
            description: "Hazardous Waste Treatment and Disposal ".to_string(),
        }),
        (562212, Code::<u32> {
            code: 562212,
            parent_code: Some(56221),
            description: "Solid Waste Landfill ".to_string(),
        }),
        (562213, Code::<u32> {
            code: 562213,
            parent_code: Some(56221),
            description: "Solid Waste Combustors and Incinerators ".to_string(),
        }),
        (562219, Code::<u32> {
            code: 562219,
            parent_code: Some(56221),
            description: "Other Nonhazardous Waste Treatment and Disposal ".to_string(),
        }),
        (5629, Code::<u32> {
            code: 5629,
            parent_code: Some(562),
            description: "Remediation and Other Waste Management Services ".to_string(),
        }),
        (56291, Code::<u32> {
            code: 56291,
            parent_code: Some(5629),
            description: "Remediation Services ".to_string(),
        }),
        (562910, Code::<u32> {
            code: 562910,
            parent_code: Some(56291),
            description: "Remediation Services ".to_string(),
        }),
        (56292, Code::<u32> {
            code: 56292,
            parent_code: Some(5629),
            description: "Materials Recovery Facilities ".to_string(),
        }),
        (562920, Code::<u32> {
            code: 562920,
            parent_code: Some(56292),
            description: "Materials Recovery Facilities ".to_string(),
        }),
        (56299, Code::<u32> {
            code: 56299,
            parent_code: Some(5629),
            description: "All Other Waste Management Services ".to_string(),
        }),
        (562991, Code::<u32> {
            code: 562991,
            parent_code: Some(56299),
            description: "Septic Tank and Related Services ".to_string(),
        }),
        (562998, Code::<u32> {
            code: 562998,
            parent_code: Some(56299),
            description: "All Other Miscellaneous Waste Management Services ".to_string(),
        }),
        (61, Code::<u32> {
            code: 61,
            parent_code: None,
            description: "Educational Services".to_string(),
        }),
        (611, Code::<u32> {
            code: 611,
            parent_code: Some(61),
            description: "Educational Services".to_string(),
        }),
        (6111, Code::<u32> {
            code: 6111,
            parent_code: Some(611),
            description: "Elementary and Secondary Schools".to_string(),
        }),
        (61111, Code::<u32> {
            code: 61111,
            parent_code: Some(6111),
            description: "Elementary and Secondary Schools ".to_string(),
        }),
        (611110, Code::<u32> {
            code: 611110,
            parent_code: Some(61111),
            description: "Elementary and Secondary Schools ".to_string(),
        }),
        (6112, Code::<u32> {
            code: 6112,
            parent_code: Some(611),
            description: "Junior Colleges".to_string(),
        }),
        (61121, Code::<u32> {
            code: 61121,
            parent_code: Some(6112),
            description: "Junior Colleges".to_string(),
        }),
        (611210, Code::<u32> {
            code: 611210,
            parent_code: Some(61121),
            description: "Junior Colleges ".to_string(),
        }),
        (6113, Code::<u32> {
            code: 6113,
            parent_code: Some(611),
            description: "Colleges, Universities, and Professional Schools".to_string(),
        }),
        (61131, Code::<u32> {
            code: 61131,
            parent_code: Some(6113),
            description: "Colleges, Universities, and Professional Schools".to_string(),
        }),
        (611310, Code::<u32> {
            code: 611310,
            parent_code: Some(61131),
            description: "Colleges, Universities, and Professional Schools ".to_string(),
        }),
        (6114, Code::<u32> {
            code: 6114,
            parent_code: Some(611),
            description: "Business Schools and Computer and Management Training".to_string(),
        }),
        (61141, Code::<u32> {
            code: 61141,
            parent_code: Some(6114),
            description: "Business and Secretarial Schools".to_string(),
        }),
        (611410, Code::<u32> {
            code: 611410,
            parent_code: Some(61141),
            description: "Business and Secretarial Schools ".to_string(),
        }),
        (61142, Code::<u32> {
            code: 61142,
            parent_code: Some(6114),
            description: "Computer Training".to_string(),
        }),
        (611420, Code::<u32> {
            code: 611420,
            parent_code: Some(61142),
            description: "Computer Training ".to_string(),
        }),
        (61143, Code::<u32> {
            code: 61143,
            parent_code: Some(6114),
            description: "Professional and Management Development Training".to_string(),
        }),
        (611430, Code::<u32> {
            code: 611430,
            parent_code: Some(61143),
            description: "Professional and Management Development Training ".to_string(),
        }),
        (6115, Code::<u32> {
            code: 6115,
            parent_code: Some(611),
            description: "Technical and Trade Schools ".to_string(),
        }),
        (61151, Code::<u32> {
            code: 61151,
            parent_code: Some(6115),
            description: "Technical and Trade Schools".to_string(),
        }),
        (611511, Code::<u32> {
            code: 611511,
            parent_code: Some(61151),
            description: "Cosmetology and Barber Schools ".to_string(),
        }),
        (611512, Code::<u32> {
            code: 611512,
            parent_code: Some(61151),
            description: "Flight Training ".to_string(),
        }),
        (611513, Code::<u32> {
            code: 611513,
            parent_code: Some(61151),
            description: "Apprenticeship Training ".to_string(),
        }),
        (611519, Code::<u32> {
            code: 611519,
            parent_code: Some(61151),
            description: "Other Technical and Trade Schools ".to_string(),
        }),
        (6116, Code::<u32> {
            code: 6116,
            parent_code: Some(611),
            description: "Other Schools and Instruction".to_string(),
        }),
        (61161, Code::<u32> {
            code: 61161,
            parent_code: Some(6116),
            description: "Fine Arts Schools".to_string(),
        }),
        (611610, Code::<u32> {
            code: 611610,
            parent_code: Some(61161),
            description: "Fine Arts Schools ".to_string(),
        }),
        (61162, Code::<u32> {
            code: 61162,
            parent_code: Some(6116),
            description: "Sports and Recreation Instruction".to_string(),
        }),
        (611620, Code::<u32> {
            code: 611620,
            parent_code: Some(61162),
            description: "Sports and Recreation Instruction ".to_string(),
        }),
        (61163, Code::<u32> {
            code: 61163,
            parent_code: Some(6116),
            description: "Language Schools".to_string(),
        }),
        (611630, Code::<u32> {
            code: 611630,
            parent_code: Some(61163),
            description: "Language Schools ".to_string(),
        }),
        (61169, Code::<u32> {
            code: 61169,
            parent_code: Some(6116),
            description: "All Other Schools and Instruction".to_string(),
        }),
        (611691, Code::<u32> {
            code: 611691,
            parent_code: Some(61169),
            description: "Exam Preparation and Tutoring ".to_string(),
        }),
        (611692, Code::<u32> {
            code: 611692,
            parent_code: Some(61169),
            description: "Automobile Driving Schools ".to_string(),
        }),
        (611699, Code::<u32> {
            code: 611699,
            parent_code: Some(61169),
            description: "All Other Miscellaneous Schools and Instruction ".to_string(),
        }),
        (6117, Code::<u32> {
            code: 6117,
            parent_code: Some(611),
            description: "Educational Support Services".to_string(),
        }),
        (61171, Code::<u32> {
            code: 61171,
            parent_code: Some(6117),
            description: "Educational Support Services".to_string(),
        }),
        (611710, Code::<u32> {
            code: 611710,
            parent_code: Some(61171),
            description: "Educational Support Services".to_string(),
        }),
        (62, Code::<u32> {
            code: 62,
            parent_code: None,
            description: "Health Care and Social Assistance".to_string(),
        }),
        (621, Code::<u32> {
            code: 621,
            parent_code: Some(62),
            description: "Ambulatory Health Care Services".to_string(),
        }),
        (6211, Code::<u32> {
            code: 6211,
            parent_code: Some(621),
            description: "Offices of Physicians".to_string(),
        }),
        (62111, Code::<u32> {
            code: 62111,
            parent_code: Some(6211),
            description: "Offices of Physicians".to_string(),
        }),
        (621111, Code::<u32> {
            code: 621111,
            parent_code: Some(62111),
            description: "Offices of Physicians (except Mental Health Specialists) ".to_string(),
        }),
        (621112, Code::<u32> {
            code: 621112,
            parent_code: Some(62111),
            description: "Offices of Physicians, Mental Health Specialists ".to_string(),
        }),
        (6212, Code::<u32> {
            code: 6212,
            parent_code: Some(621),
            description: "Offices of Dentists".to_string(),
        }),
        (62121, Code::<u32> {
            code: 62121,
            parent_code: Some(6212),
            description: "Offices of Dentists".to_string(),
        }),
        (621210, Code::<u32> {
            code: 621210,
            parent_code: Some(62121),
            description: "Offices of Dentists ".to_string(),
        }),
        (6213, Code::<u32> {
            code: 6213,
            parent_code: Some(621),
            description: "Offices of Other Health Practitioners".to_string(),
        }),
        (62131, Code::<u32> {
            code: 62131,
            parent_code: Some(6213),
            description: "Offices of Chiropractors".to_string(),
        }),
        (621310, Code::<u32> {
            code: 621310,
            parent_code: Some(62131),
            description: "Offices of Chiropractors ".to_string(),
        }),
        (62132, Code::<u32> {
            code: 62132,
            parent_code: Some(6213),
            description: "Offices of Optometrists".to_string(),
        }),
        (621320, Code::<u32> {
            code: 621320,
            parent_code: Some(62132),
            description: "Offices of Optometrists".to_string(),
        }),
        (62133, Code::<u32> {
            code: 62133,
            parent_code: Some(6213),
            description: "Offices of Mental Health Practitioners (except Physicians)".to_string(),
        }),
        (621330, Code::<u32> {
            code: 621330,
            parent_code: Some(62133),
            description: "Offices of Mental Health Practitioners (except Physicians) ".to_string(),
        }),
        (62134, Code::<u32> {
            code: 62134,
            parent_code: Some(6213),
            description: "Offices of Physical, Occupational and Speech Therapists, and Audiologists".to_string(),
        }),
        (621340, Code::<u32> {
            code: 621340,
            parent_code: Some(62134),
            description: "Offices of Physical, Occupational and Speech Therapists, and Audiologists ".to_string(),
        }),
        (62139, Code::<u32> {
            code: 62139,
            parent_code: Some(6213),
            description: "Offices of All Other Health Practitioners".to_string(),
        }),
        (621391, Code::<u32> {
            code: 621391,
            parent_code: Some(62139),
            description: "Offices of Podiatrists ".to_string(),
        }),
        (621399, Code::<u32> {
            code: 621399,
            parent_code: Some(62139),
            description: "Offices of All Other Miscellaneous Health Practitioners ".to_string(),
        }),
        (6214, Code::<u32> {
            code: 6214,
            parent_code: Some(621),
            description: "Outpatient Care Centers".to_string(),
        }),
        (62141, Code::<u32> {
            code: 62141,
            parent_code: Some(6214),
            description: "Family Planning Centers".to_string(),
        }),
        (621410, Code::<u32> {
            code: 621410,
            parent_code: Some(62141),
            description: "Family Planning Centers ".to_string(),
        }),
        (62142, Code::<u32> {
            code: 62142,
            parent_code: Some(6214),
            description: "Outpatient Mental Health and Substance Abuse Centers".to_string(),
        }),
        (621420, Code::<u32> {
            code: 621420,
            parent_code: Some(62142),
            description: "Outpatient Mental Health and Substance Abuse Centers ".to_string(),
        }),
        (62149, Code::<u32> {
            code: 62149,
            parent_code: Some(6214),
            description: "Other Outpatient Care Centers".to_string(),
        }),
        (621491, Code::<u32> {
            code: 621491,
            parent_code: Some(62149),
            description: "HMO Medical Centers ".to_string(),
        }),
        (621492, Code::<u32> {
            code: 621492,
            parent_code: Some(62149),
            description: "Kidney Dialysis Centers ".to_string(),
        }),
        (621493, Code::<u32> {
            code: 621493,
            parent_code: Some(62149),
            description: "Freestanding Ambulatory Surgical and Emergency Centers ".to_string(),
        }),
        (621498, Code::<u32> {
            code: 621498,
            parent_code: Some(62149),
            description: "All Other Outpatient Care Centers ".to_string(),
        }),
        (6215, Code::<u32> {
            code: 6215,
            parent_code: Some(621),
            description: "Medical and Diagnostic Laboratories".to_string(),
        }),
        (62151, Code::<u32> {
            code: 62151,
            parent_code: Some(6215),
            description: "Medical and Diagnostic Laboratories".to_string(),
        }),
        (621511, Code::<u32> {
            code: 621511,
            parent_code: Some(62151),
            description: "Medical Laboratories ".to_string(),
        }),
        (621512, Code::<u32> {
            code: 621512,
            parent_code: Some(62151),
            description: "Diagnostic Imaging Centers ".to_string(),
        }),
        (6216, Code::<u32> {
            code: 6216,
            parent_code: Some(621),
            description: "Home Health Care Services".to_string(),
        }),
        (62161, Code::<u32> {
            code: 62161,
            parent_code: Some(6216),
            description: "Home Health Care Services".to_string(),
        }),
        (621610, Code::<u32> {
            code: 621610,
            parent_code: Some(62161),
            description: "Home Health Care Services".to_string(),
        }),
        (6219, Code::<u32> {
            code: 6219,
            parent_code: Some(621),
            description: "Other Ambulatory Health Care Services".to_string(),
        }),
        (62191, Code::<u32> {
            code: 62191,
            parent_code: Some(6219),
            description: "Ambulance Services".to_string(),
        }),
        (621910, Code::<u32> {
            code: 621910,
            parent_code: Some(62191),
            description: "Ambulance Services ".to_string(),
        }),
        (62199, Code::<u32> {
            code: 62199,
            parent_code: Some(6219),
            description: "All Other Ambulatory Health Care Services".to_string(),
        }),
        (621991, Code::<u32> {
            code: 621991,
            parent_code: Some(62199),
            description: "Blood and Organ Banks ".to_string(),
        }),
        (621999, Code::<u32> {
            code: 621999,
            parent_code: Some(62199),
            description: "All Other Miscellaneous Ambulatory Health Care Services ".to_string(),
        }),
        (622, Code::<u32> {
            code: 622,
            parent_code: Some(62),
            description: "Hospitals".to_string(),
        }),
        (6221, Code::<u32> {
            code: 6221,
            parent_code: Some(622),
            description: "General Medical and Surgical Hospitals".to_string(),
        }),
        (62211, Code::<u32> {
            code: 62211,
            parent_code: Some(6221),
            description: "General Medical and Surgical Hospitals".to_string(),
        }),
        (622110, Code::<u32> {
            code: 622110,
            parent_code: Some(62211),
            description: "General Medical and Surgical Hospitals ".to_string(),
        }),
        (6222, Code::<u32> {
            code: 6222,
            parent_code: Some(622),
            description: "Psychiatric and Substance Abuse Hospitals".to_string(),
        }),
        (62221, Code::<u32> {
            code: 62221,
            parent_code: Some(6222),
            description: "Psychiatric and Substance Abuse Hospitals".to_string(),
        }),
        (622210, Code::<u32> {
            code: 622210,
            parent_code: Some(62221),
            description: "Psychiatric and Substance Abuse Hospitals ".to_string(),
        }),
        (6223, Code::<u32> {
            code: 6223,
            parent_code: Some(622),
            description: "Specialty (except Psychiatric and Substance Abuse) Hospitals".to_string(),
        }),
        (62231, Code::<u32> {
            code: 62231,
            parent_code: Some(6223),
            description: "Specialty (except Psychiatric and Substance Abuse) Hospitals".to_string(),
        }),
        (622310, Code::<u32> {
            code: 622310,
            parent_code: Some(62231),
            description: "Specialty (except Psychiatric and Substance Abuse) Hospitals ".to_string(),
        }),
        (623, Code::<u32> {
            code: 623,
            parent_code: Some(62),
            description: "Nursing and Residential Care Facilities".to_string(),
        }),
        (6231, Code::<u32> {
            code: 6231,
            parent_code: Some(623),
            description: "Nursing Care Facilities (Skilled Nursing Facilities)".to_string(),
        }),
        (62311, Code::<u32> {
            code: 62311,
            parent_code: Some(6231),
            description: "Nursing Care Facilities (Skilled Nursing Facilities)".to_string(),
        }),
        (623110, Code::<u32> {
            code: 623110,
            parent_code: Some(62311),
            description: "Nursing Care Facilities (Skilled Nursing Facilities) ".to_string(),
        }),
        (6232, Code::<u32> {
            code: 6232,
            parent_code: Some(623),
            description: "Residential Intellectual and Developmental Disability, Mental Health, and Substance Abuse Facilities".to_string(),
        }),
        (62321, Code::<u32> {
            code: 62321,
            parent_code: Some(6232),
            description: "Residential Intellectual and Developmental Disability Facilities".to_string(),
        }),
        (623210, Code::<u32> {
            code: 623210,
            parent_code: Some(62321),
            description: "Residential Intellectual and Developmental Disability Facilities ".to_string(),
        }),
        (62322, Code::<u32> {
            code: 62322,
            parent_code: Some(6232),
            description: "Residential Mental Health and Substance Abuse Facilities".to_string(),
        }),
        (623220, Code::<u32> {
            code: 623220,
            parent_code: Some(62322),
            description: "Residential Mental Health and Substance Abuse Facilities ".to_string(),
        }),
        (6233, Code::<u32> {
            code: 6233,
            parent_code: Some(623),
            description: "Continuing Care Retirement Communities and Assisted Living Facilities for the Elderly".to_string(),
        }),
        (62331, Code::<u32> {
            code: 62331,
            parent_code: Some(6233),
            description: "Continuing Care Retirement Communities and Assisted Living Facilities for the Elderly".to_string(),
        }),
        (623311, Code::<u32> {
            code: 623311,
            parent_code: Some(62331),
            description: "Continuing Care Retirement Communities ".to_string(),
        }),
        (623312, Code::<u32> {
            code: 623312,
            parent_code: Some(62331),
            description: "Assisted Living Facilities for the Elderly ".to_string(),
        }),
        (6239, Code::<u32> {
            code: 6239,
            parent_code: Some(623),
            description: "Other Residential Care Facilities".to_string(),
        }),
        (62399, Code::<u32> {
            code: 62399,
            parent_code: Some(6239),
            description: "Other Residential Care Facilities".to_string(),
        }),
        (623990, Code::<u32> {
            code: 623990,
            parent_code: Some(62399),
            description: "Other Residential Care Facilities ".to_string(),
        }),
        (624, Code::<u32> {
            code: 624,
            parent_code: Some(62),
            description: "Social Assistance".to_string(),
        }),
        (6241, Code::<u32> {
            code: 6241,
            parent_code: Some(624),
            description: "Individual and Family Services".to_string(),
        }),
        (62411, Code::<u32> {
            code: 62411,
            parent_code: Some(6241),
            description: "Child and Youth Services".to_string(),
        }),
        (624110, Code::<u32> {
            code: 624110,
            parent_code: Some(62411),
            description: "Child and Youth Services ".to_string(),
        }),
        (62412, Code::<u32> {
            code: 62412,
            parent_code: Some(6241),
            description: "Services for the Elderly and Persons with Disabilities".to_string(),
        }),
        (624120, Code::<u32> {
            code: 624120,
            parent_code: Some(62412),
            description: "Services for the Elderly and Persons with Disabilities ".to_string(),
        }),
        (62419, Code::<u32> {
            code: 62419,
            parent_code: Some(6241),
            description: "Other Individual and Family Services".to_string(),
        }),
        (624190, Code::<u32> {
            code: 624190,
            parent_code: Some(62419),
            description: "Other Individual and Family Services ".to_string(),
        }),
        (6242, Code::<u32> {
            code: 6242,
            parent_code: Some(624),
            description: "Community Food and Housing, and Emergency and Other Relief Services".to_string(),
        }),
        (62421, Code::<u32> {
            code: 62421,
            parent_code: Some(6242),
            description: "Community Food Services".to_string(),
        }),
        (624210, Code::<u32> {
            code: 624210,
            parent_code: Some(62421),
            description: "Community Food Services ".to_string(),
        }),
        (62422, Code::<u32> {
            code: 62422,
            parent_code: Some(6242),
            description: "Community Housing Services".to_string(),
        }),
        (624221, Code::<u32> {
            code: 624221,
            parent_code: Some(62422),
            description: "Temporary Shelters ".to_string(),
        }),
        (624229, Code::<u32> {
            code: 624229,
            parent_code: Some(62422),
            description: "Other Community Housing Services ".to_string(),
        }),
        (62423, Code::<u32> {
            code: 62423,
            parent_code: Some(6242),
            description: "Emergency and Other Relief Services".to_string(),
        }),
        (624230, Code::<u32> {
            code: 624230,
            parent_code: Some(62423),
            description: "Emergency and Other Relief Services ".to_string(),
        }),
        (6243, Code::<u32> {
            code: 6243,
            parent_code: Some(624),
            description: "Vocational Rehabilitation Services".to_string(),
        }),
        (62431, Code::<u32> {
            code: 62431,
            parent_code: Some(6243),
            description: "Vocational Rehabilitation Services".to_string(),
        }),
        (624310, Code::<u32> {
            code: 624310,
            parent_code: Some(62431),
            description: "Vocational Rehabilitation Services ".to_string(),
        }),
        (6244, Code::<u32> {
            code: 6244,
            parent_code: Some(624),
            description: "Child Day Care Services".to_string(),
        }),
        (62441, Code::<u32> {
            code: 62441,
            parent_code: Some(6244),
            description: "Child Day Care Services".to_string(),
        }),
        (624410, Code::<u32> {
            code: 624410,
            parent_code: Some(62441),
            description: "Child Day Care Services ".to_string(),
        }),
        (71, Code::<u32> {
            code: 71,
            parent_code: None,
            description: "Arts, Entertainment, and Recreation".to_string(),
        }),
        (711, Code::<u32> {
            code: 711,
            parent_code: Some(71),
            description: "Performing Arts, Spectator Sports, and Related Industries".to_string(),
        }),
        (7111, Code::<u32> {
            code: 7111,
            parent_code: Some(711),
            description: "Performing Arts Companies".to_string(),
        }),
        (71111, Code::<u32> {
            code: 71111,
            parent_code: Some(7111),
            description: "Theater Companies and Dinner Theaters".to_string(),
        }),
        (711110, Code::<u32> {
            code: 711110,
            parent_code: Some(71111),
            description: "Theater Companies and Dinner Theaters ".to_string(),
        }),
        (71112, Code::<u32> {
            code: 71112,
            parent_code: Some(7111),
            description: "Dance Companies".to_string(),
        }),
        (711120, Code::<u32> {
            code: 711120,
            parent_code: Some(71112),
            description: "Dance Companies ".to_string(),
        }),
        (71113, Code::<u32> {
            code: 71113,
            parent_code: Some(7111),
            description: "Musical Groups and Artists".to_string(),
        }),
        (711130, Code::<u32> {
            code: 711130,
            parent_code: Some(71113),
            description: "Musical Groups and Artists ".to_string(),
        }),
        (71119, Code::<u32> {
            code: 71119,
            parent_code: Some(7111),
            description: "Other Performing Arts Companies".to_string(),
        }),
        (711190, Code::<u32> {
            code: 711190,
            parent_code: Some(71119),
            description: "Other Performing Arts Companies ".to_string(),
        }),
        (7112, Code::<u32> {
            code: 7112,
            parent_code: Some(711),
            description: "Spectator Sports".to_string(),
        }),
        (71121, Code::<u32> {
            code: 71121,
            parent_code: Some(7112),
            description: "Spectator Sports".to_string(),
        }),
        (711211, Code::<u32> {
            code: 711211,
            parent_code: Some(71121),
            description: "Sports Teams and Clubs ".to_string(),
        }),
        (711212, Code::<u32> {
            code: 711212,
            parent_code: Some(71121),
            description: "Racetracks ".to_string(),
        }),
        (711219, Code::<u32> {
            code: 711219,
            parent_code: Some(71121),
            description: "Other Spectator Sports ".to_string(),
        }),
        (7113, Code::<u32> {
            code: 7113,
            parent_code: Some(711),
            description: "Promoters of Performing Arts, Sports, and Similar Events".to_string(),
        }),
        (71131, Code::<u32> {
            code: 71131,
            parent_code: Some(7113),
            description: "Promoters of Performing Arts, Sports, and Similar Events with Facilities".to_string(),
        }),
        (711310, Code::<u32> {
            code: 711310,
            parent_code: Some(71131),
            description: "Promoters of Performing Arts, Sports, and Similar Events with Facilities ".to_string(),
        }),
        (71132, Code::<u32> {
            code: 71132,
            parent_code: Some(7113),
            description: "Promoters of Performing Arts, Sports, and Similar Events without Facilities".to_string(),
        }),
        (711320, Code::<u32> {
            code: 711320,
            parent_code: Some(71132),
            description: "Promoters of Performing Arts, Sports, and Similar Events without Facilities ".to_string(),
        }),
        (7114, Code::<u32> {
            code: 7114,
            parent_code: Some(711),
            description: "Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures".to_string(),
        }),
        (71141, Code::<u32> {
            code: 71141,
            parent_code: Some(7114),
            description: "Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures".to_string(),
        }),
        (711410, Code::<u32> {
            code: 711410,
            parent_code: Some(71141),
            description: "Agents and Managers for Artists, Athletes, Entertainers, and Other Public Figures".to_string(),
        }),
        (7115, Code::<u32> {
            code: 7115,
            parent_code: Some(711),
            description: "Independent Artists, Writers, and Performers".to_string(),
        }),
        (71151, Code::<u32> {
            code: 71151,
            parent_code: Some(7115),
            description: "Independent Artists, Writers, and Performers".to_string(),
        }),
        (711510, Code::<u32> {
            code: 711510,
            parent_code: Some(71151),
            description: "Independent Artists, Writers, and Performers ".to_string(),
        }),
        (712, Code::<u32> {
            code: 712,
            parent_code: Some(71),
            description: "Museums, Historical Sites, and Similar Institutions".to_string(),
        }),
        (7121, Code::<u32> {
            code: 7121,
            parent_code: Some(712),
            description: "Museums, Historical Sites, and Similar Institutions".to_string(),
        }),
        (71211, Code::<u32> {
            code: 71211,
            parent_code: Some(7121),
            description: "Museums".to_string(),
        }),
        (712110, Code::<u32> {
            code: 712110,
            parent_code: Some(71211),
            description: "Museums ".to_string(),
        }),
        (71212, Code::<u32> {
            code: 71212,
            parent_code: Some(7121),
            description: "Historical Sites".to_string(),
        }),
        (712120, Code::<u32> {
            code: 712120,
            parent_code: Some(71212),
            description: "Historical Sites".to_string(),
        }),
        (71213, Code::<u32> {
            code: 71213,
            parent_code: Some(7121),
            description: "Zoos and Botanical Gardens".to_string(),
        }),
        (712130, Code::<u32> {
            code: 712130,
            parent_code: Some(71213),
            description: "Zoos and Botanical Gardens ".to_string(),
        }),
        (71219, Code::<u32> {
            code: 71219,
            parent_code: Some(7121),
            description: "Nature Parks and Other Similar Institutions".to_string(),
        }),
        (712190, Code::<u32> {
            code: 712190,
            parent_code: Some(71219),
            description: "Nature Parks and Other Similar Institutions".to_string(),
        }),
        (713, Code::<u32> {
            code: 713,
            parent_code: Some(71),
            description: "Amusement, Gambling, and Recreation Industries".to_string(),
        }),
        (7131, Code::<u32> {
            code: 7131,
            parent_code: Some(713),
            description: "Amusement Parks and Arcades".to_string(),
        }),
        (71311, Code::<u32> {
            code: 71311,
            parent_code: Some(7131),
            description: "Amusement and Theme Parks".to_string(),
        }),
        (713110, Code::<u32> {
            code: 713110,
            parent_code: Some(71311),
            description: "Amusement and Theme Parks ".to_string(),
        }),
        (71312, Code::<u32> {
            code: 71312,
            parent_code: Some(7131),
            description: "Amusement Arcades".to_string(),
        }),
        (713120, Code::<u32> {
            code: 713120,
            parent_code: Some(71312),
            description: "Amusement Arcades".to_string(),
        }),
        (7132, Code::<u32> {
            code: 7132,
            parent_code: Some(713),
            description: "Gambling Industries".to_string(),
        }),
        (71321, Code::<u32> {
            code: 71321,
            parent_code: Some(7132),
            description: "Casinos (except Casino Hotels)".to_string(),
        }),
        (713210, Code::<u32> {
            code: 713210,
            parent_code: Some(71321),
            description: "Casinos (except Casino Hotels)".to_string(),
        }),
        (71329, Code::<u32> {
            code: 71329,
            parent_code: Some(7132),
            description: "Other Gambling Industries".to_string(),
        }),
        (713290, Code::<u32> {
            code: 713290,
            parent_code: Some(71329),
            description: "Other Gambling Industries ".to_string(),
        }),
        (7139, Code::<u32> {
            code: 7139,
            parent_code: Some(713),
            description: "Other Amusement and Recreation Industries".to_string(),
        }),
        (71391, Code::<u32> {
            code: 71391,
            parent_code: Some(7139),
            description: "Golf Courses and Country Clubs".to_string(),
        }),
        (713910, Code::<u32> {
            code: 713910,
            parent_code: Some(71391),
            description: "Golf Courses and Country Clubs".to_string(),
        }),
        (71392, Code::<u32> {
            code: 71392,
            parent_code: Some(7139),
            description: "Skiing Facilities".to_string(),
        }),
        (713920, Code::<u32> {
            code: 713920,
            parent_code: Some(71392),
            description: "Skiing Facilities".to_string(),
        }),
        (71393, Code::<u32> {
            code: 71393,
            parent_code: Some(7139),
            description: "Marinas".to_string(),
        }),
        (713930, Code::<u32> {
            code: 713930,
            parent_code: Some(71393),
            description: "Marinas".to_string(),
        }),
        (71394, Code::<u32> {
            code: 71394,
            parent_code: Some(7139),
            description: "Fitness and Recreational Sports Centers".to_string(),
        }),
        (713940, Code::<u32> {
            code: 713940,
            parent_code: Some(71394),
            description: "Fitness and Recreational Sports Centers ".to_string(),
        }),
        (71395, Code::<u32> {
            code: 71395,
            parent_code: Some(7139),
            description: "Bowling Centers".to_string(),
        }),
        (713950, Code::<u32> {
            code: 713950,
            parent_code: Some(71395),
            description: "Bowling Centers".to_string(),
        }),
        (71399, Code::<u32> {
            code: 71399,
            parent_code: Some(7139),
            description: "All Other Amusement and Recreation Industries".to_string(),
        }),
        (713990, Code::<u32> {
            code: 713990,
            parent_code: Some(71399),
            description: "All Other Amusement and Recreation Industries ".to_string(),
        }),
        (72, Code::<u32> {
            code: 72,
            parent_code: None,
            description: "Accommodation and Food Services".to_string(),
        }),
        (721, Code::<u32> {
            code: 721,
            parent_code: Some(72),
            description: "Accommodation".to_string(),
        }),
        (7211, Code::<u32> {
            code: 7211,
            parent_code: Some(721),
            description: "Traveler Accommodation".to_string(),
        }),
        (72111, Code::<u32> {
            code: 72111,
            parent_code: Some(7211),
            description: "Hotels (except Casino Hotels) and Motels".to_string(),
        }),
        (721110, Code::<u32> {
            code: 721110,
            parent_code: Some(72111),
            description: "Hotels (except Casino Hotels) and Motels ".to_string(),
        }),
        (72112, Code::<u32> {
            code: 72112,
            parent_code: Some(7211),
            description: "Casino Hotels".to_string(),
        }),
        (721120, Code::<u32> {
            code: 721120,
            parent_code: Some(72112),
            description: "Casino Hotels".to_string(),
        }),
        (72119, Code::<u32> {
            code: 72119,
            parent_code: Some(7211),
            description: "Other Traveler Accommodation".to_string(),
        }),
        (721191, Code::<u32> {
            code: 721191,
            parent_code: Some(72119),
            description: "Bed-and-Breakfast Inns ".to_string(),
        }),
        (721199, Code::<u32> {
            code: 721199,
            parent_code: Some(72119),
            description: "All Other Traveler Accommodation ".to_string(),
        }),
        (7212, Code::<u32> {
            code: 7212,
            parent_code: Some(721),
            description: "RV (Recreational Vehicle) Parks and Recreational Camps".to_string(),
        }),
        (72121, Code::<u32> {
            code: 72121,
            parent_code: Some(7212),
            description: "RV (Recreational Vehicle) Parks and Recreational Camps".to_string(),
        }),
        (721211, Code::<u32> {
            code: 721211,
            parent_code: Some(72121),
            description: "RV (Recreational Vehicle) Parks and Campgrounds ".to_string(),
        }),
        (721214, Code::<u32> {
            code: 721214,
            parent_code: Some(72121),
            description: "Recreational and Vacation Camps (except Campgrounds) ".to_string(),
        }),
        (7213, Code::<u32> {
            code: 7213,
            parent_code: Some(721),
            description: "Rooming and Boarding Houses, Dormitories, and Workers' Camps".to_string(),
        }),
        (72131, Code::<u32> {
            code: 72131,
            parent_code: Some(7213),
            description: "Rooming and Boarding Houses, Dormitories, and Workers' Camps".to_string(),
        }),
        (721310, Code::<u32> {
            code: 721310,
            parent_code: Some(72131),
            description: "Rooming and Boarding Houses, Dormitories, and Workers' Camps ".to_string(),
        }),
        (722, Code::<u32> {
            code: 722,
            parent_code: Some(72),
            description: "Food Services and Drinking Places".to_string(),
        }),
        (7223, Code::<u32> {
            code: 7223,
            parent_code: Some(722),
            description: "Special Food Services".to_string(),
        }),
        (72231, Code::<u32> {
            code: 72231,
            parent_code: Some(7223),
            description: "Food Service Contractors".to_string(),
        }),
        (722310, Code::<u32> {
            code: 722310,
            parent_code: Some(72231),
            description: "Food Service Contractors".to_string(),
        }),
        (72232, Code::<u32> {
            code: 72232,
            parent_code: Some(7223),
            description: "Caterers".to_string(),
        }),
        (722320, Code::<u32> {
            code: 722320,
            parent_code: Some(72232),
            description: "Caterers".to_string(),
        }),
        (72233, Code::<u32> {
            code: 72233,
            parent_code: Some(7223),
            description: "Mobile Food Services".to_string(),
        }),
        (722330, Code::<u32> {
            code: 722330,
            parent_code: Some(72233),
            description: "Mobile Food Services".to_string(),
        }),
        (7224, Code::<u32> {
            code: 7224,
            parent_code: Some(722),
            description: "Drinking Places (Alcoholic Beverages)".to_string(),
        }),
        (72241, Code::<u32> {
            code: 72241,
            parent_code: Some(7224),
            description: "Drinking Places (Alcoholic Beverages)".to_string(),
        }),
        (722410, Code::<u32> {
            code: 722410,
            parent_code: Some(72241),
            description: "Drinking Places (Alcoholic Beverages) ".to_string(),
        }),
        (7225, Code::<u32> {
            code: 7225,
            parent_code: Some(722),
            description: "Restaurants and Other Eating Places".to_string(),
        }),
        (72251, Code::<u32> {
            code: 72251,
            parent_code: Some(7225),
            description: "Restaurants and Other Eating Places".to_string(),
        }),
        (722511, Code::<u32> {
            code: 722511,
            parent_code: Some(72251),
            description: "Full-Service Restaurants ".to_string(),
        }),
        (722513, Code::<u32> {
            code: 722513,
            parent_code: Some(72251),
            description: "Limited-Service Restaurants ".to_string(),
        }),
        (722514, Code::<u32> {
            code: 722514,
            parent_code: Some(72251),
            description: "Cafeterias, Grill Buffets, and Buffets ".to_string(),
        }),
        (722515, Code::<u32> {
            code: 722515,
            parent_code: Some(72251),
            description: "Snack and Nonalcoholic Beverage Bars ".to_string(),
        }),
        (81, Code::<u32> {
            code: 81,
            parent_code: None,
            description: "Other Services (except Public Administration)".to_string(),
        }),
        (811, Code::<u32> {
            code: 811,
            parent_code: Some(81),
            description: "Repair and Maintenance".to_string(),
        }),
        (8111, Code::<u32> {
            code: 8111,
            parent_code: Some(811),
            description: "Automotive Repair and Maintenance".to_string(),
        }),
        (81111, Code::<u32> {
            code: 81111,
            parent_code: Some(8111),
            description: "Automotive Mechanical and Electrical Repair and Maintenance".to_string(),
        }),
        (811111, Code::<u32> {
            code: 811111,
            parent_code: Some(81111),
            description: "General Automotive Repair ".to_string(),
        }),
        (811112, Code::<u32> {
            code: 811112,
            parent_code: Some(81111),
            description: "Automotive Exhaust System Repair ".to_string(),
        }),
        (811113, Code::<u32> {
            code: 811113,
            parent_code: Some(81111),
            description: "Automotive Transmission Repair ".to_string(),
        }),
        (811118, Code::<u32> {
            code: 811118,
            parent_code: Some(81111),
            description: "Other Automotive Mechanical and Electrical Repair and Maintenance ".to_string(),
        }),
        (81112, Code::<u32> {
            code: 81112,
            parent_code: Some(8111),
            description: "Automotive Body, Paint, Interior, and Glass Repair".to_string(),
        }),
        (811121, Code::<u32> {
            code: 811121,
            parent_code: Some(81112),
            description: "Automotive Body, Paint, and Interior Repair and Maintenance ".to_string(),
        }),
        (811122, Code::<u32> {
            code: 811122,
            parent_code: Some(81112),
            description: "Automotive Glass Replacement Shops ".to_string(),
        }),
        (81119, Code::<u32> {
            code: 81119,
            parent_code: Some(8111),
            description: "Other Automotive Repair and Maintenance".to_string(),
        }),
        (811191, Code::<u32> {
            code: 811191,
            parent_code: Some(81119),
            description: "Automotive Oil Change and Lubrication Shops ".to_string(),
        }),
        (811192, Code::<u32> {
            code: 811192,
            parent_code: Some(81119),
            description: "Car Washes ".to_string(),
        }),
        (811198, Code::<u32> {
            code: 811198,
            parent_code: Some(81119),
            description: "All Other Automotive Repair and Maintenance ".to_string(),
        }),
        (8112, Code::<u32> {
            code: 8112,
            parent_code: Some(811),
            description: "Electronic and Precision Equipment Repair and Maintenance".to_string(),
        }),
        (81121, Code::<u32> {
            code: 81121,
            parent_code: Some(8112),
            description: "Electronic and Precision Equipment Repair and Maintenance".to_string(),
        }),
        (811211, Code::<u32> {
            code: 811211,
            parent_code: Some(81121),
            description: "Consumer Electronics Repair and Maintenance ".to_string(),
        }),
        (811212, Code::<u32> {
            code: 811212,
            parent_code: Some(81121),
            description: "Computer and Office Machine Repair and Maintenance ".to_string(),
        }),
        (811213, Code::<u32> {
            code: 811213,
            parent_code: Some(81121),
            description: "Communication Equipment Repair and Maintenance ".to_string(),
        }),
        (811219, Code::<u32> {
            code: 811219,
            parent_code: Some(81121),
            description: "Other Electronic and Precision Equipment Repair and Maintenance ".to_string(),
        }),
        (8113, Code::<u32> {
            code: 8113,
            parent_code: Some(811),
            description: "Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance".to_string(),
        }),
        (81131, Code::<u32> {
            code: 81131,
            parent_code: Some(8113),
            description: "Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance".to_string(),
        }),
        (811310, Code::<u32> {
            code: 811310,
            parent_code: Some(81131),
            description: "Commercial and Industrial Machinery and Equipment (except Automotive and Electronic) Repair and Maintenance ".to_string(),
        }),
        (8114, Code::<u32> {
            code: 8114,
            parent_code: Some(811),
            description: "Personal and Household Goods Repair and Maintenance".to_string(),
        }),
        (81141, Code::<u32> {
            code: 81141,
            parent_code: Some(8114),
            description: "Home and Garden Equipment and Appliance Repair and Maintenance".to_string(),
        }),
        (811411, Code::<u32> {
            code: 811411,
            parent_code: Some(81141),
            description: "Home and Garden Equipment Repair and Maintenance ".to_string(),
        }),
        (811412, Code::<u32> {
            code: 811412,
            parent_code: Some(81141),
            description: "Appliance Repair and Maintenance ".to_string(),
        }),
        (81142, Code::<u32> {
            code: 81142,
            parent_code: Some(8114),
            description: "Reupholstery and Furniture Repair".to_string(),
        }),
        (811420, Code::<u32> {
            code: 811420,
            parent_code: Some(81142),
            description: "Reupholstery and Furniture Repair".to_string(),
        }),
        (81143, Code::<u32> {
            code: 81143,
            parent_code: Some(8114),
            description: "Footwear and Leather Goods Repair".to_string(),
        }),
        (811430, Code::<u32> {
            code: 811430,
            parent_code: Some(81143),
            description: "Footwear and Leather Goods Repair".to_string(),
        }),
        (81149, Code::<u32> {
            code: 81149,
            parent_code: Some(8114),
            description: "Other Personal and Household Goods Repair and Maintenance".to_string(),
        }),
        (811490, Code::<u32> {
            code: 811490,
            parent_code: Some(81149),
            description: "Other Personal and Household Goods Repair and Maintenance ".to_string(),
        }),
        (812, Code::<u32> {
            code: 812,
            parent_code: Some(81),
            description: "Personal and Laundry Services".to_string(),
        }),
        (8121, Code::<u32> {
            code: 8121,
            parent_code: Some(812),
            description: "Personal Care Services ".to_string(),
        }),
        (81211, Code::<u32> {
            code: 81211,
            parent_code: Some(8121),
            description: "Hair, Nail, and Skin Care Services ".to_string(),
        }),
        (812111, Code::<u32> {
            code: 812111,
            parent_code: Some(81211),
            description: "Barber Shops ".to_string(),
        }),
        (812112, Code::<u32> {
            code: 812112,
            parent_code: Some(81211),
            description: "Beauty Salons ".to_string(),
        }),
        (812113, Code::<u32> {
            code: 812113,
            parent_code: Some(81211),
            description: "Nail Salons ".to_string(),
        }),
        (81219, Code::<u32> {
            code: 81219,
            parent_code: Some(8121),
            description: "Other Personal Care Services ".to_string(),
        }),
        (812191, Code::<u32> {
            code: 812191,
            parent_code: Some(81219),
            description: "Diet and Weight Reducing Centers ".to_string(),
        }),
        (812199, Code::<u32> {
            code: 812199,
            parent_code: Some(81219),
            description: "Other Personal Care Services ".to_string(),
        }),
        (8122, Code::<u32> {
            code: 8122,
            parent_code: Some(812),
            description: "Death Care Services ".to_string(),
        }),
        (81221, Code::<u32> {
            code: 81221,
            parent_code: Some(8122),
            description: "Funeral Homes and Funeral Services ".to_string(),
        }),
        (812210, Code::<u32> {
            code: 812210,
            parent_code: Some(81221),
            description: "Funeral Homes and Funeral Services ".to_string(),
        }),
        (81222, Code::<u32> {
            code: 81222,
            parent_code: Some(8122),
            description: "Cemeteries and Crematories ".to_string(),
        }),
        (812220, Code::<u32> {
            code: 812220,
            parent_code: Some(81222),
            description: "Cemeteries and Crematories ".to_string(),
        }),
        (8123, Code::<u32> {
            code: 8123,
            parent_code: Some(812),
            description: "Drycleaning and Laundry Services ".to_string(),
        }),
        (81231, Code::<u32> {
            code: 81231,
            parent_code: Some(8123),
            description: "Coin-Operated Laundries and Drycleaners ".to_string(),
        }),
        (812310, Code::<u32> {
            code: 812310,
            parent_code: Some(81231),
            description: "Coin-Operated Laundries and Drycleaners ".to_string(),
        }),
        (81232, Code::<u32> {
            code: 81232,
            parent_code: Some(8123),
            description: "Drycleaning and Laundry Services (except Coin-Operated) ".to_string(),
        }),
        (812320, Code::<u32> {
            code: 812320,
            parent_code: Some(81232),
            description: "Drycleaning and Laundry Services (except Coin-Operated) ".to_string(),
        }),
        (81233, Code::<u32> {
            code: 81233,
            parent_code: Some(8123),
            description: "Linen and Uniform Supply ".to_string(),
        }),
        (812331, Code::<u32> {
            code: 812331,
            parent_code: Some(81233),
            description: "Linen Supply ".to_string(),
        }),
        (812332, Code::<u32> {
            code: 812332,
            parent_code: Some(81233),
            description: "Industrial Launderers ".to_string(),
        }),
        (8129, Code::<u32> {
            code: 8129,
            parent_code: Some(812),
            description: "Other Personal Services ".to_string(),
        }),
        (81291, Code::<u32> {
            code: 81291,
            parent_code: Some(8129),
            description: "Pet Care (except Veterinary) Services ".to_string(),
        }),
        (812910, Code::<u32> {
            code: 812910,
            parent_code: Some(81291),
            description: "Pet Care (except Veterinary) Services ".to_string(),
        }),
        (81292, Code::<u32> {
            code: 81292,
            parent_code: Some(8129),
            description: "Photofinishing ".to_string(),
        }),
        (812921, Code::<u32> {
            code: 812921,
            parent_code: Some(81292),
            description: "Photofinishing Laboratories (except One-Hour) ".to_string(),
        }),
        (812922, Code::<u32> {
            code: 812922,
            parent_code: Some(81292),
            description: "One-Hour Photofinishing ".to_string(),
        }),
        (81293, Code::<u32> {
            code: 81293,
            parent_code: Some(8129),
            description: "Parking Lots and Garages ".to_string(),
        }),
        (812930, Code::<u32> {
            code: 812930,
            parent_code: Some(81293),
            description: "Parking Lots and Garages ".to_string(),
        }),
        (81299, Code::<u32> {
            code: 81299,
            parent_code: Some(8129),
            description: "All Other Personal Services ".to_string(),
        }),
        (812990, Code::<u32> {
            code: 812990,
            parent_code: Some(81299),
            description: "All Other Personal Services ".to_string(),
        }),
        (813, Code::<u32> {
            code: 813,
            parent_code: Some(81),
            description: "Religious, Grantmaking, Civic, Professional, and Similar Organizations".to_string(),
        }),
        (8131, Code::<u32> {
            code: 8131,
            parent_code: Some(813),
            description: "Religious Organizations ".to_string(),
        }),
        (81311, Code::<u32> {
            code: 81311,
            parent_code: Some(8131),
            description: "Religious Organizations ".to_string(),
        }),
        (813110, Code::<u32> {
            code: 813110,
            parent_code: Some(81311),
            description: "Religious Organizations ".to_string(),
        }),
        (8132, Code::<u32> {
            code: 8132,
            parent_code: Some(813),
            description: "Grantmaking and Giving Services ".to_string(),
        }),
        (81321, Code::<u32> {
            code: 81321,
            parent_code: Some(8132),
            description: "Grantmaking and Giving Services ".to_string(),
        }),
        (813211, Code::<u32> {
            code: 813211,
            parent_code: Some(81321),
            description: "Grantmaking Foundations ".to_string(),
        }),
        (813212, Code::<u32> {
            code: 813212,
            parent_code: Some(81321),
            description: "Voluntary Health Organizations ".to_string(),
        }),
        (813219, Code::<u32> {
            code: 813219,
            parent_code: Some(81321),
            description: "Other Grantmaking and Giving Services ".to_string(),
        }),
        (8133, Code::<u32> {
            code: 8133,
            parent_code: Some(813),
            description: "Social Advocacy Organizations ".to_string(),
        }),
        (81331, Code::<u32> {
            code: 81331,
            parent_code: Some(8133),
            description: "Social Advocacy Organizations ".to_string(),
        }),
        (813311, Code::<u32> {
            code: 813311,
            parent_code: Some(81331),
            description: "Human Rights Organizations ".to_string(),
        }),
        (813312, Code::<u32> {
            code: 813312,
            parent_code: Some(81331),
            description: "Environment, Conservation and Wildlife Organizations ".to_string(),
        }),
        (813319, Code::<u32> {
            code: 813319,
            parent_code: Some(81331),
            description: "Other Social Advocacy Organizations ".to_string(),
        }),
        (8134, Code::<u32> {
            code: 8134,
            parent_code: Some(813),
            description: "Civic and Social Organizations ".to_string(),
        }),
        (81341, Code::<u32> {
            code: 81341,
            parent_code: Some(8134),
            description: "Civic and Social Organizations ".to_string(),
        }),
        (813410, Code::<u32> {
            code: 813410,
            parent_code: Some(81341),
            description: "Civic and Social Organizations ".to_string(),
        }),
        (8139, Code::<u32> {
            code: 8139,
            parent_code: Some(813),
            description: "Business, Professional, Labor, Political, and Similar Organizations ".to_string(),
        }),
        (81391, Code::<u32> {
            code: 81391,
            parent_code: Some(8139),
            description: "Business Associations ".to_string(),
        }),
        (813910, Code::<u32> {
            code: 813910,
            parent_code: Some(81391),
            description: "Business Associations ".to_string(),
        }),
        (81392, Code::<u32> {
            code: 81392,
            parent_code: Some(8139),
            description: "Professional Organizations ".to_string(),
        }),
        (813920, Code::<u32> {
            code: 813920,
            parent_code: Some(81392),
            description: "Professional Organizations ".to_string(),
        }),
        (81393, Code::<u32> {
            code: 81393,
            parent_code: Some(8139),
            description: "Labor Unions and Similar Labor Organizations ".to_string(),
        }),
        (813930, Code::<u32> {
            code: 813930,
            parent_code: Some(81393),
            description: "Labor Unions and Similar Labor Organizations ".to_string(),
        }),
        (81394, Code::<u32> {
            code: 81394,
            parent_code: Some(8139),
            description: "Political Organizations ".to_string(),
        }),
        (813940, Code::<u32> {
            code: 813940,
            parent_code: Some(81394),
            description: "Political Organizations ".to_string(),
        }),
        (81399, Code::<u32> {
            code: 81399,
            parent_code: Some(8139),
            description: "Other Similar Organizations (except Business, Professional, Labor, and Political Organizations) ".to_string(),
        }),
        (813990, Code::<u32> {
            code: 813990,
            parent_code: Some(81399),
            description: "Other Similar Organizations (except Business, Professional, Labor, and Political Organizations) ".to_string(),
        }),
        (814, Code::<u32> {
            code: 814,
            parent_code: Some(81),
            description: "Private Households".to_string(),
        }),
        (8141, Code::<u32> {
            code: 8141,
            parent_code: Some(814),
            description: "Private Households".to_string(),
        }),
        (81411, Code::<u32> {
            code: 81411,
            parent_code: Some(8141),
            description: "Private Households".to_string(),
        }),
        (814110, Code::<u32> {
            code: 814110,
            parent_code: Some(81411),
            description: "Private Households".to_string(),
        }),
        (92, Code::<u32> {
            code: 92,
            parent_code: None,
            description: "Public Administration".to_string(),
        }),
        (921, Code::<u32> {
            code: 921,
            parent_code: Some(92),
            description: "Executive, Legislative, and Other General Government Support ".to_string(),
        }),
        (9211, Code::<u32> {
            code: 9211,
            parent_code: Some(921),
            description: "Executive, Legislative, and Other General Government Support ".to_string(),
        }),
        (92111, Code::<u32> {
            code: 92111,
            parent_code: Some(9211),
            description: "Executive Offices ".to_string(),
        }),
        (921110, Code::<u32> {
            code: 921110,
            parent_code: Some(92111),
            description: "Executive Offices ".to_string(),
        }),
        (92112, Code::<u32> {
            code: 92112,
            parent_code: Some(9211),
            description: "Legislative Bodies ".to_string(),
        }),
        (921120, Code::<u32> {
            code: 921120,
            parent_code: Some(92112),
            description: "Legislative Bodies ".to_string(),
        }),
        (92113, Code::<u32> {
            code: 92113,
            parent_code: Some(9211),
            description: "Public Finance Activities ".to_string(),
        }),
        (921130, Code::<u32> {
            code: 921130,
            parent_code: Some(92113),
            description: "Public Finance Activities ".to_string(),
        }),
        (92114, Code::<u32> {
            code: 92114,
            parent_code: Some(9211),
            description: "Executive and Legislative Offices, Combined ".to_string(),
        }),
        (921140, Code::<u32> {
            code: 921140,
            parent_code: Some(92114),
            description: "Executive and Legislative Offices, Combined ".to_string(),
        }),
        (92115, Code::<u32> {
            code: 92115,
            parent_code: Some(9211),
            description: "American Indian and Alaska Native Tribal Governments ".to_string(),
        }),
        (921150, Code::<u32> {
            code: 921150,
            parent_code: Some(92115),
            description: "American Indian and Alaska Native Tribal Governments ".to_string(),
        }),
        (92119, Code::<u32> {
            code: 92119,
            parent_code: Some(9211),
            description: "Other General Government Support ".to_string(),
        }),
        (921190, Code::<u32> {
            code: 921190,
            parent_code: Some(92119),
            description: "Other General Government Support ".to_string(),
        }),
        (922, Code::<u32> {
            code: 922,
            parent_code: Some(92),
            description: "Justice, Public Order, and Safety Activities ".to_string(),
        }),
        (9221, Code::<u32> {
            code: 9221,
            parent_code: Some(922),
            description: "Justice, Public Order, and Safety Activities ".to_string(),
        }),
        (92211, Code::<u32> {
            code: 92211,
            parent_code: Some(9221),
            description: "Courts ".to_string(),
        }),
        (922110, Code::<u32> {
            code: 922110,
            parent_code: Some(92211),
            description: "Courts ".to_string(),
        }),
        (92212, Code::<u32> {
            code: 92212,
            parent_code: Some(9221),
            description: "Police Protection ".to_string(),
        }),
        (922120, Code::<u32> {
            code: 922120,
            parent_code: Some(92212),
            description: "Police Protection ".to_string(),
        }),
        (92213, Code::<u32> {
            code: 92213,
            parent_code: Some(9221),
            description: "Legal Counsel and Prosecution ".to_string(),
        }),
        (922130, Code::<u32> {
            code: 922130,
            parent_code: Some(92213),
            description: "Legal Counsel and Prosecution ".to_string(),
        }),
        (92214, Code::<u32> {
            code: 92214,
            parent_code: Some(9221),
            description: "Correctional Institutions ".to_string(),
        }),
        (922140, Code::<u32> {
            code: 922140,
            parent_code: Some(92214),
            description: "Correctional Institutions ".to_string(),
        }),
        (92215, Code::<u32> {
            code: 92215,
            parent_code: Some(9221),
            description: "Parole Offices and Probation Offices ".to_string(),
        }),
        (922150, Code::<u32> {
            code: 922150,
            parent_code: Some(92215),
            description: "Parole Offices and Probation Offices ".to_string(),
        }),
        (92216, Code::<u32> {
            code: 92216,
            parent_code: Some(9221),
            description: "Fire Protection ".to_string(),
        }),
        (922160, Code::<u32> {
            code: 922160,
            parent_code: Some(92216),
            description: "Fire Protection ".to_string(),
        }),
        (92219, Code::<u32> {
            code: 92219,
            parent_code: Some(9221),
            description: "Other Justice, Public Order, and Safety Activities ".to_string(),
        }),
        (922190, Code::<u32> {
            code: 922190,
            parent_code: Some(92219),
            description: "Other Justice, Public Order, and Safety Activities ".to_string(),
        }),
        (923, Code::<u32> {
            code: 923,
            parent_code: Some(92),
            description: "Administration of Human Resource Programs ".to_string(),
        }),
        (9231, Code::<u32> {
            code: 9231,
            parent_code: Some(923),
            description: "Administration of Human Resource Programs ".to_string(),
        }),
        (92311, Code::<u32> {
            code: 92311,
            parent_code: Some(9231),
            description: "Administration of Education Programs ".to_string(),
        }),
        (923110, Code::<u32> {
            code: 923110,
            parent_code: Some(92311),
            description: "Administration of Education Programs ".to_string(),
        }),
        (92312, Code::<u32> {
            code: 92312,
            parent_code: Some(9231),
            description: "Administration of Public Health Programs ".to_string(),
        }),
        (923120, Code::<u32> {
            code: 923120,
            parent_code: Some(92312),
            description: "Administration of Public Health Programs ".to_string(),
        }),
        (92313, Code::<u32> {
            code: 92313,
            parent_code: Some(9231),
            description: "Administration of Human Resource Programs (except Education, Public Health, and Veterans' Affairs Programs) ".to_string(),
        }),
        (923130, Code::<u32> {
            code: 923130,
            parent_code: Some(92313),
            description: "Administration of Human Resource Programs (except Education, Public Health, and Veterans' Affairs Programs) ".to_string(),
        }),
        (92314, Code::<u32> {
            code: 92314,
            parent_code: Some(9231),
            description: "Administration of Veterans' Affairs ".to_string(),
        }),
        (923140, Code::<u32> {
            code: 923140,
            parent_code: Some(92314),
            description: "Administration of Veterans' Affairs ".to_string(),
        }),
        (924, Code::<u32> {
            code: 924,
            parent_code: Some(92),
            description: "Administration of Environmental Quality Programs ".to_string(),
        }),
        (9241, Code::<u32> {
            code: 9241,
            parent_code: Some(924),
            description: "Administration of Environmental Quality Programs ".to_string(),
        }),
        (92411, Code::<u32> {
            code: 92411,
            parent_code: Some(9241),
            description: "Administration of Air and Water Resource and Solid Waste Management Programs ".to_string(),
        }),
        (924110, Code::<u32> {
            code: 924110,
            parent_code: Some(92411),
            description: "Administration of Air and Water Resource and Solid Waste Management Programs ".to_string(),
        }),
        (92412, Code::<u32> {
            code: 92412,
            parent_code: Some(9241),
            description: "Administration of Conservation Programs ".to_string(),
        }),
        (924120, Code::<u32> {
            code: 924120,
            parent_code: Some(92412),
            description: "Administration of Conservation Programs ".to_string(),
        }),
        (925, Code::<u32> {
            code: 925,
            parent_code: Some(92),
            description: "Administration of Housing Programs, Urban Planning, and Community Development ".to_string(),
        }),
        (9251, Code::<u32> {
            code: 9251,
            parent_code: Some(925),
            description: "Administration of Housing Programs, Urban Planning, and Community Development ".to_string(),
        }),
        (92511, Code::<u32> {
            code: 92511,
            parent_code: Some(9251),
            description: "Administration of Housing Programs ".to_string(),
        }),
        (925110, Code::<u32> {
            code: 925110,
            parent_code: Some(92511),
            description: "Administration of Housing Programs ".to_string(),
        }),
        (92512, Code::<u32> {
            code: 92512,
            parent_code: Some(9251),
            description: "Administration of Urban Planning and Community and Rural Development ".to_string(),
        }),
        (925120, Code::<u32> {
            code: 925120,
            parent_code: Some(92512),
            description: "Administration of Urban Planning and Community and Rural Development ".to_string(),
        }),
        (926, Code::<u32> {
            code: 926,
            parent_code: Some(92),
            description: "Administration of Economic Programs ".to_string(),
        }),
        (9261, Code::<u32> {
            code: 9261,
            parent_code: Some(926),
            description: "Administration of Economic Programs ".to_string(),
        }),
        (92611, Code::<u32> {
            code: 92611,
            parent_code: Some(9261),
            description: "Administration of General Economic Programs ".to_string(),
        }),
        (926110, Code::<u32> {
            code: 926110,
            parent_code: Some(92611),
            description: "Administration of General Economic Programs ".to_string(),
        }),
        (92612, Code::<u32> {
            code: 92612,
            parent_code: Some(9261),
            description: "Regulation and Administration of Transportation Programs ".to_string(),
        }),
        (926120, Code::<u32> {
            code: 926120,
            parent_code: Some(92612),
            description: "Regulation and Administration of Transportation Programs ".to_string(),
        }),
        (92613, Code::<u32> {
            code: 92613,
            parent_code: Some(9261),
            description: "Regulation and Administration of Communications, Electric, Gas, and Other Utilities ".to_string(),
        }),
        (926130, Code::<u32> {
            code: 926130,
            parent_code: Some(92613),
            description: "Regulation and Administration of Communications, Electric, Gas, and Other Utilities ".to_string(),
        }),
        (92614, Code::<u32> {
            code: 92614,
            parent_code: Some(9261),
            description: "Regulation of Agricultural Marketing and Commodities ".to_string(),
        }),
        (926140, Code::<u32> {
            code: 926140,
            parent_code: Some(92614),
            description: "Regulation of Agricultural Marketing and Commodities ".to_string(),
        }),
        (92615, Code::<u32> {
            code: 92615,
            parent_code: Some(9261),
            description: "Regulation, Licensing, and Inspection of Miscellaneous Commercial Sectors ".to_string(),
        }),
        (926150, Code::<u32> {
            code: 926150,
            parent_code: Some(92615),
            description: "Regulation, Licensing, and Inspection of Miscellaneous Commercial Sectors ".to_string(),
        }),
        (927, Code::<u32> {
            code: 927,
            parent_code: Some(92),
            description: "Space Research and Technology ".to_string(),
        }),
        (9271, Code::<u32> {
            code: 9271,
            parent_code: Some(927),
            description: "Space Research and Technology ".to_string(),
        }),
        (92711, Code::<u32> {
            code: 92711,
            parent_code: Some(9271),
            description: "Space Research and Technology ".to_string(),
        }),
        (927110, Code::<u32> {
            code: 927110,
            parent_code: Some(92711),
            description: "Space Research and Technology ".to_string(),
        }),
        (928, Code::<u32> {
            code: 928,
            parent_code: Some(92),
            description: "National Security and International Affairs ".to_string(),
        }),
        (9281, Code::<u32> {
            code: 9281,
            parent_code: Some(928),
            description: "National Security and International Affairs ".to_string(),
        }),
        (92811, Code::<u32> {
            code: 92811,
            parent_code: Some(9281),
            description: "National Security ".to_string(),
        }),
        (928110, Code::<u32> {
            code: 928110,
            parent_code: Some(92811),
            description: "National Security ".to_string(),
        }),
        (92812, Code::<u32> {
            code: 92812,
            parent_code: Some(9281),
            description: "International Affairs ".to_string(),
        }),
        (928120, Code::<u32> {
            code: 928120,
            parent_code: Some(92812),
            description: "International Affairs ".to_string(),
        }),
    ].iter().cloned().collect();
    table
}

