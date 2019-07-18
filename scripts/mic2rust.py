from sys import argv, exit
from datetime import date
import math
import pandas as pd

def write_date_fn(date, name):
    print("""
    fn %s() -> NaiveDate {
        NaiveDate::from_ymd(%d, %d, %d)
    }""" % (name, date.year, date.month, date.day))

def write_data_table(file_name):
    print("""
// ------------------------------------------------------------------------------------------------
// Generated Data Table
// ------------------------------------------------------------------------------------------------

#[allow(clippy::unreadable_literal)]
fn create_data_table() -> HashMap<String, Market> {
    let table: HashMap<String, Market> = 
    [""")
    write_data_from(file_name, 0,
                    {'country': 'COUNTRY', 'country_code': '_2', 'mic': 'MIC', 'op_mic': '_4',
                     'o_s': '_5', 'description': '_6', 'acronym': 'ACRONYM', 'city': 'CITY',
                     'url': 'WEBSITE', 'updated': '_10', 'status': 'STATUS', 'created': '_12',
                     'comments': 'COMMENTS'})
    write_data_from(file_name, 7,
                    {'country': 'COUNTRY', 'country_code': '_2', 'mic': 'MIC',
                     'description': '_4', 'op_mic': '_5', 'o_s': '_6', 'acronym': 'ACRONYM',
                     'city': 'CITY', 'url': 'WEBSITE', 'updated': '_10', 'status': 'STATUS',
                     'created': '_12', 'comments': 'COMMENTS'})
    print("""
    ].iter().cloned().collect();
    table
}
""")

def get_field(row, fields, field, option=False, transformer=None):
    val = getattr(row, fields[field])
    if isinstance(val, float) and math.isnan(val) and not option:
        print("OH CRAP, field %s is a nan" % field)
        print(row)
        sys.exit(-1)
    elif isinstance(val, float) and math.isnan(val) and option:
        return "None"
    elif option and not transformer is None:
        new_val = transformer(val)
        if new_val == 'None':
            return new_val
        else:
            return 'Some(%s)' % new_val
    elif option and transformer is None:
        return 'Some("%s".to_string())' % val
    elif not transformer is None:
        return transformer(val)
    return '"%s"' % val
    
def write_data_from(file_name, from_sheet, fields):
    df = pd.read_excel(file_name, sheet_name=from_sheet, keep_default_na=False,
                       na_values=['', '#N/A', '#N/A N/A', '#NA', '-1.#IND', '-1.#QNAN',
                                  '-NaN', '-nan', '1.#IND', '1.#QNAN', 'N/A', 'NULL',
                                  'NaN', 'n/a', 'nan', 'null'])
    for row in df.itertuples():
        print("        (%s.to_string(), Market {" % get_field(row, fields, 'mic'))
        print("            country_code: %s.to_string()," % get_field(row, fields, 'country_code'))
        print("            country: %s.to_string()," % get_field(row, fields, 'country'))
        print("            mic: %s.to_string()," % get_field(row, fields, 'mic'))
        print("            description: %s.to_string()," % get_field(row, fields, 'description'))
        print("            status: %s," % get_field(row, fields, 'status', True, make_status))
        
        print("            mic_type: %s," % get_field(row, fields, 'o_s', True))
        print("            city: %s," % get_field(row, fields, 'city', True))
        print("            operating_mic: %s," % get_field(row, fields, 'op_mic', True))
        print("            acronym: %s," % get_field(row, fields, 'acronym', True))
        print("            website: %s," % get_field(row, fields, 'url', True, to_lower))
        print("            last_updated: %s," % get_field(row, fields, 'updated', True, make_date))
        print("            created: %s," % get_field(row, fields, 'created', True, make_date))
        print("            comments: %s" % get_field(row, fields, 'comments', True, to_lower))
        print("        }),")

def to_lower(s):
    return '"%s".to_string()' % s.lower().replace('"', '\\"')

MONTHS = {'JANUARY': 1, 'FEBRUARY': 2, 'MARCH': 3,
          'APRIL': 4, 'MAY': 5, 'JUNE': 6,
          'JULY': 7, 'AUGUST': 8, 'SEPTEMBER': 9,
          'OCTOBER': 10, 'NOVEMBER': 11, 'DECEMBER': 12}

def make_date(s):
    ds = s.split(' ')
    if ds[0] == 'BEFORE':
        return 'None'
    else:
        return 'NaiveDate::from_ymd(%s, %s, 1)' % (ds[1], MONTHS[ds[0]])

def make_status(s):
    if s == "ACTIVE":
        return 'MarketStatus::Active'
    elif s == "DELETED":
        return 'MarketStatus::Deleted'
    else:
        return 'MarketStatus::NotOperational'

if len(argv) < 3:
    print("usage: python %s fetch-date xls-file" % argv[0])
else:
    file_name = argv[2]
    write_data_table(file_name)
