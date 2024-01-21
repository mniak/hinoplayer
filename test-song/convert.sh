#!/bin/bash
ffmpeg -loop 1 -framerate 30 -i image.jpg -i audio.mp3 -c:v libx265 -x265-params keyint=18000 -c:a copy -shortest output.mp4
