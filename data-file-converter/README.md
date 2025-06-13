# Data file converter

## Example usage

```sh
# Convert a csv file to json format
# This should also work for filesizes > current machines memory size
rs-convert --source source.csv -–destination target.json

# Display file in browser UI
rs-convert –-source A.csv –-ui

# Show structure with a few rows of data 
rs-convert –-source A.csv –-head

rs-convert –-source A.csv –-headers=false
rs-convert –-source A.csv –-include-headers='Id,First Name'
rs-convert –-source A.csv --exclude-headers='Id'
```

## Supported file formats

[ ] CSV
[ ] JSON
[ ] XML
[ ] sqlite
[ ] duckdb
[ ] Parquet
[ ] ProtoBuf
[ ] https://avro.apache.org/

## Roadmap

- CLI-tool
- Load CSV
- Write CSV
- Load JSON
- Write JSON
