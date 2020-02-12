#!/usr/bin/env bash

# Streams from a webcam
# Assumes a webcam is on /dev/video0
gst-launch-1.0 v4l2src device=/dev/video0 ! 'video/x-raw,width=640,height=480' ! videoconvert !  x264enc ! rtph264pay ! udpsink host=127.0.0.1 port=5000
