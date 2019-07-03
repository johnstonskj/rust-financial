#~/usr/bin/env sh

FETCH_DATE=$(date "+%Y-%m-%d)
FILE_NAME=./mic-$FETCH_DATE.xls
curl -o $FILE_NAME "https://www.iso20022.org/sites/default/files/ISO10383_MIC/ISO10383_MIC.xls"

python xls2rust.py $FETCH_DATE $FILE_NAME >> ../fin_data/src/markets/mod.rs