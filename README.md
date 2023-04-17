# <span style="color:orange">IGC Parser</span>
A <span style="color:orange"><strong>*high-level*</strong></span> parsing crate for IGC flight recorder files.
<br /> With most focus on: 
- <span style="color:orange"><strong>*Easy*</strong></span> to use
- <span style="color:orange"><strong>*No run-time asserts*</strong></span> meaning that any errors will be through the ```Result``` type
- A  <span style="color:orange"><em><strong>*panic free*</strong></em></span> crate
  
You should use this crate if you want to <span style="color:orange"><strong>*easily, quickly*</strong></span> and <span style="color:orange"><strong>*safely*</strong></span> parse igc files.


For additional information on the records use https://xp-soaring.github.io/igc_file_format/igc_format_2008.html

### Example 1
Parsing all fixes (B records)
```
let file = fs::read_to_string("./examples/example.igc").unwrap().parse::<String>().unwrap();
let valid_fixes = file.lines().filter_map(|line| {
    match Record::parse(line) {
        Ok(Record::B(fix)) => Some(fix),
        _ => None,
        }
    }).collect::<Vec<Fix>>();
println!("{}", valid_fixes.len())
```

### Example 2
Parsing a single record (L record aka comment)
```
let comment = match Record::parse("LCOMMENTYCOMMENT").unwrap() {
        Record::L(comment) => comment,
        _ => panic!("This was not a comment")
    };
println!("{}", comment.content);
```

### Example 3
Parsing entire file and getting all valid fixes
```
let file = fs::read_to_string("./examples/example.igc").unwrap().parse::<String>().unwrap();
let igc_file = IGCFile::parse(&file).unwrap();
let valid_fixes = igc_file.get_fixes().clone().into_iter()
    .filter_map(|fix| match fix {
       Ok(fix) => Some(fix),
       Err(_) => None,
   }).collect::<Vec<Fix>>();
println!("{}", valid_fixes.len())
```