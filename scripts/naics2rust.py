from sys import argv, exit
from datetime import date
import math
import pandas as pd

def write_data_table(file_name):
    print("""
// ------------------------------------------------------------------------------------------------
// Generated Data Table
// ------------------------------------------------------------------------------------------------
    
#[allow(clippy::unreadable_literal)]
fn create_data_table() -> HashMap<u32, Code<u32>> {
    let table: HashMap<u32, Code<u32>> = 
    [""")
    write_data_from(file_name)
    print("""    ].iter().cloned().collect();
    table
}
""")

def write_data_from(file_name):
    df = pd.read_csv(file_name, escapechar='\\')
    for row in df.itertuples():
        code = row._2
        desc = row._3
        if code.find('-') >= 0:
            codes = list(map(int, code.split('-')))
            for code in range(codes[0], codes[1]+1):
                write_one_code(str(code), desc)
        else:
            write_one_code(code, desc)

def write_one_code(code, desc):
    parent = 'None' if len(code) == 2 else 'Some(%s)' % code[:-1]
    print('        (%s, Code::<u32> {' % code)
    print('            code: %s,' % code)
    print('            parent_code: %s,' % parent)
    print('            description: "%s".to_string(),' % desc)
    print('        }),')

def to_lower(s):
    s.lower()

if len(argv) < 2:
    print("usage: python %s xls-file" % argv[0])
else:
    file_name = argv[1]
    write_data_table(file_name)
