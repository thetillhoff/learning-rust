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

# Customize headers
dfc –-source A.csv –-source-has-headers=false
dfc --source A.csv --destination-headers='Id,First Name'
dfc –-source A.csv –-include-header-by-name='Id,First Name'
dfc –-source A.csv –-include-header-by-index='0,2'
dfc –-source A.csv --exclude-header-by-name='Id'
dfc –-source A.csv --exclude-header-by-index='0,2'

# Defaults
# If no source is provided, read from stdin
# If no destination is provided, write to stdout
dfc --source-format csv --destination-format json # read from stdin, write to stdout

# Evaluate types of columns
dfc --source A.csv # auto-detect source format based on file extension
dfc --source A.csv --source-format csv # force source format
dfc --destination B.json # auto-detect destination format based on file extension
dfc --destination B.json --destination-format json # force destination format

# Database paths (idea)
dfc --source sqlite://path/to/database.db --destination target.json
dfc --source sqlite://path/to/database.db --destination json://target.dump
```

## Supported file formats

### CSV

[x] All columns are of type string
[ ] Types of columns are automatically detected
    Supported types:
    - string
    - number
    - boolean
    - date (YYYY-MM-DD)
    - time (HH:MM:SS)
    - datetime (YYYY-MM-DD HH:MM:SS)
    - timestamp (YYYY-MM-DD HH:MM:SS.SSS)

### JSON

[ ] can read list of lists (like csv, might contain header)
[ ] can read list of objects with named fields

### Others

[ ] fixed column width format
[ ] JSON
[ ] XML
[ ] sqlite
[ ] duckdb
[ ] Parquet
[ ] ProtoBuf
[ ] [Avro](https://avro.apache.org/)

## Roadmap

[x] CLI-tool
[x] Read file line by line
[ ] Write file line by line
[ ] Add tests
[ ] Load CSV
    [ ] Does the file a header row?
    [ ] What's the delimiter?
    [ ] What's the line ending?
    [ ] Is there an empty line at the end of the file?
[ ] Write CSV
[ ] Analyze types of columns
[ ] Read from stdin
[ ] Write to stdout
[ ] Read input stream character by character (like if json strings are streamed)
[ ] Write output stream character by character (like if json strings are streamed)
[ ] Load JSON
[ ] Write JSON
