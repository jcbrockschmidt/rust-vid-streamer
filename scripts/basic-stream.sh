#!/usr/bin/env bash

# Streams a test card to the local host
gst-launch-1.0 -v videotestsrc ! video/x-raw ! x264enc ! rtph264pay ! udpsink host=127.0.0.1 port=5000
