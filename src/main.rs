extern crate resvg;

#[macro_use]
extern crate nom;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use nom::{be_i8, be_i32, be_i64};

use resvg::prelude::*;

#[derive(Debug)]
pub struct TimingFrame {
    time: i32,
    offset: i32,
}

#[derive(Debug)]
pub struct Webmask {
    version: i32,
    vu: i32,
    timing_frame_count: i32,
    timing_frames: Vec<TimingFrame>
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

    println!("{} bytes read", buffer.len());

    match decode_webmask(&buffer) {
        Ok((_bytes, webmask)) => {
            println!("{:#?}", webmask);
        },
        e => {
            println!("Could not decode webmask file")
        }
    }
}

named!(pub timing_frame<TimingFrame>, do_parse!(
    test: be_i32
    >> time: be_i32
    >> test: be_i32
    >> offset: be_i32
    >> (TimingFrame {
        time,
        offset,
    })
));

named!(pub decode_webmask<Webmask>, do_parse!(
    tag!([0x4D, 0x41, 0x53, 0x4B])
    >> version: be_i32
    >> vu: be_i32
    >> timing_frame_count: be_i32
    >> timing_frames: length_count!(value!(timing_frame_count), timing_frame)
    >> (Webmask {
        version,
        vu,
        timing_frame_count,
        timing_frames,
    })
));
