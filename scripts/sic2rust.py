from sys import argv, exit
from datetime import date
import math
import pandas as pd

def write_data_table(file_name):
    print("""
// ------------------------------------------------------------------------------------------------
    
fn create_data_table() -> HashMap<u16, Code<u16>> {
    let table: HashMap<u16, Code<u16>> = 
    [""")
    write_data_from(file_name)
    print("""    ].iter().cloned().collect();
    table
}
""")

def write_data_from(file_name):
    df = pd.read_csv(file_name, escapechar='\\')
    for row in df.itertuples():
        print('        (%s, Code::<u16> {' % row.Code)
        print('            code: %s,' % row.Code)
        print('            parent_code: None,')
        print('            description: "%s".to_string(),' % row.Description)
        print('        }),')

def to_lower(s):
    s.lower()

if len(argv) < 2:
    print("usage: python %s csv-file" % argv[0])
else:
    file_name = argv[1]
    write_data_table(file_name)
