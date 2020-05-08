# rust-vid-streamer
Provides binaries for streaming and receiving a stream from a Raspberry Pi camera module

## Installation

  1) Setup a Raspberry Pi device. We use a Raspberry Pi Model B running Raspbian 9.3.
  2) [Install Rust](https://www.rust-lang.org/tools/install). We use version 1.41.0.
  3) Install packages:
  ```bash
  $ sudo apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev
  ```
  4) Build the project with `cargo build`. This will create the binaries `streamer` and `receiver` in `targer/debug/`

## Usage

### Prepare the Camera Module

Before streaming the Raspberry Pi camera module, you must expose it as a device. Run

```
sudo modprobe bcm2835-v4l2
```

Now you should see a new video device under `/dev/` (such as `/dev/video0`). You will have to run this each time you reboot the system, or alternatively add it to `/etc/rc.local` to be automatically executed.


### Start the Streamer

To begin streaming, simply run

```
cd target/debug/
./streamer $DEVICE $IP $PORT
```

where `$DEVICE` is the camera device we exposed earlier, `$IP` is the local IPv4 address of the computer being streamed to, and `$PORT` is the port to stream to.


### Receive the Stream

On another computer (or the same computer if you are using localhost), repeat the installation instructions and run the following

```
cd target/debug/
./receiver $PORT
```

This will listen to the stream we initiated in the previous section and display it to the screen. Depending on when you run this executable, it may take a few moments for video to appear due to the stream's high latency.


### Record a Video

To record a video, run

```
cd target/debug/
./recorder $DEVICE
```
