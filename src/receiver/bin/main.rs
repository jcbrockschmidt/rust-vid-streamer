extern crate ctrlc;
extern crate gstreamer as gst;
use gst::prelude::*;
use std::{env, process};

fn usage(args: Vec<String>) {
    println!("Usage: {} port", args[0]);
}

fn create_pipeline(port: &String) -> gst::Pipeline {
    let src = gst::ElementFactory::make("udpsrc", None)
	.expect("Could not create source element");
    let depay = gst::ElementFactory::make("rtph264depay", None)
        .expect("Could not create RTP depayloader");
    let dec = gst::ElementFactory::make("decodebin", None)
        .expect("Could not create decoder");

    let pipeline = gst::Pipeline::new(Some("basic stream receiver"));
    pipeline.add_many(&[&src, &depay, &dec]).unwrap();
    src.link(&depay).expect("Could not link source to depayload");
    depay.link(&dec).expect("Could not link depayloader to decoder");

    // Set port to listen to and stream properties
    src.set_property_from_str("port", port);
    src.set_property_from_str("caps", "application/x-rtp, media=(string)video, clock-rate=(int)90000, encoding-name=(string)H264, payload=(int)96");

    // Connect to decodebin's pad-added signal
    let pipeline_weak = pipeline.downgrade();
    dec.connect_pad_added(move |_, src_pad| {
	let pl = match pipeline_weak.upgrade() {
	    Some(pl) => pl,
	    None => return,
	};

	let conv = gst::ElementFactory::make("videoconvert", None)
            .expect("Could not create video converter");
	let sink = gst::ElementFactory::make("autovideosink", Some("autovideosink0"))
            .expect("Could not create video sink");

	pl.add_many(&[&conv, &sink]).unwrap();
	conv.link(&sink).expect("Could not link video converter to video sink");
	conv.sync_state_with_parent().unwrap();
	sink.sync_state_with_parent().unwrap();

	let sink_pad = conv.get_static_pad("sink").unwrap();
	src_pad
	    .link(&sink_pad)
	    .expect("Unable to link video decoder to sink pad");
    });

    return pipeline;
}

fn start_stream(port: &String) {
    // Initialize GStreamer and pipeline;
    println!("Initializing pipeline...");
    gst::init().unwrap();
    let pipeline = create_pipeline(port);

    // Gracefully handle a keyboard interrupt (ctrl-C)
    let pipeline_weak = pipeline.downgrade();
    ctrlc::set_handler(move || {
	let pipeline = match pipeline_weak.upgrade() {
	    Some(pipeline) => pipeline,
	    None => return,
	};
	println!("Stopping listener. Please wait...");
	pipeline.send_event(gst::Event::new_eos().build());
    }).expect("Error setting Ctrl-C handler");

    // Start the stream pipeline
    println!("Listening to stream at port {} (Use Ctrl-C to stop)...", port);
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
    let port = String::from(&args[1]);

    start_stream(&port);

    println!("Receiver stopped");
}
