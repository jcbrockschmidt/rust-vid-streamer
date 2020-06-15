extern crate gstreamer as gst;
use gst::prelude::*;
use std::{env, process};

extern crate gstreamer_rtsp as gst_rtsp;
extern crate gstreamer_rtsp_server as gst_rtsp_server;

use glib::glib_object_impl;
use glib::glib_object_subclass;
use glib::glib_object_wrapper;
use glib::glib_wrapper;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use gst_rtsp_server::prelude::*;
use gst_rtsp_server::subclass::prelude::*;

static SOCKET_PATH: &str = "/tmp/rust-vid-streamer-shm";

// A custom media factory for our RTSP server served to mobile devices.
mod mobile_rtsp_factory {
    use super::*;

    mod imp {
        use super::*;

        pub struct Factory {}

        impl ObjectSubclass for Factory {
            const NAME: &'static str = "RsRTSPMediaFactory";
            type ParentType = gst_rtsp_server::RTSPMediaFactory;
            type Instance = gst::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            // Provides some boilerplate code.
            glib_object_subclass!();

            fn new() -> Self {
                Self {}
            }
        }

        impl ObjectImpl for Factory {
            // Provides some boilerplate code.
            glib_object_impl!();

            fn constructed(&self, obj: &glib::Object) {
                self.parent_constructed(obj);
            }
        }

        impl RTSPMediaFactoryImpl for Factory {
            fn create_element(
                &self,
                _factory: &gst_rtsp_server::RTSPMediaFactory,
                _url: &gst_rtsp::RTSPUrl,
            ) -> Option<gst::Element> {
                let bin = gst::Bin::new(None);
                let src =
                    gst::ElementFactory::make("shmsrc", None).expect("Could not create shmsrc");
                let enc =
                    gst::ElementFactory::make("h264parse", None).expect("Could not create shmsrc");
                let pay = gst::ElementFactory::make("rtph264pay", Some("pay0"))
                    .expect("Could not create rtph264pay");

                src.set_property_from_str("socket-path", SOCKET_PATH);

                bin.add_many(&[&src, &enc, &pay]).unwrap();
                gst::Element::link_many(&[&src, &enc, &pay]).unwrap();

                Some(bin.upcast())
            }
        }
    }

    glib_wrapper! {
        pub struct Factory(
            Object<
                gst::subclass::ElementInstanceStruct<imp::Factory>,
                subclass::simple::ClassStruct<imp::Factory>,
                FactoryClass
            >
        ) @extends gst_rtsp_server::RTSPMediaFactory;

        match fn {
            get_type => || imp::Factory::get_type().to_glib(),
        }
    }

    unsafe impl Send for Factory {}
    unsafe impl Sync for Factory {}

    impl Factory {
        pub fn new() -> Factory {
            glib::Object::new(Self::static_type(), &[])
                .expect("Failed to create factory")
                .downcast()
                .expect("Created factory is of wrong type")
        }
    }
}

fn usage(args: Vec<String>) {
    println!("Usage: {} device port", args[0]);
}

fn start_stream(device: &String, port: &String) {
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("v4l2src", None).expect("Could not create source element");
    let conv = gst::ElementFactory::make("videoconvert", None)
        .expect("Could not create videoconvert element");
    let filter =
        gst::ElementFactory::make("capsfilter", None).expect("Could not create capsfilter element");
    let enc = gst::ElementFactory::make("x264enc", None).expect("Could not create x264enc");
    let pay = gst::ElementFactory::make("shmsink", None).expect("Could not create shmsink");

    let video_caps =
        gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);
    src.set_property_from_str("device", &device);
    enc.set_property_from_str("speed-preset", "ultrafast");
    enc.set_property_from_str("tune", "zerolatency");
    filter
        .set_property("caps", &video_caps.to_value())
        .expect("Failed to set video capabilities");
    pay.set_property_from_str("socket-path", SOCKET_PATH);
    pay.set_property_from_str("sync", "true");
    pay.set_property_from_str("wait-for-connection", "false");

    pipeline
        .add_many(&[&src, &conv, &filter, &enc, &pay])
        .unwrap();
    gst::Element::link_many(&[&src, &conv, &filter, &enc, &pay]).unwrap();

    pipeline.set_state(gst::State::Playing);

    let server = gst_rtsp_server::RTSPServer::new();
    server.set_service(&port);
    let mounts = server.get_mount_points().unwrap();
    let factory = mobile_rtsp_factory::Factory::new();
    factory.set_shared(true);
    mounts.add_factory("/stream", &factory);

    let id = server.attach(None);
    println!(
        "Streaming {} at rtsp://127.0.0.1:{}/stream",
        device,
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
