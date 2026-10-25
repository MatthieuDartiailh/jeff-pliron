use std::{fs, path::Path};

use jeff::{Jeff, reader::ReadJeff};
use jeff_pliron::lower_upstream_module;

fn ensure_example_file(path: &str) -> std::io::Result<()> {
    if Path::new(path).exists() {
        return Ok(());
    }

    let url = "https://raw.githubusercontent.com/unitaryfoundation/jeff/main/examples/catalyst_tket_opt/catalyst_tket_opt.jeff";
    let bytes = reqwest::blocking::get(url)
        .expect("example JEFF file should be downloadable")
        .bytes()
        .expect("example JEFF file should be readable");
    fs::write(path, bytes).expect("example JEFF file should be saved");
    Ok(())
}

#[test]
fn example_module_round_trips_through_pliron_dialect() {
    let path = "/tmp/catalyst_tket_opt.jeff";
    ensure_example_file(path).expect("example JEFF file should be available");

    let bytes = fs::read(path).expect("example JEFF file should be available");
    let jeff = Jeff::read(bytes.as_slice()).expect("example JEFF file should parse");
    let module = jeff.module();

    let lowered = lower_upstream_module(&module).expect("module should deserialize to the semantic dialect");
    assert!(!lowered.functions.is_empty(), "module should contain at least one function");

    let roundtrip = jeff_pliron::raise_module_semantic(&lowered).expect("lowered module should serialize back");
    assert_eq!(roundtrip.functions.len(), lowered.functions.len());
    assert_eq!(roundtrip.functions[0].name, lowered.functions[0].name);
}
