use std::{collections::HashMap, fs, path::Path};

use jeff::reader::optype::{
    ControlFlowOp, FuncOp, GateOpType, OpType, QubitOp, QubitRegisterOp, WellKnownGate,
};
use jeff::reader::{
    Function as UpstreamFunction, FunctionIOValue as UpstreamFunctionIOValue,
    Module as UpstreamModule, Operation as UpstreamOperation, Region as UpstreamRegion,
    WireValue as UpstreamWireValue,
};
use jeff::types::Type as UpstreamType;
use jeff::{Jeff, reader::ReadJeff};

use crate::{
    error::{JeffPlironError, JeffResult},
    ops::{JeffFunction, JeffModule, JeffOp, JeffOpKind, JeffRegion, JeffValue},
    types::JeffType,
};

#[derive(Clone, Debug, Default)]
struct DeserializationContext {
    pub string_ids: Vec<String>,
    pub value_ids: HashMap<String, usize>,
    pub op_ids: HashMap<String, usize>,
    pub function_ids: HashMap<String, usize>,
    pub name_to_index: HashMap<String, usize>,
    pub value_to_index: HashMap<String, usize>,
    pub op_to_index: HashMap<String, usize>,
    pub function_to_index: HashMap<String, usize>,
}

impl DeserializationContext {
    fn register_string(&mut self, value: &str) -> usize {
        match self.string_ids.iter().position(|entry| entry == value) {
            Some(idx) => idx,
            None => {
                self.string_ids.push(value.to_string());
                self.string_ids.len() - 1
            }
        }
    }

    fn canonical_value_name(value: &JeffValue, fallback: usize) -> String {
        value.name.clone().unwrap_or_else(|| format!("v{fallback}"))
    }

    fn canonical_op_name(op: &JeffOp, fallback: usize) -> String {
        op.name.clone().unwrap_or_else(|| format!("op{fallback}"))
    }

    fn canonical_region_name(region: &JeffRegion, fallback: usize) -> String {
        region
            .name
            .clone()
            .unwrap_or_else(|| format!("region{fallback}"))
    }

    fn canonical_function_name(function: &JeffFunction, fallback: usize) -> String {
        function
            .name
            .clone()
            .unwrap_or_else(|| format!("func{fallback}"))
    }
}

pub fn deserialize_module(module: &JeffModule) -> JeffResult<JeffModule> {
    let mut ctx = DeserializationContext::default();
    let mut rebuilt = JeffModule {
        name: module.name.clone().or(Some("module".to_string())),
        strings: crate::attributes::StringTable::new(),
        functions: Vec::new(),
        metadata: module.metadata.clone(),
    };

    let mut rebuilt_strings = crate::attributes::StringTable::new();
    for entry in &module.strings.entries {
        rebuilt_strings.intern(entry.clone());
        ctx.register_string(entry);
    }
    rebuilt.strings = rebuilt_strings;

    for (idx, function) in module.functions.iter().enumerate() {
        let key = DeserializationContext::canonical_function_name(function, idx);
        ctx.function_ids.insert(key.clone(), idx);
        ctx.function_to_index.insert(key.clone(), idx);
        rebuilt.functions.push(deserialize_function(function)?);
    }

    for (function_idx, function) in module.functions.iter().enumerate() {
        let function_key = DeserializationContext::canonical_function_name(function, function_idx);
        let rebuilt_function = &mut rebuilt.functions[function_idx];
        rebuilt_function.name = Some(function_key.clone());

        for (region_idx, region) in function.regions.iter().enumerate() {
            let region_key = DeserializationContext::canonical_region_name(region, region_idx);
            ctx.name_to_index.insert(region_key.clone(), region_idx);
            let rebuilt_region = &mut rebuilt_function.regions[region_idx];
            rebuilt_region.name = Some(region_key.clone());

            for (op_idx, op) in region.ops.iter().enumerate() {
                let op_key = DeserializationContext::canonical_op_name(op, op_idx);
                ctx.op_ids.insert(op_key.clone(), op_idx);
                ctx.op_to_index.insert(op_key.clone(), op_idx);
                let rebuilt_op = &mut rebuilt_region.ops[op_idx];
                rebuilt_op.name = Some(op_key.clone());

                for (value_idx, value) in op.inputs.iter().chain(op.outputs.iter()).enumerate() {
                    let key = DeserializationContext::canonical_value_name(value, value_idx);
                    ctx.value_ids.insert(key.clone(), value_idx);
                    ctx.value_to_index.insert(key.clone(), value_idx);
                }
            }
        }

        for (value_idx, value) in function
            .arguments
            .iter()
            .chain(function.results.iter())
            .enumerate()
        {
            let key = DeserializationContext::canonical_value_name(value, value_idx);
            ctx.value_ids.insert(key.clone(), value_idx);
            ctx.value_to_index.insert(key.clone(), value_idx);
        }
    }

    for function in &mut rebuilt.functions {
        for (idx, value) in function.arguments.iter_mut().enumerate() {
            let key = DeserializationContext::canonical_value_name(value, idx);
            if value.name.is_none() {
                value.name = Some(key.clone());
            }
            value.index = Some(idx);
        }
        for (idx, value) in function.results.iter_mut().enumerate() {
            let key =
                DeserializationContext::canonical_value_name(value, idx + function.arguments.len());
            if value.name.is_none() {
                value.name = Some(key.clone());
            }
            value.index = Some(idx + function.arguments.len());
        }
        for region in &mut function.regions {
            for op in &mut region.ops {
                let rebuilt_op = deserialize_op(op)?;
                *op = rebuilt_op;
            }
        }
    }

    Ok(rebuilt)
}

