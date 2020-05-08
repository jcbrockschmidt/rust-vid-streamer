#!/usr/bin/env bash

# Records video from a webcam
# Assumes a webcam is on /dev/video0
gst-launch-1.0 v4l2src device=/dev/video0 ! videoconvert ! x264enc ! mp4mux moov-recovery-file=webcam.atom ! filesink location=webcam.mp4 -e
