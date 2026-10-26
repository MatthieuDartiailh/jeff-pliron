use std::{collections::HashMap, fs, path::Path};

use jeff::reader::optype::{
    ControlFlowOp, FuncOp, GateOpType, OpType, QubitOp, QubitRegisterOp, WellKnownGate,
};
use jeff::reader::{
    Function as UpstreamFunction, FunctionIOValue as UpstreamFunctionIOValue, Module as UpstreamModule,
    Operation as UpstreamOperation, Region as UpstreamRegion, WireValue as UpstreamWireValue,
};
use jeff::types::Type as UpstreamType;

use crate::{
    attributes::StringTable,
    error::{JeffPlironError, JeffResult},
    ops::{JeffFunction, JeffModule, JeffOp, JeffOpKind, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct SerializationContext {
    pub string_table: StringTable,
    pub value_to_id: HashMap<String, usize>,
    pub value_order: Vec<String>,
    pub op_to_id: HashMap<String, usize>,
    pub region_to_id: HashMap<String, usize>,
    pub function_to_id: HashMap<String, usize>,
    pub module_metadata: Vec<crate::attributes::JeffMeta>,
    pub string_id_map: HashMap<String, usize>,

    pub value_map: HashMap<String, usize>,
    pub function_map: HashMap<String, usize>,
    pub op_map: HashMap<String, usize>,
    pub region_map: HashMap<String, usize>,
}

impl SerializationContext {
    fn sync_aliases(&mut self) {
        self.value_map = self.value_to_id.clone();
        self.function_map = self.function_to_id.clone();
        self.op_map = self.op_to_id.clone();
        self.region_map = self.region_to_id.clone();
    }

}

#[derive(Clone, Debug, Default)]
pub struct SemanticSerialization {
    pub module: JeffModule,
    pub value_map: HashMap<String, usize>,
    pub op_map: HashMap<String, usize>,
    pub function_map: HashMap<String, usize>,
    pub region_map: HashMap<String, usize>,
}

fn canonicalize_value_name(value: &JeffValue, fallback: usize) -> String {
    value.name.clone().unwrap_or_else(|| format!("v{fallback}"))
}

fn canonicalize_op_name(op: &JeffOp, fallback: usize) -> String {
    op.name.clone().unwrap_or_else(|| format!("op{fallback}"))
}

fn canonicalize_region_name(region: &JeffRegion, fallback: usize) -> String {
    region.name.clone().unwrap_or_else(|| format!("region{fallback}"))
}

fn canonicalize_function_name(function: &JeffFunction, fallback: usize) -> String {
    function.name.clone().unwrap_or_else(|| format!("func{fallback}"))
}

pub trait UpstreamValueLike {
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

pub fn serialize_module(module: &JeffModule) -> JeffResult<JeffModule> {
    let mut ctx = SerializationContext::default();
    let mut lowered = module.clone();

    lowered.strings = serialize_string_table(&module.strings)?;
    ctx.string_table = lowered.strings.clone();
    lowered.metadata = serialize_metadata(&module.metadata)?;
    ctx.module_metadata = lowered.metadata.clone();

    if lowered.name.is_none() {
        lowered.name = Some("module".to_string());
    }

    for (function_idx, function) in lowered.functions.iter_mut().enumerate() {
        let function_key = canonicalize_function_name(function, function_idx);
        if function.name.is_none() {
            function.name = Some(function_key.clone());
        }
        ctx.function_to_id.insert(function_key.clone(), function_idx);
        ctx.function_map.insert(function_key.clone(), function_idx);

        for (region_idx, region) in function.regions.iter_mut().enumerate() {
            let region_key = canonicalize_region_name(region, region_idx);
            if region.name.is_none() {
                region.name = Some(region_key.clone());
            }
            ctx.region_to_id.insert(region_key.clone(), region_idx);
            ctx.region_map.insert(region_key.clone(), region_idx);

            for (op_idx, op) in region.ops.iter_mut().enumerate() {
                let op_key = canonicalize_op_name(op, op_idx);
                if op.name.is_none() {
                    op.name = Some(op_key.clone());
                }
                ctx.op_to_id.insert(op_key.clone(), op_idx);
                ctx.op_map.insert(op_key.clone(), op_idx);

                for value in &mut op.inputs {
                    let key = canonicalize_value_name(value, ctx.value_order.len());
                    if value.name.is_none() {
                        value.name = Some(key.clone());
                    }
                    serialize_value_index(value, &mut ctx)?;
                }

                for value in &mut op.outputs {
                    let key = canonicalize_value_name(value, ctx.value_order.len());
                    if value.name.is_none() {
                        value.name = Some(key.clone());
                    }
                    serialize_value_index(value, &mut ctx)?;
                }
            }
        }

        for (value_idx, value) in function.arguments.iter_mut().enumerate() {
            let key = canonicalize_value_name(value, value_idx + ctx.value_order.len());
            if value.name.is_none() {
                value.name = Some(key.clone());
            }
            serialize_value_index(value, &mut ctx)?;
        }

        for (value_idx, value) in function.results.iter_mut().enumerate() {
            let key = canonicalize_value_name(value, value_idx + function.arguments.len() + ctx.value_order.len());
            if value.name.is_none() {
                value.name = Some(key.clone());
            }
            serialize_value_index(value, &mut ctx)?;
        }
    }

    ctx.sync_aliases();
    lowered.strings = ctx.string_table.clone();
    lowered.metadata = ctx.module_metadata.clone();
    Ok(lowered)
}

pub fn serialize_jeff_module(module: &JeffModule) -> JeffResult<JeffModule> {
    Ok(module.clone())
}

pub fn serialize_to_file(module: &JeffModule, path: impl AsRef<Path>) -> JeffResult<()> {
    let rendered = serialize_module(module)?;
    fs::write(path, format!("{rendered:#?}"))?;
    Ok(())
}

pub fn deserialize_module(module: &UpstreamModule<'_>) -> JeffResult<JeffModule> {
    deserialize_module_from_upstream(module)
}

pub fn deserialize_module_from_upstream(module: &UpstreamModule<'_>) -> JeffResult<JeffModule> {
    let functions = module
        .functions()
        .map(|function| deserialize_upstream_function(&function))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(JeffModule {
        name: None,
        strings: StringTable::new(),
        functions,
        metadata: Vec::new(),
    })
}

fn deserialize_upstream_value_like<V: UpstreamValueLike>(value: &V) -> JeffResult<JeffValue> {
    Ok(JeffValue {
        ty: JeffType::from(value.jeff_type()),
        name: None,
        index: None,
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

pub fn deserialize_module_from_jeff(module: &JeffModule) -> JeffResult<JeffModule> {
    serialize_module(module)
}

pub fn serialize_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn deserialize_type(ty: &UpstreamType) -> JeffResult<JeffType> {
    Ok(JeffType::from(*ty))
}

pub fn serialize_value(value: &JeffValue) -> JeffResult<JeffValue> {
    let mut lowered = value.clone();
    if lowered.index.is_none() {
        lowered.index = Some(0);
    }
    Ok(lowered)
}

pub fn serialize_op(op: &JeffOp) -> JeffResult<JeffOp> {
    let mut lowered = op.clone();
    if lowered.name.is_none() {
        lowered.name = Some(format!("op{}", lowered.inputs.len() + lowered.outputs.len()));
    }
    Ok(lowered)
}

pub fn deserialize_op(op: &UpstreamOperation<'_>) -> JeffResult<JeffOp> {
    deserialize_upstream_op(op)
}

pub fn deserialize_upstream_op(op: &UpstreamOperation<'_>) -> JeffResult<JeffOp> {
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
            jeff::reader::optype::IntOp::DivS | jeff::reader::optype::IntOp::DivU => JeffOpKind::IntDiv,
            jeff::reader::optype::IntOp::Pow => JeffOpKind::IntMul,
            jeff::reader::optype::IntOp::And => JeffOpKind::IntBitAnd,
            jeff::reader::optype::IntOp::Or => JeffOpKind::IntBitOr,
            jeff::reader::optype::IntOp::Xor => JeffOpKind::IntBitXor,
            jeff::reader::optype::IntOp::Not => JeffOpKind::IntBitXor,
            jeff::reader::optype::IntOp::MinS | jeff::reader::optype::IntOp::MinU => JeffOpKind::IntCmp,
            jeff::reader::optype::IntOp::MaxS | jeff::reader::optype::IntOp::MaxU => JeffOpKind::IntCmp,
            jeff::reader::optype::IntOp::Eq
            | jeff::reader::optype::IntOp::LtS
            | jeff::reader::optype::IntOp::LteS
            | jeff::reader::optype::IntOp::LtU
            | jeff::reader::optype::IntOp::LteU => JeffOpKind::IntCmp,
            jeff::reader::optype::IntOp::Abs => JeffOpKind::IntCmp,
            jeff::reader::optype::IntOp::RemS | jeff::reader::optype::IntOp::RemU => JeffOpKind::IntDiv,
            jeff::reader::optype::IntOp::Shl | jeff::reader::optype::IntOp::Shr => JeffOpKind::IntBitOr,
            jeff::reader::optype::IntOp::Select => JeffOpKind::IntSelect,
            jeff::reader::optype::IntOp::ExtS | jeff::reader::optype::IntOp::ExtU => JeffOpKind::IntSelect,
            jeff::reader::optype::IntOp::Trunc => JeffOpKind::IntSelect,
            jeff::reader::optype::IntOp::ToFloatS | jeff::reader::optype::IntOp::ToFloatU => JeffOpKind::IntCmp,
            _ => JeffOpKind::Custom,
        },
        OpType::IntArrayOp(_) => JeffOpKind::IntConst,
        OpType::FloatOp(float_op) => match float_op {
            jeff::reader::optype::FloatOp::Const32(_) | jeff::reader::optype::FloatOp::Const64(_) => {
                JeffOpKind::FloatConst
            }
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

pub fn serialize_value_index(value: &mut JeffValue, ctx: &mut SerializationContext) -> JeffResult<usize> {
    let key = value.name.clone().unwrap_or_else(|| format!("v{}", ctx.value_order.len()));
    let idx = match ctx.value_to_id.get(&key) {
        Some(&idx) => idx,
        None => {
            let idx = ctx.value_order.len();
            ctx.value_order.push(key.clone());
            ctx.value_to_id.insert(key.clone(), idx);
            ctx.value_map.insert(key.clone(), idx);
            idx
        }
    };
    if value.index.is_none() {
        value.index = Some(idx);
    }
    Ok(idx)
}

pub fn serialize_op_with_context(op: &JeffOp, ctx: &mut SerializationContext) -> JeffResult<usize> {
    let key = op.name.clone().unwrap_or_else(|| format!("op{}", ctx.op_to_id.len()));
    let idx = match ctx.op_to_id.get(&key) {
        Some(&idx) => idx,
        None => {
            let idx = ctx.op_to_id.len();
            ctx.op_to_id.insert(key.clone(), idx);
            ctx.op_map.insert(key.clone(), idx);
            idx
        }
    };
    Ok(idx)
}

pub fn serialize_float_precision(precision: FloatPrecision) -> JeffResult<u8> {
    Ok(precision.bits())
}

pub fn serialize_metadata(metadata: &[crate::attributes::JeffMeta]) -> JeffResult<Vec<crate::attributes::JeffMeta>> {
    Ok(metadata.to_vec())
}

pub fn serialize_string_table(strings: &StringTable) -> JeffResult<StringTable> {
    let mut table = StringTable::new();
    for entry in &strings.entries {
        table.intern(entry.clone());
    }
    Ok(table)
}

pub fn serialize_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    let mut lowered = function.clone();
    lowered.metadata = serialize_metadata(&function.metadata)?;
    if lowered.name.is_none() {
        lowered.name = Some(format!("func{}", lowered.arguments.len() + lowered.results.len()));
    }
    for (idx, value) in lowered.arguments.iter_mut().enumerate() {
        if value.name.is_none() {
            value.name = Some(format!("arg{idx}"));
        }
        if value.index.is_none() {
            value.index = Some(idx);
        }
    }
    for (idx, value) in lowered.results.iter_mut().enumerate() {
        if value.name.is_none() {
            value.name = Some(format!("res{idx}"));
        }
        if value.index.is_none() {
            value.index = Some(idx + lowered.arguments.len());
        }
    }
    Ok(lowered)
}

pub fn deserialize_function(function: &UpstreamFunction<'_>) -> JeffResult<JeffFunction> {
    deserialize_upstream_function(function)
}

pub fn deserialize_upstream_function(function: &UpstreamFunction<'_>) -> JeffResult<JeffFunction> {
    let name = Some(function.name().to_string());
    let mut arguments = Vec::new();
    let mut results = Vec::new();
    let mut regions = Vec::new();

    match function {
        UpstreamFunction::Declaration(decl) => {
            for value in decl.input_types() {
                let value = value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                arguments.push(deserialize_upstream_function_io(value)?);
            }
            for value in decl.output_types() {
                let value = value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                results.push(deserialize_upstream_function_io(value)?);
            }
        }
        UpstreamFunction::Definition(def) => {
            for value in def.input_types() {
                let value = value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
                arguments.push(deserialize_upstream_function_io(value)?);
            }
            for value in def.output_types() {
                let value = value.map_err(|err| JeffPlironError::DeserializationFailed(err.to_string()))?;
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

pub fn serialize_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    let mut lowered = region.clone();
    lowered.metadata = serialize_metadata(&region.metadata)?;
    if lowered.name.is_none() {
        lowered.name = Some(format!("region{}", lowered.ops.len()));
    }
    for (idx, op) in lowered.ops.iter_mut().enumerate() {
        if op.name.is_none() {
            op.name = Some(format!("op{idx}"));
        }
    }
    Ok(lowered)
}

pub fn deserialize_region(region: &UpstreamRegion<'_>) -> JeffResult<JeffRegion> {
    deserialize_upstream_region(region)
}

pub fn deserialize_upstream_region(region: &UpstreamRegion<'_>) -> JeffResult<JeffRegion> {
    let ops = region
        .operations()
        .map(|op| deserialize_upstream_op(&op).map_err(|err| JeffPlironError::DeserializationFailed(err.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(JeffRegion {
        name: None,
        ops,
        metadata: Vec::new(),
    })
}

pub fn serialize_module_semantic(module: &JeffModule) -> JeffResult<SemanticSerialization> {
    let mut value_map = HashMap::new();
    let mut op_map = HashMap::new();
    let mut function_map = HashMap::new();
    let mut region_map = HashMap::new();

    for (function_idx, function) in module.functions.iter().enumerate() {
        let function_key = function.name.clone().unwrap_or_else(|| format!("func{function_idx}"));
        function_map.insert(function_key, function_idx);

        for (region_idx, region) in function.regions.iter().enumerate() {
            let region_key = region.name.clone().unwrap_or_else(|| format!("region{region_idx}"));
            region_map.insert(region_key, region_idx);

            for (op_idx, op) in region.ops.iter().enumerate() {
                let op_key = op.name.clone().unwrap_or_else(|| format!("op{op_idx}"));
                op_map.insert(op_key, op_idx);
            }
        }

        for (value_idx, value) in function.arguments.iter().chain(function.results.iter()).enumerate() {
            if let Some(ref name) = value.name {
                value_map.insert(name.clone(), value_idx);
            }
        }
    }

    Ok(SemanticSerialization {
        module: module.clone(),
        value_map,
        op_map,
        function_map,
        region_map,
    })
}

pub fn serialize_metadata_summary(op: &JeffOp) -> JeffResult<usize> {
    Ok(op.metadata.len())
}
