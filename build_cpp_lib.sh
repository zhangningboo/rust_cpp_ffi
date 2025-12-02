#!/bin/bash

export OpenCV_HOME=/data/3rd/opencv_4.9.0_ffmpeg
export FFMPEG_DIR=/data/3rd/ffmpeg_n7.1.1

cd cpp/build
rm -rf ./* && cmake .. && make && make install
cd ../..