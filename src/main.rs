extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the elements
    let src = gst::ElementFactory::make("videotestsrc", None)
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
    pipeline.add_many(&[&src, &enc, &pay, &sink]).unwrap();
    src.link(&enc).expect("Could not link source to encoder");
    enc.link(&pay).expect("Could not link encoder to RTP payload");
    pay.link(&sink).expect("Could not link encoder to RTP payload");

    // Modify the source's properties
    sink.set_property_from_str("host", "127.0.0.1");
    sink.set_property_from_str("port", "5000");

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
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
