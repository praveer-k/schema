# Schema 
---
### Generate and fix conflicts in json schema
---
Output from an API often tends to have variable outputs. Sometimes, a field that has to be an array is a number. A field that has to be a string turns out to be a number. 

By this project I intend to accomplish the idea that if lines of json objects are having variable schema we convert such a set of JSON objects to comply to generic schema. We then use that schema to fix any conflicts that may pose due to that generic schema.

We parse JSONL file to find the generic schema and then fix conflicts and create a new file with fixed json objects with correct datatypes.

Currently implemented as,
```
cargo build
cargo run ./src/mock/test.jsonl
```

Intended API:
```
schema --generate 
       --infile="<filepath>.jsonl" 
       --outfile="<filepath>.schema.json"

schema --fix-conflicts
       --infile="<filepath>.jsonl"
       --outfile="<filepath>.corrected.jsonl"

schema --generate
       --fix-conflicts
       --infile="<filepath>.jsonl"
       --outfile="<filepath>.corrected.jsonl"
```

Also, planning to add python bindings as well.
