extern crate resvg;

#[macro_use]
extern crate nom;

extern crate libflate;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use nom::{be_i8, be_i32, be_i64, rest};
use nom::{le_i32};
use libflate::gzip;

use resvg::prelude::*;

#[derive(Debug)]
pub struct TimingFrame {
    time: i32,
    offset: i32,
}

#[derive(Debug)]
pub struct Frame<'a> {
    data_length: i32,
    time: i32,
    data: &'a [u8]
}

pub struct FrameSegment<'a> {
    frames: Vec<Frame<'a>>
}

#[derive(Debug)]
pub struct Webmask<'a> {
    version: i32,
    vu: i32,
    timing_frame_count: i32,
    timing_frames: Vec<TimingFrame>,
    frame_data: &'a [u8],
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\twebmask_renderer <in-webmask>");
        return;
    }

    let mut file = File::open(&args[1]).expect(&format!("Could not open file: {}", &args[1]));
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("Could not read file.");

    let buffer_size = buffer.len();

    let final_frame = TimingFrame {
        time: 0,
        offset: buffer_size as i32,
    };

    match decode_webmask(&buffer) {
        Ok((_bytes, webmask)) => {
            let initial_offset = &webmask.timing_frames[0].offset;
            let number_of_timing_frames: usize = webmask.timing_frames.len();

            for n in 0..number_of_timing_frames {
                let current_frame = &webmask.timing_frames[n];
                let next_frame = if n == number_of_timing_frames - 1 {
                    &final_frame
                } else {
                    &webmask.timing_frames[n + 1]
                };

                let start_offset = current_frame.offset - initial_offset;
                let end_offset = next_frame.offset - initial_offset;
                let slice = &webmask.frame_data[start_offset as usize..end_offset as usize];
                let mut decoder = gzip::Decoder::new(slice).unwrap();
                let mut decoded_data = Vec::new();

                decoder.read_to_end(&mut decoded_data).unwrap();

                println!("frame_segments {}-{}: decoded_size={}", start_offset, end_offset, decoded_data.len());

                match frame_segment(&decoded_data) {
                    Ok((_bytes, frame_segment)) => {
                    },
                    e => {
                        println!("Could not decode webmask frame segment")
                    }
                }
            }
        },
        e => {
            println!("Could not decode webmask file")
        }
    }
}

named!(pub timing_frame<TimingFrame>, do_parse!(
    unknown_1: be_i32
    >> time: be_i32
    >> unknown_2: be_i32
    >> offset: be_i32
    >> (TimingFrame {
        time,
        offset,
    })
));

named!(pub frame<Frame>, do_parse!(
    data_length: be_i32
    >> unknown_1: be_i32
    >> time: be_i32
    >> data: take!(data_length)
    >> (Frame {
        data_length,
        time,
        data
    })
));

named!(pub frame_segment<FrameSegment>, do_parse!(
    frames: many0!(frame)
    >> (FrameSegment {
        frames
    })
));

named!(pub decode_webmask<Webmask>, do_parse!(
    tag!([0x4D, 0x41, 0x53, 0x4B])
    >> version: be_i32
    >> vu: be_i32
    >> timing_frame_count: be_i32
    >> timing_frames: length_count!(value!(timing_frame_count), timing_frame)
    >> rest: rest
    >> (Webmask {
        version,
        vu,
        timing_frame_count,
        timing_frames,
        frame_data: rest,
    })
));
