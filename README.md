# jeff-pliron

A small Rust repository that defines a semantic `jeff` dialect for the pliron IR, with a focus on preserving the JEFF binary dataflow model instead of modeling a source-language AST.

## Purpose

This crate acts as a bridge between:

- the JEFF binary exchange format and its semantic `Module`/`Function`/`Region`/`Op`/`Value` structure
- pliron's extensible IR machinery and dialect registration model

The dialect name is intentionally `jeff`, matching the schema and the existing `jeff-mlir` naming precedent.

## Core design

- JEFF type semantics are modeled explicitly: qubit, qureg, int, intArray, float, and floatArray
- JEFF operation families are represented as a semantic op model with typed values and metadata
- generic string tables and metadata entries are preserved as explicit structures
- serialization/deserialization passes keep the JEFF semantic graph deterministic and faithful
- the public API follows the JEFF exchange format terminology used by `jeff-mlir`, with explicit serialization/deserialization boundaries

## Repository layout

- `src/dialect.rs` — dialect registration and name
- `src/types.rs` — JEFF semantic type model
- `src/ops.rs` — JEFF op families and value model
- `src/attributes.rs` — metadata and string table helpers
- `src/serialization.rs` — JEFF-to-pliron serialization logic
- `src/deserialization.rs` — pliron-to-JEFF deserialization helpers
- `src/error.rs` — crate error types

## Validation

This repository is kept intentionally small and focused; the current implementation provides the semantic bridge and round-trip-oriented tests for core JEFF types, operation families, and example modules.
