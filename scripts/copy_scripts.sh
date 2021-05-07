#!/bin/bash

# Copies all necessary scripts on the target system using scp. 

echo "Skript-copy-tool"
echo "Current working directory on local machine $(PWD)"


ssh pi@raspberrypi 'sudo mkdir -p /usr/handler' 
rsync -avR i2c/ pi@raspberrypi:/usr/handler