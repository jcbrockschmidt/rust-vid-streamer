#!/usr/bin/env bash

# Shows a webcam
# Assumes a webcam is on /dev/video0
gst-launch-1.0 v4l2src device=/dev/video0 ! video/x-raw,width=640,height=480 ! autovideosink
