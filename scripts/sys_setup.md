# sys_setup
## Contents

Setup-Skript für den Raspberry PI, setzt voraus das für den Zugriff auf den 
RPI ein Zugriffszertifikat für SSH hinterlegt worden ist. Führt durch:
1. Update
2. Python3-Packages installieren
3. MQTT-Brocker installieren (Mosquitto)
4. MQTT-Client installieren 
5. pip und mqtt-paho installieren

## Host
remote="pi@raspberrypi"

# Updating system via apt-get"
sudo apt-get update -y
sudo apt-get upgrade -y

# Installing Mosquitto MQTT Broker"
wget http://repo.mosquitto.org/debian/mosquitto-repo.gpg.key
sudo apt-key add mosquitto-repo.gpg.key
cd /etc/apt/sources.list.d/
sudo wget http://repo.mosquitto.org/debian/mosquitto-wheezy.list
sudo -i
apt-get update
apt-get install mosquitto

# Installing Mosquitto Client"
sudo apt-get install mosquitto-clients

# Autostart MQTT-Broker"
sudo systemctl enable mosquitto

# Python Module die installiert werden müssen
sudo apt install python3-pip # Zuerst muss pip installiert werden auf dem System
pip3 install paho-mqtt

# Git 
> Wird genutzt um die Sourcen/skripte auf das System zu bekommen
sudo apt install git