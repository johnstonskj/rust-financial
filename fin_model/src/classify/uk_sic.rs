use std::collections::HashMap;

use chrono::NaiveDate;

use super::{Code, ClassificationScheme};

struct SIC {
    codes: HashMap<u32, Code<u32>>
}

impl ClassificationScheme<u32> for SIC {

    fn new() -> Self {
        SIC { codes: create_data_table() }
    }

    fn name() -> String {
        "Standard industrial classification of economic activities".to_string()
    }

    fn acronym() -> String {
        "SIC".to_string()
    }

    fn source() -> String {
        "https://www.gov.uk/government/uploads/system/uploads/attachment_data/file/527619/SIC07_CH_condensed_list_en.csv".to_string()
    }

    fn governing_body() -> Option<String> {
        Some("UK Companies House".to_string())
    }

    fn last_updated() -> Option<NaiveDate> {
        Some(NaiveDate::from_ymd(2018, 1, 9))
    }

    fn get(&self, code: u32) -> Option<&Code<u32>> {
        self.codes.get(&code)
    }

    fn get_children(&self, _parent: u32) -> Option<Vec<&Code<u32>>> {
        None
    }
}
// ------------------------------------------------------------------------------------------------
    
fn create_data_table() -> HashMap<u32, Code<u32>> {
    let table: HashMap<u32, Code<u32>> =
    [
        (1110, Code::<u32> {
            code: 1110,
            parent_code: None,
            description: "Growing of cereals (except rice), leguminous crops and oil seeds".to_string(),
        }),
        (1120, Code::<u32> {
            code: 1120,
            parent_code: None,
            description: "Growing of rice".to_string(),
        }),
        (1130, Code::<u32> {
            code: 1130,
            parent_code: None,
            description: "Growing of vegetables and melons, roots and tubers".to_string(),
        }),
        (1140, Code::<u32> {
            code: 1140,
            parent_code: None,
            description: "Growing of sugar cane".to_string(),
        }),
        (1150, Code::<u32> {
            code: 1150,
            parent_code: None,
            description: "Growing of tobacco".to_string(),
        }),
        (1160, Code::<u32> {
            code: 1160,
            parent_code: None,
            description: "Growing of fibre crops".to_string(),
        }),
        (1190, Code::<u32> {
            code: 1190,
            parent_code: None,
            description: "Growing of other non-perennial crops".to_string(),
        }),
        (1210, Code::<u32> {
            code: 1210,
            parent_code: None,
            description: "Growing of grapes".to_string(),
        }),
        (1220, Code::<u32> {
            code: 1220,
            parent_code: None,
            description: "Growing of tropical and subtropical fruits".to_string(),
        }),
        (1230, Code::<u32> {
            code: 1230,
            parent_code: None,
            description: "Growing of citrus fruits".to_string(),
        }),
        (1240, Code::<u32> {
            code: 1240,
            parent_code: None,
            description: "Growing of pome fruits and stone fruits".to_string(),
        }),
        (1250, Code::<u32> {
            code: 1250,
            parent_code: None,
            description: "Growing of other tree and bush fruits and nuts".to_string(),
        }),
        (1260, Code::<u32> {
            code: 1260,
            parent_code: None,
            description: "Growing of oleaginous fruits".to_string(),
        }),
        (1270, Code::<u32> {
            code: 1270,
            parent_code: None,
            description: "Growing of beverage crops".to_string(),
        }),
        (1280, Code::<u32> {
            code: 1280,
            parent_code: None,
            description: "Growing of spices, aromatic, drug and pharmaceutical crops".to_string(),
        }),
        (1290, Code::<u32> {
            code: 1290,
            parent_code: None,
            description: "Growing of other perennial crops".to_string(),
        }),
        (1300, Code::<u32> {
            code: 1300,
            parent_code: None,
            description: "Plant propagation".to_string(),
        }),
        (1410, Code::<u32> {
            code: 1410,
            parent_code: None,
            description: "Raising of dairy cattle".to_string(),
        }),
        (1420, Code::<u32> {
            code: 1420,
            parent_code: None,
            description: "Raising of other cattle and buffaloes".to_string(),
        }),
        (1430, Code::<u32> {
            code: 1430,
            parent_code: None,
            description: "Raising of horses and other equines".to_string(),
        }),
        (1440, Code::<u32> {
            code: 1440,
            parent_code: None,
            description: "Raising of camels and camelids".to_string(),
        }),
        (1450, Code::<u32> {
            code: 1450,
            parent_code: None,
            description: "Raising of sheep and  goats".to_string(),
        }),
        (1460, Code::<u32> {
            code: 1460,
            parent_code: None,
            description: "Raising of swine/pigs".to_string(),
        }),
        (1470, Code::<u32> {
            code: 1470,
            parent_code: None,
            description: "Raising of poultry".to_string(),
        }),
        (1490, Code::<u32> {
            code: 1490,
            parent_code: None,
            description: "Raising of other animals".to_string(),
        }),
        (1500, Code::<u32> {
            code: 1500,
            parent_code: None,
            description: "Mixed farming".to_string(),
        }),
        (1610, Code::<u32> {
            code: 1610,
            parent_code: None,
            description: "Support activities for crop production".to_string(),
        }),
        (1621, Code::<u32> {
            code: 1621,
            parent_code: None,
            description: "Farm animal boarding and care".to_string(),
        }),
        (1629, Code::<u32> {
            code: 1629,
            parent_code: None,
            description: "Support activities for animal production (other than farm animal boarding and care) n.e.c.".to_string(),
        }),
        (1630, Code::<u32> {
            code: 1630,
            parent_code: None,
            description: "Post-harvest crop activities".to_string(),
        }),
        (1640, Code::<u32> {
            code: 1640,
            parent_code: None,
            description: "Seed processing for propagation".to_string(),
        }),
        (1700, Code::<u32> {
            code: 1700,
            parent_code: None,
            description: "Hunting, trapping and related service activities".to_string(),
        }),
        (2100, Code::<u32> {
            code: 2100,
            parent_code: None,
            description: "Silviculture and other forestry activities".to_string(),
        }),
        (2200, Code::<u32> {
            code: 2200,
            parent_code: None,
            description: "Logging".to_string(),
        }),
        (2300, Code::<u32> {
            code: 2300,
            parent_code: None,
            description: "Gathering of wild growing non-wood products".to_string(),
        }),
        (2400, Code::<u32> {
            code: 2400,
            parent_code: None,
            description: "Support services to forestry".to_string(),
        }),
        (3110, Code::<u32> {
            code: 3110,
            parent_code: None,
            description: "Marine fishing".to_string(),
        }),
        (3120, Code::<u32> {
            code: 3120,
            parent_code: None,
            description: "Freshwater fishing".to_string(),
        }),
        (3210, Code::<u32> {
            code: 3210,
            parent_code: None,
            description: "Marine aquaculture".to_string(),
        }),
        (3220, Code::<u32> {
            code: 3220,
            parent_code: None,
            description: "Freshwater aquaculture".to_string(),
        }),
        (5101, Code::<u32> {
            code: 5101,
            parent_code: None,
            description: "Deep coal mines".to_string(),
        }),
        (5102, Code::<u32> {
            code: 5102,
            parent_code: None,
            description: "Open cast coal working".to_string(),
        }),
        (5200, Code::<u32> {
            code: 5200,
            parent_code: None,
            description: "Mining of lignite".to_string(),
        }),
        (6100, Code::<u32> {
            code: 6100,
            parent_code: None,
            description: "Extraction of crude petroleum".to_string(),
        }),
        (6200, Code::<u32> {
            code: 6200,
            parent_code: None,
            description: "Extraction of natural gas".to_string(),
        }),
        (7100, Code::<u32> {
            code: 7100,
            parent_code: None,
            description: "Mining of iron ores".to_string(),
        }),
        (7210, Code::<u32> {
            code: 7210,
            parent_code: None,
            description: "Mining of uranium and thorium ores".to_string(),
        }),
        (7290, Code::<u32> {
            code: 7290,
            parent_code: None,
            description: "Mining of other non-ferrous metal ores".to_string(),
        }),
        (8110, Code::<u32> {
            code: 8110,
            parent_code: None,
            description: "Quarrying of ornamental and building stone, limestone, gypsum, chalk and slate".to_string(),
        }),
        (8120, Code::<u32> {
            code: 8120,
            parent_code: None,
            description: "Operation of gravel and sand pits; mining of clays and kaolin".to_string(),
        }),
        (8910, Code::<u32> {
            code: 8910,
            parent_code: None,
            description: "Mining of chemical and fertilizer minerals".to_string(),
        }),
        (8920, Code::<u32> {
            code: 8920,
            parent_code: None,
            description: "Extraction of peat".to_string(),
        }),
        (8930, Code::<u32> {
            code: 8930,
            parent_code: None,
            description: "Extraction of salt".to_string(),
        }),
        (8990, Code::<u32> {
            code: 8990,
            parent_code: None,
            description: "Other mining and quarrying n.e.c.".to_string(),
        }),
        (9100, Code::<u32> {
            code: 9100,
            parent_code: None,
            description: "Support activities for petroleum and natural gas extraction".to_string(),
        }),
        (9900, Code::<u32> {
            code: 9900,
            parent_code: None,
            description: "Support activities for other mining and quarrying".to_string(),
        }),
        (10110, Code::<u32> {
            code: 10110,
            parent_code: None,
            description: "Processing and preserving of meat".to_string(),
        }),
        (10120, Code::<u32> {
            code: 10120,
            parent_code: None,
            description: "Processing and preserving of poultry meat".to_string(),
        }),
        (10130, Code::<u32> {
            code: 10130,
            parent_code: None,
            description: "Production of meat and poultry meat products".to_string(),
        }),
        (10200, Code::<u32> {
            code: 10200,
            parent_code: None,
            description: "Processing and preserving of fish, crustaceans and molluscs".to_string(),
        }),
        (10310, Code::<u32> {
            code: 10310,
            parent_code: None,
            description: "Processing and preserving of potatoes".to_string(),
        }),
        (10320, Code::<u32> {
            code: 10320,
            parent_code: None,
            description: "Manufacture of fruit and vegetable juice".to_string(),
        }),
        (10390, Code::<u32> {
            code: 10390,
            parent_code: None,
            description: "Other processing and preserving of fruit and vegetables".to_string(),
        }),
        (10410, Code::<u32> {
            code: 10410,
            parent_code: None,
            description: "Manufacture of oils and fats".to_string(),
        }),
        (10420, Code::<u32> {
            code: 10420,
            parent_code: None,
            description: "Manufacture of margarine and similar edible fats".to_string(),
        }),
        (10511, Code::<u32> {
            code: 10511,
            parent_code: None,
            description: "Liquid milk and cream production".to_string(),
        }),
        (10512, Code::<u32> {
            code: 10512,
            parent_code: None,
            description: "Butter and cheese production".to_string(),
        }),
        (10519, Code::<u32> {
            code: 10519,
            parent_code: None,
            description: "Manufacture of other milk products".to_string(),
        }),
        (10520, Code::<u32> {
            code: 10520,
            parent_code: None,
            description: "Manufacture of ice cream".to_string(),
        }),
        (10611, Code::<u32> {
            code: 10611,
            parent_code: None,
            description: "Grain milling".to_string(),
        }),
        (10612, Code::<u32> {
            code: 10612,
            parent_code: None,
            description: "Manufacture of breakfast cereals and cereals-based food".to_string(),
        }),
        (10620, Code::<u32> {
            code: 10620,
            parent_code: None,
            description: "Manufacture of starches and starch products".to_string(),
        }),
        (10710, Code::<u32> {
            code: 10710,
            parent_code: None,
            description: "Manufacture of bread; manufacture of fresh pastry goods and cakes".to_string(),
        }),
        (10720, Code::<u32> {
            code: 10720,
            parent_code: None,
            description: "Manufacture of rusks and biscuits; manufacture of preserved pastry goods and cakes".to_string(),
        }),
        (10730, Code::<u32> {
            code: 10730,
            parent_code: None,
            description: "Manufacture of macaroni, noodles, couscous and similar farinaceous products".to_string(),
        }),
        (10810, Code::<u32> {
            code: 10810,
            parent_code: None,
            description: "Manufacture of sugar".to_string(),
        }),
        (10821, Code::<u32> {
            code: 10821,
            parent_code: None,
            description: "Manufacture of cocoa and chocolate confectionery".to_string(),
        }),
        (10822, Code::<u32> {
            code: 10822,
            parent_code: None,
            description: "Manufacture of sugar confectionery".to_string(),
        }),
        (10831, Code::<u32> {
            code: 10831,
            parent_code: None,
            description: "Tea processing".to_string(),
        }),
        (10832, Code::<u32> {
            code: 10832,
            parent_code: None,
            description: "Production of coffee and coffee substitutes".to_string(),
        }),
        (10840, Code::<u32> {
            code: 10840,
            parent_code: None,
            description: "Manufacture of condiments and seasonings".to_string(),
        }),
        (10850, Code::<u32> {
            code: 10850,
            parent_code: None,
            description: "Manufacture of prepared meals and dishes".to_string(),
        }),
        (10860, Code::<u32> {
            code: 10860,
            parent_code: None,
            description: "Manufacture of homogenized food preparations and dietetic food".to_string(),
        }),
        (10890, Code::<u32> {
            code: 10890,
            parent_code: None,
            description: "Manufacture of other food products n.e.c.".to_string(),
        }),
        (10910, Code::<u32> {
            code: 10910,
            parent_code: None,
            description: "Manufacture of prepared feeds for farm animals".to_string(),
        }),
        (10920, Code::<u32> {
            code: 10920,
            parent_code: None,
            description: "Manufacture of prepared pet foods".to_string(),
        }),
        (11010, Code::<u32> {
            code: 11010,
            parent_code: None,
            description: "Distilling, rectifying and blending of spirits".to_string(),
        }),
        (11020, Code::<u32> {
            code: 11020,
            parent_code: None,
            description: "Manufacture of wine from grape".to_string(),
        }),
        (11030, Code::<u32> {
            code: 11030,
            parent_code: None,
            description: "Manufacture of cider and other fruit wines".to_string(),
        }),
        (11040, Code::<u32> {
            code: 11040,
            parent_code: None,
            description: "Manufacture of other non-distilled fermented beverages".to_string(),
        }),
        (11050, Code::<u32> {
            code: 11050,
            parent_code: None,
            description: "Manufacture of beer".to_string(),
        }),
        (11060, Code::<u32> {
            code: 11060,
            parent_code: None,
            description: "Manufacture of malt".to_string(),
        }),
        (11070, Code::<u32> {
            code: 11070,
            parent_code: None,
            description: "Manufacture of soft drinks; production of mineral waters and other bottled waters".to_string(),
        }),
        (12000, Code::<u32> {
            code: 12000,
            parent_code: None,
            description: "Manufacture of tobacco products".to_string(),
        }),
        (13100, Code::<u32> {
            code: 13100,
            parent_code: None,
            description: "Preparation and spinning of textile fibres".to_string(),
        }),
        (13200, Code::<u32> {
            code: 13200,
            parent_code: None,
            description: "Weaving of textiles".to_string(),
        }),
        (13300, Code::<u32> {
            code: 13300,
            parent_code: None,
            description: "Finishing of textiles".to_string(),
        }),
        (13910, Code::<u32> {
            code: 13910,
            parent_code: None,
            description: "Manufacture of knitted and crocheted fabrics".to_string(),
        }),
        (13921, Code::<u32> {
            code: 13921,
            parent_code: None,
            description: "Manufacture of soft furnishings".to_string(),
        }),
        (13922, Code::<u32> {
            code: 13922,
            parent_code: None,
            description: "manufacture of canvas goods, sacks, etc.".to_string(),
        }),
        (13923, Code::<u32> {
            code: 13923,
            parent_code: None,
            description: "manufacture of household textiles".to_string(),
        }),
        (13931, Code::<u32> {
            code: 13931,
            parent_code: None,
            description: "Manufacture of woven or tufted carpets and rugs".to_string(),
        }),
        (13939, Code::<u32> {
            code: 13939,
            parent_code: None,
            description: "Manufacture of other carpets and rugs".to_string(),
        }),
        (13940, Code::<u32> {
            code: 13940,
            parent_code: None,
            description: "Manufacture of cordage, rope, twine and netting".to_string(),
        }),
        (13950, Code::<u32> {
            code: 13950,
            parent_code: None,
            description: "Manufacture of non-wovens and articles made from non-wovens, except apparel".to_string(),
        }),
        (13960, Code::<u32> {
            code: 13960,
            parent_code: None,
            description: "Manufacture of other technical and industrial textiles".to_string(),
        }),
        (13990, Code::<u32> {
            code: 13990,
            parent_code: None,
            description: "Manufacture of other textiles n.e.c.".to_string(),
        }),
        (14110, Code::<u32> {
            code: 14110,
            parent_code: None,
            description: "Manufacture of leather clothes".to_string(),
        }),
        (14120, Code::<u32> {
            code: 14120,
            parent_code: None,
            description: "Manufacture of workwear".to_string(),
        }),
        (14131, Code::<u32> {
            code: 14131,
            parent_code: None,
            description: "Manufacture of other men's outerwear".to_string(),
        }),
        (14132, Code::<u32> {
            code: 14132,
            parent_code: None,
            description: "Manufacture of other women's outerwear".to_string(),
        }),
        (14141, Code::<u32> {
            code: 14141,
            parent_code: None,
            description: "Manufacture of men's underwear".to_string(),
        }),
        (14142, Code::<u32> {
            code: 14142,
            parent_code: None,
            description: "Manufacture of women's underwear".to_string(),
        }),
        (14190, Code::<u32> {
            code: 14190,
            parent_code: None,
            description: "Manufacture of other wearing apparel and accessories n.e.c.".to_string(),
        }),
        (14200, Code::<u32> {
            code: 14200,
            parent_code: None,
            description: "Manufacture of articles of fur".to_string(),
        }),
        (14310, Code::<u32> {
            code: 14310,
            parent_code: None,
            description: "Manufacture of knitted and crocheted hosiery".to_string(),
        }),
        (14390, Code::<u32> {
            code: 14390,
            parent_code: None,
            description: "Manufacture of other knitted and crocheted apparel".to_string(),
        }),
        (15110, Code::<u32> {
            code: 15110,
            parent_code: None,
            description: "Tanning and dressing of leather; dressing and dyeing of fur".to_string(),
        }),
        (15120, Code::<u32> {
            code: 15120,
            parent_code: None,
            description: "Manufacture of luggage, handbags and the like, saddlery and harness".to_string(),
        }),
        (15200, Code::<u32> {
            code: 15200,
            parent_code: None,
            description: "Manufacture of footwear".to_string(),
        }),
        (16100, Code::<u32> {
            code: 16100,
            parent_code: None,
            description: "Sawmilling and planing of wood".to_string(),
        }),
        (16210, Code::<u32> {
            code: 16210,
            parent_code: None,
            description: "Manufacture of veneer sheets and wood-based panels".to_string(),
        }),
        (16220, Code::<u32> {
            code: 16220,
            parent_code: None,
            description: "Manufacture of assembled parquet floors".to_string(),
        }),
        (16230, Code::<u32> {
            code: 16230,
            parent_code: None,
            description: "Manufacture of other builders' carpentry and joinery".to_string(),
        }),
        (16240, Code::<u32> {
            code: 16240,
            parent_code: None,
            description: "Manufacture of wooden containers".to_string(),
        }),
        (16290, Code::<u32> {
            code: 16290,
            parent_code: None,
            description: "Manufacture of other products of wood; manufacture of articles of cork, straw and plaiting materials".to_string(),
        }),
        (17110, Code::<u32> {
            code: 17110,
            parent_code: None,
            description: "Manufacture of pulp".to_string(),
        }),
        (17120, Code::<u32> {
            code: 17120,
            parent_code: None,
            description: "Manufacture of paper and paperboard".to_string(),
        }),
        (17211, Code::<u32> {
            code: 17211,
            parent_code: None,
            description: "Manufacture of corrugated paper and paperboard, sacks and bags".to_string(),
        }),
        (17219, Code::<u32> {
            code: 17219,
            parent_code: None,
            description: "Manufacture of other paper and paperboard containers".to_string(),
        }),
        (17220, Code::<u32> {
            code: 17220,
            parent_code: None,
            description: "Manufacture of household and sanitary goods and of toilet requisites".to_string(),
        }),
        (17230, Code::<u32> {
            code: 17230,
            parent_code: None,
            description: "Manufacture of paper stationery".to_string(),
        }),
        (17240, Code::<u32> {
            code: 17240,
            parent_code: None,
            description: "Manufacture of wallpaper".to_string(),
        }),
        (17290, Code::<u32> {
            code: 17290,
            parent_code: None,
            description: "Manufacture of other articles of paper and paperboard n.e.c.".to_string(),
        }),
        (18110, Code::<u32> {
            code: 18110,
            parent_code: None,
            description: "Printing of newspapers".to_string(),
        }),
        (18121, Code::<u32> {
            code: 18121,
            parent_code: None,
            description: "Manufacture of printed labels".to_string(),
        }),
        (18129, Code::<u32> {
            code: 18129,
            parent_code: None,
            description: "Printing n.e.c.".to_string(),
        }),
        (18130, Code::<u32> {
            code: 18130,
            parent_code: None,
            description: "Pre-press and pre-media services".to_string(),
        }),
        (18140, Code::<u32> {
            code: 18140,
            parent_code: None,
            description: "Binding and related services".to_string(),
        }),
        (18201, Code::<u32> {
            code: 18201,
            parent_code: None,
            description: "Reproduction of sound recording".to_string(),
        }),
        (18202, Code::<u32> {
            code: 18202,
            parent_code: None,
            description: "Reproduction of video recording".to_string(),
        }),
        (18203, Code::<u32> {
            code: 18203,
            parent_code: None,
            description: "Reproduction of computer media".to_string(),
        }),
        (19100, Code::<u32> {
            code: 19100,
            parent_code: None,
            description: "Manufacture of coke oven products".to_string(),
        }),
        (19201, Code::<u32> {
            code: 19201,
            parent_code: None,
            description: "Mineral oil refining".to_string(),
        }),
        (19209, Code::<u32> {
            code: 19209,
            parent_code: None,
            description: "Other treatment of petroleum products (excluding petrochemicals manufacture)".to_string(),
        }),
        (20110, Code::<u32> {
            code: 20110,
            parent_code: None,
            description: "Manufacture of industrial gases".to_string(),
        }),
        (20120, Code::<u32> {
            code: 20120,
            parent_code: None,
            description: "Manufacture of dyes and pigments".to_string(),
        }),
        (20130, Code::<u32> {
            code: 20130,
            parent_code: None,
            description: "Manufacture of other inorganic basic chemicals".to_string(),
        }),
        (20140, Code::<u32> {
            code: 20140,
            parent_code: None,
            description: "Manufacture of other organic basic chemicals".to_string(),
        }),
        (20150, Code::<u32> {
            code: 20150,
            parent_code: None,
            description: "Manufacture of fertilizers and nitrogen compounds".to_string(),
        }),
        (20160, Code::<u32> {
            code: 20160,
            parent_code: None,
            description: "Manufacture of plastics in primary forms".to_string(),
        }),
        (20170, Code::<u32> {
            code: 20170,
            parent_code: None,
            description: "Manufacture of synthetic rubber in primary forms".to_string(),
        }),
        (20200, Code::<u32> {
            code: 20200,
            parent_code: None,
            description: "Manufacture of pesticides and other agrochemical products".to_string(),
        }),
        (20301, Code::<u32> {
            code: 20301,
            parent_code: None,
            description: "Manufacture of paints, varnishes and similar coatings, mastics and sealants".to_string(),
        }),
        (20302, Code::<u32> {
            code: 20302,
            parent_code: None,
            description: "Manufacture of printing ink".to_string(),
        }),
        (20411, Code::<u32> {
            code: 20411,
            parent_code: None,
            description: "Manufacture of soap and detergents".to_string(),
        }),
        (20412, Code::<u32> {
            code: 20412,
            parent_code: None,
            description: "Manufacture of cleaning and polishing preparations".to_string(),
        }),
        (20420, Code::<u32> {
            code: 20420,
            parent_code: None,
            description: "Manufacture of perfumes and toilet preparations".to_string(),
        }),
        (20510, Code::<u32> {
            code: 20510,
            parent_code: None,
            description: "Manufacture of explosives".to_string(),
        }),
        (20520, Code::<u32> {
            code: 20520,
            parent_code: None,
            description: "Manufacture of glues".to_string(),
        }),
        (20530, Code::<u32> {
            code: 20530,
            parent_code: None,
            description: "Manufacture of essential oils".to_string(),
        }),
        (20590, Code::<u32> {
            code: 20590,
            parent_code: None,
            description: "Manufacture of other chemical products n.e.c.".to_string(),
        }),
        (20600, Code::<u32> {
            code: 20600,
            parent_code: None,
            description: "Manufacture of man-made fibres".to_string(),
        }),
        (21100, Code::<u32> {
            code: 21100,
            parent_code: None,
            description: "Manufacture of basic pharmaceutical products".to_string(),
        }),
        (21200, Code::<u32> {
            code: 21200,
            parent_code: None,
            description: "Manufacture of pharmaceutical preparations".to_string(),
        }),
        (22110, Code::<u32> {
            code: 22110,
            parent_code: None,
            description: "Manufacture of rubber tyres and tubes; retreading and rebuilding of rubber tyres".to_string(),
        }),
        (22190, Code::<u32> {
            code: 22190,
            parent_code: None,
            description: "Manufacture of other rubber products".to_string(),
        }),
        (22210, Code::<u32> {
            code: 22210,
            parent_code: None,
            description: "Manufacture of plastic plates, sheets, tubes and profiles".to_string(),
        }),
        (22220, Code::<u32> {
            code: 22220,
            parent_code: None,
            description: "Manufacture of plastic packing goods".to_string(),
        }),
        (22230, Code::<u32> {
            code: 22230,
            parent_code: None,
            description: "Manufacture of builders  ware of plastic".to_string(),
        }),
        (22290, Code::<u32> {
            code: 22290,
            parent_code: None,
            description: "Manufacture of other plastic products".to_string(),
        }),
        (23110, Code::<u32> {
            code: 23110,
            parent_code: None,
            description: "Manufacture of flat glass".to_string(),
        }),
        (23120, Code::<u32> {
            code: 23120,
            parent_code: None,
            description: "Shaping and processing of flat glass".to_string(),
        }),
        (23130, Code::<u32> {
            code: 23130,
            parent_code: None,
            description: "Manufacture of hollow glass".to_string(),
        }),
        (23140, Code::<u32> {
            code: 23140,
            parent_code: None,
            description: "Manufacture of glass fibres".to_string(),
        }),
        (23190, Code::<u32> {
            code: 23190,
            parent_code: None,
            description: "Manufacture and processing of other glass, including technical glassware".to_string(),
        }),
        (23200, Code::<u32> {
            code: 23200,
            parent_code: None,
            description: "Manufacture of refractory products".to_string(),
        }),
        (23310, Code::<u32> {
            code: 23310,
            parent_code: None,
            description: "Manufacture of ceramic tiles and flags".to_string(),
        }),
        (23320, Code::<u32> {
            code: 23320,
            parent_code: None,
            description: "Manufacture of bricks, tiles and construction products, in baked clay".to_string(),
        }),
        (23410, Code::<u32> {
            code: 23410,
            parent_code: None,
            description: "Manufacture of ceramic household and ornamental articles".to_string(),
        }),
        (23420, Code::<u32> {
            code: 23420,
            parent_code: None,
            description: "Manufacture of ceramic sanitary fixtures".to_string(),
        }),
        (23430, Code::<u32> {
            code: 23430,
            parent_code: None,
            description: "Manufacture of ceramic insulators and insulating fittings".to_string(),
        }),
        (23440, Code::<u32> {
            code: 23440,
            parent_code: None,
            description: "Manufacture of other technical ceramic products".to_string(),
        }),
        (23490, Code::<u32> {
            code: 23490,
            parent_code: None,
            description: "Manufacture of other ceramic products n.e.c.".to_string(),
        }),
        (23510, Code::<u32> {
            code: 23510,
            parent_code: None,
            description: "Manufacture of cement".to_string(),
        }),
        (23520, Code::<u32> {
            code: 23520,
            parent_code: None,
            description: "Manufacture of lime and plaster".to_string(),
        }),
        (23610, Code::<u32> {
            code: 23610,
            parent_code: None,
            description: "Manufacture of concrete products for construction purposes".to_string(),
        }),
        (23620, Code::<u32> {
            code: 23620,
            parent_code: None,
            description: "Manufacture of plaster products for construction purposes".to_string(),
        }),
        (23630, Code::<u32> {
            code: 23630,
            parent_code: None,
            description: "Manufacture of ready-mixed concrete".to_string(),
        }),
        (23640, Code::<u32> {
            code: 23640,
            parent_code: None,
            description: "Manufacture of mortars".to_string(),
        }),
        (23650, Code::<u32> {
            code: 23650,
            parent_code: None,
            description: "Manufacture of fibre cement".to_string(),
        }),
        (23690, Code::<u32> {
            code: 23690,
            parent_code: None,
            description: "Manufacture of other articles of concrete, plaster and cement".to_string(),
        }),
        (23700, Code::<u32> {
            code: 23700,
            parent_code: None,
            description: "Cutting, shaping and finishing of stone".to_string(),
        }),
        (23910, Code::<u32> {
            code: 23910,
            parent_code: None,
            description: "Production of abrasive products".to_string(),
        }),
        (23990, Code::<u32> {
            code: 23990,
            parent_code: None,
            description: "Manufacture of other non-metallic mineral products n.e.c.".to_string(),
        }),
        (24100, Code::<u32> {
            code: 24100,
            parent_code: None,
            description: "Manufacture of basic iron and steel and of ferro-alloys".to_string(),
        }),
        (24200, Code::<u32> {
            code: 24200,
            parent_code: None,
            description: "Manufacture of tubes, pipes, hollow profiles and related fittings, of steel".to_string(),
        }),
        (24310, Code::<u32> {
            code: 24310,
            parent_code: None,
            description: "Cold drawing of bars".to_string(),
        }),
        (24320, Code::<u32> {
            code: 24320,
            parent_code: None,
            description: "Cold rolling of narrow strip".to_string(),
        }),
        (24330, Code::<u32> {
            code: 24330,
            parent_code: None,
            description: "Cold forming or folding".to_string(),
        }),
        (24340, Code::<u32> {
            code: 24340,
            parent_code: None,
            description: "Cold drawing of wire".to_string(),
        }),
        (24410, Code::<u32> {
            code: 24410,
            parent_code: None,
            description: "Precious metals production".to_string(),
        }),
        (24420, Code::<u32> {
            code: 24420,
            parent_code: None,
            description: "Aluminium production".to_string(),
        }),
        (24430, Code::<u32> {
            code: 24430,
            parent_code: None,
            description: "Lead, zinc and tin production".to_string(),
        }),
        (24440, Code::<u32> {
            code: 24440,
            parent_code: None,
            description: "Copper production".to_string(),
        }),
        (24450, Code::<u32> {
            code: 24450,
            parent_code: None,
            description: "Other non-ferrous metal production".to_string(),
        }),
        (24460, Code::<u32> {
            code: 24460,
            parent_code: None,
            description: "Processing of nuclear fuel".to_string(),
        }),
        (24510, Code::<u32> {
            code: 24510,
            parent_code: None,
            description: "Casting of iron".to_string(),
        }),
        (24520, Code::<u32> {
            code: 24520,
            parent_code: None,
            description: "Casting of steel".to_string(),
        }),
        (24530, Code::<u32> {
            code: 24530,
            parent_code: None,
            description: "Casting of light metals".to_string(),
        }),
        (24540, Code::<u32> {
            code: 24540,
            parent_code: None,
            description: "Casting of other non-ferrous metals".to_string(),
        }),
        (25110, Code::<u32> {
            code: 25110,
            parent_code: None,
            description: "Manufacture of metal structures and parts of structures".to_string(),
        }),
        (25120, Code::<u32> {
            code: 25120,
            parent_code: None,
            description: "Manufacture of doors and windows of metal".to_string(),
        }),
        (25210, Code::<u32> {
            code: 25210,
            parent_code: None,
            description: "Manufacture of central heating radiators and boilers".to_string(),
        }),
        (25290, Code::<u32> {
            code: 25290,
            parent_code: None,
            description: "Manufacture of other tanks, reservoirs and containers of metal".to_string(),
        }),
        (25300, Code::<u32> {
            code: 25300,
            parent_code: None,
            description: "Manufacture of steam generators, except central heating hot water boilers".to_string(),
        }),
        (25400, Code::<u32> {
            code: 25400,
            parent_code: None,
            description: "Manufacture of weapons and ammunition".to_string(),
        }),
        (25500, Code::<u32> {
            code: 25500,
            parent_code: None,
            description: "Forging, pressing, stamping and roll-forming of metal; powder metallurgy".to_string(),
        }),
        (25610, Code::<u32> {
            code: 25610,
            parent_code: None,
            description: "Treatment and coating of metals".to_string(),
        }),
        (25620, Code::<u32> {
            code: 25620,
            parent_code: None,
            description: "Machining".to_string(),
        }),
        (25710, Code::<u32> {
            code: 25710,
            parent_code: None,
            description: "Manufacture of cutlery".to_string(),
        }),
        (25720, Code::<u32> {
            code: 25720,
            parent_code: None,
            description: "Manufacture of locks and hinges".to_string(),
        }),
        (25730, Code::<u32> {
            code: 25730,
            parent_code: None,
            description: "Manufacture of tools".to_string(),
        }),
        (25910, Code::<u32> {
            code: 25910,
            parent_code: None,
            description: "Manufacture of steel drums and similar containers".to_string(),
        }),
        (25920, Code::<u32> {
            code: 25920,
            parent_code: None,
            description: "Manufacture of light metal packaging".to_string(),
        }),
        (25930, Code::<u32> {
            code: 25930,
            parent_code: None,
            description: "Manufacture of wire products, chain and springs".to_string(),
        }),
        (25940, Code::<u32> {
            code: 25940,
            parent_code: None,
            description: "Manufacture of fasteners and screw machine products".to_string(),
        }),
        (25990, Code::<u32> {
            code: 25990,
            parent_code: None,
            description: "Manufacture of other fabricated metal products n.e.c.".to_string(),
        }),
        (26110, Code::<u32> {
            code: 26110,
            parent_code: None,
            description: "Manufacture of electronic components".to_string(),
        }),
        (26120, Code::<u32> {
            code: 26120,
            parent_code: None,
            description: "Manufacture of loaded electronic boards".to_string(),
        }),
        (26200, Code::<u32> {
            code: 26200,
            parent_code: None,
            description: "Manufacture of computers and peripheral equipment".to_string(),
        }),
        (26301, Code::<u32> {
            code: 26301,
            parent_code: None,
            description: "Manufacture of telegraph and telephone apparatus and equipment".to_string(),
        }),
        (26309, Code::<u32> {
            code: 26309,
            parent_code: None,
            description: "Manufacture of communication equipment other than telegraph, and telephone apparatus and equipment".to_string(),
        }),
        (26400, Code::<u32> {
            code: 26400,
            parent_code: None,
            description: "Manufacture of consumer electronics".to_string(),
        }),
        (26511, Code::<u32> {
            code: 26511,
            parent_code: None,
            description: "Manufacture of electronic measuring, testing etc. equipment, not for industrial process control".to_string(),
        }),
        (26512, Code::<u32> {
            code: 26512,
            parent_code: None,
            description: "Manufacture of electronic industrial process control equipment".to_string(),
        }),
        (26513, Code::<u32> {
            code: 26513,
            parent_code: None,
            description: "Manufacture of non-electronic measuring, testing etc. equipment, not for industrial process control".to_string(),
        }),
        (26514, Code::<u32> {
            code: 26514,
            parent_code: None,
            description: "Manufacture of non-electronic industrial process control equipment".to_string(),
        }),
        (26520, Code::<u32> {
            code: 26520,
            parent_code: None,
            description: "Manufacture of watches and clocks".to_string(),
        }),
        (26600, Code::<u32> {
            code: 26600,
            parent_code: None,
            description: "Manufacture of irradiation, electromedical and electrotherapeutic equipment".to_string(),
        }),
        (26701, Code::<u32> {
            code: 26701,
            parent_code: None,
            description: "Manufacture of optical precision instruments".to_string(),
        }),
        (26702, Code::<u32> {
            code: 26702,
            parent_code: None,
            description: "Manufacture of photographic and cinematographic equipment".to_string(),
        }),
        (26800, Code::<u32> {
            code: 26800,
            parent_code: None,
            description: "Manufacture of magnetic and optical media".to_string(),
        }),
        (27110, Code::<u32> {
            code: 27110,
            parent_code: None,
            description: "Manufacture of electric motors, generators and transformers".to_string(),
        }),
        (27120, Code::<u32> {
            code: 27120,
            parent_code: None,
            description: "Manufacture of electricity distribution and control apparatus".to_string(),
        }),
        (27200, Code::<u32> {
            code: 27200,
            parent_code: None,
            description: "Manufacture of batteries and accumulators".to_string(),
        }),
        (27310, Code::<u32> {
            code: 27310,
            parent_code: None,
            description: "Manufacture of fibre optic cables".to_string(),
        }),
        (27320, Code::<u32> {
            code: 27320,
            parent_code: None,
            description: "Manufacture of other electronic and electric wires and cables".to_string(),
        }),
        (27330, Code::<u32> {
            code: 27330,
            parent_code: None,
            description: "Manufacture of wiring devices".to_string(),
        }),
        (27400, Code::<u32> {
            code: 27400,
            parent_code: None,
            description: "Manufacture of electric lighting equipment".to_string(),
        }),
        (27510, Code::<u32> {
            code: 27510,
            parent_code: None,
            description: "Manufacture of electric domestic appliances".to_string(),
        }),
        (27520, Code::<u32> {
            code: 27520,
            parent_code: None,
            description: "Manufacture of non-electric domestic appliances".to_string(),
        }),
        (27900, Code::<u32> {
            code: 27900,
            parent_code: None,
            description: "Manufacture of other electrical equipment".to_string(),
        }),
        (28110, Code::<u32> {
            code: 28110,
            parent_code: None,
            description: "Manufacture of engines and turbines, except aircraft, vehicle and cycle engines".to_string(),
        }),
        (28120, Code::<u32> {
            code: 28120,
            parent_code: None,
            description: "Manufacture of fluid power equipment".to_string(),
        }),
        (28131, Code::<u32> {
            code: 28131,
            parent_code: None,
            description: "Manufacture of pumps".to_string(),
        }),
        (28132, Code::<u32> {
            code: 28132,
            parent_code: None,
            description: "Manufacture of compressors".to_string(),
        }),
        (28140, Code::<u32> {
            code: 28140,
            parent_code: None,
            description: "Manufacture of taps and valves".to_string(),
        }),
        (28150, Code::<u32> {
            code: 28150,
            parent_code: None,
            description: "Manufacture of bearings, gears, gearing and driving elements".to_string(),
        }),
        (28210, Code::<u32> {
            code: 28210,
            parent_code: None,
            description: "Manufacture of ovens, furnaces and furnace burners".to_string(),
        }),
        (28220, Code::<u32> {
            code: 28220,
            parent_code: None,
            description: "Manufacture of lifting and handling equipment".to_string(),
        }),
        (28230, Code::<u32> {
            code: 28230,
            parent_code: None,
            description: "Manufacture of office machinery and equipment (except computers and peripheral equipment)".to_string(),
        }),
        (28240, Code::<u32> {
            code: 28240,
            parent_code: None,
            description: "Manufacture of power-driven hand tools".to_string(),
        }),
        (28250, Code::<u32> {
            code: 28250,
            parent_code: None,
            description: "Manufacture of non-domestic cooling and ventilation equipment".to_string(),
        }),
        (28290, Code::<u32> {
            code: 28290,
            parent_code: None,
            description: "Manufacture of other general-purpose machinery n.e.c.".to_string(),
        }),
        (28301, Code::<u32> {
            code: 28301,
            parent_code: None,
            description: "Manufacture of agricultural tractors".to_string(),
        }),
        (28302, Code::<u32> {
            code: 28302,
            parent_code: None,
            description: "Manufacture of agricultural and forestry machinery other than tractors".to_string(),
        }),
        (28410, Code::<u32> {
            code: 28410,
            parent_code: None,
            description: "Manufacture of metal forming machinery".to_string(),
        }),
        (28490, Code::<u32> {
            code: 28490,
            parent_code: None,
            description: "Manufacture of other machine tools".to_string(),
        }),
        (28910, Code::<u32> {
            code: 28910,
            parent_code: None,
            description: "Manufacture of machinery for metallurgy".to_string(),
        }),
        (28921, Code::<u32> {
            code: 28921,
            parent_code: None,
            description: "Manufacture of machinery for mining".to_string(),
        }),
        (28922, Code::<u32> {
            code: 28922,
            parent_code: None,
            description: "Manufacture of earthmoving equipment".to_string(),
        }),
        (28923, Code::<u32> {
            code: 28923,
            parent_code: None,
            description: "Manufacture of equipment for concrete crushing and screening and roadworks".to_string(),
        }),
        (28930, Code::<u32> {
            code: 28930,
            parent_code: None,
            description: "Manufacture of machinery for food, beverage and tobacco processing".to_string(),
        }),
        (28940, Code::<u32> {
            code: 28940,
            parent_code: None,
            description: "Manufacture of machinery for textile, apparel and leather production".to_string(),
        }),
        (28950, Code::<u32> {
            code: 28950,
            parent_code: None,
            description: "Manufacture of machinery for paper and paperboard production".to_string(),
        }),
        (28960, Code::<u32> {
            code: 28960,
            parent_code: None,
            description: "Manufacture of plastics and rubber machinery".to_string(),
        }),
        (28990, Code::<u32> {
            code: 28990,
            parent_code: None,
            description: "Manufacture of other special-purpose machinery n.e.c.".to_string(),
        }),
        (29100, Code::<u32> {
            code: 29100,
            parent_code: None,
            description: "Manufacture of motor vehicles".to_string(),
        }),
        (29201, Code::<u32> {
            code: 29201,
            parent_code: None,
            description: "Manufacture of bodies (coachwork) for motor vehicles (except caravans)".to_string(),
        }),
        (29202, Code::<u32> {
            code: 29202,
            parent_code: None,
            description: "Manufacture of trailers and semi-trailers".to_string(),
        }),
        (29203, Code::<u32> {
            code: 29203,
            parent_code: None,
            description: "Manufacture of caravans".to_string(),
        }),
        (29310, Code::<u32> {
            code: 29310,
            parent_code: None,
            description: "Manufacture of electrical and electronic equipment for motor vehicles and their engines".to_string(),
        }),
        (29320, Code::<u32> {
            code: 29320,
            parent_code: None,
            description: "Manufacture of other parts and accessories for motor vehicles".to_string(),
        }),
        (30110, Code::<u32> {
            code: 30110,
            parent_code: None,
            description: "Building of ships and floating structures".to_string(),
        }),
        (30120, Code::<u32> {
            code: 30120,
            parent_code: None,
            description: "Building of pleasure and sporting boats".to_string(),
        }),
        (30200, Code::<u32> {
            code: 30200,
            parent_code: None,
            description: "Manufacture of railway locomotives and rolling stock".to_string(),
        }),
        (30300, Code::<u32> {
            code: 30300,
            parent_code: None,
            description: "Manufacture of air and spacecraft and related machinery".to_string(),
        }),
        (30400, Code::<u32> {
            code: 30400,
            parent_code: None,
            description: "Manufacture of military fighting vehicles".to_string(),
        }),
        (30910, Code::<u32> {
            code: 30910,
            parent_code: None,
            description: "Manufacture of motorcycles".to_string(),
        }),
        (30920, Code::<u32> {
            code: 30920,
            parent_code: None,
            description: "Manufacture of bicycles and invalid carriages".to_string(),
        }),
        (30990, Code::<u32> {
            code: 30990,
            parent_code: None,
            description: "Manufacture of other transport equipment n.e.c.".to_string(),
        }),
        (31010, Code::<u32> {
            code: 31010,
            parent_code: None,
            description: "Manufacture of office and shop furniture".to_string(),
        }),
        (31020, Code::<u32> {
            code: 31020,
            parent_code: None,
            description: "Manufacture of kitchen furniture".to_string(),
        }),
        (31030, Code::<u32> {
            code: 31030,
            parent_code: None,
            description: "Manufacture of mattresses".to_string(),
        }),
        (31090, Code::<u32> {
            code: 31090,
            parent_code: None,
            description: "Manufacture of other furniture".to_string(),
        }),
        (32110, Code::<u32> {
            code: 32110,
            parent_code: None,
            description: "Striking of coins".to_string(),
        }),
        (32120, Code::<u32> {
            code: 32120,
            parent_code: None,
            description: "Manufacture of jewellery and related articles".to_string(),
        }),
        (32130, Code::<u32> {
            code: 32130,
            parent_code: None,
            description: "Manufacture of imitation jewellery and related articles".to_string(),
        }),
        (32200, Code::<u32> {
            code: 32200,
            parent_code: None,
            description: "Manufacture of musical instruments".to_string(),
        }),
        (32300, Code::<u32> {
            code: 32300,
            parent_code: None,
            description: "Manufacture of sports goods".to_string(),
        }),
        (32401, Code::<u32> {
            code: 32401,
            parent_code: None,
            description: "Manufacture of professional and arcade games and toys".to_string(),
        }),
        (32409, Code::<u32> {
            code: 32409,
            parent_code: None,
            description: "Manufacture of other games and toys, n.e.c.".to_string(),
        }),
        (32500, Code::<u32> {
            code: 32500,
            parent_code: None,
            description: "Manufacture of medical and dental instruments and supplies".to_string(),
        }),
        (32910, Code::<u32> {
            code: 32910,
            parent_code: None,
            description: "Manufacture of brooms and brushes".to_string(),
        }),
        (32990, Code::<u32> {
            code: 32990,
            parent_code: None,
            description: "Other manufacturing n.e.c.".to_string(),
        }),
        (33110, Code::<u32> {
            code: 33110,
            parent_code: None,
            description: "Repair of fabricated metal products".to_string(),
        }),
        (33120, Code::<u32> {
            code: 33120,
            parent_code: None,
            description: "Repair of machinery".to_string(),
        }),
        (33130, Code::<u32> {
            code: 33130,
            parent_code: None,
            description: "Repair of electronic and optical equipment".to_string(),
        }),
        (33140, Code::<u32> {
            code: 33140,
            parent_code: None,
            description: "Repair of electrical equipment".to_string(),
        }),
        (33150, Code::<u32> {
            code: 33150,
            parent_code: None,
            description: "Repair and maintenance of ships and boats".to_string(),
        }),
        (33160, Code::<u32> {
            code: 33160,
            parent_code: None,
            description: "Repair and maintenance of aircraft and spacecraft".to_string(),
        }),
        (33170, Code::<u32> {
            code: 33170,
            parent_code: None,
            description: "Repair and maintenance of other transport equipment n.e.c.".to_string(),
        }),
        (33190, Code::<u32> {
            code: 33190,
            parent_code: None,
            description: "Repair of other equipment".to_string(),
        }),
        (33200, Code::<u32> {
            code: 33200,
            parent_code: None,
            description: "Installation of industrial machinery and equipment".to_string(),
        }),
        (35110, Code::<u32> {
            code: 35110,
            parent_code: None,
            description: "Production of electricity".to_string(),
        }),
        (35120, Code::<u32> {
            code: 35120,
            parent_code: None,
            description: "Transmission of electricity".to_string(),
        }),
        (35130, Code::<u32> {
            code: 35130,
            parent_code: None,
            description: "Distribution of electricity".to_string(),
        }),
        (35140, Code::<u32> {
            code: 35140,
            parent_code: None,
            description: "Trade of electricity".to_string(),
        }),
        (35210, Code::<u32> {
            code: 35210,
            parent_code: None,
            description: "Manufacture of gas".to_string(),
        }),
        (35220, Code::<u32> {
            code: 35220,
            parent_code: None,
            description: "Distribution of gaseous fuels through mains".to_string(),
        }),
        (35230, Code::<u32> {
            code: 35230,
            parent_code: None,
            description: "Trade of gas through mains".to_string(),
        }),
        (35300, Code::<u32> {
            code: 35300,
            parent_code: None,
            description: "Steam and air conditioning supply".to_string(),
        }),
        (36000, Code::<u32> {
            code: 36000,
            parent_code: None,
            description: "Water collection, treatment and supply".to_string(),
        }),
        (37000, Code::<u32> {
            code: 37000,
            parent_code: None,
            description: "Sewerage".to_string(),
        }),
        (38110, Code::<u32> {
            code: 38110,
            parent_code: None,
            description: "Collection of non-hazardous waste".to_string(),
        }),
        (38120, Code::<u32> {
            code: 38120,
            parent_code: None,
            description: "Collection of hazardous waste".to_string(),
        }),
        (38210, Code::<u32> {
            code: 38210,
            parent_code: None,
            description: "Treatment and disposal of non-hazardous waste".to_string(),
        }),
        (38220, Code::<u32> {
            code: 38220,
            parent_code: None,
            description: "Treatment and disposal of hazardous waste".to_string(),
        }),
        (38310, Code::<u32> {
            code: 38310,
            parent_code: None,
            description: "Dismantling of wrecks".to_string(),
        }),
        (38320, Code::<u32> {
            code: 38320,
            parent_code: None,
            description: "Recovery of sorted materials".to_string(),
        }),
        (39000, Code::<u32> {
            code: 39000,
            parent_code: None,
            description: "Remediation activities and other waste management services".to_string(),
        }),
        (41100, Code::<u32> {
            code: 41100,
            parent_code: None,
            description: "Development of building projects".to_string(),
        }),
        (41201, Code::<u32> {
            code: 41201,
            parent_code: None,
            description: "Construction of commercial buildings".to_string(),
        }),
        (41202, Code::<u32> {
            code: 41202,
            parent_code: None,
            description: "Construction of domestic buildings".to_string(),
        }),
        (42110, Code::<u32> {
            code: 42110,
            parent_code: None,
            description: "Construction of roads and motorways".to_string(),
        }),
        (42120, Code::<u32> {
            code: 42120,
            parent_code: None,
            description: "Construction of railways and underground railways".to_string(),
        }),
        (42130, Code::<u32> {
            code: 42130,
            parent_code: None,
            description: "Construction of bridges and tunnels".to_string(),
        }),
        (42210, Code::<u32> {
            code: 42210,
            parent_code: None,
            description: "Construction of utility projects for fluids".to_string(),
        }),
        (42220, Code::<u32> {
            code: 42220,
            parent_code: None,
            description: "Construction of utility projects for electricity and telecommunications".to_string(),
        }),
        (42910, Code::<u32> {
            code: 42910,
            parent_code: None,
            description: "Construction of water projects".to_string(),
        }),
        (42990, Code::<u32> {
            code: 42990,
            parent_code: None,
            description: "Construction of other civil engineering projects n.e.c.".to_string(),
        }),
        (43110, Code::<u32> {
            code: 43110,
            parent_code: None,
            description: "Demolition".to_string(),
        }),
        (43120, Code::<u32> {
            code: 43120,
            parent_code: None,
            description: "Site preparation".to_string(),
        }),
        (43130, Code::<u32> {
            code: 43130,
            parent_code: None,
            description: "Test drilling and boring".to_string(),
        }),
        (43210, Code::<u32> {
            code: 43210,
            parent_code: None,
            description: "Electrical installation".to_string(),
        }),
        (43220, Code::<u32> {
            code: 43220,
            parent_code: None,
            description: "Plumbing, heat and air-conditioning installation".to_string(),
        }),
        (43290, Code::<u32> {
            code: 43290,
            parent_code: None,
            description: "Other construction installation".to_string(),
        }),
        (43310, Code::<u32> {
            code: 43310,
            parent_code: None,
            description: "Plastering".to_string(),
        }),
        (43320, Code::<u32> {
            code: 43320,
            parent_code: None,
            description: "Joinery installation".to_string(),
        }),
        (43330, Code::<u32> {
            code: 43330,
            parent_code: None,
            description: "Floor and wall covering".to_string(),
        }),
        (43341, Code::<u32> {
            code: 43341,
            parent_code: None,
            description: "Painting".to_string(),
        }),
        (43342, Code::<u32> {
            code: 43342,
            parent_code: None,
            description: "Glazing".to_string(),
        }),
        (43390, Code::<u32> {
            code: 43390,
            parent_code: None,
            description: "Other building completion and finishing".to_string(),
        }),
        (43910, Code::<u32> {
            code: 43910,
            parent_code: None,
            description: "Roofing activities".to_string(),
        }),
        (43991, Code::<u32> {
            code: 43991,
            parent_code: None,
            description: "Scaffold erection".to_string(),
        }),
        (43999, Code::<u32> {
            code: 43999,
            parent_code: None,
            description: "Other specialised construction activities n.e.c.".to_string(),
        }),
        (45111, Code::<u32> {
            code: 45111,
            parent_code: None,
            description: "Sale of new cars and light motor vehicles".to_string(),
        }),
        (45112, Code::<u32> {
            code: 45112,
            parent_code: None,
            description: "Sale of used cars and light motor vehicles".to_string(),
        }),
        (45190, Code::<u32> {
            code: 45190,
            parent_code: None,
            description: "Sale of other motor vehicles".to_string(),
        }),
        (45200, Code::<u32> {
            code: 45200,
            parent_code: None,
            description: "Maintenance and repair of motor vehicles".to_string(),
        }),
        (45310, Code::<u32> {
            code: 45310,
            parent_code: None,
            description: "Wholesale trade of motor vehicle parts and accessories".to_string(),
        }),
        (45320, Code::<u32> {
            code: 45320,
            parent_code: None,
            description: "Retail trade of motor vehicle parts and accessories".to_string(),
        }),
        (45400, Code::<u32> {
            code: 45400,
            parent_code: None,
            description: "Sale, maintenance and repair of motorcycles and related parts and accessories".to_string(),
        }),
        (46110, Code::<u32> {
            code: 46110,
            parent_code: None,
            description: "Agents selling agricultural raw materials, livestock, textile raw materials and semi-finished goods".to_string(),
        }),
        (46120, Code::<u32> {
            code: 46120,
            parent_code: None,
            description: "Agents involved in the sale of fuels, ores, metals and industrial chemicals".to_string(),
        }),
        (46130, Code::<u32> {
            code: 46130,
            parent_code: None,
            description: "Agents involved in the sale of timber and building materials".to_string(),
        }),
        (46140, Code::<u32> {
            code: 46140,
            parent_code: None,
            description: "Agents involved in the sale of machinery, industrial equipment, ships and aircraft".to_string(),
        }),
        (46150, Code::<u32> {
            code: 46150,
            parent_code: None,
            description: "Agents involved in the sale of furniture, household goods, hardware and ironmongery".to_string(),
        }),
        (46160, Code::<u32> {
            code: 46160,
            parent_code: None,
            description: "Agents involved in the sale of textiles, clothing, fur, footwear and leather goods".to_string(),
        }),
        (46170, Code::<u32> {
            code: 46170,
            parent_code: None,
            description: "Agents involved in the sale of food, beverages and tobacco".to_string(),
        }),
        (46180, Code::<u32> {
            code: 46180,
            parent_code: None,
            description: "Agents specialized in the sale of other particular products".to_string(),
        }),
        (46190, Code::<u32> {
            code: 46190,
            parent_code: None,
            description: "Agents involved in the sale of a variety of goods".to_string(),
        }),
        (46210, Code::<u32> {
            code: 46210,
            parent_code: None,
            description: "Wholesale of grain, unmanufactured tobacco, seeds and animal feeds".to_string(),
        }),
        (46220, Code::<u32> {
            code: 46220,
            parent_code: None,
            description: "Wholesale of flowers and plants".to_string(),
        }),
        (46230, Code::<u32> {
            code: 46230,
            parent_code: None,
            description: "Wholesale of live animals".to_string(),
        }),
        (46240, Code::<u32> {
            code: 46240,
            parent_code: None,
            description: "Wholesale of hides, skins and leather".to_string(),
        }),
        (46310, Code::<u32> {
            code: 46310,
            parent_code: None,
            description: "Wholesale of fruit and vegetables".to_string(),
        }),
        (46320, Code::<u32> {
            code: 46320,
            parent_code: None,
            description: "Wholesale of meat and meat products".to_string(),
        }),
        (46330, Code::<u32> {
            code: 46330,
            parent_code: None,
            description: "Wholesale of dairy products, eggs and edible oils and fats".to_string(),
        }),
        (46341, Code::<u32> {
            code: 46341,
            parent_code: None,
            description: "Wholesale of fruit and vegetable juices, mineral water and soft drinks".to_string(),
        }),
        (46342, Code::<u32> {
            code: 46342,
            parent_code: None,
            description: "Wholesale of wine, beer, spirits and other alcoholic beverages".to_string(),
        }),
        (46350, Code::<u32> {
            code: 46350,
            parent_code: None,
            description: "Wholesale of tobacco products".to_string(),
        }),
        (46360, Code::<u32> {
            code: 46360,
            parent_code: None,
            description: "Wholesale of sugar and chocolate and sugar confectionery".to_string(),
        }),
        (46370, Code::<u32> {
            code: 46370,
            parent_code: None,
            description: "Wholesale of coffee, tea, cocoa and spices".to_string(),
        }),
        (46380, Code::<u32> {
            code: 46380,
            parent_code: None,
            description: "Wholesale of other food, including fish, crustaceans and molluscs".to_string(),
        }),
        (46390, Code::<u32> {
            code: 46390,
            parent_code: None,
            description: "Non-specialised wholesale of food, beverages and tobacco".to_string(),
        }),
        (46410, Code::<u32> {
            code: 46410,
            parent_code: None,
            description: "Wholesale of textiles".to_string(),
        }),
        (46420, Code::<u32> {
            code: 46420,
            parent_code: None,
            description: "Wholesale of clothing and footwear".to_string(),
        }),
        (46431, Code::<u32> {
            code: 46431,
            parent_code: None,
            description: "Wholesale of audio tapes, records, CDs and video tapes and the equipment on which these are played".to_string(),
        }),
        (46439, Code::<u32> {
            code: 46439,
            parent_code: None,
            description: "Wholesale of radio, television goods & electrical household appliances (other than records, tapes, CD's & video tapes and the equipment used for playing them)".to_string(),
        }),
        (46440, Code::<u32> {
            code: 46440,
            parent_code: None,
            description: "Wholesale of china and glassware and cleaning materials".to_string(),
        }),
        (46450, Code::<u32> {
            code: 46450,
            parent_code: None,
            description: "Wholesale of perfume and cosmetics".to_string(),
        }),
        (46460, Code::<u32> {
            code: 46460,
            parent_code: None,
            description: "Wholesale of pharmaceutical goods".to_string(),
        }),
        (46470, Code::<u32> {
            code: 46470,
            parent_code: None,
            description: "Wholesale of furniture, carpets and lighting equipment".to_string(),
        }),
        (46480, Code::<u32> {
            code: 46480,
            parent_code: None,
            description: "Wholesale of watches and jewellery".to_string(),
        }),
        (46491, Code::<u32> {
            code: 46491,
            parent_code: None,
            description: "Wholesale of musical instruments".to_string(),
        }),
        (46499, Code::<u32> {
            code: 46499,
            parent_code: None,
            description: "Wholesale of household goods (other than musical instruments) n.e.c.".to_string(),
        }),
        (46510, Code::<u32> {
            code: 46510,
            parent_code: None,
            description: "Wholesale of computers, computer peripheral equipment and software".to_string(),
        }),
        (46520, Code::<u32> {
            code: 46520,
            parent_code: None,
            description: "Wholesale of electronic and telecommunications equipment and parts".to_string(),
        }),
        (46610, Code::<u32> {
            code: 46610,
            parent_code: None,
            description: "Wholesale of agricultural machinery, equipment and supplies".to_string(),
        }),
        (46620, Code::<u32> {
            code: 46620,
            parent_code: None,
            description: "Wholesale of machine tools".to_string(),
        }),
        (46630, Code::<u32> {
            code: 46630,
            parent_code: None,
            description: "Wholesale of mining, construction and civil engineering machinery".to_string(),
        }),
        (46640, Code::<u32> {
            code: 46640,
            parent_code: None,
            description: "Wholesale of machinery for the textile industry and of sewing and knitting machines".to_string(),
        }),
        (46650, Code::<u32> {
            code: 46650,
            parent_code: None,
            description: "Wholesale of office furniture".to_string(),
        }),
        (46660, Code::<u32> {
            code: 46660,
            parent_code: None,
            description: "Wholesale of other office machinery and equipment".to_string(),
        }),
        (46690, Code::<u32> {
            code: 46690,
            parent_code: None,
            description: "Wholesale of other machinery and equipment".to_string(),
        }),
        (46711, Code::<u32> {
            code: 46711,
            parent_code: None,
            description: "Wholesale of petroleum and petroleum products".to_string(),
        }),
        (46719, Code::<u32> {
            code: 46719,
            parent_code: None,
            description: "Wholesale of other fuels and related products".to_string(),
        }),
        (46720, Code::<u32> {
            code: 46720,
            parent_code: None,
            description: "Wholesale of metals and metal ores".to_string(),
        }),
        (46730, Code::<u32> {
            code: 46730,
            parent_code: None,
            description: "Wholesale of wood, construction materials and sanitary equipment".to_string(),
        }),
        (46740, Code::<u32> {
            code: 46740,
            parent_code: None,
            description: "Wholesale of hardware, plumbing and heating equipment and supplies".to_string(),
        }),
        (46750, Code::<u32> {
            code: 46750,
            parent_code: None,
            description: "Wholesale of chemical products".to_string(),
        }),
        (46760, Code::<u32> {
            code: 46760,
            parent_code: None,
            description: "Wholesale of other intermediate products".to_string(),
        }),
        (46770, Code::<u32> {
            code: 46770,
            parent_code: None,
            description: "Wholesale of waste and scrap".to_string(),
        }),
        (46900, Code::<u32> {
            code: 46900,
            parent_code: None,
            description: "Non-specialised wholesale trade".to_string(),
        }),
        (47110, Code::<u32> {
            code: 47110,
            parent_code: None,
            description: "Retail sale in non-specialised stores with food, beverages or tobacco predominating".to_string(),
        }),
        (47190, Code::<u32> {
            code: 47190,
            parent_code: None,
            description: "Other retail sale in non-specialised stores".to_string(),
        }),
        (47210, Code::<u32> {
            code: 47210,
            parent_code: None,
            description: "Retail sale of fruit and vegetables in specialised stores".to_string(),
        }),
        (47220, Code::<u32> {
            code: 47220,
            parent_code: None,
            description: "Retail sale of meat and meat products in specialised stores".to_string(),
        }),
        (47230, Code::<u32> {
            code: 47230,
            parent_code: None,
            description: "Retail sale of fish, crustaceans and molluscs in specialised stores".to_string(),
        }),
        (47240, Code::<u32> {
            code: 47240,
            parent_code: None,
            description: "Retail sale of bread, cakes, flour confectionery and sugar confectionery in specialised stores".to_string(),
        }),
        (47250, Code::<u32> {
            code: 47250,
            parent_code: None,
            description: "Retail sale of beverages in specialised stores".to_string(),
        }),
        (47260, Code::<u32> {
            code: 47260,
            parent_code: None,
            description: "Retail sale of tobacco products in specialised stores".to_string(),
        }),
        (47290, Code::<u32> {
            code: 47290,
            parent_code: None,
            description: "Other retail sale of food in specialised stores".to_string(),
        }),
        (47300, Code::<u32> {
            code: 47300,
            parent_code: None,
            description: "Retail sale of automotive fuel in specialised stores".to_string(),
        }),
        (47410, Code::<u32> {
            code: 47410,
            parent_code: None,
            description: "Retail sale of computers, peripheral units and software in specialised stores".to_string(),
        }),
        (47421, Code::<u32> {
            code: 47421,
            parent_code: None,
            description: "Retail sale of mobile telephones".to_string(),
        }),
        (47429, Code::<u32> {
            code: 47429,
            parent_code: None,
            description: "Retail sale of telecommunications equipment other than mobile telephones".to_string(),
        }),
        (47430, Code::<u32> {
            code: 47430,
            parent_code: None,
            description: "Retail sale of audio and video equipment in specialised stores".to_string(),
        }),
        (47510, Code::<u32> {
            code: 47510,
            parent_code: None,
            description: "Retail sale of textiles in specialised stores".to_string(),
        }),
        (47520, Code::<u32> {
            code: 47520,
            parent_code: None,
            description: "Retail sale of hardware, paints and glass in specialised stores".to_string(),
        }),
        (47530, Code::<u32> {
            code: 47530,
            parent_code: None,
            description: "Retail sale of carpets, rugs, wall and floor coverings in specialised stores".to_string(),
        }),
        (47540, Code::<u32> {
            code: 47540,
            parent_code: None,
            description: "Retail sale of electrical household appliances in specialised stores".to_string(),
        }),
        (47591, Code::<u32> {
            code: 47591,
            parent_code: None,
            description: "Retail sale of musical instruments and scores".to_string(),
        }),
        (47599, Code::<u32> {
            code: 47599,
            parent_code: None,
            description: "Retail of furniture, lighting, and similar (not musical instruments or scores) in specialised store".to_string(),
        }),
        (47610, Code::<u32> {
            code: 47610,
            parent_code: None,
            description: "Retail sale of books in specialised stores".to_string(),
        }),
        (47620, Code::<u32> {
            code: 47620,
            parent_code: None,
            description: "Retail sale of newspapers and stationery in specialised stores".to_string(),
        }),
        (47630, Code::<u32> {
            code: 47630,
            parent_code: None,
            description: "Retail sale of music and video recordings in specialised stores".to_string(),
        }),
        (47640, Code::<u32> {
            code: 47640,
            parent_code: None,
            description: "Retail sale of sports goods, fishing gear, camping goods, boats and bicycles".to_string(),
        }),
        (47650, Code::<u32> {
            code: 47650,
            parent_code: None,
            description: "Retail sale of games and toys in specialised stores".to_string(),
        }),
        (47710, Code::<u32> {
            code: 47710,
            parent_code: None,
            description: "Retail sale of clothing in specialised stores".to_string(),
        }),
        (47721, Code::<u32> {
            code: 47721,
            parent_code: None,
            description: "Retail sale of footwear in specialised stores".to_string(),
        }),
        (47722, Code::<u32> {
            code: 47722,
            parent_code: None,
            description: "Retail sale of leather goods in specialised stores".to_string(),
        }),
        (47730, Code::<u32> {
            code: 47730,
            parent_code: None,
            description: "Dispensing chemist in specialised stores".to_string(),
        }),
        (47741, Code::<u32> {
            code: 47741,
            parent_code: None,
            description: "Retail sale of hearing aids".to_string(),
        }),
        (47749, Code::<u32> {
            code: 47749,
            parent_code: None,
            description: "Retail sale of medical and orthopaedic goods in specialised stores (not incl. hearing aids) n.e.c.".to_string(),
        }),
        (47750, Code::<u32> {
            code: 47750,
            parent_code: None,
            description: "Retail sale of cosmetic and toilet articles in specialised stores".to_string(),
        }),
        (47760, Code::<u32> {
            code: 47760,
            parent_code: None,
            description: "Retail sale of flowers, plants, seeds, fertilizers, pet animals and pet food in specialised stores".to_string(),
        }),
        (47770, Code::<u32> {
            code: 47770,
            parent_code: None,
            description: "Retail sale of watches and jewellery in specialised stores".to_string(),
        }),
        (47781, Code::<u32> {
            code: 47781,
            parent_code: None,
            description: "Retail sale in commercial art galleries".to_string(),
        }),
        (47782, Code::<u32> {
            code: 47782,
            parent_code: None,
            description: "Retail sale by opticians".to_string(),
        }),
        (47789, Code::<u32> {
            code: 47789,
            parent_code: None,
            description: "Other retail sale of new goods in specialised stores (not commercial art galleries and opticians)".to_string(),
        }),
        (47791, Code::<u32> {
            code: 47791,
            parent_code: None,
            description: "Retail sale of antiques including antique books in stores".to_string(),
        }),
        (47799, Code::<u32> {
            code: 47799,
            parent_code: None,
            description: "Retail sale of other second-hand goods in stores (not incl. antiques)".to_string(),
        }),
        (47810, Code::<u32> {
            code: 47810,
            parent_code: None,
            description: "Retail sale via stalls and markets of food, beverages and tobacco products".to_string(),
        }),
        (47820, Code::<u32> {
            code: 47820,
            parent_code: None,
            description: "Retail sale via stalls and markets of textiles, clothing and footwear".to_string(),
        }),
        (47890, Code::<u32> {
            code: 47890,
            parent_code: None,
            description: "Retail sale via stalls and markets of other goods".to_string(),
        }),
        (47910, Code::<u32> {
            code: 47910,
            parent_code: None,
            description: "Retail sale via mail order houses or via Internet".to_string(),
        }),
        (47990, Code::<u32> {
            code: 47990,
            parent_code: None,
            description: "Other retail sale not in stores, stalls or markets".to_string(),
        }),
        (49100, Code::<u32> {
            code: 49100,
            parent_code: None,
            description: "Passenger rail transport, interurban".to_string(),
        }),
        (49200, Code::<u32> {
            code: 49200,
            parent_code: None,
            description: "Freight rail transport".to_string(),
        }),
        (49311, Code::<u32> {
            code: 49311,
            parent_code: None,
            description: "Urban and suburban passenger railway transportation by underground, metro and similar systems".to_string(),
        }),
        (49319, Code::<u32> {
            code: 49319,
            parent_code: None,
            description: "Other urban, suburban or metropolitan passenger land transport (not underground, metro or similar)".to_string(),
        }),
        (49320, Code::<u32> {
            code: 49320,
            parent_code: None,
            description: "Taxi operation".to_string(),
        }),
        (49390, Code::<u32> {
            code: 49390,
            parent_code: None,
            description: "Other passenger land transport".to_string(),
        }),
        (49410, Code::<u32> {
            code: 49410,
            parent_code: None,
            description: "Freight transport by road".to_string(),
        }),
        (49420, Code::<u32> {
            code: 49420,
            parent_code: None,
            description: "Removal services".to_string(),
        }),
        (49500, Code::<u32> {
            code: 49500,
            parent_code: None,
            description: "Transport via pipeline".to_string(),
        }),
        (50100, Code::<u32> {
            code: 50100,
            parent_code: None,
            description: "Sea and coastal passenger water transport".to_string(),
        }),
        (50200, Code::<u32> {
            code: 50200,
            parent_code: None,
            description: "Sea and coastal freight water transport".to_string(),
        }),
        (50300, Code::<u32> {
            code: 50300,
            parent_code: None,
            description: "Inland passenger water transport".to_string(),
        }),
        (50400, Code::<u32> {
            code: 50400,
            parent_code: None,
            description: "Inland freight water transport".to_string(),
        }),
        (51101, Code::<u32> {
            code: 51101,
            parent_code: None,
            description: "Scheduled passenger air transport".to_string(),
        }),
        (51102, Code::<u32> {
            code: 51102,
            parent_code: None,
            description: "Non-scheduled passenger air transport".to_string(),
        }),
        (51210, Code::<u32> {
            code: 51210,
            parent_code: None,
            description: "Freight air transport".to_string(),
        }),
        (51220, Code::<u32> {
            code: 51220,
            parent_code: None,
            description: "Space transport".to_string(),
        }),
        (52101, Code::<u32> {
            code: 52101,
            parent_code: None,
            description: "Operation of warehousing and storage facilities for water transport activities".to_string(),
        }),
        (52102, Code::<u32> {
            code: 52102,
            parent_code: None,
            description: "Operation of warehousing and storage facilities for air transport activities".to_string(),
        }),
        (52103, Code::<u32> {
            code: 52103,
            parent_code: None,
            description: "Operation of warehousing and storage facilities for land transport activities".to_string(),
        }),
        (52211, Code::<u32> {
            code: 52211,
            parent_code: None,
            description: "Operation of rail freight terminals".to_string(),
        }),
        (52212, Code::<u32> {
            code: 52212,
            parent_code: None,
            description: "Operation of rail passenger facilities at railway stations".to_string(),
        }),
        (52213, Code::<u32> {
            code: 52213,
            parent_code: None,
            description: "Operation of bus and coach passenger facilities at bus and coach stations".to_string(),
        }),
        (52219, Code::<u32> {
            code: 52219,
            parent_code: None,
            description: "Other service activities incidental to land transportation, n.e.c.".to_string(),
        }),
        (52220, Code::<u32> {
            code: 52220,
            parent_code: None,
            description: "Service activities incidental to water transportation".to_string(),
        }),
        (52230, Code::<u32> {
            code: 52230,
            parent_code: None,
            description: "Service activities incidental to air transportation".to_string(),
        }),
        (52241, Code::<u32> {
            code: 52241,
            parent_code: None,
            description: "Cargo handling for water transport activities".to_string(),
        }),
        (52242, Code::<u32> {
            code: 52242,
            parent_code: None,
            description: "Cargo handling for air transport activities".to_string(),
        }),
        (52243, Code::<u32> {
            code: 52243,
            parent_code: None,
            description: "Cargo handling for land transport activities".to_string(),
        }),
        (52290, Code::<u32> {
            code: 52290,
            parent_code: None,
            description: "Other transportation support activities".to_string(),
        }),
        (53100, Code::<u32> {
            code: 53100,
            parent_code: None,
            description: "Postal activities under universal service obligation".to_string(),
        }),
        (53201, Code::<u32> {
            code: 53201,
            parent_code: None,
            description: "Licensed carriers".to_string(),
        }),
        (53202, Code::<u32> {
            code: 53202,
            parent_code: None,
            description: "Unlicensed carrier".to_string(),
        }),
        (55100, Code::<u32> {
            code: 55100,
            parent_code: None,
            description: "Hotels and similar accommodation".to_string(),
        }),
        (55201, Code::<u32> {
            code: 55201,
            parent_code: None,
            description: "Holiday centres and villages".to_string(),
        }),
        (55202, Code::<u32> {
            code: 55202,
            parent_code: None,
            description: "Youth hostels".to_string(),
        }),
        (55209, Code::<u32> {
            code: 55209,
            parent_code: None,
            description: "Other holiday and other collective accommodation".to_string(),
        }),
        (55300, Code::<u32> {
            code: 55300,
            parent_code: None,
            description: "Recreational vehicle parks, trailer parks and camping grounds".to_string(),
        }),
        (55900, Code::<u32> {
            code: 55900,
            parent_code: None,
            description: "Other accommodation".to_string(),
        }),
        (56101, Code::<u32> {
            code: 56101,
            parent_code: None,
            description: "Licensed restaurants".to_string(),
        }),
        (56102, Code::<u32> {
            code: 56102,
            parent_code: None,
            description: "Unlicensed restaurants and cafes".to_string(),
        }),
        (56103, Code::<u32> {
            code: 56103,
            parent_code: None,
            description: "Take-away food shops and mobile food stands".to_string(),
        }),
        (56210, Code::<u32> {
            code: 56210,
            parent_code: None,
            description: "Event catering activities".to_string(),
        }),
        (56290, Code::<u32> {
            code: 56290,
            parent_code: None,
            description: "Other food services".to_string(),
        }),
        (56301, Code::<u32> {
            code: 56301,
            parent_code: None,
            description: "Licensed clubs".to_string(),
        }),
        (56302, Code::<u32> {
            code: 56302,
            parent_code: None,
            description: "Public houses and bars".to_string(),
        }),
        (58110, Code::<u32> {
            code: 58110,
            parent_code: None,
            description: "Book publishing".to_string(),
        }),
        (58120, Code::<u32> {
            code: 58120,
            parent_code: None,
            description: "Publishing of directories and mailing lists".to_string(),
        }),
        (58130, Code::<u32> {
            code: 58130,
            parent_code: None,
            description: "Publishing of newspapers".to_string(),
        }),
        (58141, Code::<u32> {
            code: 58141,
            parent_code: None,
            description: "Publishing of learned journals".to_string(),
        }),
        (58142, Code::<u32> {
            code: 58142,
            parent_code: None,
            description: "Publishing of  consumer and business journals and periodicals".to_string(),
        }),
        (58190, Code::<u32> {
            code: 58190,
            parent_code: None,
            description: "Other publishing activities".to_string(),
        }),
        (58210, Code::<u32> {
            code: 58210,
            parent_code: None,
            description: "Publishing of computer games".to_string(),
        }),
        (58290, Code::<u32> {
            code: 58290,
            parent_code: None,
            description: "Other software publishing".to_string(),
        }),
        (59111, Code::<u32> {
            code: 59111,
            parent_code: None,
            description: "Motion picture production activities".to_string(),
        }),
        (59112, Code::<u32> {
            code: 59112,
            parent_code: None,
            description: "Video production activities".to_string(),
        }),
        (59113, Code::<u32> {
            code: 59113,
            parent_code: None,
            description: "Television programme production activities".to_string(),
        }),
        (59120, Code::<u32> {
            code: 59120,
            parent_code: None,
            description: "Motion picture, video and television programme post-production activities".to_string(),
        }),
        (59131, Code::<u32> {
            code: 59131,
            parent_code: None,
            description: "Motion picture distribution activities".to_string(),
        }),
        (59132, Code::<u32> {
            code: 59132,
            parent_code: None,
            description: "Video distribution activities".to_string(),
        }),
        (59133, Code::<u32> {
            code: 59133,
            parent_code: None,
            description: "Television programme distribution activities".to_string(),
        }),
        (59140, Code::<u32> {
            code: 59140,
            parent_code: None,
            description: "Motion picture projection activities".to_string(),
        }),
        (59200, Code::<u32> {
            code: 59200,
            parent_code: None,
            description: "Sound recording and music publishing activities".to_string(),
        }),
        (60100, Code::<u32> {
            code: 60100,
            parent_code: None,
            description: "Radio broadcasting".to_string(),
        }),
        (60200, Code::<u32> {
            code: 60200,
            parent_code: None,
            description: "Television programming and broadcasting activities".to_string(),
        }),
        (61100, Code::<u32> {
            code: 61100,
            parent_code: None,
            description: "Wired telecommunications activities".to_string(),
        }),
        (61200, Code::<u32> {
            code: 61200,
            parent_code: None,
            description: "Wireless telecommunications activities".to_string(),
        }),
        (61300, Code::<u32> {
            code: 61300,
            parent_code: None,
            description: "Satellite telecommunications activities".to_string(),
        }),
        (61900, Code::<u32> {
            code: 61900,
            parent_code: None,
            description: "Other telecommunications activities".to_string(),
        }),
        (62011, Code::<u32> {
            code: 62011,
            parent_code: None,
            description: "Ready-made interactive leisure and entertainment software development".to_string(),
        }),
        (62012, Code::<u32> {
            code: 62012,
            parent_code: None,
            description: "Business and domestic software development".to_string(),
        }),
        (62020, Code::<u32> {
            code: 62020,
            parent_code: None,
            description: "Information technology consultancy activities".to_string(),
        }),
        (62030, Code::<u32> {
            code: 62030,
            parent_code: None,
            description: "Computer facilities management activities".to_string(),
        }),
        (62090, Code::<u32> {
            code: 62090,
            parent_code: None,
            description: "Other information technology service activities".to_string(),
        }),
        (63110, Code::<u32> {
            code: 63110,
            parent_code: None,
            description: "Data processing, hosting and related activities".to_string(),
        }),
        (63120, Code::<u32> {
            code: 63120,
            parent_code: None,
            description: "Web portals".to_string(),
        }),
        (63910, Code::<u32> {
            code: 63910,
            parent_code: None,
            description: "News agency activities".to_string(),
        }),
        (63990, Code::<u32> {
            code: 63990,
            parent_code: None,
            description: "Other information service activities n.e.c.".to_string(),
        }),
        (64110, Code::<u32> {
            code: 64110,
            parent_code: None,
            description: "Central banking".to_string(),
        }),
        (64191, Code::<u32> {
            code: 64191,
            parent_code: None,
            description: "Banks".to_string(),
        }),
        (64192, Code::<u32> {
            code: 64192,
            parent_code: None,
            description: "Building societies".to_string(),
        }),
        (64201, Code::<u32> {
            code: 64201,
            parent_code: None,
            description: "Activities of agricultural holding companies".to_string(),
        }),
        (64202, Code::<u32> {
            code: 64202,
            parent_code: None,
            description: "Activities of production holding companies".to_string(),
        }),
        (64203, Code::<u32> {
            code: 64203,
            parent_code: None,
            description: "Activities of construction holding companies".to_string(),
        }),
        (64204, Code::<u32> {
            code: 64204,
            parent_code: None,
            description: "Activities of distribution holding companies".to_string(),
        }),
        (64205, Code::<u32> {
            code: 64205,
            parent_code: None,
            description: "Activities of financial services holding companies".to_string(),
        }),
        (64209, Code::<u32> {
            code: 64209,
            parent_code: None,
            description: "Activities of other holding companies n.e.c.".to_string(),
        }),
        (64301, Code::<u32> {
            code: 64301,
            parent_code: None,
            description: "Activities of investment trusts".to_string(),
        }),
        (64302, Code::<u32> {
            code: 64302,
            parent_code: None,
            description: "Activities of unit trusts".to_string(),
        }),
        (64303, Code::<u32> {
            code: 64303,
            parent_code: None,
            description: "Activities of venture and development capital companies".to_string(),
        }),
        (64304, Code::<u32> {
            code: 64304,
            parent_code: None,
            description: "Activities of open-ended investment companies".to_string(),
        }),
        (64305, Code::<u32> {
            code: 64305,
            parent_code: None,
            description: "Activities of property unit trusts".to_string(),
        }),
        (64306, Code::<u32> {
            code: 64306,
            parent_code: None,
            description: "Activities of real estate investment trusts".to_string(),
        }),
        (64910, Code::<u32> {
            code: 64910,
            parent_code: None,
            description: "Financial leasing".to_string(),
        }),
        (64921, Code::<u32> {
            code: 64921,
            parent_code: None,
            description: "Credit granting by non-deposit taking finance houses and other specialist consumer credit grantors".to_string(),
        }),
        (64922, Code::<u32> {
            code: 64922,
            parent_code: None,
            description: "Activities of mortgage finance companies".to_string(),
        }),
        (64929, Code::<u32> {
            code: 64929,
            parent_code: None,
            description: "Other credit granting n.e.c.".to_string(),
        }),
        (64991, Code::<u32> {
            code: 64991,
            parent_code: None,
            description: "Security dealing on own account".to_string(),
        }),
        (64992, Code::<u32> {
            code: 64992,
            parent_code: None,
            description: "Factoring".to_string(),
        }),
        (64999, Code::<u32> {
            code: 64999,
            parent_code: None,
            description: "Financial intermediation not elsewhere classified".to_string(),
        }),
        (65110, Code::<u32> {
            code: 65110,
            parent_code: None,
            description: "Life insurance".to_string(),
        }),
        (65120, Code::<u32> {
            code: 65120,
            parent_code: None,
            description: "Non-life insurance".to_string(),
        }),
        (65201, Code::<u32> {
            code: 65201,
            parent_code: None,
            description: "Life reinsurance".to_string(),
        }),
        (65202, Code::<u32> {
            code: 65202,
            parent_code: None,
            description: "Non-life reinsurance".to_string(),
        }),
        (65300, Code::<u32> {
            code: 65300,
            parent_code: None,
            description: "Pension funding".to_string(),
        }),
        (66110, Code::<u32> {
            code: 66110,
            parent_code: None,
            description: "Administration of financial markets".to_string(),
        }),
        (66120, Code::<u32> {
            code: 66120,
            parent_code: None,
            description: "Security and commodity contracts dealing activities".to_string(),
        }),
        (66190, Code::<u32> {
            code: 66190,
            parent_code: None,
            description: "Activities auxiliary to financial intermediation n.e.c.".to_string(),
        }),
        (66210, Code::<u32> {
            code: 66210,
            parent_code: None,
            description: "Risk and damage evaluation".to_string(),
        }),
        (66220, Code::<u32> {
            code: 66220,
            parent_code: None,
            description: "Activities of insurance agents and brokers".to_string(),
        }),
        (66290, Code::<u32> {
            code: 66290,
            parent_code: None,
            description: "Other activities auxiliary to insurance and pension funding".to_string(),
        }),
        (66300, Code::<u32> {
            code: 66300,
            parent_code: None,
            description: "Fund management activities".to_string(),
        }),
        (68100, Code::<u32> {
            code: 68100,
            parent_code: None,
            description: "Buying and selling of own real estate".to_string(),
        }),
        (68201, Code::<u32> {
            code: 68201,
            parent_code: None,
            description: "Renting and operating of Housing Association real estate".to_string(),
        }),
        (68202, Code::<u32> {
            code: 68202,
            parent_code: None,
            description: "Letting and operating of conference and exhibition centres".to_string(),
        }),
        (68209, Code::<u32> {
            code: 68209,
            parent_code: None,
            description: "Other letting and operating of own or leased real estate".to_string(),
        }),
        (68310, Code::<u32> {
            code: 68310,
            parent_code: None,
            description: "Real estate agencies".to_string(),
        }),
        (68320, Code::<u32> {
            code: 68320,
            parent_code: None,
            description: "Management of real estate on a fee or contract basis".to_string(),
        }),
        (69101, Code::<u32> {
            code: 69101,
            parent_code: None,
            description: "Barristers at law".to_string(),
        }),
        (69102, Code::<u32> {
            code: 69102,
            parent_code: None,
            description: "Solicitors".to_string(),
        }),
        (69109, Code::<u32> {
            code: 69109,
            parent_code: None,
            description: "Activities of patent and copyright agents; other legal activities n.e.c.".to_string(),
        }),
        (69201, Code::<u32> {
            code: 69201,
            parent_code: None,
            description: "Accounting and auditing activities".to_string(),
        }),
        (69202, Code::<u32> {
            code: 69202,
            parent_code: None,
            description: "Bookkeeping activities".to_string(),
        }),
        (69203, Code::<u32> {
            code: 69203,
            parent_code: None,
            description: "Tax consultancy".to_string(),
        }),
        (70100, Code::<u32> {
            code: 70100,
            parent_code: None,
            description: "Activities of head offices".to_string(),
        }),
        (70210, Code::<u32> {
            code: 70210,
            parent_code: None,
            description: "Public relations and communications activities".to_string(),
        }),
        (70221, Code::<u32> {
            code: 70221,
            parent_code: None,
            description: "Financial management".to_string(),
        }),
        (70229, Code::<u32> {
            code: 70229,
            parent_code: None,
            description: "Management consultancy activities other than financial management".to_string(),
        }),
        (71111, Code::<u32> {
            code: 71111,
            parent_code: None,
            description: "Architectural activities".to_string(),
        }),
        (71112, Code::<u32> {
            code: 71112,
            parent_code: None,
            description: "Urban planning and landscape architectural activities".to_string(),
        }),
        (71121, Code::<u32> {
            code: 71121,
            parent_code: None,
            description: "Engineering design activities for industrial process and production".to_string(),
        }),
        (71122, Code::<u32> {
            code: 71122,
            parent_code: None,
            description: "Engineering related scientific and technical consulting activities".to_string(),
        }),
        (71129, Code::<u32> {
            code: 71129,
            parent_code: None,
            description: "Other engineering activities".to_string(),
        }),
        (71200, Code::<u32> {
            code: 71200,
            parent_code: None,
            description: "Technical testing and analysis".to_string(),
        }),
        (72110, Code::<u32> {
            code: 72110,
            parent_code: None,
            description: "Research and experimental development on biotechnology".to_string(),
        }),
        (72190, Code::<u32> {
            code: 72190,
            parent_code: None,
            description: "Other research and experimental development on natural sciences and engineering".to_string(),
        }),
        (72200, Code::<u32> {
            code: 72200,
            parent_code: None,
            description: "Research and experimental development on social sciences and humanities".to_string(),
        }),
        (73110, Code::<u32> {
            code: 73110,
            parent_code: None,
            description: "Advertising agencies".to_string(),
        }),
        (73120, Code::<u32> {
            code: 73120,
            parent_code: None,
            description: "Media representation services".to_string(),
        }),
        (73200, Code::<u32> {
            code: 73200,
            parent_code: None,
            description: "Market research and public opinion polling".to_string(),
        }),
        (74100, Code::<u32> {
            code: 74100,
            parent_code: None,
            description: "specialised design activities".to_string(),
        }),
        (74201, Code::<u32> {
            code: 74201,
            parent_code: None,
            description: "Portrait photographic activities".to_string(),
        }),
        (74202, Code::<u32> {
            code: 74202,
            parent_code: None,
            description: "Other specialist photography".to_string(),
        }),
        (74203, Code::<u32> {
            code: 74203,
            parent_code: None,
            description: "Film processing".to_string(),
        }),
        (74209, Code::<u32> {
            code: 74209,
            parent_code: None,
            description: "Photographic activities not elsewhere classified".to_string(),
        }),
        (74300, Code::<u32> {
            code: 74300,
            parent_code: None,
            description: "Translation and interpretation activities".to_string(),
        }),
        (74901, Code::<u32> {
            code: 74901,
            parent_code: None,
            description: "Environmental consulting activities".to_string(),
        }),
        (74902, Code::<u32> {
            code: 74902,
            parent_code: None,
            description: "Quantity surveying activities".to_string(),
        }),
        (74909, Code::<u32> {
            code: 74909,
            parent_code: None,
            description: "Other professional, scientific and technical activities n.e.c.".to_string(),
        }),
        (74990, Code::<u32> {
            code: 74990,
            parent_code: None,
            description: "Non-trading company".to_string(),
        }),
        (75000, Code::<u32> {
            code: 75000,
            parent_code: None,
            description: "Veterinary activities".to_string(),
        }),
        (77110, Code::<u32> {
            code: 77110,
            parent_code: None,
            description: "Renting and leasing of cars and light motor vehicles".to_string(),
        }),
        (77120, Code::<u32> {
            code: 77120,
            parent_code: None,
            description: "Renting and leasing of trucks and other heavy vehicles".to_string(),
        }),
        (77210, Code::<u32> {
            code: 77210,
            parent_code: None,
            description: "Renting and leasing of recreational and sports goods".to_string(),
        }),
        (77220, Code::<u32> {
            code: 77220,
            parent_code: None,
            description: "Renting of video tapes and disks".to_string(),
        }),
        (77291, Code::<u32> {
            code: 77291,
            parent_code: None,
            description: "Renting and leasing of media entertainment equipment".to_string(),
        }),
        (77299, Code::<u32> {
            code: 77299,
            parent_code: None,
            description: "Renting and leasing of other personal and household goods".to_string(),
        }),
        (77310, Code::<u32> {
            code: 77310,
            parent_code: None,
            description: "Renting and leasing of agricultural machinery and equipment".to_string(),
        }),
        (77320, Code::<u32> {
            code: 77320,
            parent_code: None,
            description: "Renting and leasing of construction and civil engineering machinery and equipment".to_string(),
        }),
        (77330, Code::<u32> {
            code: 77330,
            parent_code: None,
            description: "Renting and leasing of office machinery and equipment (including computers)".to_string(),
        }),
        (77341, Code::<u32> {
            code: 77341,
            parent_code: None,
            description: "Renting and leasing of passenger water transport equipment".to_string(),
        }),
        (77342, Code::<u32> {
            code: 77342,
            parent_code: None,
            description: "Renting and leasing of freight water transport equipment".to_string(),
        }),
        (77351, Code::<u32> {
            code: 77351,
            parent_code: None,
            description: "Renting and leasing of air passenger transport equipment".to_string(),
        }),
        (77352, Code::<u32> {
            code: 77352,
            parent_code: None,
            description: "Renting and leasing of freight air transport equipment".to_string(),
        }),
        (77390, Code::<u32> {
            code: 77390,
            parent_code: None,
            description: "Renting and leasing of other machinery, equipment and tangible goods n.e.c.".to_string(),
        }),
        (77400, Code::<u32> {
            code: 77400,
            parent_code: None,
            description: "Leasing of intellectual property and similar products, except copyright works".to_string(),
        }),
        (78101, Code::<u32> {
            code: 78101,
            parent_code: None,
            description: "Motion picture, television and other theatrical casting activities".to_string(),
        }),
        (78109, Code::<u32> {
            code: 78109,
            parent_code: None,
            description: "Other activities of employment placement agencies".to_string(),
        }),
        (78200, Code::<u32> {
            code: 78200,
            parent_code: None,
            description: "Temporary employment agency activities".to_string(),
        }),
        (78300, Code::<u32> {
            code: 78300,
            parent_code: None,
            description: "Human resources provision and management of human resources functions".to_string(),
        }),
        (79110, Code::<u32> {
            code: 79110,
            parent_code: None,
            description: "Travel agency activities".to_string(),
        }),
        (79120, Code::<u32> {
            code: 79120,
            parent_code: None,
            description: "Tour operator activities".to_string(),
        }),
        (79901, Code::<u32> {
            code: 79901,
            parent_code: None,
            description: "Activities of tourist guides".to_string(),
        }),
        (79909, Code::<u32> {
            code: 79909,
            parent_code: None,
            description: "Other reservation service activities n.e.c.".to_string(),
        }),
        (80100, Code::<u32> {
            code: 80100,
            parent_code: None,
            description: "Private security activities".to_string(),
        }),
        (80200, Code::<u32> {
            code: 80200,
            parent_code: None,
            description: "Security systems service activities".to_string(),
        }),
        (80300, Code::<u32> {
            code: 80300,
            parent_code: None,
            description: "Investigation activities".to_string(),
        }),
        (81100, Code::<u32> {
            code: 81100,
            parent_code: None,
            description: "Combined facilities support activities".to_string(),
        }),
        (81210, Code::<u32> {
            code: 81210,
            parent_code: None,
            description: "General cleaning of buildings".to_string(),
        }),
        (81221, Code::<u32> {
            code: 81221,
            parent_code: None,
            description: "Window cleaning services".to_string(),
        }),
        (81222, Code::<u32> {
            code: 81222,
            parent_code: None,
            description: "Specialised cleaning services".to_string(),
        }),
        (81223, Code::<u32> {
            code: 81223,
            parent_code: None,
            description: "Furnace and chimney cleaning services".to_string(),
        }),
        (81229, Code::<u32> {
            code: 81229,
            parent_code: None,
            description: "Other building and industrial cleaning activities".to_string(),
        }),
        (81291, Code::<u32> {
            code: 81291,
            parent_code: None,
            description: "Disinfecting and exterminating services".to_string(),
        }),
        (81299, Code::<u32> {
            code: 81299,
            parent_code: None,
            description: "Other cleaning services".to_string(),
        }),
        (81300, Code::<u32> {
            code: 81300,
            parent_code: None,
            description: "Landscape service activities".to_string(),
        }),
        (82110, Code::<u32> {
            code: 82110,
            parent_code: None,
            description: "Combined office administrative service activities".to_string(),
        }),
        (82190, Code::<u32> {
            code: 82190,
            parent_code: None,
            description: "Photocopying, document preparation and other specialised office support activities".to_string(),
        }),
        (82200, Code::<u32> {
            code: 82200,
            parent_code: None,
            description: "Activities of call centres".to_string(),
        }),
        (82301, Code::<u32> {
            code: 82301,
            parent_code: None,
            description: "Activities of exhibition and fair organisers".to_string(),
        }),
        (82302, Code::<u32> {
            code: 82302,
            parent_code: None,
            description: "Activities of conference organisers".to_string(),
        }),
        (82911, Code::<u32> {
            code: 82911,
            parent_code: None,
            description: "Activities of collection agencies".to_string(),
        }),
        (82912, Code::<u32> {
            code: 82912,
            parent_code: None,
            description: "Activities of credit bureaus".to_string(),
        }),
        (82920, Code::<u32> {
            code: 82920,
            parent_code: None,
            description: "Packaging activities".to_string(),
        }),
        (82990, Code::<u32> {
            code: 82990,
            parent_code: None,
            description: "Other business support service activities n.e.c.".to_string(),
        }),
        (84110, Code::<u32> {
            code: 84110,
            parent_code: None,
            description: "General public administration activities".to_string(),
        }),
        (84120, Code::<u32> {
            code: 84120,
            parent_code: None,
            description: "Regulation of health care, education, cultural and other social services, not incl. social security".to_string(),
        }),
        (84130, Code::<u32> {
            code: 84130,
            parent_code: None,
            description: "Regulation of and contribution to more efficient operation of businesses".to_string(),
        }),
        (84210, Code::<u32> {
            code: 84210,
            parent_code: None,
            description: "Foreign affairs".to_string(),
        }),
        (84220, Code::<u32> {
            code: 84220,
            parent_code: None,
            description: "Defence activities".to_string(),
        }),
        (84230, Code::<u32> {
            code: 84230,
            parent_code: None,
            description: "Justice and judicial activities".to_string(),
        }),
        (84240, Code::<u32> {
            code: 84240,
            parent_code: None,
            description: "Public order and safety activities".to_string(),
        }),
        (84250, Code::<u32> {
            code: 84250,
            parent_code: None,
            description: "Fire service activities".to_string(),
        }),
        (84300, Code::<u32> {
            code: 84300,
            parent_code: None,
            description: "Compulsory social security activities".to_string(),
        }),
        (85100, Code::<u32> {
            code: 85100,
            parent_code: None,
            description: "Pre-primary education".to_string(),
        }),
        (85200, Code::<u32> {
            code: 85200,
            parent_code: None,
            description: "Primary education".to_string(),
        }),
        (85310, Code::<u32> {
            code: 85310,
            parent_code: None,
            description: "General secondary education".to_string(),
        }),
        (85320, Code::<u32> {
            code: 85320,
            parent_code: None,
            description: "Technical and vocational secondary education".to_string(),
        }),
        (85410, Code::<u32> {
            code: 85410,
            parent_code: None,
            description: "Post-secondary non-tertiary education".to_string(),
        }),
        (85421, Code::<u32> {
            code: 85421,
            parent_code: None,
            description: "First-degree level higher education".to_string(),
        }),
        (85422, Code::<u32> {
            code: 85422,
            parent_code: None,
            description: "Post-graduate level higher education".to_string(),
        }),
        (85510, Code::<u32> {
            code: 85510,
            parent_code: None,
            description: "Sports and recreation education".to_string(),
        }),
        (85520, Code::<u32> {
            code: 85520,
            parent_code: None,
            description: "Cultural education".to_string(),
        }),
        (85530, Code::<u32> {
            code: 85530,
            parent_code: None,
            description: "Driving school activities".to_string(),
        }),
        (85590, Code::<u32> {
            code: 85590,
            parent_code: None,
            description: "Other education n.e.c.".to_string(),
        }),
        (85600, Code::<u32> {
            code: 85600,
            parent_code: None,
            description: "Educational support services".to_string(),
        }),
        (86101, Code::<u32> {
            code: 86101,
            parent_code: None,
            description: "Hospital activities".to_string(),
        }),
        (86102, Code::<u32> {
            code: 86102,
            parent_code: None,
            description: "Medical nursing home activities".to_string(),
        }),
        (86210, Code::<u32> {
            code: 86210,
            parent_code: None,
            description: "General medical practice activities".to_string(),
        }),
        (86220, Code::<u32> {
            code: 86220,
            parent_code: None,
            description: "Specialists medical practice activities".to_string(),
        }),
        (86230, Code::<u32> {
            code: 86230,
            parent_code: None,
            description: "Dental practice activities".to_string(),
        }),
        (86900, Code::<u32> {
            code: 86900,
            parent_code: None,
            description: "Other human health activities".to_string(),
        }),
        (87100, Code::<u32> {
            code: 87100,
            parent_code: None,
            description: "Residential nursing care facilities".to_string(),
        }),
        (87200, Code::<u32> {
            code: 87200,
            parent_code: None,
            description: "Residential care activities for learning difficulties, mental health and substance abuse".to_string(),
        }),
        (87300, Code::<u32> {
            code: 87300,
            parent_code: None,
            description: "Residential care activities for the elderly and disabled".to_string(),
        }),
        (87900, Code::<u32> {
            code: 87900,
            parent_code: None,
            description: "Other residential care activities n.e.c.".to_string(),
        }),
        (88100, Code::<u32> {
            code: 88100,
            parent_code: None,
            description: "Social work activities without accommodation for the elderly and disabled".to_string(),
        }),
        (88910, Code::<u32> {
            code: 88910,
            parent_code: None,
            description: "Child day-care activities".to_string(),
        }),
        (88990, Code::<u32> {
            code: 88990,
            parent_code: None,
            description: "Other social work activities without accommodation n.e.c.".to_string(),
        }),
        (90010, Code::<u32> {
            code: 90010,
            parent_code: None,
            description: "Performing arts".to_string(),
        }),
        (90020, Code::<u32> {
            code: 90020,
            parent_code: None,
            description: "Support activities to performing arts".to_string(),
        }),
        (90030, Code::<u32> {
            code: 90030,
            parent_code: None,
            description: "Artistic creation".to_string(),
        }),
        (90040, Code::<u32> {
            code: 90040,
            parent_code: None,
            description: "Operation of arts facilities".to_string(),
        }),
        (91011, Code::<u32> {
            code: 91011,
            parent_code: None,
            description: "Library activities".to_string(),
        }),
        (91012, Code::<u32> {
            code: 91012,
            parent_code: None,
            description: "Archives activities".to_string(),
        }),
        (91020, Code::<u32> {
            code: 91020,
            parent_code: None,
            description: "Museums activities".to_string(),
        }),
        (91030, Code::<u32> {
            code: 91030,
            parent_code: None,
            description: "Operation of historical sites and buildings and similar visitor attractions".to_string(),
        }),
        (91040, Code::<u32> {
            code: 91040,
            parent_code: None,
            description: "Botanical and zoological gardens and nature reserves activities".to_string(),
        }),
        (92000, Code::<u32> {
            code: 92000,
            parent_code: None,
            description: "Gambling and betting activities".to_string(),
        }),
        (93110, Code::<u32> {
            code: 93110,
            parent_code: None,
            description: "Operation of sports facilities".to_string(),
        }),
        (93120, Code::<u32> {
            code: 93120,
            parent_code: None,
            description: "Activities of sport clubs".to_string(),
        }),
        (93130, Code::<u32> {
            code: 93130,
            parent_code: None,
            description: "Fitness facilities".to_string(),
        }),
        (93191, Code::<u32> {
            code: 93191,
            parent_code: None,
            description: "Activities of racehorse owners".to_string(),
        }),
        (93199, Code::<u32> {
            code: 93199,
            parent_code: None,
            description: "Other sports activities".to_string(),
        }),
        (93210, Code::<u32> {
            code: 93210,
            parent_code: None,
            description: "Activities of amusement parks and theme parks".to_string(),
        }),
        (93290, Code::<u32> {
            code: 93290,
            parent_code: None,
            description: "Other amusement and recreation activities n.e.c.".to_string(),
        }),
        (94110, Code::<u32> {
            code: 94110,
            parent_code: None,
            description: "Activities of business and employers membership organizations".to_string(),
        }),
        (94120, Code::<u32> {
            code: 94120,
            parent_code: None,
            description: "Activities of professional membership organizations".to_string(),
        }),
        (94200, Code::<u32> {
            code: 94200,
            parent_code: None,
            description: "Activities of trade unions".to_string(),
        }),
        (94910, Code::<u32> {
            code: 94910,
            parent_code: None,
            description: "Activities of religious organizations".to_string(),
        }),
        (94920, Code::<u32> {
            code: 94920,
            parent_code: None,
            description: "Activities of political organizations".to_string(),
        }),
        (94990, Code::<u32> {
            code: 94990,
            parent_code: None,
            description: "Activities of other membership organizations n.e.c.".to_string(),
        }),
        (95110, Code::<u32> {
            code: 95110,
            parent_code: None,
            description: "Repair of computers and peripheral equipment".to_string(),
        }),
        (95120, Code::<u32> {
            code: 95120,
            parent_code: None,
            description: "Repair of communication equipment".to_string(),
        }),
        (95210, Code::<u32> {
            code: 95210,
            parent_code: None,
            description: "Repair of consumer electronics".to_string(),
        }),
        (95220, Code::<u32> {
            code: 95220,
            parent_code: None,
            description: "Repair of household appliances and home and garden equipment".to_string(),
        }),
        (95230, Code::<u32> {
            code: 95230,
            parent_code: None,
            description: "Repair of footwear and leather goods".to_string(),
        }),
        (95240, Code::<u32> {
            code: 95240,
            parent_code: None,
            description: "Repair of furniture and home furnishings".to_string(),
        }),
        (95250, Code::<u32> {
            code: 95250,
            parent_code: None,
            description: "Repair of watches, clocks and jewellery".to_string(),
        }),
        (95290, Code::<u32> {
            code: 95290,
            parent_code: None,
            description: "Repair of personal and household goods n.e.c.".to_string(),
        }),
        (96010, Code::<u32> {
            code: 96010,
            parent_code: None,
            description: "Washing and (dry-)cleaning of textile and fur products".to_string(),
        }),
        (96020, Code::<u32> {
            code: 96020,
            parent_code: None,
            description: "Hairdressing and other beauty treatment".to_string(),
        }),
        (96030, Code::<u32> {
            code: 96030,
            parent_code: None,
            description: "Funeral and related activities".to_string(),
        }),
        (96040, Code::<u32> {
            code: 96040,
            parent_code: None,
            description: "Physical well-being activities".to_string(),
        }),
        (96090, Code::<u32> {
            code: 96090,
            parent_code: None,
            description: "Other service activities n.e.c.".to_string(),
        }),
        (97000, Code::<u32> {
            code: 97000,
            parent_code: None,
            description: "Activities of households as employers of domestic personnel".to_string(),
        }),
        (98000, Code::<u32> {
            code: 98000,
            parent_code: None,
            description: "Residents property management".to_string(),
        }),
        (98100, Code::<u32> {
            code: 98100,
            parent_code: None,
            description: "Undifferentiated goods-producing activities of private households for own use".to_string(),
        }),
        (98200, Code::<u32> {
            code: 98200,
            parent_code: None,
            description: "Undifferentiated service-producing activities of private households for own use".to_string(),
        }),
        (99000, Code::<u32> {
            code: 99000,
            parent_code: None,
            description: "Activities of extraterritorial organizations and bodies".to_string(),
        }),
        (99999, Code::<u32> {
            code: 99999,
            parent_code: None,
            description: "Dormant Company".to_string(),
        }),
    ].iter().cloned().collect();
    table
}

