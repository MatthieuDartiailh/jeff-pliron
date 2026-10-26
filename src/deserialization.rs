use std::{collections::HashMap, fs, path::Path};

use jeff::{Jeff, reader::ReadJeff};

use crate::{
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct DeserializationContext {
    pub string_ids: Vec<String>,
    pub value_ids: HashMap<String, usize>,
    pub op_ids: HashMap<String, usize>,
    pub function_ids: HashMap<String, usize>,
    pub func_signature_registry: HashMap<String, Vec<usize>>,
    pub module_metadata: Vec<crate::attributes::JeffMeta>,
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
    crate::serialization::deserialize_module_from_upstream(&module)
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

pub fn deserialize_float_precision(bits: u8) -> JeffResult<FloatPrecision> {
    match bits {
        32 => Ok(FloatPrecision::Float32),
        64 => Ok(FloatPrecision::Float64),
        _ => Err(crate::error::JeffPlironError::DeserializationFailed(
            format!("unsupported float precision bits: {bits}"),
        )),
    }
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

pub fn deserialize_module_semantic(module: &JeffModule) -> JeffResult<JeffModule> {
    deserialize_module(module)
}

pub fn deserialize_string_table(entries: &[String]) -> JeffResult<Vec<String>> {
    Ok(entries.to_vec())
}