pub fn deserialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    deserialize_module(module)
}

pub fn deserialize_from_file(path: impl AsRef<Path>) -> JeffResult<JeffModule> {
    let bytes = fs::read(path)?;
    let jeff = Jeff::read(bytes.as_slice())?;
    let module = jeff.module();
    deserialize_module_from_upstream(&module)
}

fn deserialize_upstream_value_like<V: UpstreamValueLike>(value: &V) -> JeffResult<JeffValue> {
    Ok(JeffValue {
        ty: JeffType::from(value.jeff_type()),
        name: None,
        index: None,
    })
}

trait UpstreamValueLike {
    fn jeff_type(&self) -> UpstreamType;
}

impl<'a> UpstreamValueLike for UpstreamWireValue<'a> {
    fn jeff_type(&self) -> UpstreamType {
        self.ty()
    }
}

impl<'a> UpstreamValueLike for UpstreamFunctionIOValue<'a> {
    fn jeff_type(&self) -> UpstreamType {
        self.ty()
    }
}

pub(crate) fn deserialize_module_from_upstream(
    module: &UpstreamModule<'_>,
) -> JeffResult<JeffModule> {
    let functions = module
        .functions()
        .map(|function| deserialize_upstream_function(&function))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(JeffModule {
        name: None,
        strings: crate::attributes::StringTable::new(),
        functions,
        metadata: Vec::new(),
    })
}

fn deserialize_upstream_function_io(value: UpstreamFunctionIOValue<'_>) -> JeffResult<JeffValue> {
    deserialize_upstream_value_like(&value)
}

