extern crate gstreamer as gst;
use gst::prelude::*;
use std::{ env, process };

fn usage(args: Vec<String>) {
    println!("Usage: {} device ipv4 port", args[0]);
}

fn create_pipeline(device: &String, ip: &String, port: &String) -> gst::Pipeline {
    let src = gst::ElementFactory::make("v4l2src", None)
	.expect("Could not create source element");
    let conv = gst::ElementFactory::make("videoconvert", None)
	.expect("Could not create source element");
    let enc = gst::ElementFactory::make("x264enc", None)
        .expect("Could not create x264 encoder");
    let pay = gst::ElementFactory::make("rtph264pay", None)
        .expect("Could not create RTP payload");
    let sink = gst::ElementFactory::make("udpsink", None)
        .expect("Could not create UDP sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new(Some("basic streamer"));

    // Build the pipeline
    pipeline.add_many(&[&src, &conv, &enc, &pay, &sink]).unwrap();
    src.link(&conv).expect("Could not link source to vidoe converter");
    conv.link(&enc).expect("Could not link video converter to encoder");
    enc.link(&pay).expect("Could not link encoder to RTP payload");
    pay.link(&sink).expect("Could not link encoder to RTP payload");

    // Direct the source to the specified camera device
    src.set_property_from_str("device", device);

    // Direct the sink to our host
    sink.set_property_from_str("host", ip);
    sink.set_property_from_str("port", port);

    return pipeline
}

fn start_stream(device: &String, ip: &String, port: &String) {
    // Initialize GStreamer and pipeline;
    println!("Initializing pipeline...");
    gst::init().unwrap();
    let pipeline = create_pipeline(device, ip, port);

    // Start the stream pipeline
    println!("Streaming {} to {}:{}...", device, ip, port);
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
    if args.len() < 4 {
	usage(args);
	process::exit(1);
    }
    let device = String::from(&args[1]);
    let ipv4 = String::from(&args[2]);
    let port = String::from(&args[3]);

    start_stream(&device, &ipv4, &port);

    println!("Stream finished");
}
