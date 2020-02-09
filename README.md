# rust-vid-streamer
Streams video from a Raspberry Pi camera module to a host machine

## Installation

  1) Setup a Raspberry Pi device. We use the Raspberry Pi Model B with Debian TODO.
  2) [Install Rust](https://www.rust-lang.org/tools/install). We use version 1.41.0.
  3) Install packages:
  ```bash
  $ sudo apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev
  ```
  4) Build the project: `cargo build`
