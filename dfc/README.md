# Data format converter

## Example usage

```sh
# Convert a csv file to json format
# This should also work for filesizes > current machines memory size
dfc --source source.csv -–destination target.json

# Display file in browser UI
dfc –-source A.csv –-ui

# Show structure with a few rows of data 
dfc –-source A.csv –-head

dfc –-source A.csv –-headers=false
dfc –-source A.csv –-include-headers='Id,First Name'
dfc –-source A.csv --exclude-headers='Id'

# Defaults
# If no source is provided, read from stdin
# If no destination is provided, write to stdout
dfc # read from stdin, write to stdout

# Evaluate types of columns
dfc --source A.csv # auto-detect source format based on file extension
dfc --source A.csv --source-format csv # force source format
dfc --destination B.json # auto-detect destination format based on file extension
dfc --destination B.json --destination-format json # force destination format
```

## Supported file formats

### CSV
[ ] All columns are of type string
[ ] Types of columns are automatically detected
    Supported types:
    - string
    - number
    - boolean
    - date (YYYY-MM-DD)
    - time (HH:MM:SS)
    - datetime (YYYY-MM-DD HH:MM:SS)
    - timestamp (YYYY-MM-DD HH:MM:SS.SSS)

[ ] JSON
[ ] XML
[ ] sqlite
[ ] duckdb
[ ] Parquet
[ ] ProtoBuf
[ ] https://avro.apache.org/

## Roadmap

[x] CLI-tool
[ ] Load CSV
[ ] Write CSV
[ ] Load JSON
[ ] Write JSON
