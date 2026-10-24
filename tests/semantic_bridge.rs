use jeff::types::{FloatPrecision as JeffFloatPrecision, Type as JeffTypeFromCrate};
use jeff_pliron::{
    ops::{JeffFunction, JeffModule, JeffOp, JeffOpKind, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[test]
fn type_semantics_round_trip() {
    let qubit = JeffType::from(JeffTypeFromCrate::Qubit);
    let int = JeffType::from(JeffTypeFromCrate::Int { bits: 8 });
    let float_array = JeffType::from(JeffTypeFromCrate::FloatArray {
        precision: JeffFloatPrecision::Float64,
        length: Some(16),
    });

    assert_eq!(qubit.to_string(), "!jeff.qubit");
    assert_eq!(int.to_string(), "!jeff.int<8>");
    assert_eq!(float_array.to_string(), "!jeff.floatArray<64, static(16)>");
    assert_eq!(jeff_pliron::lower_type(&qubit).unwrap(), JeffType::Qubit);
    assert_eq!(JeffTypeFromCrate::from(qubit), JeffTypeFromCrate::Qubit);
    assert_eq!(JeffTypeFromCrate::from(int), JeffTypeFromCrate::Int { bits: 8 });
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

    let lowered = jeff_pliron::lower_value(&value).unwrap();
    assert_eq!(lowered.name, Some("angle".to_string()));
    assert_eq!(
        lowered.ty,
        JeffType::Float {
            precision: FloatPrecision::Float32,
        }
    );
}

#[test]
fn module_lowering_is_stable() {
    let module = JeffModule {
        name: Some("demo".to_string()),
        strings: jeff_pliron::StringTable::new(),
        functions: vec![JeffFunction {
            name: Some("entry".to_string()),
            arguments: vec![JeffValue::named(JeffType::Qubit, "q0")],
            results: vec![JeffValue::named(JeffType::Qubit, "q1")],
            regions: vec![JeffRegion {
                name: Some("entry_region".to_string()),
                ops: vec![JeffOp::new(JeffOpKind::QubitAlloc).with_name("alloc0")],
                metadata: vec![],
            }],
            metadata: vec![],
        }],
        metadata: vec![],
    };

    let lowered = jeff_pliron::lower_module(&module).unwrap();
    assert_eq!(lowered.name, Some("demo".to_string()));
    assert_eq!(lowered.functions.len(), 1);
    assert_eq!(lowered.functions[0].regions.len(), 1);
    assert_eq!(lowered.functions[0].arguments[0].name.as_deref(), Some("q0"));
    assert_eq!(lowered.functions[0].results[0].name.as_deref(), Some("q1"));
    assert_eq!(lowered.functions[0].regions[0].ops[0].name.as_deref(), Some("alloc0"));

    let semantic = jeff_pliron::lower_module_semantic(&module).unwrap();
    assert_eq!(semantic.function_map.get("entry"), Some(&0));
    assert_eq!(semantic.region_map.get("entry_region"), Some(&0));
    assert_eq!(semantic.op_map.get("alloc0"), Some(&0));
    assert_eq!(semantic.value_map.get("q0"), Some(&0));
    assert_eq!(semantic.value_map.get("q1"), Some(&1));
}

#[test]
fn module_roundtrips_through_dialect_boundary() {
    let module = JeffModule {
        name: Some("demo".to_string()),
        strings: jeff_pliron::StringTable::new(),
        functions: vec![JeffFunction {
            name: Some("main".to_string()),
            arguments: vec![JeffValue::named(JeffType::Qubit, "q0")],
            results: vec![JeffValue::named(JeffType::Qubit, "q1")],
            regions: vec![JeffRegion {
                name: Some("body".to_string()),
                ops: vec![
                    JeffOp::new(JeffOpKind::QubitAlloc)
                        .with_name("alloc0")
                        .with_output(JeffValue::named(JeffType::Qubit, "q2")),
                    JeffOp::new(JeffOpKind::X)
                        .with_name("gate_x")
                        .with_input(JeffValue::named(JeffType::Qubit, "q2"))
                        .with_output(JeffValue::named(JeffType::Qubit, "q3")),
                ],
                metadata: vec![],
            }],
            metadata: vec![],
        }],
        metadata: vec![],
    };

    let lowered = jeff_pliron::lower_module(&module).unwrap();
    assert_eq!(lowered.name, Some("demo".to_string()));
    assert_eq!(lowered.functions[0].name.as_deref(), Some("main"));
    assert_eq!(lowered.functions[0].regions[0].ops.len(), 2);
    assert_eq!(lowered.functions[0].regions[0].ops[0].outputs[0].name.as_deref(), Some("q2"));
    assert_eq!(lowered.functions[0].regions[0].ops[1].inputs[0].name.as_deref(), Some("q2"));

    let raised = jeff_pliron::raise_module_semantic(&lowered).unwrap();
    assert_eq!(raised.functions[0].regions[0].ops[1].semantic_name(), "jeff.x");
    assert_eq!(raised.functions[0].regions[0].ops[0].outputs[0].ty, JeffType::Qubit);
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
