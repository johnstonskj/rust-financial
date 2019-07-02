from sys import argv, exit
from datetime import date
import math
import pandas as pd

def write_file_header(file_name, fetched):
    today = date.today()
    print("""/*
DO NOT MODIFY THIS FILE
=======================
Generated from:
  <https://www.iso20022.org/sites/default/files/ISO10383_MIC/ISO10383_MIC.xls>
Fetched:
  %s
Generated on:
  %d-%d-%d
*/

use crate::exchange::ExchangeRegistration;
""" % (fetched, today.year, today.month, today.day))
    df = pd.read_excel(file_name, sheet_name=8, skiprows=2, header=None)
    write_date_fn(df.iat[0,1], "last_modified")
    write_date_fn(df.iat[1,1], "next_publication")

def write_date_fn(date, name):
    print("""
pub fn get_%s() -> DateTime<Local> {
    Local.ymd(%d, %d, %d)
}""" % (name, date.year, date.month, date.day))

def write_data_table(file_name):
    print("""
fn create_mic_table() -> HashMap<String,...> {
    let table: HashMap<String,...> = 
    [
""")
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
}
""")

def get_field(row, fields, field, option=False):
    val = getattr(row, fields[field])
    if isinstance(val, float) and math.isnan(val) and not option:
        print("OH CRAP, field %s is a nan" % field)
        print(row)
        sys.exit(-1)
    elif isinstance(val, float) and math.isnan(val) and option:
        return "None"
    elif option:
        return 'Some("%s")' % val
    return '"%s"' % val
    
def write_data_from(file_name, from_sheet, fields):
    df = pd.read_excel(file_name, sheet_name=from_sheet, keep_default_na=False,
                       na_values=['', '#N/A', '#N/A N/A', '#NA', '-1.#IND', '-1.#QNAN',
                                  '-NaN', '-nan', '1.#IND', '1.#QNAN', 'N/A', 'NULL',
                                  'NaN', 'n/a', 'nan', 'null'])
    for row in df.itertuples():
        print("        (%s, ExchangeRegistration {" % get_field(row, fields, 'mic'))
        print("            country_code: %s," % get_field(row, fields, 'country_code'))
        print("            country: %s," % get_field(row, fields, 'country'))
        print("            mic: %s," % get_field(row, fields, 'mic'))
        print("            description: %s," % get_field(row, fields, 'description'))
        print("            status: %s," % get_field(row, fields, 'status'))
        
        print("            mic_type: %s," % get_field(row, fields, 'o_s', True))
        print("            city: %s," % get_field(row, fields, 'city', True))
        print("            operating_mic: %s," % get_field(row, fields, 'op_mic', True))
        print("            acronym: %s," % get_field(row, fields, 'acronym', True))
        print("            website: %s," % get_field(row, fields, 'url', True))
        print("            last_updated: %s," % get_field(row, fields, 'updated', True))
        print("            created: %s," % get_field(row, fields, 'created', True))
        print("            comments: %s" % get_field(row, fields, 'comments', True))
        print("        }),")

if len(argv) < 3:
    print("usage: python %s fetch-date xls-file" % argv[0])
else:
    file_name = argv[2]
    write_file_header(file_name, argv[1])
    write_data_table(file_name)
