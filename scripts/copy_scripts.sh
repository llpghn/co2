#!/bin/bash

# Copies all necessary scripts on the target system using scp. 

echo "Skript-copy-tool"
echo "Current working directory on local machine $(PWD)"


ssh pi@raspberrypi -T << 'EOF'
  mkdir -p /usr/handler
  cd /usr/handler
  git clone https://github.com/llpghn/co2
EOF
