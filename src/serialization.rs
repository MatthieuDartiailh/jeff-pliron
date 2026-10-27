use std::{collections::HashMap, fs, path::Path};

use crate::{
    attributes::StringTable,
    error::JeffResult,
    ops::{JeffFunction, JeffModule, JeffOp, JeffRegion, JeffValue},
    types::JeffType,
};

#[derive(Clone, Debug, Default)]
struct SerializationContext {
    pub string_table: StringTable,
    pub value_to_id: HashMap<String, usize>,
    pub value_order: Vec<String>,
    pub op_to_id: HashMap<String, usize>,
    pub region_to_id: HashMap<String, usize>,
    pub function_to_id: HashMap<String, usize>,
    pub module_metadata: Vec<crate::attributes::JeffMeta>,

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

fn canonicalize_value_name(value: &JeffValue, fallback: usize) -> String {
    value.name.clone().unwrap_or_else(|| format!("v{fallback}"))
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
        ctx.function_to_id
            .insert(function_key.clone(), function_idx);
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
            let key = canonicalize_value_name(
                value,
                value_idx + function.arguments.len() + ctx.value_order.len(),
            );
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

pub fn serialize_type(ty: &JeffType) -> JeffResult<JeffType> {
    Ok(ty.clone())
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
        lowered.name = Some(format!(
            "op{}",
            lowered.inputs.len() + lowered.outputs.len()
        ));
    }
    Ok(lowered)
}

fn serialize_value_index(value: &mut JeffValue, ctx: &mut SerializationContext) -> JeffResult<usize> {
    let key = value
        .name
        .clone()
        .unwrap_or_else(|| format!("v{}", ctx.value_order.len()));
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

pub fn serialize_metadata(
    metadata: &[crate::attributes::JeffMeta],
) -> JeffResult<Vec<crate::attributes::JeffMeta>> {
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
        lowered.name = Some(format!(
            "func{}",
            lowered.arguments.len() + lowered.results.len()
        ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ops::{JeffOp, JeffOpKind}, types::FloatPrecision};

    #[test]
    fn serialize_value_assigns_default_index_without_dropping_semantics() {
        let value = JeffValue::named(
            JeffType::Float {
                precision: FloatPrecision::Float64,
            },
            "theta",
        );

        let lowered = serialize_value(&value).unwrap();
        assert_eq!(lowered.name.as_deref(), Some("theta"));
        assert_eq!(lowered.index, Some(0));
        assert_eq!(
            lowered.ty,
            JeffType::Float {
                precision: FloatPrecision::Float64,
            }
        );
    }

    #[test]
    fn serialize_module_fills_missing_names_and_preserves_metadata() {
        let module = JeffModule {
            name: Some("demo".to_string()),
            strings: StringTable::new(),
            functions: vec![JeffFunction {
                name: None,
                arguments: vec![JeffValue::new(JeffType::Qubit)],
                results: vec![JeffValue::new(JeffType::Qubit)],
                regions: vec![JeffRegion {
                    name: None,
                    ops: vec![JeffOp::new(JeffOpKind::QubitAlloc)],
                    metadata: vec![crate::attributes::JeffMeta {
                        name: "region.meta".to_string(),
                        value: "alpha".to_string(),
                    }],
                }],
                metadata: vec![crate::attributes::JeffMeta {
                    name: "function.meta".to_string(),
                    value: "beta".to_string(),
                }],
            }],
            metadata: vec![crate::attributes::JeffMeta {
                name: "module.meta".to_string(),
                value: "gamma".to_string(),
            }],
        };

        let lowered = serialize_module(&module).unwrap();
        assert_eq!(lowered.name.as_deref(), Some("demo"));
        assert_eq!(lowered.functions[0].name.as_deref(), Some("func0"));
        assert_eq!(lowered.functions[0].regions[0].name.as_deref(), Some("region0"));
        assert_eq!(lowered.functions[0].regions[0].ops[0].name.as_deref(), Some("op0"));
        assert_eq!(lowered.metadata[0].value, "gamma");
        assert_eq!(lowered.functions[0].metadata[0].value, "beta");
        assert_eq!(lowered.functions[0].regions[0].metadata[0].value, "alpha");
        assert_eq!(lowered.functions[0].arguments[0].index, Some(0));
        assert_eq!(lowered.functions[0].results[0].index, Some(1));
    }
}

