


fn main() {
    use std::fs;
    use igc_parser::igc_file::IGCFile;
    use igc_parser::records::fix::Fix;
    let file = fs::read_to_string("./examples/example.igc").unwrap().parse::<String>().unwrap();
    let igc_file = IGCFile::parse(&file).unwrap();
    let valid_fixes = igc_file.get_fixes().clone().into_iter().filter_map(|fix| match fix {
        Ok(fix) => Some(fix),
        Err(_) => None,
    }).collect::<Vec<Fix>>();
    println!("{}", valid_fixes.len())
}
