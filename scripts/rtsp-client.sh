#!/usr/bin/env bash

ADDR="127.0.0.1"
PORT=5000

# Displays an RTSP stream from rtsp://$ADDR:$PORT/stream
gst-launch-1.0 rtspsrc latency=50 location=rtsp://$ADDR:$PORT/stream ! rtph264depay ! decodebin ! videoconvert ! ximagesink
