#!/bin/bash

# rustc passes the --as-needed flag to cc
# This causes mmal_vc_client to not be included in the executable
# This flag is the first in the list of arguments
# Just call cc without the first argument!

echo "${@:2}" >> /home/pi/camera-test/args

cc "${@:2}"

