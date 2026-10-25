use jeff::types::{FloatPrecision as JeffFloatPrecision, Type as JeffTypeFromCrate};
use jeff_pliron::{
    JeffMeta, StringTable,
    ops::{JeffFunction, JeffModule, JeffOp, JeffOpKind, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

fn normalize_metadata(metadata: &[JeffMeta]) -> Vec<(String, String)> {
    let mut entries: Vec<_> = metadata
        .iter()
        .map(|meta| (meta.name.clone(), meta.value.clone()))
        .collect();
    entries.sort();
    entries
}

fn assert_metadata_equal(left: &[JeffMeta], right: &[JeffMeta], context: &str) {
    assert_eq!(normalize_metadata(left), normalize_metadata(right), "metadata mismatch at {context}");
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

fn assert_module_equal(left: &JeffModule, right: &JeffModule, context: &str) {
    assert_eq!(left.name, right.name, "module name mismatch at {context}");
    assert_eq!(left.functions.len(), right.functions.len(), "module function count mismatch at {context}");
    assert_eq!(left.strings.entries, right.strings.entries, "string table mismatch at {context}");
    for (idx, (left_function, right_function)) in left.functions.iter().zip(&right.functions).enumerate() {
        assert_function_equal(left_function, right_function, &format!("{context} function[{idx}]"));
    }
    assert_metadata_equal(&left.metadata, &right.metadata, context);
}

fn round_trip_module(module: &JeffModule) -> JeffModule {
    let lowered = jeff_pliron::lower_module(module).expect("module should lower");
    jeff_pliron::raise_module_semantic(&lowered).expect("module should raise back")
}

fn build_module(name: &str, ops: Vec<JeffOp>) -> JeffModule {
    JeffModule {
        name: Some(name.to_string()),
        strings: StringTable::new(),
        functions: vec![JeffFunction {
            name: Some("main".to_string()),
            arguments: vec![],
            results: vec![],
            regions: vec![JeffRegion {
                name: Some("entry".to_string()),
                ops,
                metadata: vec![
                    JeffMeta { name: "zeta".to_string(), value: "last".to_string() },
                    JeffMeta { name: "alpha".to_string(), value: "first".to_string() },
                ],
            }],
            metadata: vec![
                JeffMeta { name: "beta".to_string(), value: "second".to_string() },
                JeffMeta { name: "alpha".to_string(), value: "first".to_string() },
            ],
        }],
        metadata: vec![
            JeffMeta { name: "gamma".to_string(), value: "three".to_string() },
            JeffMeta { name: "alpha".to_string(), value: "first".to_string() },
        ],
    }
}

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
fn generator_style_qubit_family_members_round_trip() {
    let qubit_cases = [
        JeffOp::new(JeffOpKind::QubitAlloc).with_name("alloc"),
        JeffOp::new(JeffOpKind::QubitFree).with_name("free").with_input(JeffValue::named(JeffType::Qubit, "q0")),
        JeffOp::new(JeffOpKind::QubitFreeZero).with_name("free_zero").with_input(JeffValue::named(JeffType::Qubit, "q0")),
        JeffOp::new(JeffOpKind::QubitMeasure).with_name("measure").with_input(JeffValue::named(JeffType::Qubit, "q0")).with_output(JeffValue::named(JeffType::Int { bits: 1 }, "m0")),
        JeffOp::new(JeffOpKind::QubitMeasureNd).with_name("measure_nd").with_input(JeffValue::named(JeffType::Qubit, "q0")).with_output(JeffValue::named(JeffType::Qubit, "q1")).with_output(JeffValue::named(JeffType::Int { bits: 1 }, "m0")),
        JeffOp::new(JeffOpKind::QubitReset).with_name("reset").with_input(JeffValue::named(JeffType::Qubit, "q0")).with_output(JeffValue::named(JeffType::Qubit, "q1")),
        JeffOp::new(JeffOpKind::X).with_name("x").with_input(JeffValue::named(JeffType::Qubit, "q0")).with_output(JeffValue::named(JeffType::Qubit, "q1")),
        JeffOp::new(JeffOpKind::H).with_name("h").with_input(JeffValue::named(JeffType::Qubit, "q0")).with_output(JeffValue::named(JeffType::Qubit, "q1")),
        JeffOp::new(JeffOpKind::GPhase).with_name("gphase").with_input(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float64 }, "angle")).with_output(JeffValue::named(JeffType::Qubit, "q1")),
    ];

    for (idx, op) in qubit_cases.iter().enumerate() {
        let module = build_module("qubit_family", vec![op.clone()]);
        let roundtrip = round_trip_module(&module);
        assert_module_equal(&module, &roundtrip, &format!("qubit_case[{idx}]"));
    }
}

#[test]
fn generator_style_qureg_family_members_round_trip() {
    let qureg_cases = [
        JeffOp::new(JeffOpKind::QuregAlloc).with_name("alloc").with_input(JeffValue::named(JeffType::Int { bits: 32 }, "n")).with_output(JeffValue::named(JeffType::Qureg { length: None }, "r0")),
        JeffOp::new(JeffOpKind::QuregLength).with_name("length").with_input(JeffValue::named(JeffType::Qureg { length: None }, "r0")).with_output(JeffValue::named(JeffType::Int { bits: 32 }, "len")),
        JeffOp::new(JeffOpKind::QuregCreate).with_name("create").with_input(JeffValue::named(JeffType::Qubit, "q0")).with_input(JeffValue::named(JeffType::Qubit, "q1")).with_output(JeffValue::named(JeffType::Qureg { length: Some(2) }, "r0")),
        JeffOp::new(JeffOpKind::QuregSplit).with_name("split").with_input(JeffValue::named(JeffType::Qureg { length: Some(4) }, "r0")).with_input(JeffValue::named(JeffType::Int { bits: 32 }, "i")).with_output(JeffValue::named(JeffType::Qureg { length: Some(2) }, "r1")).with_output(JeffValue::named(JeffType::Qureg { length: Some(2) }, "r2")),
        JeffOp::new(JeffOpKind::QuregJoin).with_name("join").with_input(JeffValue::named(JeffType::Qureg { length: Some(2) }, "r1")).with_input(JeffValue::named(JeffType::Qureg { length: Some(2) }, "r2")).with_output(JeffValue::named(JeffType::Qureg { length: Some(4) }, "r0")),
    ];

    for (idx, op) in qureg_cases.iter().enumerate() {
        let module = build_module("qureg_family", vec![op.clone()]);
        let roundtrip = round_trip_module(&module);
        assert_module_equal(&module, &roundtrip, &format!("qureg_case[{idx}]"));
    }
}

#[test]
fn generator_style_int_and_float_family_members_round_trip() {
    let int_cases = [
        JeffOp::new(JeffOpKind::IntConst).with_name("const8").with_output(JeffValue::named(JeffType::Int { bits: 8 }, "c8")),
        JeffOp::new(JeffOpKind::IntAdd).with_name("add").with_input(JeffValue::named(JeffType::Int { bits: 32 }, "lhs")).with_input(JeffValue::named(JeffType::Int { bits: 32 }, "rhs")).with_output(JeffValue::named(JeffType::Int { bits: 32 }, "sum")),
        JeffOp::new(JeffOpKind::IntBitAnd).with_name("and").with_input(JeffValue::named(JeffType::Int { bits: 32 }, "lhs")).with_input(JeffValue::named(JeffType::Int { bits: 32 }, "rhs")).with_output(JeffValue::named(JeffType::Int { bits: 32 }, "mask")),
        JeffOp::new(JeffOpKind::IntSelect).with_name("select").with_input(JeffValue::named(JeffType::Int { bits: 1 }, "cond")).with_input(JeffValue::named(JeffType::Int { bits: 32 }, "t")).with_input(JeffValue::named(JeffType::Int { bits: 32 }, "f")).with_output(JeffValue::named(JeffType::Int { bits: 32 }, "out")),
    ];
    let float_cases = [
        JeffOp::new(JeffOpKind::FloatConst).with_name("const64").with_output(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float64 }, "c64")),
        JeffOp::new(JeffOpKind::FloatAdd).with_name("fadd").with_input(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float32 }, "lhs")).with_input(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float32 }, "rhs")).with_output(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float32 }, "sum")),
        JeffOp::new(JeffOpKind::FloatCmp).with_name("fcmp").with_input(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float64 }, "lhs")).with_input(JeffValue::named(JeffType::Float { precision: FloatPrecision::Float64 }, "rhs")).with_output(JeffValue::named(JeffType::Int { bits: 1 }, "cmp")),
    ];

    for (idx, op) in int_cases.iter().chain(float_cases.iter()).enumerate() {
        let module = build_module("numeric_family", vec![op.clone()]);
        let roundtrip = round_trip_module(&module);
        assert_module_equal(&module, &roundtrip, &format!("numeric_case[{idx}]"));
    }
}

#[test]
fn generator_style_scf_and_function_cases_round_trip() {
    let scf_case = JeffOp::new(JeffOpKind::ScfSwitch)
        .with_name("switch")
        .with_input(JeffValue::named(JeffType::Int { bits: 1 }, "cond"))
        .with_output(JeffValue::named(JeffType::Int { bits: 32 }, "res"));

    let function_case = JeffOp::new(JeffOpKind::FuncCall)
        .with_name("call_main")
        .with_input(JeffValue::named(JeffType::Int { bits: 32 }, "arg"))
        .with_output(JeffValue::named(JeffType::Int { bits: 32 }, "ret"));

    for (idx, op) in [scf_case, function_case].iter().enumerate() {
        let module = build_module("scf_family", vec![op.clone()]);
        let roundtrip = round_trip_module(&module);
        assert_module_equal(&module, &roundtrip, &format!("scf_case[{idx}]"));
    }
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
