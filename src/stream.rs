use crate::{wav::write_input_data, WriterHandles};
use anyhow::{bail, Result};
use cpal::{traits::DeviceTrait, FromSample, Sample};
use std::sync::{Arc, Mutex};

pub fn build(
    device: &cpal::Device,
    config: cpal::SupportedStreamConfig,
    channels_to_record: &[usize],
    writers_in_stream: Arc<Mutex<Option<WriterHandles>>>,
    bit_depth: Option<crate::config::WavBitDepth>,
) -> Result<cpal::Stream> {
    let stream_error_callback = move |err| {
        eprintln!("An error occurred on the input stream: {err}");
    };

    match config.sample_format() {
        cpal::SampleFormat::I8 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<i8, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) => Ok(device.build_input_stream(&config.into(), process::<i8, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<i8, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                None => Ok(device.build_input_stream(&config.into(), process::<i8, i8>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::U8 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<u8, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) => Ok(device.build_input_stream(&config.into(), process::<u8, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<u8, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                None => Ok(device.build_input_stream(&config.into(), process::<u8, i8>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::I16 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) | None => Ok(device.build_input_stream(&config.into(), process::<i16, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) => Ok(device.build_input_stream(&config.into(), process::<i16, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<i16, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::U16 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) | None => Ok(device.build_input_stream(&config.into(), process::<u16, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) => Ok(device.build_input_stream(&config.into(), process::<u16, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<u16, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }

        cpal::SampleFormat::I32 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<i32, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) | None => Ok(device.build_input_stream(&config.into(), process::<i32, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<i32, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::U32 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<u32, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) | None => Ok(device.build_input_stream(&config.into(), process::<u32, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<u32, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::I64 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<i64, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) | None => Ok(device.build_input_stream(&config.into(), process::<i64, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<i64, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::U64 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<u64, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) | None => Ok(device.build_input_stream(&config.into(), process::<u64, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) => Ok(device.build_input_stream(&config.into(), process::<u64, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::F32 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<f32, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) => Ok(device.build_input_stream(&config.into(), process::<f32, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) | None => Ok(device.build_input_stream(&config.into(), process::<f32, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        cpal::SampleFormat::F64 => {
            match bit_depth {
                Some(crate::config::WavBitDepth::Bits16) => Ok(device.build_input_stream(&config.into(), process::<f64, i16>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Bits24 | crate::config::WavBitDepth::Bits32) => Ok(device.build_input_stream(&config.into(), process::<f64, i32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
                Some(crate::config::WavBitDepth::Float32) | None => Ok(device.build_input_stream(&config.into(), process::<f64, f32>(channels_to_record.to_vec(), writers_in_stream), stream_error_callback, None)?),
            }
        }
        sample_format => bail!(
            "Sample format {:?} is not supported by this program.",
            sample_format
        ),
    }
}

#[allow(clippy::type_complexity)]
fn process<T, U>(
    channels_to_record: Vec<usize>,
    writers_in_stream: Arc<Mutex<Option<WriterHandles>>>,
) -> Box<dyn FnMut(&[T], &cpal::InputCallbackInfo) + Send + 'static>
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    Box::new(move |data: &[T], _: &_| {
        // We really don't do much here. We just record the data to the files.
        // So avoiding continuous allocation is not a priority.
        // We have a lot of time to do processing in every call to this function, so we can afford to do some allocation.
        // Premature optimization is the root of all evil. :)
        let mut channel_buffer = Vec::<Vec<T>>::with_capacity(channels_to_record.len());

        for _ in 0..channels_to_record.len() {
            channel_buffer.push(Vec::with_capacity(data.len()));
        }

        // Channels to record has an ascending order, so does the interleaved data.

        // Process the frame
        for frame in data.chunks(channels_to_record.len()) {
            // We have one sample for each channel in this frame since we're recording mono.

            for (channel_idx, sample) in frame.iter().enumerate() {
                // Put that sample in the corresponding channel buffer.
                // De-interleave the data in other words.
                channel_buffer[channel_idx].push(*sample);
            }
        }

        if let Some(writers) = writers_in_stream.lock().unwrap().as_ref() {
            let writers_in_stream = writers.clone();
            // Write the de-interleaved buffer to the files.
            for (channel_idx, channel_data) in channel_buffer.iter().enumerate() {
                write_input_data::<T, U>(channel_data, &writers_in_stream[channel_idx]);
            }
        }
    })
}
