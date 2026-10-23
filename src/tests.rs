use crate::{
    lowering::{lower_module, lower_type, lower_value},
    ops::{JeffOp, JeffOpKind, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[test]
fn type_semantics_round_trip() {
    let qubit = JeffType::Qubit;
    let int = JeffType::Int { bits: 8 };
    let float_array = JeffType::FloatArray {
        precision: FloatPrecision::Float64,
        length: Some(16),
    };

    assert_eq!(qubit.to_string(), "!jeff.qubit");
    assert_eq!(int.to_string(), "!jeff.int<8>");
    assert_eq!(
        float_array.to_string(),
        "!jeff.floatArray<64, static(16)>"
    );
    assert_eq!(lower_type(&qubit).unwrap(), "!jeff.qubit");
}

#[test]
fn value_lowering_keeps_semantics() {
    let value = JeffValue {
        ty: JeffType::Float { precision: FloatPrecision::Float32 },
        name: Some("angle".to_string()),
    };

    let lowered = lower_value(&value).unwrap();
    assert!(lowered.contains("angle"));
    assert!(lowered.contains("!jeff.float<32>"));
}

#[test]
fn module_lowering_is_stable() {
    let lowered = lower_module("demo").unwrap();
    assert_eq!(lowered.kind, JeffOpKind::FuncCall);
}

#[test]
fn op_metadata_round_trip() {
    let mut op = JeffOp::new(JeffOpKind::QubitAlloc);
    op.metadata.push(crate::attributes::JeffMeta {
        name: "gate.name".to_string(),
        value: "x".to_string(),
    });
    assert_eq!(op.metadata.len(), 1);
    assert_eq!(op.metadata[0].name, "gate.name");
}