fn deserialize_upstream_gate_kind(gate_type: GateOpType<'_>) -> JeffOpKind {
    match gate_type {
        GateOpType::WellKnown(WellKnownGate::GPhase) => JeffOpKind::GPhase,
        GateOpType::WellKnown(WellKnownGate::I) => JeffOpKind::I,
        GateOpType::WellKnown(WellKnownGate::X) => JeffOpKind::X,
        GateOpType::WellKnown(WellKnownGate::Y) => JeffOpKind::Y,
        GateOpType::WellKnown(WellKnownGate::Z) => JeffOpKind::Z,
        GateOpType::WellKnown(WellKnownGate::S) => JeffOpKind::S,
        GateOpType::WellKnown(WellKnownGate::T) => JeffOpKind::T,
        GateOpType::WellKnown(WellKnownGate::R1) => JeffOpKind::R1,
        GateOpType::WellKnown(WellKnownGate::Rx) => JeffOpKind::Rx,
        GateOpType::WellKnown(WellKnownGate::Ry) => JeffOpKind::Ry,
        GateOpType::WellKnown(WellKnownGate::Rz) => JeffOpKind::Rz,
        GateOpType::WellKnown(WellKnownGate::H) => JeffOpKind::H,
        GateOpType::WellKnown(WellKnownGate::U) => JeffOpKind::U,
        GateOpType::WellKnown(WellKnownGate::Swap) => JeffOpKind::Swap,
        GateOpType::WellKnown(_) => JeffOpKind::Custom,
        GateOpType::Custom { name, .. } => match name.to_ascii_lowercase().as_str() {
            "cx" | "cnot" => JeffOpKind::X,
            "swap" => JeffOpKind::Swap,
            "t" => JeffOpKind::T,
            "s" => JeffOpKind::S,
            "h" => JeffOpKind::H,
            "x" => JeffOpKind::X,
            "y" => JeffOpKind::Y,
            "z" => JeffOpKind::Z,
            "r1" => JeffOpKind::R1,
            "rx" => JeffOpKind::Rx,
            "ry" => JeffOpKind::Ry,
            "rz" => JeffOpKind::Rz,
            "u" => JeffOpKind::U,
            _ => JeffOpKind::Custom,
        },
        GateOpType::PauliProdRotation { .. } => JeffOpKind::Custom,
    }
}

