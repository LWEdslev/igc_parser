use igc_parser::parser_builder::ParserBuilder;

fn main() {
    use igc_parser::parser_builder::*;
    use std::fs;
    let file = fs::read_to_string("./examples/example.igc").unwrap().parse::<String>().unwrap();
    let builder = ParserBuilder::new().parse_fixes().parse_comments().parse_task_info();
    let parsed = builder.on_file(&file)?;
    let fixes = parsed.get_fixes();
}