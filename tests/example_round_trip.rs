use std::{fs, path::Path};

use jeff::{Jeff, reader::ReadJeff};
use jeff_pliron::{
    JeffFunction, JeffMeta, JeffModule, JeffOp, JeffRegion, JeffValue, StringTable,
    serialize_jeff_module,
};

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

fn normalize_metadata(metadata: &[JeffMeta]) -> Vec<(String, String)> {
    let mut entries: Vec<_> = metadata
        .iter()
        .map(|meta| (meta.name.clone(), meta.value.clone()))
        .collect();
    entries.sort();
    entries
}

fn assert_metadata_equal(left: &[JeffMeta], right: &[JeffMeta], context: &str) {
    assert_eq!(
        normalize_metadata(left),
        normalize_metadata(right),
        "metadata mismatch at {context}"
    );
}

fn assert_value_equal(left: &JeffValue, right: &JeffValue, context: &str) {
    assert_eq!(left.ty, right.ty, "value type mismatch at {context}");
    assert_eq!(left.name, right.name, "value name mismatch at {context}");
    assert_eq!(left.index, right.index, "value index mismatch at {context}");
}

fn assert_op_equal(left: &JeffOp, right: &JeffOp, context: &str) {
    assert_eq!(left.kind, right.kind, "op kind mismatch at {context}");
    assert_eq!(left.name, right.name, "op name mismatch at {context}");
    assert_eq!(left.inputs.len(), right.inputs.len(), "op input count mismatch at {context}");
    assert_eq!(left.outputs.len(), right.outputs.len(), "op output count mismatch at {context}");

    for (idx, (left_value, right_value)) in left.inputs.iter().zip(&right.inputs).enumerate() {
        assert_value_equal(left_value, right_value, &format!("{context} input[{idx}]"));
    }
    for (idx, (left_value, right_value)) in left.outputs.iter().zip(&right.outputs).enumerate() {
        assert_value_equal(left_value, right_value, &format!("{context} output[{idx}]"));
    }
    assert_metadata_equal(&left.metadata, &right.metadata, context);
}

fn assert_region_equal(left: &JeffRegion, right: &JeffRegion, context: &str) {
    assert_eq!(left.name, right.name, "region name mismatch at {context}");
    assert_eq!(left.ops.len(), right.ops.len(), "region op count mismatch at {context}");
    for (idx, (left_op, right_op)) in left.ops.iter().zip(&right.ops).enumerate() {
        assert_op_equal(left_op, right_op, &format!("{context} op[{idx}]"));
    }
    assert_metadata_equal(&left.metadata, &right.metadata, context);
}

fn assert_function_equal(left: &JeffFunction, right: &JeffFunction, context: &str) {
    assert_eq!(left.name, right.name, "function name mismatch at {context}");
    assert_eq!(left.arguments.len(), right.arguments.len(), "function argument count mismatch at {context}");
    assert_eq!(left.results.len(), right.results.len(), "function result count mismatch at {context}");
    assert_eq!(left.regions.len(), right.regions.len(), "function region count mismatch at {context}");

    for (idx, (left_arg, right_arg)) in left.arguments.iter().zip(&right.arguments).enumerate() {
        assert_value_equal(left_arg, right_arg, &format!("{context} arg[{idx}]"));
    }
    for (idx, (left_res, right_res)) in left.results.iter().zip(&right.results).enumerate() {
        assert_value_equal(left_res, right_res, &format!("{context} result[{idx}]"));
    }
    for (idx, (left_region, right_region)) in left.regions.iter().zip(&right.regions).enumerate() {
        assert_region_equal(left_region, right_region, &format!("{context} region[{idx}]"));
    }
    assert_metadata_equal(&left.metadata, &right.metadata, context);
}

fn assert_string_table_equal(left: &StringTable, right: &StringTable, context: &str) {
    assert_eq!(left.entries, right.entries, "string table mismatch at {context}");
}

fn assert_module_equal(left: &JeffModule, right: &JeffModule, context: &str) {
    assert_eq!(left.name, right.name, "module name mismatch at {context}");
    assert_eq!(left.functions.len(), right.functions.len(), "module function count mismatch at {context}");
    assert_string_table_equal(&left.strings, &right.strings, context);
    for (idx, (left_function, right_function)) in left.functions.iter().zip(&right.functions).enumerate() {
        assert_function_equal(left_function, right_function, &format!("{context} function[{idx}]"));
    }
    assert_metadata_equal(&left.metadata, &right.metadata, context);
}

#[test]
fn example_module_round_trips_through_pliron_dialect() {
    let path = "/tmp/catalyst_tket_opt.jeff";
    ensure_example_file(path).expect("example JEFF file should be available");

    let bytes = fs::read(path).expect("example JEFF file should be available");
    let jeff = Jeff::read(bytes.as_slice()).expect("example JEFF file should parse");
    let module = jeff.module();

    let lowered = jeff_pliron::serialization::deserialize_module_from_upstream(&module)
        .expect("module should deserialize to the semantic dialect");
    assert!(!lowered.functions.is_empty(), "module should contain at least one function");

    let roundtrip = serialize_jeff_module(&lowered).expect("lowered module should serialize back");
    assert_module_equal(&lowered, &roundtrip, "module round-trip comparison");
    assert_eq!(roundtrip.functions.len(), lowered.functions.len());
    assert_eq!(roundtrip.functions[0].name, lowered.functions[0].name);
}