fn deserialize_upstream_op(op: &UpstreamOperation<'_>) -> JeffResult<JeffOp> {
    let kind = match op.op_type() {
        OpType::QubitOp(qubit_op) => match qubit_op {
            QubitOp::Alloc => JeffOpKind::QubitAlloc,
            QubitOp::Free => JeffOpKind::QubitFree,
            QubitOp::FreeZero => JeffOpKind::QubitFreeZero,
            QubitOp::Measure => JeffOpKind::QubitMeasure,
            QubitOp::MeasureNd => JeffOpKind::QubitMeasureNd,
            QubitOp::Reset => JeffOpKind::QubitReset,
            QubitOp::Gate(gate) => deserialize_upstream_gate_kind(gate.gate_type),
            _ => JeffOpKind::Custom,
        },
        OpType::QubitRegisterOp(qureg_op) => match qureg_op {
            QubitRegisterOp::Alloc => JeffOpKind::QuregAlloc,
            QubitRegisterOp::Free => JeffOpKind::QuregFree,
            QubitRegisterOp::FreeZero => JeffOpKind::QuregFreeZero,
            QubitRegisterOp::ExtractIndex => JeffOpKind::QuregExtractIndex,
            QubitRegisterOp::InsertIndex => JeffOpKind::QuregInsertIndex,
            QubitRegisterOp::ExtractSlice => JeffOpKind::QuregExtractSlice,
            QubitRegisterOp::InsertSlice => JeffOpKind::QuregInsertSlice,
            QubitRegisterOp::Length => JeffOpKind::QuregLength,
            QubitRegisterOp::Split => JeffOpKind::QuregSplit,
            QubitRegisterOp::Join => JeffOpKind::QuregJoin,
            QubitRegisterOp::Create => JeffOpKind::QuregCreate,
            _ => JeffOpKind::Custom,
        },
        OpType::IntOp(int_op) => match int_op {
            jeff::reader::optype::IntOp::Const1(_) => JeffOpKind::IntConst,
            jeff::reader::optype::IntOp::Const8(_) => JeffOpKind::IntConst,
            jeff::reader::optype::IntOp::Const16(_) => JeffOpKind::IntConst,
            jeff::reader::optype::IntOp::Const32(_) => JeffOpKind::IntConst,
            jeff::reader::optype::IntOp::Const64(_) => JeffOpKind::IntConst,
            jeff::reader::optype::IntOp::Add => JeffOpKind::IntAdd,
            jeff::reader::optype::IntOp::Sub => JeffOpKind::IntSub,
            jeff::reader::optype::IntOp::Mul => JeffOpKind::IntMul,
            jeff::reader::optype::IntOp::DivS | jeff::reader::optype::IntOp::DivU => {
                JeffOpKind::IntDiv
            }
            jeff::reader::optype::IntOp::Pow => JeffOpKind::IntMul,
            jeff::reader::optype::IntOp::And => JeffOpKind::IntBitAnd,
            jeff::reader::optype::IntOp::Or => JeffOpKind::IntBitOr,
            jeff::reader::optype::IntOp::Xor => JeffOpKind::IntBitXor,
            jeff::reader::optype::IntOp::Not => JeffOpKind::IntBitXor,
            jeff::reader::optype::IntOp::MinS | jeff::reader::optype::IntOp::MinU => {
                JeffOpKind::IntCmp
            }
            jeff::reader::optype::IntOp::MaxS | jeff::reader::optype::IntOp::MaxU => {
                JeffOpKind::IntCmp
            }
            jeff::reader::optype::IntOp::Eq
            | jeff::reader::optype::IntOp::LtS
            | jeff::reader::optype::IntOp::LteS
            | jeff::reader::optype::IntOp::LtU
            | jeff::reader::optype::IntOp::LteU => JeffOpKind::IntCmp,
            jeff::reader::optype::IntOp::Abs => JeffOpKind::IntCmp,
            jeff::reader::optype::IntOp::RemS | jeff::reader::optype::IntOp::RemU => {
                JeffOpKind::IntDiv
            }
            jeff::reader::optype::IntOp::Shl | jeff::reader::optype::IntOp::Shr => {
                JeffOpKind::IntBitOr
            }
            jeff::reader::optype::IntOp::Select => JeffOpKind::IntSelect,
            jeff::reader::optype::IntOp::ExtS | jeff::reader::optype::IntOp::ExtU => {
                JeffOpKind::IntSelect
            }
            jeff::reader::optype::IntOp::Trunc => JeffOpKind::IntSelect,
            jeff::reader::optype::IntOp::ToFloatS | jeff::reader::optype::IntOp::ToFloatU => {
                JeffOpKind::IntCmp
            }
            _ => JeffOpKind::Custom,
        },
        OpType::IntArrayOp(_) => JeffOpKind::IntConst,
        OpType::FloatOp(float_op) => match float_op {
            jeff::reader::optype::FloatOp::Const32(_)
            | jeff::reader::optype::FloatOp::Const64(_) => JeffOpKind::FloatConst,
            jeff::reader::optype::FloatOp::Add | jeff::reader::optype::FloatOp::Sub => {
                JeffOpKind::FloatAdd
            }
            jeff::reader::optype::FloatOp::Mul | jeff::reader::optype::FloatOp::Div => {
                JeffOpKind::FloatMul
            }
            jeff::reader::optype::FloatOp::Eq
            | jeff::reader::optype::FloatOp::Lt
            | jeff::reader::optype::FloatOp::Lte
            | jeff::reader::optype::FloatOp::Max
            | jeff::reader::optype::FloatOp::Min => JeffOpKind::FloatCmp,
            _ => JeffOpKind::FloatConst,
        },
        OpType::FloatArrayOp(_) => JeffOpKind::FloatConst,
        OpType::ControlFlowOp(control_flow) => match *control_flow {
            ControlFlowOp::Switch(_) => JeffOpKind::ScfSwitch,
            ControlFlowOp::For { .. } => JeffOpKind::ScfFor,
            ControlFlowOp::While { .. } => JeffOpKind::ScfWhile,
        },
        OpType::FuncOp(FuncOp { .. }) => JeffOpKind::FuncCall,
        _ => JeffOpKind::Custom,
    };

    let mut inputs = Vec::new();
    for value in op.inputs() {
        let value = value.map_err(|err| JeffPlironError::InvalidValueMapping(err.to_string()))?;
        inputs.push(deserialize_upstream_value_like(&value)?);
    }

    let mut outputs = Vec::new();
    for value in op.outputs() {
        let value = value.map_err(|err| JeffPlironError::InvalidValueMapping(err.to_string()))?;
        outputs.push(deserialize_upstream_value_like(&value)?);
    }

    Ok(JeffOp {
        kind,
        name: None,
        inputs,
        outputs,
        metadata: Vec::new(),
    })
}

