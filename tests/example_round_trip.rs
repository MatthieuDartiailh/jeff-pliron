use std::fs;

use jeff::{Jeff, reader::ReadJeff};
use jeff_pliron::lower_upstream_module;

#[test]
fn example_module_round_trips_through_pliron_dialect() {
    let bytes = fs::read("/tmp/catalyst_tket_opt.jeff").expect("example JEFF file should be available");
    let jeff = Jeff::read(bytes.as_slice()).expect("example JEFF file should parse");
    let module = jeff.module();

    let lowered = lower_upstream_module(&module).expect("module should lower to the semantic dialect");
    assert!(!lowered.functions.is_empty(), "module should contain at least one function");

    let roundtrip = jeff_pliron::raise_module_semantic(&lowered).expect("lowered module should raise back");
    assert_eq!(roundtrip.functions.len(), lowered.functions.len());
    assert_eq!(roundtrip.functions[0].name, lowered.functions[0].name);
}
