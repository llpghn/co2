## Hardwareaufbau
```
            Web-Browser
            -----------
              :     :
    ..(HTTP)..:     :..(WS)...
    :                        : 
---------------           ---------       +-----+
Webserver (SPA)           websocket |-----|     |
                                          |     |
     redis_db |-----| label_handler |-----|  M  |-----| process-engine
                                          |  Q  |
          led |-----| LED_Handler |-------|  T  |
                                          |  T  |
       bmc260 |-----+                     |     |
                    +-| i2c_handler |-----|     |
    o2_sensor |-----+                     +-----+
                    |
 light_sensor |-----+
```

-----

## i2c_handler
  
  > Liest die Daten der Sensoren am I2C-Bus aus. Keine Steuerung nur Datenlesen.

  ### Funktionen
  * Zyklisches Auslesen der Sensorwerte
  * Verbinden zum MQTT-Broker
  * Providen von Sensorwerten

  ### Topics
  #### Emits

  * /sensor/value/temperature
  * /sensor/value/humidity
  * /sensor/value/co2
  * /sensor/value/lightlevel

  #### Process

  * /action/system/shutdown
  * /action/system/sleepmode

-----

## process-engine

  > Sorgt für einen Zentralen Ansatz bei dem jedes Stellglied keine große Logik benötigt.

  ### Funktionen
  * lauscht auf events die aufkommen (Soll)
  * generiert neue actions die aktionen auslösen (Ist)

  Beispiel: Event CO2 > 50, dann generiere event LED an.

  ### Topics
  #### Emits

  * TBD
  
  #### Process

  * TBD

-----

## label_handler

  > Zentrale Datenbank die alle vorkommnisse im System speichert, vorallem aber das was der User über die Webpage einstellt

  ### Funktionen

  * Speichere Situationsbewertungen 
  * Stelle Grenzwerte bereit

  ### Topics
  #### Emits

  /setpoint/preference/temperature
  /setpoint/preference/humidity
  /setpoint/preference/CO2
  /setpoint/preference/lightlevel

  #### Process

  /setpoint/add/temperature
  /setpoint/add/humidity
  /setpoint/add/CO2
  /setpoint/add/lightlevel

## processor

* Lauschen auf allen Werten (Prozess-Werte)
* Steuern von Stellwerten

## LED_Handler

* Subscribe auf Signal für LED
* Steuerung der LED An/Aus

## Webpage

* Abrufen von: Temperatur, Luftfeuchtigkeit, CO2, Lichtlevel
* Aktualisieren der Anzeige wen Websocket neue Daten sendet
* Bewerten des aktuellen Zustandes: Good/Bad