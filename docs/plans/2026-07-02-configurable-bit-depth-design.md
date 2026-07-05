# Design: Configurable WAV Recording Bit Depth

## Overview
This design details the implementation of a configurable output bit depth feature for `smrec`. The user will be able to specify the output WAV bit depth and format (16-bit integer, 24-bit integer, 32-bit integer, and 32-bit float) using both a command-line flag (`--bit-depth`) and a configuration setting in `config.toml` (`bit_depth`). 

Furthermore, to maintain compatibility with modern audio devices and the latest versions of the `cpal` library, the program's input stream routing is updated to fully support all 12 `cpal::SampleFormat` variants (including `I24`, `U24`, `I64`, `U64`, `U8`, `U16`, `U32`, `F64`), converting them to the closest compatible target WAV formats.

## Goals
- Support custom target WAV bit depths (`16`, `24`, `32`, `f32`).
- Retain the current default behavior (native device format) when no bit depth is specified.
- Introduce configuration support via both `config.toml` and CLI flags.
- Support all 12 input formats from CPAL to ensure compatibility with modern professional audio interfaces.

## Selected Approach (Zero-cost Compile-time Dispatch)
Sample format conversion is handled during the audio stream creation by mapping the input stream's format and the target format to a typed `process::<T, U>` audio recording loop where `T` is the input sample type and `U` is the target output sample type. Because the sample conversion is done using `FromSample<T>` within Rust generics, it is fully optimized by the compiler with zero runtime overhead or heap allocations inside the audio callback loop.

## Changes

### 1. Configuration & CLI (`src/config.rs` and `src/main.rs`)
- Add `--bit-depth <16|24|32|f32>` to CLI parser.
- Define a `WavBitDepth` enum representing the supported output formats.
- Add `bit_depth` to `SmrecConfig`.
- Implement a custom deserializer `deserialize_bit_depth` for `SmrecConfig::bit_depth` that parses both TOML strings (`"16"`) and integers (`16`).
- Precedence: CLI flag overrides configuration file.

### 2. WAV Format Spec (`src/wav.rs`)
- Update `spec_from_config` to accept `Option<WavBitDepth>`.
- Map the target `hound::WavSpec` values (`bits_per_sample` and `sample_format`) depending on the choice, or fallback to the device's native format properties when `None`.

### 3. Stream Routing (`src/stream.rs`)
- Update `stream::build` to take `bit_depth: Option<WavBitDepth>`.
- Expand the match statement in `build` to cover all 12 `cpal::SampleFormat` variants:
  - `I8`, `U8` -> Target `i8`
  - `I16`, `U16` -> Target `i16`
  - `I24`, `U24`, `I32`, `U32`, `I64`, `U64` -> Target `i32`
  - `F32`, `F64` -> Target `f32`
- Map these input formats to `<T, U>` where `U` corresponds to the configured override (e.g. `i16` for `--bit-depth 16`) or the default mapping.

## Verification & Testing
- Run `cargo check` (once environment is configured or manually verified).
- Check that the program compiles with the master branch of `cpal`.
- Manually verify that specifying `--bit-depth 16` produces 16-bit WAV files.
