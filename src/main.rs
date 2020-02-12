extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    println!("Initializing pipeline...");
    let device = "/dev/video0";
    let ipv4 = "127.0.0.1";
    let port = "5000";

    // Initialize GStreamer
    gst::init().unwrap();

    // Create the elements
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
    sink.set_property_from_str("host", ipv4);
    sink.set_property_from_str("port", port);

    // Start the stream pipeline
    println!("Starting stream...");
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

    println!("Stream finished");
}