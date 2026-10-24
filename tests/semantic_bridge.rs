use jeff_pliron::{
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
    assert_eq!(float_array.to_string(), "!jeff.floatArray<64, static(16)>");
    assert_eq!(lower_type(&qubit).unwrap(), "!jeff.qubit");
}

#[test]
fn value_lowering_keeps_semantics() {
    let value = JeffValue {
        ty: JeffType::Float {
            precision: FloatPrecision::Float32,
        },
        name: Some("angle".to_string()),
        index: None,
    };

    let lowered = lower_value(&value).unwrap();
    assert!(lowered.contains("angle"));
    assert!(lowered.contains("!jeff.float<32>"));
}

#[test]
fn module_lowering_is_stable() {
    let lowered = lower_module("demo").unwrap();
    assert!(matches!(lowered.kind, JeffOpKind::FuncCall));
    assert_eq!(lowered.outputs.len(), 0);
}

#[test]
fn op_metadata_round_trip() {
    let mut op = JeffOp::new(JeffOpKind::QubitAlloc);
    op.metadata.push(jeff_pliron::JeffMeta {
        name: "gate.name".to_string(),
        value: "x".to_string(),
    });
    assert_eq!(op.metadata.len(), 1);
    assert_eq!(op.metadata[0].name, "gate.name");
}

#[test]
fn lowering_context_keeps_stable_value_order() {
    let mut ctx = jeff_pliron::LoweringContext::default();
    let value_a = JeffValue::named(JeffType::Qubit, "q0");
    let value_b = JeffValue::named(JeffType::Qubit, "q0");

    let idx_a = jeff_pliron::lower_value_index(&value_a, &mut ctx).unwrap();
    let idx_b = jeff_pliron::lower_value_index(&value_b, &mut ctx).unwrap();

    assert_eq!(idx_a, 0);
    assert_eq!(idx_b, 0);
    assert_eq!(ctx.value_order.len(), 1);
}
