#!/bin/bash

# Autor: Patrick, Berendes
# Created on: 2021-05-08

# Copies all necessary scripts on the target system using scp. 
# All handler that are needed on the target system have to be placed in the 
# folder /handler.  
# Danach sind die Daten dann auch dem System unter /usr/handler

echo "Skript-copy-tool"
echo "Current working directory on local machine $(PWD)"

ssh -T pi@raspberrypi <<'EOF'

  sudo mkdir -p /usr/temp
  sudo rm -r /usr/handler
  sudo mkdir -p /usr/handler
  cd /usr/temp
  sudo git clone https://github.com/llpghn/co2
  sudo cp -r /usr/temp/co2/handler/ /usr/
  sudo rm -r /usr/temp
EOF
