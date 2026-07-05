use cpal::{FromSample, Sample};
use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

pub fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn spec_from_config(
    config: &cpal::SupportedStreamConfig,
    bit_depth: Option<crate::config::WavBitDepth>,
) -> hound::WavSpec {
    let (bits_per_sample, sample_format) = match bit_depth {
        Some(crate::config::WavBitDepth::Bits16) => (16, hound::SampleFormat::Int),
        Some(crate::config::WavBitDepth::Bits24) => (24, hound::SampleFormat::Int),
        Some(crate::config::WavBitDepth::Bits32) => (32, hound::SampleFormat::Int),
        Some(crate::config::WavBitDepth::Float32) => (32, hound::SampleFormat::Float),
        None => match config.sample_format() {
            cpal::SampleFormat::I8 | cpal::SampleFormat::U8 => (8, hound::SampleFormat::Int),
            cpal::SampleFormat::I16 | cpal::SampleFormat::U16 => (16, hound::SampleFormat::Int),
            cpal::SampleFormat::I32 | cpal::SampleFormat::U32 | cpal::SampleFormat::I64 | cpal::SampleFormat::U64 => (32, hound::SampleFormat::Int),
            cpal::SampleFormat::F32 | cpal::SampleFormat::F64 => (32, hound::SampleFormat::Float),
            _ => (
                (config.sample_format().sample_size() * 8) as u16,
                if config.sample_format().is_float() {
                    hound::SampleFormat::Float
                } else {
                    hound::SampleFormat::Int
                },
            ),
        },
    };

    hound::WavSpec {
        // Hardcoded because channels will be always mono.
        channels: 1,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample,
        sample_format,
    }
}

pub fn write_input_data<T, U>(
    input: &[T],
    writer: &Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>,
) where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
