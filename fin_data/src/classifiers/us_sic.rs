/*!
US Standard Industrial Classification registry implementation.

The data for this file was generated automatically.
*/

use std::collections::HashMap;

use fin_model::prelude::*;
use fin_model::classification::Code;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct Scheme {
    codes: HashMap<u16, Code<u16>>
}

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl Registry<u16, Code<u16>> for Scheme {

    fn new() -> Self {
        Scheme { codes: create_data_table() }
    }

    fn name(&self) -> String {
        "Standard Industrial Classification".to_string()
    }

    fn acronym(&self) -> String {
        "SIC".to_string()
    }

    fn source(&self) -> String {
        "https://www.sec.gov/info/edgar/siccodes.htm".to_string()
    }

    fn governing_body(&self) -> String {
        "US Securities and Exchange Commission (SEC)".to_string()
    }

    fn last_updated(&self) -> Option<Date> {
        None
    }

    fn next_publication(&self) -> Option<Date> {
        None
    }

    fn get(&self, code: u16) -> Option<&Code<u16>> {
        self.codes.get(&code)
    }

    fn get_children(&self, _parent: u16) -> Option<Vec<&Code<u16>>> {
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Generated Data Table
// ------------------------------------------------------------------------------------------------

fn create_data_table() -> HashMap<u16, Code<u16>> {
    let table: HashMap<u16, Code<u16>> = 
    [
        (100, Code::<u16> {
            code: 100,
            parent_code: None,
            description: "AGRICULTURAL PRODUCTION-CROPS".to_string(),
        }),
        (200, Code::<u16> {
            code: 200,
            parent_code: None,
            description: "AGRICULTURAL PROD-LIVESTOCK & ANIMAL SPECIALTIES".to_string(),
        }),
        (700, Code::<u16> {
            code: 700,
            parent_code: None,
            description: "AGRICULTURAL SERVICES".to_string(),
        }),
        (800, Code::<u16> {
            code: 800,
            parent_code: None,
            description: "FORESTRY".to_string(),
        }),
        (900, Code::<u16> {
            code: 900,
            parent_code: None,
            description: "FISHING, HUNTING AND TRAPPING".to_string(),
        }),
        (1000, Code::<u16> {
            code: 1000,
            parent_code: None,
            description: "METAL MINING".to_string(),
        }),
        (1040, Code::<u16> {
            code: 1040,
            parent_code: None,
            description: "GOLD AND SILVER ORES".to_string(),
        }),
        (1090, Code::<u16> {
            code: 1090,
            parent_code: None,
            description: "MISCELLANEOUS METAL ORES".to_string(),
        }),
        (1220, Code::<u16> {
            code: 1220,
            parent_code: None,
            description: "BITUMINOUS COAL & LIGNITE MINING".to_string(),
        }),
        (1221, Code::<u16> {
            code: 1221,
            parent_code: None,
            description: "BITUMINOUS COAL & LIGNITE SURFACE MINING".to_string(),
        }),
        (1311, Code::<u16> {
            code: 1311,
            parent_code: None,
            description: "CRUDE PETROLEUM & NATURAL GAS".to_string(),
        }),
        (1381, Code::<u16> {
            code: 1381,
            parent_code: None,
            description: "DRILLING OIL & GAS WELLS".to_string(),
        }),
        (1382, Code::<u16> {
            code: 1382,
            parent_code: None,
            description: "OIL & GAS FIELD EXPLORATION SERVICES".to_string(),
        }),
        (1389, Code::<u16> {
            code: 1389,
            parent_code: None,
            description: "OIL & GAS FIELD SERVICES, NEC".to_string(),
        }),
        (1400, Code::<u16> {
            code: 1400,
            parent_code: None,
            description: "MINING & QUARRYING OF NONMETALLIC MINERALS (NO FUELS)".to_string(),
        }),
        (1520, Code::<u16> {
            code: 1520,
            parent_code: None,
            description: "GENERAL BLDG CONTRACTORS - RESIDENTIAL BLDGS".to_string(),
        }),
        (1531, Code::<u16> {
            code: 1531,
            parent_code: None,
            description: "OPERATIVE BUILDERS".to_string(),
        }),
        (1540, Code::<u16> {
            code: 1540,
            parent_code: None,
            description: "GENERAL BLDG CONTRACTORS - NONRESIDENTIAL BLDGS".to_string(),
        }),
        (1600, Code::<u16> {
            code: 1600,
            parent_code: None,
            description: "HEAVY CONSTRUCTION OTHER THAN BLDG CONST - CONTRACTORS".to_string(),
        }),
        (1623, Code::<u16> {
            code: 1623,
            parent_code: None,
            description: "WATER, SEWER, PIPELINE, COMM & POWER LINE CONSTRUCTION".to_string(),
        }),
        (1700, Code::<u16> {
            code: 1700,
            parent_code: None,
            description: "CONSTRUCTION - SPECIAL TRADE CONTRACTORS".to_string(),
        }),
        (1731, Code::<u16> {
            code: 1731,
            parent_code: None,
            description: "ELECTRICAL WORK".to_string(),
        }),
        (2000, Code::<u16> {
            code: 2000,
            parent_code: None,
            description: "FOOD AND KINDRED PRODUCTS".to_string(),
        }),
        (2011, Code::<u16> {
            code: 2011,
            parent_code: None,
            description: "MEAT PACKING PLANTS".to_string(),
        }),
        (2013, Code::<u16> {
            code: 2013,
            parent_code: None,
            description: "SAUSAGES & OTHER PREPARED MEAT PRODUCTS".to_string(),
        }),
        (2015, Code::<u16> {
            code: 2015,
            parent_code: None,
            description: "POULTRY SLAUGHTERING AND PROCESSING".to_string(),
        }),
        (2020, Code::<u16> {
            code: 2020,
            parent_code: None,
            description: "DAIRY PRODUCTS".to_string(),
        }),
        (2024, Code::<u16> {
            code: 2024,
            parent_code: None,
            description: "ICE CREAM & FROZEN DESSERTS".to_string(),
        }),
        (2030, Code::<u16> {
            code: 2030,
            parent_code: None,
            description: "CANNED, FROZEN & PRESERVED FRUIT, VEG & FOOD SPECIALTIES".to_string(),
        }),
        (2033, Code::<u16> {
            code: 2033,
            parent_code: None,
            description: "CANNED, FRUITS, VEG, PRESERVES, JAMS & JELLIES".to_string(),
        }),
        (2040, Code::<u16> {
            code: 2040,
            parent_code: None,
            description: "GRAIN MILL PRODUCTS".to_string(),
        }),
        (2050, Code::<u16> {
            code: 2050,
            parent_code: None,
            description: "BAKERY PRODUCTS".to_string(),
        }),
        (2052, Code::<u16> {
            code: 2052,
            parent_code: None,
            description: "COOKIES & CRACKERS".to_string(),
        }),
        (2060, Code::<u16> {
            code: 2060,
            parent_code: None,
            description: "SUGAR & CONFECTIONERY PRODUCTS".to_string(),
        }),
        (2070, Code::<u16> {
            code: 2070,
            parent_code: None,
            description: "FATS & OILS".to_string(),
        }),
        (2080, Code::<u16> {
            code: 2080,
            parent_code: None,
            description: "BEVERAGES".to_string(),
        }),
        (2082, Code::<u16> {
            code: 2082,
            parent_code: None,
            description: "MALT BEVERAGES".to_string(),
        }),
        (2086, Code::<u16> {
            code: 2086,
            parent_code: None,
            description: "BOTTLED & CANNED SOFT DRINKS & CARBONATED WATERS".to_string(),
        }),
        (2090, Code::<u16> {
            code: 2090,
            parent_code: None,
            description: "MISCELLANEOUS FOOD PREPARATIONS & KINDRED PRODUCTS".to_string(),
        }),
        (2092, Code::<u16> {
            code: 2092,
            parent_code: None,
            description: "PREPARED FRESH OR FROZEN FISH & SEAFOODS".to_string(),
        }),
        (2100, Code::<u16> {
            code: 2100,
            parent_code: None,
            description: "TOBACCO PRODUCTS".to_string(),
        }),
        (2111, Code::<u16> {
            code: 2111,
            parent_code: None,
            description: "CIGARETTES".to_string(),
        }),
        (2200, Code::<u16> {
            code: 2200,
            parent_code: None,
            description: "TEXTILE MILL PRODUCTS".to_string(),
        }),
        (2211, Code::<u16> {
            code: 2211,
            parent_code: None,
            description: "BROADWOVEN FABRIC MILLS, COTTON".to_string(),
        }),
        (2221, Code::<u16> {
            code: 2221,
            parent_code: None,
            description: "BROADWOVEN FABRIC MILLS, MAN MADE FIBER & SILK".to_string(),
        }),
        (2250, Code::<u16> {
            code: 2250,
            parent_code: None,
            description: "KNITTING MILLS".to_string(),
        }),
        (2253, Code::<u16> {
            code: 2253,
            parent_code: None,
            description: "KNIT OUTERWEAR MILLS".to_string(),
        }),
        (2273, Code::<u16> {
            code: 2273,
            parent_code: None,
            description: "CARPETS & RUGS".to_string(),
        }),
        (2300, Code::<u16> {
            code: 2300,
            parent_code: None,
            description: "APPAREL & OTHER FINISHED PRODS OF FABRICS & SIMILAR MATL".to_string(),
        }),
        (2320, Code::<u16> {
            code: 2320,
            parent_code: None,
            description: "MEN'S & BOYS' FURNISHINGS, WORK CLOTHING, & ALLIED GARMENTS".to_string(),
        }),
        (2330, Code::<u16> {
            code: 2330,
            parent_code: None,
            description: "WOMEN'S, MISSES', AND JUNIORS OUTERWEAR".to_string(),
        }),
        (2340, Code::<u16> {
            code: 2340,
            parent_code: None,
            description: "WOMEN'S, MISSES', CHILDREN'S & INFANTS' UNDERGARMENTS".to_string(),
        }),
        (2390, Code::<u16> {
            code: 2390,
            parent_code: None,
            description: "MISCELLANEOUS FABRICATED TEXTILE PRODUCTS".to_string(),
        }),
        (2400, Code::<u16> {
            code: 2400,
            parent_code: None,
            description: "LUMBER & WOOD PRODUCTS (NO FURNITURE)".to_string(),
        }),
        (2421, Code::<u16> {
            code: 2421,
            parent_code: None,
            description: "SAWMILLS & PLANTING MILLS, GENERAL".to_string(),
        }),
        (2430, Code::<u16> {
            code: 2430,
            parent_code: None,
            description: "MILLWOOD, VENEER, PLYWOOD, & STRUCTURAL WOOD MEMBERS".to_string(),
        }),
        (2451, Code::<u16> {
            code: 2451,
            parent_code: None,
            description: "MOBILE HOMES".to_string(),
        }),
        (2452, Code::<u16> {
            code: 2452,
            parent_code: None,
            description: "PREFABRICATED WOOD BLDGS & COMPONENTS".to_string(),
        }),
        (2510, Code::<u16> {
            code: 2510,
            parent_code: None,
            description: "HOUSEHOLD FURNITURE".to_string(),
        }),
        (2511, Code::<u16> {
            code: 2511,
            parent_code: None,
            description: "WOOD HOUSEHOLD FURNITURE, (NO UPHOLSTERED)".to_string(),
        }),
        (2520, Code::<u16> {
            code: 2520,
            parent_code: None,
            description: "OFFICE FURNITURE".to_string(),
        }),
        (2522, Code::<u16> {
            code: 2522,
            parent_code: None,
            description: "OFFICE FURNITURE (NO WOOD)".to_string(),
        }),
        (2531, Code::<u16> {
            code: 2531,
            parent_code: None,
            description: "PUBLIC BLDG & RELATED FURNITURE".to_string(),
        }),
        (2540, Code::<u16> {
            code: 2540,
            parent_code: None,
            description: "PARTITIONS, SHELVING, LOCKERS, & OFFICE & STORE FIXTURES".to_string(),
        }),
        (2590, Code::<u16> {
            code: 2590,
            parent_code: None,
            description: "MISCELLANEOUS FURNITURE & FIXTURES".to_string(),
        }),
        (2600, Code::<u16> {
            code: 2600,
            parent_code: None,
            description: "PAPERS & ALLIED PRODUCTS".to_string(),
        }),
        (2611, Code::<u16> {
            code: 2611,
            parent_code: None,
            description: "PULP MILLS".to_string(),
        }),
        (2621, Code::<u16> {
            code: 2621,
            parent_code: None,
            description: "PAPER MILLS".to_string(),
        }),
        (2631, Code::<u16> {
            code: 2631,
            parent_code: None,
            description: "PAPERBOARD MILLS".to_string(),
        }),
        (2650, Code::<u16> {
            code: 2650,
            parent_code: None,
            description: "PAPERBOARD CONTAINERS & BOXES".to_string(),
        }),
        (2670, Code::<u16> {
            code: 2670,
            parent_code: None,
            description: "CONVERTED PAPER & PAPERBOARD PRODS (NO CONTANERS/BOXES)".to_string(),
        }),
        (2673, Code::<u16> {
            code: 2673,
            parent_code: None,
            description: "PLASTICS, FOIL & COATED PAPER BAGS".to_string(),
        }),
        (2711, Code::<u16> {
            code: 2711,
            parent_code: None,
            description: "NEWSPAPERS: PUBLISHING OR PUBLISHING & PRINTING".to_string(),
        }),
        (2721, Code::<u16> {
            code: 2721,
            parent_code: None,
            description: "PERIODICALS: PUBLISHING OR PUBLISHING & PRINTING".to_string(),
        }),
        (2731, Code::<u16> {
            code: 2731,
            parent_code: None,
            description: "BOOKS: PUBLISHING OR PUBLISHING & PRINTING".to_string(),
        }),
        (2732, Code::<u16> {
            code: 2732,
            parent_code: None,
            description: "BOOK PRINTING".to_string(),
        }),
        (2741, Code::<u16> {
            code: 2741,
            parent_code: None,
            description: "MISCELLANEOUS PUBLISHING".to_string(),
        }),
        (2750, Code::<u16> {
            code: 2750,
            parent_code: None,
            description: "COMMERCIAL PRINTING".to_string(),
        }),
        (2761, Code::<u16> {
            code: 2761,
            parent_code: None,
            description: "MANIFOLD BUSINESS FORMS".to_string(),
        }),
        (2771, Code::<u16> {
            code: 2771,
            parent_code: None,
            description: "GREETING CARDS".to_string(),
        }),
        (2780, Code::<u16> {
            code: 2780,
            parent_code: None,
            description: "BLANKBOOKS, LOOSELEAF BINDERS & BOOKBINDING & RELATED WORK".to_string(),
        }),
        (2790, Code::<u16> {
            code: 2790,
            parent_code: None,
            description: "SERVICE INDUSTRIES FOR THE PRINTING TRADE".to_string(),
        }),
        (2800, Code::<u16> {
            code: 2800,
            parent_code: None,
            description: "CHEMICALS & ALLIED PRODUCTS".to_string(),
        }),
        (2810, Code::<u16> {
            code: 2810,
            parent_code: None,
            description: "INDUSTRIAL INORGANIC CHEMICALS".to_string(),
        }),
        (2820, Code::<u16> {
            code: 2820,
            parent_code: None,
            description: "PLASTIC MATERIAL, SYNTH RESIN/RUBBER, CELLULOS (NO GLASS)".to_string(),
        }),
        (2821, Code::<u16> {
            code: 2821,
            parent_code: None,
            description: "PLASTIC MATERIALS, SYNTH RESINS & NONVULCAN ELASTOMERS".to_string(),
        }),
        (2833, Code::<u16> {
            code: 2833,
            parent_code: None,
            description: "MEDICINAL CHEMICALS & BOTANICAL PRODUCTS".to_string(),
        }),
        (2834, Code::<u16> {
            code: 2834,
            parent_code: None,
            description: "PHARMACEUTICAL PREPARATIONS".to_string(),
        }),
        (2835, Code::<u16> {
            code: 2835,
            parent_code: None,
            description: "IN VITRO & IN VIVO DIAGNOSTIC SUBSTANCES".to_string(),
        }),
        (2836, Code::<u16> {
            code: 2836,
            parent_code: None,
            description: "BIOLOGICAL PRODUCTS, (NO DIAGNOSTIC SUBSTANCES)".to_string(),
        }),
        (2840, Code::<u16> {
            code: 2840,
            parent_code: None,
            description: "SOAP, DETERGENTS, CLEANG PREPARATIONS, PERFUMES, COSMETICS".to_string(),
        }),
        (2842, Code::<u16> {
            code: 2842,
            parent_code: None,
            description: "SPECIALTY CLEANING, POLISHING AND SANITATION PREPARATIONS".to_string(),
        }),
        (2844, Code::<u16> {
            code: 2844,
            parent_code: None,
            description: "PERFUMES, COSMETICS & OTHER TOILET PREPARATIONS".to_string(),
        }),
        (2851, Code::<u16> {
            code: 2851,
            parent_code: None,
            description: "PAINTS, VARNISHES, LACQUERS, ENAMELS & ALLIED PRODS".to_string(),
        }),
        (2860, Code::<u16> {
            code: 2860,
            parent_code: None,
            description: "INDUSTRIAL ORGANIC CHEMICALS".to_string(),
        }),
        (2870, Code::<u16> {
            code: 2870,
            parent_code: None,
            description: "AGRICULTURAL CHEMICALS".to_string(),
        }),
        (2890, Code::<u16> {
            code: 2890,
            parent_code: None,
            description: "MISCELLANEOUS CHEMICAL PRODUCTS".to_string(),
        }),
        (2891, Code::<u16> {
            code: 2891,
            parent_code: None,
            description: "ADHESIVES & SEALANTS".to_string(),
        }),
        (2911, Code::<u16> {
            code: 2911,
            parent_code: None,
            description: "PETROLEUM REFINING".to_string(),
        }),
        (2950, Code::<u16> {
            code: 2950,
            parent_code: None,
            description: "ASPHALT PAVING & ROOFING MATERIALS".to_string(),
        }),
        (2990, Code::<u16> {
            code: 2990,
            parent_code: None,
            description: "MISCELLANEOUS PRODUCTS OF PETROLEUM & COAL".to_string(),
        }),
        (3011, Code::<u16> {
            code: 3011,
            parent_code: None,
            description: "TIRES & INNER TUBES".to_string(),
        }),
        (3021, Code::<u16> {
            code: 3021,
            parent_code: None,
            description: "RUBBER & PLASTICS FOOTWEAR".to_string(),
        }),
        (3050, Code::<u16> {
            code: 3050,
            parent_code: None,
            description: "GASKETS, PACKING & SEALING DEVICES & RUBBER & PLASTICS HOSE".to_string(),
        }),
        (3060, Code::<u16> {
            code: 3060,
            parent_code: None,
            description: "FABRICATED RUBBER PRODUCTS, NEC".to_string(),
        }),
        (3080, Code::<u16> {
            code: 3080,
            parent_code: None,
            description: "MISCELLANEOUS PLASTICS PRODUCTS".to_string(),
        }),
        (3081, Code::<u16> {
            code: 3081,
            parent_code: None,
            description: "UNSUPPORTED PLASTICS FILM & SHEET".to_string(),
        }),
        (3086, Code::<u16> {
            code: 3086,
            parent_code: None,
            description: "PLASTICS FOAM PRODUCTS".to_string(),
        }),
        (3089, Code::<u16> {
            code: 3089,
            parent_code: None,
            description: "PLASTICS PRODUCTS, NEC".to_string(),
        }),
        (3100, Code::<u16> {
            code: 3100,
            parent_code: None,
            description: "LEATHER & LEATHER PRODUCTS".to_string(),
        }),
        (3140, Code::<u16> {
            code: 3140,
            parent_code: None,
            description: "FOOTWEAR, (NO RUBBER)".to_string(),
        }),
        (3211, Code::<u16> {
            code: 3211,
            parent_code: None,
            description: "FLAT GLASS".to_string(),
        }),
        (3220, Code::<u16> {
            code: 3220,
            parent_code: None,
            description: "GLASS & GLASSWARE, PRESSED OR BLOWN".to_string(),
        }),
        (3221, Code::<u16> {
            code: 3221,
            parent_code: None,
            description: "GLASS CONTAINERS".to_string(),
        }),
        (3231, Code::<u16> {
            code: 3231,
            parent_code: None,
            description: "GLASS PRODUCTS, MADE OF PURCHASED GLASS".to_string(),
        }),
        (3241, Code::<u16> {
            code: 3241,
            parent_code: None,
            description: "CEMENT, HYDRAULIC".to_string(),
        }),
        (3250, Code::<u16> {
            code: 3250,
            parent_code: None,
            description: "STRUCTURAL CLAY PRODUCTS".to_string(),
        }),
        (3260, Code::<u16> {
            code: 3260,
            parent_code: None,
            description: "POTTERY & RELATED PRODUCTS".to_string(),
        }),
        (3270, Code::<u16> {
            code: 3270,
            parent_code: None,
            description: "CONCRETE, GYPSUM & PLASTER PRODUCTS".to_string(),
        }),
        (3272, Code::<u16> {
            code: 3272,
            parent_code: None,
            description: "CONCRETE PRODUCTS, EXCEPT BLOCK & BRICK".to_string(),
        }),
        (3281, Code::<u16> {
            code: 3281,
            parent_code: None,
            description: "CUT STONE & STONE PRODUCTS".to_string(),
        }),
        (3290, Code::<u16> {
            code: 3290,
            parent_code: None,
            description: "ABRASIVE, ASBESTOS & MISC NONMETALLIC MINERAL PRODS".to_string(),
        }),
        (3310, Code::<u16> {
            code: 3310,
            parent_code: None,
            description: "STEEL WORKS, BLAST FURNACES & ROLLING & FINISHING MILLS".to_string(),
        }),
        (3312, Code::<u16> {
            code: 3312,
            parent_code: None,
            description: "STEEL WORKS, BLAST FURNACES & ROLLING MILLS (COKE OVENS)".to_string(),
        }),
        (3317, Code::<u16> {
            code: 3317,
            parent_code: None,
            description: "STEEL PIPE & TUBES".to_string(),
        }),
        (3320, Code::<u16> {
            code: 3320,
            parent_code: None,
            description: "IRON & STEEL FOUNDRIES".to_string(),
        }),
        (3330, Code::<u16> {
            code: 3330,
            parent_code: None,
            description: "PRIMARY SMELTING & REFINING OF NONFERROUS METALS".to_string(),
        }),
        (3334, Code::<u16> {
            code: 3334,
            parent_code: None,
            description: "PRIMARY PRODUCTION OF ALUMINUM".to_string(),
        }),
        (3341, Code::<u16> {
            code: 3341,
            parent_code: None,
            description: "SECONDARY SMELTING & REFINING OF NONFERROUS METALS".to_string(),
        }),
        (3350, Code::<u16> {
            code: 3350,
            parent_code: None,
            description: "ROLLING DRAWING & EXTRUDING OF NONFERROUS METALS".to_string(),
        }),
        (3357, Code::<u16> {
            code: 3357,
            parent_code: None,
            description: "DRAWING & INSULATING OF NONFERROUS WIRE".to_string(),
        }),
        (3360, Code::<u16> {
            code: 3360,
            parent_code: None,
            description: "NONFERROUS FOUNDRIES (CASTINGS)".to_string(),
        }),
        (3390, Code::<u16> {
            code: 3390,
            parent_code: None,
            description: "MISCELLANEOUS PRIMARY METAL PRODUCTS".to_string(),
        }),
        (3411, Code::<u16> {
            code: 3411,
            parent_code: None,
            description: "METAL CANS".to_string(),
        }),
        (3412, Code::<u16> {
            code: 3412,
            parent_code: None,
            description: "METAL SHIPPING BARRELS, DRUMS, KEGS & PAILS".to_string(),
        }),
        (3420, Code::<u16> {
            code: 3420,
            parent_code: None,
            description: "CUTLERY, HAND TOOLS & GENERAL HARDWARE".to_string(),
        }),
        (3430, Code::<u16> {
            code: 3430,
            parent_code: None,
            description: "HEATING EQUIPMENT, EXCEPT ELECTRIC & WARM AIR; & PLUMBING FIXTURES".to_string(),
        }),
        (3433, Code::<u16> {
            code: 3433,
            parent_code: None,
            description: "HEATING EQUIPMENT, EXCEPT ELECTRIC & WARM AIR FURNACES".to_string(),
        }),
        (3440, Code::<u16> {
            code: 3440,
            parent_code: None,
            description: "FABRICATED STRUCTURAL METAL PRODUCTS".to_string(),
        }),
        (3442, Code::<u16> {
            code: 3442,
            parent_code: None,
            description: "METAL DOORS, SASH, FRAMES, MOLDINGS & TRIM".to_string(),
        }),
        (3443, Code::<u16> {
            code: 3443,
            parent_code: None,
            description: "FABRICATED PLATE WORK (BOILER SHOPS)".to_string(),
        }),
        (3444, Code::<u16> {
            code: 3444,
            parent_code: None,
            description: "SHEET METAL WORK".to_string(),
        }),
        (3448, Code::<u16> {
            code: 3448,
            parent_code: None,
            description: "PREFABRICATED METAL BUILDINGS & COMPONENTS".to_string(),
        }),
        (3451, Code::<u16> {
            code: 3451,
            parent_code: None,
            description: "SCREW MACHINE PRODUCTS".to_string(),
        }),
        (3452, Code::<u16> {
            code: 3452,
            parent_code: None,
            description: "BOLTS, NUTS, SCREWS, RIVETS & WASHERS".to_string(),
        }),
        (3460, Code::<u16> {
            code: 3460,
            parent_code: None,
            description: "METAL FORGINGS & STAMPINGS".to_string(),
        }),
        (3470, Code::<u16> {
            code: 3470,
            parent_code: None,
            description: "COATING, ENGRAVING & ALLIED SERVICES".to_string(),
        }),
        (3480, Code::<u16> {
            code: 3480,
            parent_code: None,
            description: "ORDNANCE & ACCESSORIES, (NO VEHICLES/GUIDED MISSILES)".to_string(),
        }),
        (3490, Code::<u16> {
            code: 3490,
            parent_code: None,
            description: "MISCELLANEOUS FABRICATED METAL PRODUCTS".to_string(),
        }),
        (3510, Code::<u16> {
            code: 3510,
            parent_code: None,
            description: "ENGINES & TURBINES".to_string(),
        }),
        (3523, Code::<u16> {
            code: 3523,
            parent_code: None,
            description: "FARM MACHINERY & EQUIPMENT".to_string(),
        }),
        (3524, Code::<u16> {
            code: 3524,
            parent_code: None,
            description: "LAWN & GARDEN TRACTORS & HOME LAWN & GARDENS EQUIP".to_string(),
        }),
        (3530, Code::<u16> {
            code: 3530,
            parent_code: None,
            description: "CONSTRUCTION, MINING & MATERIALS HANDLING MACHINERY & EQUIP".to_string(),
        }),
        (3531, Code::<u16> {
            code: 3531,
            parent_code: None,
            description: "CONSTRUCTION MACHINERY & EQUIP".to_string(),
        }),
        (3532, Code::<u16> {
            code: 3532,
            parent_code: None,
            description: "MINING MACHINERY & EQUIP (NO OIL & GAS FIELD MACH & EQUIP)".to_string(),
        }),
        (3533, Code::<u16> {
            code: 3533,
            parent_code: None,
            description: "OIL & GAS FIELD MACHINERY & EQUIPMENT".to_string(),
        }),
        (3537, Code::<u16> {
            code: 3537,
            parent_code: None,
            description: "INDUSTRIAL TRUCKS, TRACTORS, TRAILORS & STACKERS".to_string(),
        }),
        (3540, Code::<u16> {
            code: 3540,
            parent_code: None,
            description: "METALWORKING MACHINERY & EQUIPMENT".to_string(),
        }),
        (3541, Code::<u16> {
            code: 3541,
            parent_code: None,
            description: "MACHINE TOOLS, METAL CUTTING TYPES".to_string(),
        }),
        (3550, Code::<u16> {
            code: 3550,
            parent_code: None,
            description: "SPECIAL INDUSTRY MACHINERY (NO METALWORKING MACHINERY)".to_string(),
        }),
        (3555, Code::<u16> {
            code: 3555,
            parent_code: None,
            description: "PRINTING TRADES MACHINERY & EQUIPMENT".to_string(),
        }),
        (3559, Code::<u16> {
            code: 3559,
            parent_code: None,
            description: "SPECIAL INDUSTRY MACHINERY, NEC".to_string(),
        }),
        (3560, Code::<u16> {
            code: 3560,
            parent_code: None,
            description: "GENERAL INDUSTRIAL MACHINERY & EQUIPMENT".to_string(),
        }),
        (3561, Code::<u16> {
            code: 3561,
            parent_code: None,
            description: "PUMPS & PUMPING EQUIPMENT".to_string(),
        }),
        (3562, Code::<u16> {
            code: 3562,
            parent_code: None,
            description: "BALL & ROLLER BEARINGS".to_string(),
        }),
        (3564, Code::<u16> {
            code: 3564,
            parent_code: None,
            description: "INDUSTRIAL & COMMERCIAL FANS & BLOWERS & AIR PURIFYING EQUIP".to_string(),
        }),
        (3567, Code::<u16> {
            code: 3567,
            parent_code: None,
            description: "INDUSTRIAL PROCESS FURNACES & OVENS".to_string(),
        }),
        (3569, Code::<u16> {
            code: 3569,
            parent_code: None,
            description: "GENERAL INDUSTRIAL MACHINERY & EQUIPMENT, NEC".to_string(),
        }),
        (3570, Code::<u16> {
            code: 3570,
            parent_code: None,
            description: "COMPUTER & OFFICE EQUIPMENT".to_string(),
        }),
        (3571, Code::<u16> {
            code: 3571,
            parent_code: None,
            description: "ELECTRONIC COMPUTERS".to_string(),
        }),
        (3572, Code::<u16> {
            code: 3572,
            parent_code: None,
            description: "COMPUTER STORAGE DEVICES".to_string(),
        }),
        (3575, Code::<u16> {
            code: 3575,
            parent_code: None,
            description: "COMPUTER TERMINALS".to_string(),
        }),
        (3576, Code::<u16> {
            code: 3576,
            parent_code: None,
            description: "COMPUTER COMMUNICATIONS EQUIPMENT".to_string(),
        }),
        (3577, Code::<u16> {
            code: 3577,
            parent_code: None,
            description: "COMPUTER PERIPHERAL EQUIPMENT, NEC".to_string(),
        }),
        (3578, Code::<u16> {
            code: 3578,
            parent_code: None,
            description: "CALCULATING & ACCOUNTING MACHINES (NO ELECTRONIC COMPUTERS)".to_string(),
        }),
        (3579, Code::<u16> {
            code: 3579,
            parent_code: None,
            description: "OFFICE MACHINES, NEC".to_string(),
        }),
        (3580, Code::<u16> {
            code: 3580,
            parent_code: None,
            description: "REFRIGERATION & SERVICE INDUSTRY MACHINERY".to_string(),
        }),
        (3585, Code::<u16> {
            code: 3585,
            parent_code: None,
            description: "AIR-COND & WARM AIR HEATING EQUIP & COMM & INDL REFRIG EQUIP".to_string(),
        }),
        (3590, Code::<u16> {
            code: 3590,
            parent_code: None,
            description: "MISC INDUSTRIAL & COMMERCIAL MACHINERY & EQUIPMENT".to_string(),
        }),
        (3600, Code::<u16> {
            code: 3600,
            parent_code: None,
            description: "ELECTRONIC & OTHER ELECTRICAL EQUIPMENT (NO COMPUTER EQUIP)".to_string(),
        }),
        (3612, Code::<u16> {
            code: 3612,
            parent_code: None,
            description: "POWER, DISTRIBUTION & SPECIALTY TRANSFORMERS".to_string(),
        }),
        (3613, Code::<u16> {
            code: 3613,
            parent_code: None,
            description: "SWITCHGEAR & SWITCHBOARD APPARATUS".to_string(),
        }),
        (3620, Code::<u16> {
            code: 3620,
            parent_code: None,
            description: "ELECTRICAL INDUSTRIAL APPARATUS".to_string(),
        }),
        (3621, Code::<u16> {
            code: 3621,
            parent_code: None,
            description: "MOTORS & GENERATORS".to_string(),
        }),
        (3630, Code::<u16> {
            code: 3630,
            parent_code: None,
            description: "HOUSEHOLD APPLIANCES".to_string(),
        }),
        (3634, Code::<u16> {
            code: 3634,
            parent_code: None,
            description: "ELECTRIC HOUSEWARES & FANS".to_string(),
        }),
        (3640, Code::<u16> {
            code: 3640,
            parent_code: None,
            description: "ELECTRIC LIGHTING & WIRING EQUIPMENT".to_string(),
        }),
        (3651, Code::<u16> {
            code: 3651,
            parent_code: None,
            description: "HOUSEHOLD AUDIO & VIDEO EQUIPMENT".to_string(),
        }),
        (3652, Code::<u16> {
            code: 3652,
            parent_code: None,
            description: "PHONOGRAPH RECORDS & PRERECORDED AUDIO TAPES & DISKS".to_string(),
        }),
        (3661, Code::<u16> {
            code: 3661,
            parent_code: None,
            description: "TELEPHONE & TELEGRAPH APPARATUS".to_string(),
        }),
        (3663, Code::<u16> {
            code: 3663,
            parent_code: None,
            description: "RADIO & TV BROADCASTING & COMMUNICATIONS EQUIPMENT".to_string(),
        }),
        (3669, Code::<u16> {
            code: 3669,
            parent_code: None,
            description: "COMMUNICATIONS EQUIPMENT, NEC".to_string(),
        }),
        (3670, Code::<u16> {
            code: 3670,
            parent_code: None,
            description: "ELECTRONIC COMPONENTS & ACCESSORIES".to_string(),
        }),
        (3672, Code::<u16> {
            code: 3672,
            parent_code: None,
            description: "PRINTED CIRCUIT BOARDS".to_string(),
        }),
        (3674, Code::<u16> {
            code: 3674,
            parent_code: None,
            description: "SEMICONDUCTORS & RELATED DEVICES".to_string(),
        }),
        (3677, Code::<u16> {
            code: 3677,
            parent_code: None,
            description: "ELECTRONIC COILS, TRANSFORMERS & OTHER INDUCTORS".to_string(),
        }),
        (3678, Code::<u16> {
            code: 3678,
            parent_code: None,
            description: "ELECTRONIC CONNECTORS".to_string(),
        }),
        (3679, Code::<u16> {
            code: 3679,
            parent_code: None,
            description: "ELECTRONIC COMPONENTS, NEC".to_string(),
        }),
        (3690, Code::<u16> {
            code: 3690,
            parent_code: None,
            description: "MISCELLANEOUS ELECTRICAL MACHINERY, EQUIPMENT & SUPPLIES".to_string(),
        }),
        (3695, Code::<u16> {
            code: 3695,
            parent_code: None,
            description: "MAGNETIC & OPTICAL RECORDING MEDIA".to_string(),
        }),
        (3711, Code::<u16> {
            code: 3711,
            parent_code: None,
            description: "MOTOR VEHICLES & PASSENGER CAR BODIES".to_string(),
        }),
        (3713, Code::<u16> {
            code: 3713,
            parent_code: None,
            description: "TRUCK & BUS BODIES".to_string(),
        }),
        (3714, Code::<u16> {
            code: 3714,
            parent_code: None,
            description: "MOTOR VEHICLE PARTS & ACCESSORIES".to_string(),
        }),
        (3715, Code::<u16> {
            code: 3715,
            parent_code: None,
            description: "TRUCK TRAILERS".to_string(),
        }),
        (3716, Code::<u16> {
            code: 3716,
            parent_code: None,
            description: "MOTOR HOMES".to_string(),
        }),
        (3720, Code::<u16> {
            code: 3720,
            parent_code: None,
            description: "AIRCRAFT & PARTS".to_string(),
        }),
        (3721, Code::<u16> {
            code: 3721,
            parent_code: None,
            description: "AIRCRAFT".to_string(),
        }),
        (3724, Code::<u16> {
            code: 3724,
            parent_code: None,
            description: "AIRCRAFT ENGINES & ENGINE PARTS".to_string(),
        }),
        (3728, Code::<u16> {
            code: 3728,
            parent_code: None,
            description: "AIRCRAFT PARTS & AUXILIARY EQUIPMENT, NEC".to_string(),
        }),
        (3730, Code::<u16> {
            code: 3730,
            parent_code: None,
            description: "SHIP & BOAT BUILDING & REPAIRING".to_string(),
        }),
        (3743, Code::<u16> {
            code: 3743,
            parent_code: None,
            description: "RAILROAD EQUIPMENT".to_string(),
        }),
        (3751, Code::<u16> {
            code: 3751,
            parent_code: None,
            description: "MOTORCYCLES, BICYCLES & PARTS".to_string(),
        }),
        (3760, Code::<u16> {
            code: 3760,
            parent_code: None,
            description: "GUIDED MISSILES & SPACE VEHICLES & PARTS".to_string(),
        }),
        (3790, Code::<u16> {
            code: 3790,
            parent_code: None,
            description: "MISCELLANEOUS TRANSPORTATION EQUIPMENT".to_string(),
        }),
        (3812, Code::<u16> {
            code: 3812,
            parent_code: None,
            description: "SEARCH, DETECTION, NAVIGATION, GUIDANCE, AERONAUTICAL SYS".to_string(),
        }),
        (3821, Code::<u16> {
            code: 3821,
            parent_code: None,
            description: "LABORATORY APPARATUS & FURNITURE".to_string(),
        }),
        (3822, Code::<u16> {
            code: 3822,
            parent_code: None,
            description: "AUTO CONTROLS FOR REGULATING RESIDENTIAL & COMML ENVIRONMENTS".to_string(),
        }),
        (3823, Code::<u16> {
            code: 3823,
            parent_code: None,
            description: "INDUSTRIAL INSTRUMENTS FOR MEASUREMENT, DISPLAY, AND CONTROL".to_string(),
        }),
        (3824, Code::<u16> {
            code: 3824,
            parent_code: None,
            description: "TOTALIZING FLUID METERS & COUNTING DEVICES".to_string(),
        }),
        (3825, Code::<u16> {
            code: 3825,
            parent_code: None,
            description: "INSTRUMENTS FOR MEAS & TESTING OF ELECTRICITY & ELEC SIGNALS".to_string(),
        }),
        (3826, Code::<u16> {
            code: 3826,
            parent_code: None,
            description: "LABORATORY ANALYTICAL INSTRUMENTS".to_string(),
        }),
        (3827, Code::<u16> {
            code: 3827,
            parent_code: None,
            description: "OPTICAL INSTRUMENTS & LENSES".to_string(),
        }),
        (3829, Code::<u16> {
            code: 3829,
            parent_code: None,
            description: "MEASURING & CONTROLLING DEVICES, NEC".to_string(),
        }),
        (3841, Code::<u16> {
            code: 3841,
            parent_code: None,
            description: "SURGICAL & MEDICAL INSTRUMENTS & APPARATUS".to_string(),
        }),
        (3842, Code::<u16> {
            code: 3842,
            parent_code: None,
            description: "ORTHOPEDIC, PROSTHETIC & SURGICAL APPLIANCES & SUPPLIES".to_string(),
        }),
        (3843, Code::<u16> {
            code: 3843,
            parent_code: None,
            description: "DENTAL EQUIPMENT & SUPPLIES".to_string(),
        }),
        (3844, Code::<u16> {
            code: 3844,
            parent_code: None,
            description: "X-RAY APPARATUS & TUBES & RELATED IRRADIATION APPARATUS".to_string(),
        }),
        (3845, Code::<u16> {
            code: 3845,
            parent_code: None,
            description: "ELECTRO-MEDICAL & ELECTRO-THERAPEUTIC APPARATUS".to_string(),
        }),
        (3851, Code::<u16> {
            code: 3851,
            parent_code: None,
            description: "OPHTHALMIC GOODS".to_string(),
        }),
        (3861, Code::<u16> {
            code: 3861,
            parent_code: None,
            description: "PHOTOGRAPHIC EQUIPMENT & SUPPLIES".to_string(),
        }),
        (3873, Code::<u16> {
            code: 3873,
            parent_code: None,
            description: "WATCHES, CLOCKS, CLOCKWORK OPERATED DEVICES/PARTS".to_string(),
        }),
        (3910, Code::<u16> {
            code: 3910,
            parent_code: None,
            description: "JEWELRY, SILVERWARE & PLATED WARE".to_string(),
        }),
        (3911, Code::<u16> {
            code: 3911,
            parent_code: None,
            description: "JEWELRY, PRECIOUS METAL".to_string(),
        }),
        (3931, Code::<u16> {
            code: 3931,
            parent_code: None,
            description: "MUSICAL INSTRUMENTS".to_string(),
        }),
        (3942, Code::<u16> {
            code: 3942,
            parent_code: None,
            description: "DOLLS & STUFFED TOYS".to_string(),
        }),
        (3944, Code::<u16> {
            code: 3944,
            parent_code: None,
            description: "GAMES, TOYS & CHILDREN'S VEHICLES (NO DOLLS & BICYCLES)".to_string(),
        }),
        (3949, Code::<u16> {
            code: 3949,
            parent_code: None,
            description: "SPORTING & ATHLETIC GOODS, NEC".to_string(),
        }),
        (3950, Code::<u16> {
            code: 3950,
            parent_code: None,
            description: "PENS, PENCILS & OTHER ARTISTS' MATERIALS".to_string(),
        }),
        (3960, Code::<u16> {
            code: 3960,
            parent_code: None,
            description: "COSTUME JEWELRY & NOVELTIES".to_string(),
        }),
        (3990, Code::<u16> {
            code: 3990,
            parent_code: None,
            description: "MISCELLANEOUS MANUFACTURING INDUSTRIES".to_string(),
        }),
        (4011, Code::<u16> {
            code: 4011,
            parent_code: None,
            description: "RAILROADS, LINE-HAUL OPERATING".to_string(),
        }),
        (4013, Code::<u16> {
            code: 4013,
            parent_code: None,
            description: "RAILROAD SWITCHING & TERMINAL ESTABLISHMENTS".to_string(),
        }),
        (4100, Code::<u16> {
            code: 4100,
            parent_code: None,
            description: "LOCAL & SUBURBAN TRANSIT & INTERURBAN HWY PASSENGER TRANS".to_string(),
        }),
        (4210, Code::<u16> {
            code: 4210,
            parent_code: None,
            description: "TRUCKING & COURIER SERVICES (NO AIR)".to_string(),
        }),
        (4213, Code::<u16> {
            code: 4213,
            parent_code: None,
            description: "TRUCKING (NO LOCAL)".to_string(),
        }),
        (4220, Code::<u16> {
            code: 4220,
            parent_code: None,
            description: "PUBLIC WAREHOUSING & STORAGE".to_string(),
        }),
        (4231, Code::<u16> {
            code: 4231,
            parent_code: None,
            description: "TERMINAL MAINTENANCE FACILITIES FOR MOTOR FREIGHT TRANSPORT".to_string(),
        }),
        (4400, Code::<u16> {
            code: 4400,
            parent_code: None,
            description: "WATER TRANSPORTATION".to_string(),
        }),
        (4412, Code::<u16> {
            code: 4412,
            parent_code: None,
            description: "DEEP SEA FOREIGN TRANSPORTATION OF FREIGHT".to_string(),
        }),
        (4512, Code::<u16> {
            code: 4512,
            parent_code: None,
            description: "AIR TRANSPORTATION, SCHEDULED".to_string(),
        }),
        (4513, Code::<u16> {
            code: 4513,
            parent_code: None,
            description: "AIR COURIER SERVICES".to_string(),
        }),
        (4522, Code::<u16> {
            code: 4522,
            parent_code: None,
            description: "AIR TRANSPORTATION, NONSCHEDULED".to_string(),
        }),
        (4581, Code::<u16> {
            code: 4581,
            parent_code: None,
            description: "AIRPORTS, FLYING FIELDS & AIRPORT TERMINAL SERVICES".to_string(),
        }),
        (4610, Code::<u16> {
            code: 4610,
            parent_code: None,
            description: "PIPE LINES (NO NATURAL GAS)".to_string(),
        }),
        (4700, Code::<u16> {
            code: 4700,
            parent_code: None,
            description: "TRANSPORTATION SERVICES".to_string(),
        }),
        (4731, Code::<u16> {
            code: 4731,
            parent_code: None,
            description: "ARRANGEMENT OF TRANSPORTATION OF FREIGHT & CARGO".to_string(),
        }),
        (4812, Code::<u16> {
            code: 4812,
            parent_code: None,
            description: "RADIOTELEPHONE COMMUNICATIONS".to_string(),
        }),
        (4813, Code::<u16> {
            code: 4813,
            parent_code: None,
            description: "TELEPHONE COMMUNICATIONS (NO RADIOTELEPHONE)".to_string(),
        }),
        (4822, Code::<u16> {
            code: 4822,
            parent_code: None,
            description: "TELEGRAPH & OTHER MESSAGE COMMUNICATIONS".to_string(),
        }),
        (4832, Code::<u16> {
            code: 4832,
            parent_code: None,
            description: "RADIO BROADCASTING STATIONS".to_string(),
        }),
        (4833, Code::<u16> {
            code: 4833,
            parent_code: None,
            description: "TELEVISION BROADCASTING STATIONS".to_string(),
        }),
        (4841, Code::<u16> {
            code: 4841,
            parent_code: None,
            description: "CABLE & OTHER PAY TELEVISION SERVICES".to_string(),
        }),
        (4899, Code::<u16> {
            code: 4899,
            parent_code: None,
            description: "COMMUNICATIONS SERVICES, NEC".to_string(),
        }),
        (4900, Code::<u16> {
            code: 4900,
            parent_code: None,
            description: "ELECTRIC, GAS & SANITARY SERVICES".to_string(),
        }),
        (4911, Code::<u16> {
            code: 4911,
            parent_code: None,
            description: "ELECTRIC SERVICES".to_string(),
        }),
        (4922, Code::<u16> {
            code: 4922,
            parent_code: None,
            description: "NATURAL GAS TRANSMISSION".to_string(),
        }),
        (4923, Code::<u16> {
            code: 4923,
            parent_code: None,
            description: "NATURAL GAS TRANSMISSION & DISTRIBUTION".to_string(),
        }),
        (4924, Code::<u16> {
            code: 4924,
            parent_code: None,
            description: "NATURAL GAS DISTRIBUTION".to_string(),
        }),
        (4931, Code::<u16> {
            code: 4931,
            parent_code: None,
            description: "ELECTRIC & OTHER SERVICES COMBINED".to_string(),
        }),
        (4932, Code::<u16> {
            code: 4932,
            parent_code: None,
            description: "GAS & OTHER SERVICES COMBINED".to_string(),
        }),
        (4941, Code::<u16> {
            code: 4941,
            parent_code: None,
            description: "WATER SUPPLY".to_string(),
        }),
        (4950, Code::<u16> {
            code: 4950,
            parent_code: None,
            description: "SANITARY SERVICES".to_string(),
        }),
        (4953, Code::<u16> {
            code: 4953,
            parent_code: None,
            description: "REFUSE SYSTEMS".to_string(),
        }),
        (4955, Code::<u16> {
            code: 4955,
            parent_code: None,
            description: "HAZARDOUS WASTE MANAGEMENT".to_string(),
        }),
        (4961, Code::<u16> {
            code: 4961,
            parent_code: None,
            description: "STEAM & AIR-CONDITIONING SUPPLY".to_string(),
        }),
        (4991, Code::<u16> {
            code: 4991,
            parent_code: None,
            description: "CO-GENERATION SERVICES & SMALL POWER PRODUCERS".to_string(),
        }),
        (5000, Code::<u16> {
            code: 5000,
            parent_code: None,
            description: "WHOLESALE-DURABLE GOODS".to_string(),
        }),
        (5010, Code::<u16> {
            code: 5010,
            parent_code: None,
            description: "WHOLESALE-MOTOR VEHICLES & MOTOR VEHICLE PARTS & SUPPLIES".to_string(),
        }),
        (5013, Code::<u16> {
            code: 5013,
            parent_code: None,
            description: "WHOLESALE-MOTOR VEHICLE SUPPLIES & NEW PARTS".to_string(),
        }),
        (5020, Code::<u16> {
            code: 5020,
            parent_code: None,
            description: "WHOLESALE-FURNITURE & HOME FURNISHINGS".to_string(),
        }),
        (5030, Code::<u16> {
            code: 5030,
            parent_code: None,
            description: "WHOLESALE-LUMBER & OTHER CONSTRUCTION MATERIALS".to_string(),
        }),
        (5031, Code::<u16> {
            code: 5031,
            parent_code: None,
            description: "WHOLESALE-LUMBER, PLYWOOD, MILLWORK & WOOD PANELS".to_string(),
        }),
        (5040, Code::<u16> {
            code: 5040,
            parent_code: None,
            description: "WHOLESALE-PROFESSIONAL & COMMERCIAL EQUIPMENT & SUPPLIES".to_string(),
        }),
        (5045, Code::<u16> {
            code: 5045,
            parent_code: None,
            description: "WHOLESALE-COMPUTERS & PERIPHERAL EQUIPMENT & SOFTWARE".to_string(),
        }),
        (5047, Code::<u16> {
            code: 5047,
            parent_code: None,
            description: "WHOLESALE-MEDICAL, DENTAL & HOSPITAL EQUIPMENT & SUPPLIES".to_string(),
        }),
        (5050, Code::<u16> {
            code: 5050,
            parent_code: None,
            description: "WHOLESALE-METALS & MINERALS (NO PETROLEUM)".to_string(),
        }),
        (5051, Code::<u16> {
            code: 5051,
            parent_code: None,
            description: "WHOLESALE-METALS SERVICE CENTERS & OFFICES".to_string(),
        }),
        (5063, Code::<u16> {
            code: 5063,
            parent_code: None,
            description: "WHOLESALE-ELECTRICAL APPARATUS & EQUIPMENT, WIRING SUPPLIES".to_string(),
        }),
        (5064, Code::<u16> {
            code: 5064,
            parent_code: None,
            description: "WHOLESALE-ELECTRICAL APPLIANCES, TV & RADIO SETS".to_string(),
        }),
        (5065, Code::<u16> {
            code: 5065,
            parent_code: None,
            description: "WHOLESALE-ELECTRONIC PARTS & EQUIPMENT, NEC".to_string(),
        }),
        (5070, Code::<u16> {
            code: 5070,
            parent_code: None,
            description: "WHOLESALE-HARDWARE & PLUMBING & HEATING EQUIPMENT & SUPPLIES".to_string(),
        }),
        (5072, Code::<u16> {
            code: 5072,
            parent_code: None,
            description: "WHOLESALE-HARDWARE".to_string(),
        }),
        (5080, Code::<u16> {
            code: 5080,
            parent_code: None,
            description: "WHOLESALE-MACHINERY, EQUIPMENT & SUPPLIES".to_string(),
        }),
        (5082, Code::<u16> {
            code: 5082,
            parent_code: None,
            description: "WHOLESALE-CONSTRUCTION & MINING (NO PETRO) MACHINERY & EQUIP".to_string(),
        }),
        (5084, Code::<u16> {
            code: 5084,
            parent_code: None,
            description: "WHOLESALE-INDUSTRIAL MACHINERY & EQUIPMENT".to_string(),
        }),
        (5090, Code::<u16> {
            code: 5090,
            parent_code: None,
            description: "WHOLESALE-MISC DURABLE GOODS".to_string(),
        }),
        (5094, Code::<u16> {
            code: 5094,
            parent_code: None,
            description: "WHOLESALE-JEWELRY, WATCHES, PRECIOUS STONES & METALS".to_string(),
        }),
        (5099, Code::<u16> {
            code: 5099,
            parent_code: None,
            description: "WHOLESALE-DURABLE GOODS, NEC".to_string(),
        }),
        (5110, Code::<u16> {
            code: 5110,
            parent_code: None,
            description: "WHOLESALE-PAPER & PAPER PRODUCTS".to_string(),
        }),
        (5122, Code::<u16> {
            code: 5122,
            parent_code: None,
            description: "WHOLESALE-DRUGS, PROPRIETARIES & DRUGGISTS' SUNDRIES".to_string(),
        }),
        (5130, Code::<u16> {
            code: 5130,
            parent_code: None,
            description: "WHOLESALE-APPAREL, PIECE GOODS & NOTIONS".to_string(),
        }),
        (5140, Code::<u16> {
            code: 5140,
            parent_code: None,
            description: "WHOLESALE-GROCERIES & RELATED PRODUCTS".to_string(),
        }),
        (5141, Code::<u16> {
            code: 5141,
            parent_code: None,
            description: "WHOLESALE-GROCERIES, GENERAL LINE".to_string(),
        }),
        (5150, Code::<u16> {
            code: 5150,
            parent_code: None,
            description: "WHOLESALE-FARM PRODUCT RAW MATERIALS".to_string(),
        }),
        (5160, Code::<u16> {
            code: 5160,
            parent_code: None,
            description: "WHOLESALE-CHEMICALS & ALLIED PRODUCTS".to_string(),
        }),
        (5171, Code::<u16> {
            code: 5171,
            parent_code: None,
            description: "WHOLESALE-PETROLEUM BULK STATIONS & TERMINALS".to_string(),
        }),
        (5172, Code::<u16> {
            code: 5172,
            parent_code: None,
            description: "WHOLESALE-PETROLEUM & PETROLEUM PRODUCTS (NO BULK STATIONS)".to_string(),
        }),
        (5180, Code::<u16> {
            code: 5180,
            parent_code: None,
            description: "WHOLESALE-BEER, WINE & DISTILLED ALCOHOLIC BEVERAGES".to_string(),
        }),
        (5190, Code::<u16> {
            code: 5190,
            parent_code: None,
            description: "WHOLESALE-MISCELLANEOUS NON-DURABLE GOODS".to_string(),
        }),
        (5200, Code::<u16> {
            code: 5200,
            parent_code: None,
            description: "RETAIL-BUILDING MATERIALS, HARDWARE, GARDEN SUPPLY".to_string(),
        }),
        (5211, Code::<u16> {
            code: 5211,
            parent_code: None,
            description: "RETAIL-LUMBER & OTHER BUILDING MATERIALS DEALERS".to_string(),
        }),
        (5271, Code::<u16> {
            code: 5271,
            parent_code: None,
            description: "RETAIL-MOBILE HOME DEALERS".to_string(),
        }),
        (5311, Code::<u16> {
            code: 5311,
            parent_code: None,
            description: "RETAIL-DEPARTMENT STORES".to_string(),
        }),
        (5331, Code::<u16> {
            code: 5331,
            parent_code: None,
            description: "RETAIL-VARIETY STORES".to_string(),
        }),
        (5399, Code::<u16> {
            code: 5399,
            parent_code: None,
            description: "RETAIL-MISC GENERAL MERCHANDISE STORES".to_string(),
        }),
        (5400, Code::<u16> {
            code: 5400,
            parent_code: None,
            description: "RETAIL-FOOD STORES".to_string(),
        }),
        (5411, Code::<u16> {
            code: 5411,
            parent_code: None,
            description: "RETAIL-GROCERY STORES".to_string(),
        }),
        (5412, Code::<u16> {
            code: 5412,
            parent_code: None,
            description: "RETAIL-CONVENIENCE STORES".to_string(),
        }),
        (5500, Code::<u16> {
            code: 5500,
            parent_code: None,
            description: "RETAIL-AUTO DEALERS & GASOLINE STATIONS".to_string(),
        }),
        (5531, Code::<u16> {
            code: 5531,
            parent_code: None,
            description: "RETAIL-AUTO & HOME SUPPLY STORES".to_string(),
        }),
        (5600, Code::<u16> {
            code: 5600,
            parent_code: None,
            description: "RETAIL-APPAREL & ACCESSORY STORES".to_string(),
        }),
        (5621, Code::<u16> {
            code: 5621,
            parent_code: None,
            description: "RETAIL-WOMEN'S CLOTHING STORES".to_string(),
        }),
        (5651, Code::<u16> {
            code: 5651,
            parent_code: None,
            description: "RETAIL-FAMILY CLOTHING STORES".to_string(),
        }),
        (5661, Code::<u16> {
            code: 5661,
            parent_code: None,
            description: "RETAIL-SHOE STORES".to_string(),
        }),
        (5700, Code::<u16> {
            code: 5700,
            parent_code: None,
            description: "RETAIL-HOME FURNITURE, FURNISHINGS & EQUIPMENT STORES".to_string(),
        }),
        (5712, Code::<u16> {
            code: 5712,
            parent_code: None,
            description: "RETAIL-FURNITURE STORES".to_string(),
        }),
        (5731, Code::<u16> {
            code: 5731,
            parent_code: None,
            description: "RETAIL-RADIO, TV & CONSUMER ELECTRONICS STORES".to_string(),
        }),
        (5734, Code::<u16> {
            code: 5734,
            parent_code: None,
            description: "RETAIL-COMPUTER & COMPUTER SOFTWARE STORES".to_string(),
        }),
        (5735, Code::<u16> {
            code: 5735,
            parent_code: None,
            description: "RETAIL-RECORD & PRERECORDED TAPE STORES".to_string(),
        }),
        (5810, Code::<u16> {
            code: 5810,
            parent_code: None,
            description: "RETAIL-EATING & DRINKING PLACES".to_string(),
        }),
        (5812, Code::<u16> {
            code: 5812,
            parent_code: None,
            description: "RETAIL-EATING PLACES".to_string(),
        }),
        (5900, Code::<u16> {
            code: 5900,
            parent_code: None,
            description: "RETAIL-MISCELLANEOUS RETAIL".to_string(),
        }),
        (5912, Code::<u16> {
            code: 5912,
            parent_code: None,
            description: "RETAIL-DRUG STORES AND PROPRIETARY STORES".to_string(),
        }),
        (5940, Code::<u16> {
            code: 5940,
            parent_code: None,
            description: "RETAIL-MISCELLANEOUS SHOPPING GOODS STORES".to_string(),
        }),
        (5944, Code::<u16> {
            code: 5944,
            parent_code: None,
            description: "RETAIL-JEWELRY STORES".to_string(),
        }),
        (5945, Code::<u16> {
            code: 5945,
            parent_code: None,
            description: "RETAIL-HOBBY, TOY & GAME SHOPS".to_string(),
        }),
        (5960, Code::<u16> {
            code: 5960,
            parent_code: None,
            description: "RETAIL-NON-STORE RETAILERS".to_string(),
        }),
        (5961, Code::<u16> {
            code: 5961,
            parent_code: None,
            description: "RETAIL-CATALOG & MAIL-ORDER HOUSES".to_string(),
        }),
        (5990, Code::<u16> {
            code: 5990,
            parent_code: None,
            description: "RETAIL-RETAIL STORES, NEC".to_string(),
        }),
        (6021, Code::<u16> {
            code: 6021,
            parent_code: None,
            description: "NATIONAL COMMERCIAL BANKS".to_string(),
        }),
        (6022, Code::<u16> {
            code: 6022,
            parent_code: None,
            description: "STATE COMMERCIAL BANKS".to_string(),
        }),
        (6029, Code::<u16> {
            code: 6029,
            parent_code: None,
            description: "COMMERCIAL BANKS, NEC".to_string(),
        }),
        (6035, Code::<u16> {
            code: 6035,
            parent_code: None,
            description: "SAVINGS INSTITUTIONS, FEDERALLY CHARTERED".to_string(),
        }),
        (6036, Code::<u16> {
            code: 6036,
            parent_code: None,
            description: "SAVINGS INSTITUTIONS, NOT FEDERALLY CHARTERED".to_string(),
        }),
        (6099, Code::<u16> {
            code: 6099,
            parent_code: None,
            description: "FUNCTIONS RELATED TO DEPOSITORY BANKING, NEC".to_string(),
        }),
        (6111, Code::<u16> {
            code: 6111,
            parent_code: None,
            description: "FEDERAL & FEDERALLY-SPONSORED CREDIT AGENCIES".to_string(),
        }),
        (6141, Code::<u16> {
            code: 6141,
            parent_code: None,
            description: "PERSONAL CREDIT INSTITUTIONS".to_string(),
        }),
        (6153, Code::<u16> {
            code: 6153,
            parent_code: None,
            description: "SHORT-TERM BUSINESS CREDIT INSTITUTIONS".to_string(),
        }),
        (6159, Code::<u16> {
            code: 6159,
            parent_code: None,
            description: "MISCELLANEOUS BUSINESS CREDIT INSTITUTION".to_string(),
        }),
        (6162, Code::<u16> {
            code: 6162,
            parent_code: None,
            description: "MORTGAGE BANKERS & LOAN CORRESPONDENTS".to_string(),
        }),
        (6163, Code::<u16> {
            code: 6163,
            parent_code: None,
            description: "LOAN BROKERS".to_string(),
        }),
        (6172, Code::<u16> {
            code: 6172,
            parent_code: None,
            description: "FINANCE LESSORS".to_string(),
        }),
        (6189, Code::<u16> {
            code: 6189,
            parent_code: None,
            description: "ASSET-BACKED SECURITIES".to_string(),
        }),
        (6199, Code::<u16> {
            code: 6199,
            parent_code: None,
            description: "FINANCE SERVICES".to_string(),
        }),
        (6200, Code::<u16> {
            code: 6200,
            parent_code: None,
            description: "SECURITY & COMMODITY BROKERS, DEALERS, EXCHANGES & SERVICES".to_string(),
        }),
        (6211, Code::<u16> {
            code: 6211,
            parent_code: None,
            description: "SECURITY BROKERS, DEALERS & FLOTATION COMPANIES".to_string(),
        }),
        (6221, Code::<u16> {
            code: 6221,
            parent_code: None,
            description: "COMMODITY CONTRACTS BROKERS & DEALERS".to_string(),
        }),
        (6282, Code::<u16> {
            code: 6282,
            parent_code: None,
            description: "INVESTMENT ADVICE".to_string(),
        }),
        (6311, Code::<u16> {
            code: 6311,
            parent_code: None,
            description: "LIFE INSURANCE".to_string(),
        }),
        (6321, Code::<u16> {
            code: 6321,
            parent_code: None,
            description: "ACCIDENT & HEALTH INSURANCE".to_string(),
        }),
        (6324, Code::<u16> {
            code: 6324,
            parent_code: None,
            description: "HOSPITAL & MEDICAL SERVICE PLANS".to_string(),
        }),
        (6331, Code::<u16> {
            code: 6331,
            parent_code: None,
            description: "FIRE, MARINE & CASUALTY INSURANCE".to_string(),
        }),
        (6351, Code::<u16> {
            code: 6351,
            parent_code: None,
            description: "SURETY INSURANCE".to_string(),
        }),
        (6361, Code::<u16> {
            code: 6361,
            parent_code: None,
            description: "TITLE INSURANCE".to_string(),
        }),
        (6399, Code::<u16> {
            code: 6399,
            parent_code: None,
            description: "INSURANCE CARRIERS, NEC".to_string(),
        }),
        (6411, Code::<u16> {
            code: 6411,
            parent_code: None,
            description: "INSURANCE AGENTS, BROKERS & SERVICE".to_string(),
        }),
        (6500, Code::<u16> {
            code: 6500,
            parent_code: None,
            description: "REAL ESTATE".to_string(),
        }),
        (6510, Code::<u16> {
            code: 6510,
            parent_code: None,
            description: "REAL ESTATE OPERATORS (NO DEVELOPERS) & LESSORS".to_string(),
        }),
        (6512, Code::<u16> {
            code: 6512,
            parent_code: None,
            description: "OPERATORS OF NONRESIDENTIAL BUILDINGS".to_string(),
        }),
        (6513, Code::<u16> {
            code: 6513,
            parent_code: None,
            description: "OPERATORS OF APARTMENT BUILDINGS".to_string(),
        }),
        (6519, Code::<u16> {
            code: 6519,
            parent_code: None,
            description: "LESSORS OF REAL PROPERTY, NEC".to_string(),
        }),
        (6531, Code::<u16> {
            code: 6531,
            parent_code: None,
            description: "REAL ESTATE AGENTS & MANAGERS (FOR OTHERS)".to_string(),
        }),
        (6532, Code::<u16> {
            code: 6532,
            parent_code: None,
            description: "REAL ESTATE DEALERS (FOR THEIR OWN ACCOUNT)".to_string(),
        }),
        (6552, Code::<u16> {
            code: 6552,
            parent_code: None,
            description: "LAND SUBDIVIDERS & DEVELOPERS (NO CEMETERIES)".to_string(),
        }),
        (6770, Code::<u16> {
            code: 6770,
            parent_code: None,
            description: "BLANK CHECKS".to_string(),
        }),
        (6792, Code::<u16> {
            code: 6792,
            parent_code: None,
            description: "OIL ROYALTY TRADERS".to_string(),
        }),
        (6794, Code::<u16> {
            code: 6794,
            parent_code: None,
            description: "PATENT OWNERS & LESSORS".to_string(),
        }),
        (6795, Code::<u16> {
            code: 6795,
            parent_code: None,
            description: "MINERAL ROYALTY TRADERS".to_string(),
        }),
        (6798, Code::<u16> {
            code: 6798,
            parent_code: None,
            description: "REAL ESTATE INVESTMENT TRUSTS".to_string(),
        }),
        (6799, Code::<u16> {
            code: 6799,
            parent_code: None,
            description: "INVESTORS, NEC".to_string(),
        }),
        (7000, Code::<u16> {
            code: 7000,
            parent_code: None,
            description: "HOTELS, ROOMING HOUSES, CAMPS & OTHER LODGING PLACES".to_string(),
        }),
        (7011, Code::<u16> {
            code: 7011,
            parent_code: None,
            description: "HOTELS & MOTELS".to_string(),
        }),
        (7200, Code::<u16> {
            code: 7200,
            parent_code: None,
            description: "SERVICES-PERSONAL SERVICES".to_string(),
        }),
        (7310, Code::<u16> {
            code: 7310,
            parent_code: None,
            description: "SERVICES-ADVERTISING".to_string(),
        }),
        (7311, Code::<u16> {
            code: 7311,
            parent_code: None,
            description: "SERVICES-ADVERTISING AGENCIES".to_string(),
        }),
        (7320, Code::<u16> {
            code: 7320,
            parent_code: None,
            description: "SERVICES-CONSUMER CREDIT REPORTING, COLLECTION AGENCIES".to_string(),
        }),
        (7330, Code::<u16> {
            code: 7330,
            parent_code: None,
            description: "SERVICES-MAILING, REPRODUCTION, COMMERCIAL ART & PHOTOGRAPHY".to_string(),
        }),
        (7331, Code::<u16> {
            code: 7331,
            parent_code: None,
            description: "SERVICES-DIRECT MAIL ADVERTISING SERVICES".to_string(),
        }),
        (7340, Code::<u16> {
            code: 7340,
            parent_code: None,
            description: "SERVICES-TO DWELLINGS & OTHER BUILDINGS".to_string(),
        }),
        (7350, Code::<u16> {
            code: 7350,
            parent_code: None,
            description: "SERVICES-MISCELLANEOUS EQUIPMENT RENTAL & LEASING".to_string(),
        }),
        (7359, Code::<u16> {
            code: 7359,
            parent_code: None,
            description: "SERVICES-EQUIPMENT RENTAL & LEASING, NEC".to_string(),
        }),
        (7361, Code::<u16> {
            code: 7361,
            parent_code: None,
            description: "SERVICES-EMPLOYMENT AGENCIES".to_string(),
        }),
        (7363, Code::<u16> {
            code: 7363,
            parent_code: None,
            description: "SERVICES-HELP SUPPLY SERVICES".to_string(),
        }),
        (7370, Code::<u16> {
            code: 7370,
            parent_code: None,
            description: "SERVICES-COMPUTER PROGRAMMING, DATA PROCESSING, ETC.".to_string(),
        }),
        (7371, Code::<u16> {
            code: 7371,
            parent_code: None,
            description: "SERVICES-COMPUTER PROGRAMMING SERVICES".to_string(),
        }),
        (7372, Code::<u16> {
            code: 7372,
            parent_code: None,
            description: "SERVICES-PREPACKAGED SOFTWARE".to_string(),
        }),
        (7373, Code::<u16> {
            code: 7373,
            parent_code: None,
            description: "SERVICES-COMPUTER INTEGRATED SYSTEMS DESIGN".to_string(),
        }),
        (7374, Code::<u16> {
            code: 7374,
            parent_code: None,
            description: "SERVICES-COMPUTER PROCESSING & DATA PREPARATION".to_string(),
        }),
        (7377, Code::<u16> {
            code: 7377,
            parent_code: None,
            description: "SERVICES-COMPUTER RENTAL & LEASING".to_string(),
        }),
        (7380, Code::<u16> {
            code: 7380,
            parent_code: None,
            description: "SERVICES-MISCELLANEOUS BUSINESS SERVICES".to_string(),
        }),
        (7381, Code::<u16> {
            code: 7381,
            parent_code: None,
            description: "SERVICES-DETECTIVE, GUARD & ARMORED CAR SERVICES".to_string(),
        }),
        (7384, Code::<u16> {
            code: 7384,
            parent_code: None,
            description: "SERVICES-PHOTOFINISHING LABORATORIES".to_string(),
        }),
        (7385, Code::<u16> {
            code: 7385,
            parent_code: None,
            description: "SERVICES-TELEPHONE INTERCONNECT SYSTEMS".to_string(),
        }),
        (7389, Code::<u16> {
            code: 7389,
            parent_code: None,
            description: "SERVICES-BUSINESS SERVICES, NEC".to_string(),
        }),
        (7500, Code::<u16> {
            code: 7500,
            parent_code: None,
            description: "SERVICES-AUTOMOTIVE REPAIR, SERVICES & PARKING".to_string(),
        }),
        (7510, Code::<u16> {
            code: 7510,
            parent_code: None,
            description: "SERVICES-AUTO RENTAL & LEASING (NO DRIVERS)".to_string(),
        }),
        (7600, Code::<u16> {
            code: 7600,
            parent_code: None,
            description: "SERVICES-MISCELLANEOUS REPAIR SERVICES".to_string(),
        }),
        (7812, Code::<u16> {
            code: 7812,
            parent_code: None,
            description: "SERVICES-MOTION PICTURE & VIDEO TAPE PRODUCTION".to_string(),
        }),
        (7819, Code::<u16> {
            code: 7819,
            parent_code: None,
            description: "SERVICES-ALLIED TO MOTION PICTURE PRODUCTION".to_string(),
        }),
        (7822, Code::<u16> {
            code: 7822,
            parent_code: None,
            description: "SERVICES-MOTION PICTURE & VIDEO TAPE DISTRIBUTION".to_string(),
        }),
        (7829, Code::<u16> {
            code: 7829,
            parent_code: None,
            description: "SERVICES-ALLIED TO MOTION PICTURE DISTRIBUTION".to_string(),
        }),
        (7830, Code::<u16> {
            code: 7830,
            parent_code: None,
            description: "SERVICES-MOTION PICTURE THEATERS".to_string(),
        }),
        (7841, Code::<u16> {
            code: 7841,
            parent_code: None,
            description: "SERVICES-VIDEO TAPE RENTAL".to_string(),
        }),
        (7900, Code::<u16> {
            code: 7900,
            parent_code: None,
            description: "SERVICES-AMUSEMENT & RECREATION SERVICES".to_string(),
        }),
        (7948, Code::<u16> {
            code: 7948,
            parent_code: None,
            description: "SERVICES-RACING, INCLUDING TRACK OPERATION".to_string(),
        }),
        (7990, Code::<u16> {
            code: 7990,
            parent_code: None,
            description: "SERVICES-MISCELLANEOUS AMUSEMENT & RECREATION".to_string(),
        }),
        (7997, Code::<u16> {
            code: 7997,
            parent_code: None,
            description: "SERVICES-MEMBERSHIP SPORTS & RECREATION CLUBS".to_string(),
        }),
        (8000, Code::<u16> {
            code: 8000,
            parent_code: None,
            description: "SERVICES-HEALTH SERVICES".to_string(),
        }),
        (8011, Code::<u16> {
            code: 8011,
            parent_code: None,
            description: "SERVICES-OFFICES & CLINICS OF DOCTORS OF MEDICINE".to_string(),
        }),
        (8050, Code::<u16> {
            code: 8050,
            parent_code: None,
            description: "SERVICES-NURSING & PERSONAL CARE FACILITIES".to_string(),
        }),
        (8051, Code::<u16> {
            code: 8051,
            parent_code: None,
            description: "SERVICES-SKILLED NURSING CARE FACILITIES".to_string(),
        }),
        (8060, Code::<u16> {
            code: 8060,
            parent_code: None,
            description: "SERVICES-HOSPITALS".to_string(),
        }),
        (8062, Code::<u16> {
            code: 8062,
            parent_code: None,
            description: "SERVICES-GENERAL MEDICAL & SURGICAL HOSPITALS, NEC".to_string(),
        }),
        (8071, Code::<u16> {
            code: 8071,
            parent_code: None,
            description: "SERVICES-MEDICAL LABORATORIES".to_string(),
        }),
        (8082, Code::<u16> {
            code: 8082,
            parent_code: None,
            description: "SERVICES-HOME HEALTH CARE SERVICES".to_string(),
        }),
        (8090, Code::<u16> {
            code: 8090,
            parent_code: None,
            description: "SERVICES-MISC HEALTH & ALLIED SERVICES, NEC".to_string(),
        }),
        (8093, Code::<u16> {
            code: 8093,
            parent_code: None,
            description: "SERVICES-SPECIALTY OUTPATIENT FACILITIES, NEC".to_string(),
        }),
        (8111, Code::<u16> {
            code: 8111,
            parent_code: None,
            description: "SERVICES-LEGAL SERVICES".to_string(),
        }),
        (8200, Code::<u16> {
            code: 8200,
            parent_code: None,
            description: "SERVICES-EDUCATIONAL SERVICES".to_string(),
        }),
        (8300, Code::<u16> {
            code: 8300,
            parent_code: None,
            description: "SERVICES-SOCIAL SERVICES".to_string(),
        }),
        (8351, Code::<u16> {
            code: 8351,
            parent_code: None,
            description: "SERVICES-CHILD DAY CARE SERVICES".to_string(),
        }),
        (8600, Code::<u16> {
            code: 8600,
            parent_code: None,
            description: "SERVICES-MEMBERSHIP ORGANIZATIONS".to_string(),
        }),
        (8700, Code::<u16> {
            code: 8700,
            parent_code: None,
            description: "SERVICES-ENGINEERING, ACCOUNTING, RESEARCH, MANAGEMENT".to_string(),
        }),
        (8711, Code::<u16> {
            code: 8711,
            parent_code: None,
            description: "SERVICES-ENGINEERING SERVICES".to_string(),
        }),
        (8731, Code::<u16> {
            code: 8731,
            parent_code: None,
            description: "SERVICES-COMMERCIAL PHYSICAL & BIOLOGICAL RESEARCH".to_string(),
        }),
        (8734, Code::<u16> {
            code: 8734,
            parent_code: None,
            description: "SERVICES-TESTING LABORATORIES".to_string(),
        }),
        (8741, Code::<u16> {
            code: 8741,
            parent_code: None,
            description: "SERVICES-MANAGEMENT SERVICES".to_string(),
        }),
        (8742, Code::<u16> {
            code: 8742,
            parent_code: None,
            description: "SERVICES-MANAGEMENT CONSULTING SERVICES".to_string(),
        }),
        (8744, Code::<u16> {
            code: 8744,
            parent_code: None,
            description: "SERVICES-FACILITIES SUPPORT MANAGEMENT SERVICES".to_string(),
        }),
        (8880, Code::<u16> {
            code: 8880,
            parent_code: None,
            description: "AMERICAN DEPOSITARY RECEIPTS".to_string(),
        }),
        (8888, Code::<u16> {
            code: 8888,
            parent_code: None,
            description: "FOREIGN GOVERNMENTS".to_string(),
        }),
        (8900, Code::<u16> {
            code: 8900,
            parent_code: None,
            description: "SERVICES-SERVICES, NEC".to_string(),
        }),
        (9721, Code::<u16> {
            code: 9721,
            parent_code: None,
            description: "INTERNATIONAL AFFAIRS".to_string(),
        }),
        (9995, Code::<u16> {
            code: 9995,
            parent_code: None,
            description: "NON-OPERATING ESTABLISHMENTS".to_string(),
        }),
    ].iter().cloned().collect();
    table
}

