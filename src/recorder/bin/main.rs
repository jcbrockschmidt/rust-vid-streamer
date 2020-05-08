extern crate ctrlc;
extern crate gstreamer as gst;
use gst::prelude::*;
use std::{env, process};

const VIDEO_PATH : &str = "recording.mp4";
const RECOVERY_PATH : &str = "recording.atom";

fn usage(args: Vec<String>) {
    println!("Usage: {} device", args[0]);
}

fn create_pipeline(device: &String) -> Result<gst::Pipeline, ()> {
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("v4l2src", None).unwrap();
    let scale = gst::ElementFactory::make("videoscale", None).unwrap();
    let filter = gst::ElementFactory::make("capsfilter", None).unwrap();
    let conv = gst::ElementFactory::make("videoconvert", None).unwrap();
    let queue = gst::ElementFactory::make("queue", None).unwrap();
    let enc = gst::ElementFactory::make("x264enc", None).unwrap();
    let mux = gst::ElementFactory::make("mp4mux", None).unwrap();
    let sink = gst::ElementFactory::make("filesink", None).unwrap();

    let video_caps =
        gst::Caps::new_simple("video/x-raw", &[("width", &1920i32), ("height", &1080i32)]);

    src.set_property_from_str("device", device);
    filter.set_property("caps", &video_caps.to_value());
    mux.set_property_from_str("moov-recovery-file", RECOVERY_PATH);
    sink.set_property_from_str("location", VIDEO_PATH);

    // Build the pipeline
    pipeline.add_many(&[&src, &scale, &filter, &conv, &queue, &enc, &mux, &sink]).unwrap();
    gst::Element::link_many(&[&src, &scale, &filter, &conv, &queue, &enc, &mux, &sink]).unwrap();

    return Ok(pipeline);
}

fn start_recording(device: &String) {
    // Initialize GStreamer and pipeline;
    println!("Initializing pipeline...");
    gst::init().unwrap();
    let pipeline = create_pipeline(device).unwrap();

    // Gracefully handle a keyboard interrupt (ctrl-C)
    let pipeline_weak = pipeline.downgrade();
    ctrlc::set_handler(move || {
	let pipeline = match pipeline_weak.upgrade() {
	    Some(pipeline) => pipeline,
	    None => return,
	};
	println!("Ending recording. Please wait...");
	pipeline.send_event(gst::Event::new_eos().build());
    }).expect("Error setting Ctrl-C handler");

    // Start the stream pipeline
    println!("Recording from {}... (Use Ctrl-C to end recording)", device);
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error()
                );
                eprintln!("Debugging information: {:?}", err.get_debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	usage(args);
	process::exit(1);
    }
    let device = String::from(&args[1]);

    start_recording(&device);

    println!("Recording finished");
}
