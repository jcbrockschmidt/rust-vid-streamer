extern crate gstreamer as gst;
use gst::prelude::*;
use std::{env, fmt, process};

extern crate gstreamer_rtsp_server as gst_rtsp_server;
use gst_rtsp_server::prelude::*;

fn usage(args: Vec<String>) {
    println!("Usage: {} device port", args[0]);
}

fn start_stream(device: &String, port: &String) {
    let server = gst_rtsp_server::RTSPServer::new();
    server.set_service(&port);
    let mounts = server.get_mount_points().unwrap();
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    factory.set_launch(&*format!("v4l2src device={} ! videoconvert ! video/x-raw,framerate=30/1 ! x264enc speed-preset=ultrafast tune=zerolatency ! rtph264pay name=pay0 pt=96", device));
    factory.set_shared(true);
    mounts.add_factory("/stream", &factory);

    let id = server.attach(None);
    println!(
        "Stream ready at rtsp://127.0.0.1:{}/stream",
        server.get_bound_port()
    );
    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
    glib::source_remove(id);
}

fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage(args);
        process::exit(1);
    }
    let device = String::from(&args[1]);
    let port = String::from(&args[2]);

    gst::init().unwrap();
    start_stream(&device, &port);

    println!("Stream finished");
}
