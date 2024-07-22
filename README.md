# IGC Parser
A parsing crate for IGC flight recorder files.
Features: 
- No run-time asserts meaning that any errors will be through the ```Result``` type
- Owned types, meaning no lifetimes
- `Rc<str>` and `Arc<str>` for immutable string types to improve cloning efficiency compared to `String`
- Builder to parse specific items efficiently

For additional information on the records use https://xp-soaring.github.io/igc_file_format/igc_format_2008.html

### Example: Specific kind of records
Use builder pattern to parse only specific kinds of records, this is more efficient than parsing everything
```rust
let file = fs::read_to_string("./examples/example.igc")?;
let builder = parser_builder::new_builder()
    .parse_a_records()
    .parse_e_records()
    .parse_b_records();
let parsed = builder.on_file(&file)?;

// Then we can get the records we specified
let a_records = parsed.get_a_records();
let e_records = parsed.get_e_records();
let b_records = parsed.get_b_records();

// NOTE This below does not compile since our builder was not set to parse C records
// let c_records = parsed.get_c_records();
```

### Example: Series of records
Parsing all fixes (B records)
```rust
let file = fs::read_to_string("./examples/example.igc")?;
let valid_fixes: Vec<Fix> = file
    .lines()
    .filter_map(|line| {
        if let Record::B(fix) = Record::parse(line).ok()? { 
            Some(fix) 
        } else { 
            None
        }
    })
    .collect();
println!("{}", valid_fixes.len());
```

### Example: Single record
Parsing a single record (L record aka comment)
```rust
let comment = match Record::parse("LCOMMENTYCOMMENT").unwrap() {
        Record::L(comment) => comment,
        _ => panic!("This was not a comment")
    };
println!("{}", comment.content);
```

### Example: Entire file
Parsing entire file and getting all valid fixes
```rust
let file = fs::read_to_string("./examples/example.igc")?;
let igc_file = IGCFile::parse(&file)?;
let valid_fixes: Vec<Fix> = igc_file
    .get_fixes()
    .clone()
    .into_iter()
    .filter_map(|fix| fix.ok())
    .collect();
println!("{}", valid_fixes.len());
```

### New in 0.1.6
- Added typestate pattern for the builder to avoid returning `Option`s
- Changed from `String` to `Rc<str>` to allow more efficient cloning
- Added `thread-safe` feature where the string types will be `Arc<str>`