fn deserialize_upstream_function(function: &UpstreamFunction<'_>) -> JeffResult<JeffFunction> {
    let name = Some(function.name().to_string());
    let mut arguments = Vec::new();
    let mut results = Vec::new();
    let mut regions = Vec::new();

    match function {
        UpstreamFunction::Declaration(decl) => {
            for value in decl.input_types() {
                let value =
                    value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                arguments.push(deserialize_upstream_function_io(value)?);
            }
            for value in decl.output_types() {
                let value =
                    value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                results.push(deserialize_upstream_function_io(value)?);
            }
        }
        UpstreamFunction::Definition(def) => {
            for value in def.input_types() {
                let value =
                    value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                arguments.push(deserialize_upstream_function_io(value)?);
            }
            for value in def.output_types() {
                let value =
                    value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                results.push(deserialize_upstream_function_io(value)?);
            }
            regions.push(deserialize_upstream_region(&def.body())?);
        }
    }

    Ok(JeffFunction {
        name,
        arguments,
        results,
        regions,
        metadata: Vec::new(),
    })
}

fn deserialize_upstream_region(region: &UpstreamRegion<'_>) -> JeffResult<JeffRegion> {
    let ops = region
        .operations()
        .map(|op| {
            deserialize_upstream_op(&op)
                .map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(JeffRegion {
        name: None,
        ops,
        metadata: Vec::new(),
    })
}

pub fn deserialize_value(value: &JeffValue) -> JeffResult<JeffValue> {
    let mut rebuilt = value.clone();
    if rebuilt.name.is_none() {
        rebuilt.name = Some(format!("v{}", rebuilt.index.unwrap_or(0)));
    }
    if rebuilt.index.is_none() {
        rebuilt.index = Some(0);
    }
    Ok(rebuilt)
}

pub fn deserialize_op(op: &JeffOp) -> JeffResult<JeffOp> {
    let mut rebuilt = op.clone();
    if rebuilt.name.is_none() {
        rebuilt.name = Some(format!(
            "op{}",
            rebuilt.inputs.len() + rebuilt.outputs.len()
        ));
    }

    let mut next_index = 0usize;
    for value in &mut rebuilt.inputs {
        if value.name.is_none() {
            value.name = Some(format!("v{next_index}"));
        }
        value.index = Some(next_index);
        next_index += 1;
    }

    for value in &mut rebuilt.outputs {
        if value.name.is_none() {
            value.name = Some(format!("v{next_index}"));
        }
        value.index = Some(next_index);
        next_index += 1;
    }

    Ok(rebuilt)
}

pub fn deserialize_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn deserialize_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    let mut rebuilt = function.clone();
    if rebuilt.name.is_none() {
        rebuilt.name = Some(format!(
            "func{}",
            rebuilt.arguments.len() + rebuilt.results.len()
        ));
    }
    for (idx, value) in rebuilt.arguments.iter_mut().enumerate() {
        if value.name.is_none() {
            value.name = Some(format!("arg{idx}"));
        }
        value.index = Some(idx);
    }
    for (idx, value) in rebuilt.results.iter_mut().enumerate() {
        if value.name.is_none() {
            value.name = Some(format!("res{idx}"));
        }
        value.index = Some(idx + rebuilt.arguments.len());
    }
    for (idx, region) in rebuilt.regions.iter_mut().enumerate() {
        if region.name.is_none() {
            region.name = Some(format!("region{idx}"));
        }
        for (op_idx, op) in region.ops.iter_mut().enumerate() {
            if op.name.is_none() {
                op.name = Some(format!("op{op_idx}"));
            }
            let rebuilt_op = deserialize_op(op)?;
            *op = rebuilt_op;
        }
    }
    Ok(rebuilt)
}

pub fn deserialize_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    let mut rebuilt = region.clone();
    if rebuilt.name.is_none() {
        rebuilt.name = Some(format!("region{}", rebuilt.ops.len()));
    }
    for (idx, op) in rebuilt.ops.iter_mut().enumerate() {
        if op.name.is_none() {
            op.name = Some(format!("op{idx}"));
        }
        let rebuilt_op = deserialize_op(op)?;
        *op = rebuilt_op;
    }
    Ok(rebuilt)
}
