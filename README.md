# igc_parser
A high-level parsing crate for IGC flight recorder files.
<br /> With most focus on: 
- Ease of use
- No run-time asserts meaning that any errors will be through the ```Result``` type
- Use of the ```#[no_panic]``` macro which means that the crate will never panic at runtime

You should use this crate if you want to <ins>easily</ins>, <ins>quickly</ins> and <ins>safely</ins> parse igc files.
