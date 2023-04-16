# <span style="color:orange">IGC Parser</span>
A <span style="color:orange"><strong>*high-level*</strong></span> parsing crate for IGC flight recorder files.
<br /> With most focus on: 
- <span style="color:orange"><strong>*Easy*</strong></span> to use
- <span style="color:orange"><strong>*No run-time asserts*</strong></span> meaning that any errors will be through the ```Result``` type
- A  <span style="color:orange"><em><strong>*panic free*</strong></em></span> crate
  
You should use this crate if you want to <span style="color:orange"><strong>*easily, quickly*</strong></span> and <span style="color:orange"><strong>*safely*</strong></span> parse igc files.
### Example
```
use std::fs;
use igc_parser::records::{fix::Fix, Record};
let file = fs::read_to_string("./examples/example.igc").unwrap().parse::<String>().unwrap();
let valid_fixes = file.lines().filter_map(|line| {
    match Record::parse(line) {
        Ok(Record::B(fix)) => Some(fix),
        _ => None,
        }
    }).collect::<Vec<Fix>>();
println!("{}", valid_fixes.len())
```