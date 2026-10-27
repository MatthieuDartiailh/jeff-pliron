# jeff-pliron

A Rust semantic bridge between the JEFF binary exchange format and the Pliron IR model.

This repository defines a JEFF-oriented semantic model for Pliron:
explicit typed values, operation families, metadata, and string tables that
mirror the JEFF schema without forcing a source-language AST onto the
representation.

## Purpose

The crate is meant to provide the first translation layer between:

- the JEFF exchange format and its structured `Module`/`Function`/`Region`/`Op`/`Value` graph
- the Pliron IR ecosystem and its dialect registration model

The public model stays close to the JEFF schema and its terminology,
which makes it useful as a semantic bridge and round-trip layer for JEFF-aware
Rust code.

## Core design

- JEFF type semantics are modeled explicitly: qubit, qureg, int, intArray, float, and floatArray
- JEFF operation families are represented in a semantic op model with typed
values and metadata
- string tables and metadata entries are preserved as explicit structures
- serialization and deserialization helpers keep the semantic graph faithful to
  the upstream JEFF representation
- the API exposes explicit boundaries between JEFF-native data and Pliron-side
  bridge structures

## Current scope

This repository is intentionally focused on the semantic bridge layer:

- parse JEFF modules and functions into structured Rust types
- preserve the JEFF graph shape and value/signature semantics
- round-trip modules and example programs through the bridge
- expose stable entry points for serializing and deserializing the semantic model
- validate behavioral stability with round-trip-oriented tests

## Repository layout

- `src/dialect.rs` — dialect registration and naming
- `src/types.rs` — JEFF semantic type model
- `src/ops.rs` — JEFF op families and value model
- `src/attributes.rs` — metadata and string table helpers
- `src/serialization.rs` — JEFF-to-semantic serialization logic
- `src/deserialization.rs` — semantic-to-JEFF deserialization helpers
- `src/error.rs` — crate error types

## Scope boundaries

This crate is not a general-purpose frontend or a full pass-integration layer yet.
Its goal is to provide a faithful JEFF semantic model for Pliron and a stable
bridge for Rust-side work built around that schema.
