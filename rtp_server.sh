#!/bin/bash
# ffmpeg -re -i test.wav -c:a copy -sdp_file rtp-forwarder.sdp -f rtp rtp://localhost:8080/rtp-forwarder.sdp

ffmpeg -re -i test.wav -f rtp rtp://localhost:8080 > test.sdp