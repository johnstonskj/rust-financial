# Code-Generation Scripts

The following scripts are used to generate modules in the fin_data crate.

* `makemic.sh` fetches the latest data on MIC and calls `mic2rust.py`
* `mic2rust.py` converts the downloaded Excel registry into `fin_data::markets::mod.rs`
* `sic2rust.py` converts downloaded CSV files for US and US SIC codes into `fin_data::classifiers::??_sic.rs`
* `naics2rust.py` converts downloaded CSV file into `fin_data::classifiers::naics.rs`
