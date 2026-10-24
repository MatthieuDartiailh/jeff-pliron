use std::collections::HashMap;

use crate::{
    attributes::StringTable,
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::{FloatPrecision, JeffType},
};

#[derive(Clone, Debug, Default)]
pub struct LoweringContext {
    pub value_map: HashMap<String, usize>,
    pub value_order: Vec<String>,
    pub function_map: HashMap<String, usize>,
    pub op_map: HashMap<String, usize>,
    pub region_map: HashMap<String, usize>,
}

#[derive(Clone, Debug, Default)]
pub struct SemanticLowering {
    pub module: JeffModule,
    pub value_map: HashMap<String, usize>,
    pub op_map: HashMap<String, usize>,
    pub function_map: HashMap<String, usize>,
    pub region_map: HashMap<String, usize>,
}

fn canonicalize_value_name(value: &JeffValue, fallback: usize) -> String {
    value
        .name
        .clone()
        .unwrap_or_else(|| format!("v{fallback}"))
}

fn canonicalize_op_name(op: &JeffOp, fallback: usize) -> String {
    op.name.clone().unwrap_or_else(|| format!("op{fallback}"))
}

fn canonicalize_region_name(region: &JeffRegion, fallback: usize) -> String {
    region
        .name
        .clone()
        .unwrap_or_else(|| format!("region{fallback}"))
}

fn canonicalize_function_name(function: &JeffFunction, fallback: usize) -> String {
    function
        .name
        .clone()
        .unwrap_or_else(|| format!("func{fallback}"))
}

pub fn lower_module(module: &JeffModule) -> JeffResult<JeffModule> {
    let mut lowered = module.clone();
    let mut ctx = LoweringContext::default();

    for (function_idx, function) in lowered.functions.iter_mut().enumerate() {
        let function_key = canonicalize_function_name(function, function_idx);
        if function.name.is_none() {
            function.name = Some(function_key.clone());
        }
        ctx.function_map.insert(function_key, function_idx);

        for (region_idx, region) in function.regions.iter_mut().enumerate() {
            let region_key = canonicalize_region_name(region, region_idx);
            if region.name.is_none() {
                region.name = Some(region_key.clone());
            }
            ctx.region_map.insert(region_key, region_idx);

            for (op_idx, op) in region.ops.iter_mut().enumerate() {
                let op_key = canonicalize_op_name(op, op_idx);
                if op.name.is_none() {
                    op.name = Some(op_key.clone());
                }
                ctx.op_map.insert(op_key, op_idx);

                for value in &mut op.inputs {
                    let key = canonicalize_value_name(value, ctx.value_order.len());
                    if value.name.is_none() {
                        value.name = Some(key.clone());
                    }
                    lower_value_index(value, &mut ctx)?;
                }

                for value in &mut op.outputs {
                    let key = canonicalize_value_name(value, ctx.value_order.len());
                    if value.name.is_none() {
                        value.name = Some(key.clone());
                    }
                    lower_value_index(value, &mut ctx)?;
                }
            }
        }

        for (value_idx, value) in function.arguments.iter_mut().enumerate() {
            let key = canonicalize_value_name(value, value_idx);
            if value.name.is_none() {
                value.name = Some(key.clone());
            }
            lower_value_index(value, &mut ctx)?;
        }

        for (value_idx, value) in function.results.iter_mut().enumerate() {
            let key = canonicalize_value_name(value, value_idx + function.arguments.len());
            if value.name.is_none() {
                value.name = Some(key.clone());
            }
            lower_value_index(value, &mut ctx)?;
        }
    }

    Ok(lowered)
}

pub fn lower_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
}

pub fn lower_value(value: &JeffValue) -> JeffResult<JeffValue> {
    let mut lowered = value.clone();
    if lowered.index.is_none() {
        lowered.index = Some(0);
    }
    Ok(lowered)
}

pub fn lower_op(op: &JeffOp) -> JeffResult<JeffOp> {
    let mut lowered = op.clone();
    if lowered.name.is_none() {
        lowered.name = Some(format!("op{}", lowered.inputs.len() + lowered.outputs.len()));
    }
    Ok(lowered)
}

pub fn lower_value_index(value: &JeffValue, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let key = value
        .name
        .clone()
        .unwrap_or_else(|| format!("v{}", ctx.value_order.len()));
    let idx = match ctx.value_map.get(&key) {
        Some(&idx) => idx,
        None => {
            let idx = ctx.value_order.len();
            ctx.value_order.push(key.clone());
            ctx.value_map.insert(key, idx);
            idx
        }
    };
    Ok(idx)
}

pub fn lower_op_with_context(op: &JeffOp, ctx: &mut LoweringContext) -> JeffResult<usize> {
    let key = op
        .name
        .clone()
        .unwrap_or_else(|| format!("op{}", ctx.op_map.len()));
    let idx = match ctx.op_map.get(&key) {
        Some(&idx) => idx,
        None => {
            let idx = ctx.op_map.len();
            ctx.op_map.insert(key, idx);
            idx
        }
    };
    Ok(idx)
}

pub fn lower_float_precision(precision: FloatPrecision) -> JeffResult<u8> {
    Ok(precision.bits())
}

pub fn lower_function(function: &JeffFunction) -> JeffResult<JeffFunction> {
    let mut lowered = function.clone();
    if lowered.name.is_none() {
        lowered.name = Some(format!("func{}", lowered.arguments.len() + lowered.results.len()));
    }
    for (idx, value) in lowered.arguments.iter_mut().enumerate() {
        if value.name.is_none() {
            value.name = Some(format!("arg{idx}"));
        }
        value.index = Some(idx);
    }
    for (idx, value) in lowered.results.iter_mut().enumerate() {
        if value.name.is_none() {
            value.name = Some(format!("res{idx}"));
        }
        value.index = Some(idx);
    }
    Ok(lowered)
}

pub fn lower_region(region: &JeffRegion) -> JeffResult<JeffRegion> {
    let mut lowered = region.clone();
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

pub fn lower_module_semantic(module: &JeffModule) -> JeffResult<SemanticLowering> {
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

        for (value_idx, value) in function
            .arguments
            .iter()
            .chain(function.results.iter())
            .enumerate()
        {
            if let Some(ref name) = value.name {
                value_map.insert(name.clone(), value_idx);
            }
        }
    }

    Ok(SemanticLowering {
        module: module.clone(),
        value_map,
        op_map,
        function_map,
        region_map,
    })
}

pub fn lower_metadata_summary(op: &JeffOp) -> JeffResult<usize> {
    Ok(op.metadata.len())
}

pub fn lower_string_table(strings: &StringTable) -> JeffResult<StringTable> {
    Ok(strings.clone())
}
