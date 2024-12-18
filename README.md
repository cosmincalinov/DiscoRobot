# DiscoRobot
## Table of Contents
[1. Introduction](#introduction)\
[2. Overview](#overview)\
[3. Hardware Design](#hardware-design)\
[4. Software Design](#software-design)\
[5. Setup Instructions](#software-design)\
[6. Final Results](#final-results)\
[7. Conclusions](#conclusions)\
[8. Journal](#journal)\
[9. Source Code](#source-code)\
[10. Resources](#resources)

## Introduction
This robot is designed to entertain by combining mobility and music. It can be controlled via Bluetooth using an Xbox PC controller, and it connects to Spotify over WiFi to play songs. In automatic mode, it navigates the room while avoiding obstacles using its sensors.

Its purpose is to entertain people, adding fun and humor with a selection of songs.

This robot can be used as a moveable speaker when you want extra ambiance around the house or at parties. It can also be used as a toy for children.

This is a software and hardware heavy project.

For future work, I would like to incorporate a vacuum feature similar to a Roomba's. This would combine functionality with fun.

## Overview
### Features
- Ultrasonic sensor: avoids obsacles
- Bluetooth module + Xbox Controller: manual control
- Raspberry Pi Pico WH's Wi-Fi Chip + buzzer: access to songs via the Spotify API.
### Block Diagram

![block_diagram](https://github.com/user-attachments/assets/6a542cdb-7a4a-4f57-b136-b0680c6028ed)

## Hardware Design

![circuit](https://github.com/user-attachments/assets/88ec4bdd-8634-49de-808d-bb042b1cdfb2)

### Electric Diagram
![electric_diagram](https://github.com/user-attachments/assets/b9a2d9d8-ff3f-4020-ac35-92873e7376e2)

### Components (BoM)
| Component                 | Quantity | Datasheet                                                          | Place of Acquisition | Description |
| :------------------------ | :------: | :----------------------------------------------------------------: | :------------------: | :---------: |
| Raspberry Pi Pico WH      |     1    | [Link](https://datasheets.raspberrypi.com/picow/pico-w-datasheet.pdf)      | Optimus Digital      | Acts as the main microcontroller.   |
| Raspberry Pi Pico Debug Probe | 1    | -                                                                          | Optimus Digital      | Uploads the code to the Pico. | 
| HC-SR04 Ultrasonic Sensor |     1    | [Link](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf)   | -                    | Helps avoid obstacles in auto mode. |
| Servo Motor               |     1    | -                                                                   | -                    | Moves the Ultrasonic Sensor so that it ensures that no obstacles are nearby. |      
| DC Motor                  |     4    | -                                                                  | -                    | Moves the wheels. |
| Wheels                    |     4    | -                                                                  | -                    | Switches to automatic mode or to manual mode. |
| Button                    |     1    | -                                                                  | -                    | Chnages the state of the robot from auto to manual and vice-versa. |
| Buzzer                    |     1    | [Link](https://www.farnell.com/datasheets/2171929.pdf)                     | -                    | Plays the songs. |
| Bluetooth Module HC-05    |     1    | [Link](https://components101.com/sites/default/files/component_datasheet/HC-05%20Datasheet.pdf)                                                                   | Optimus Digital      | Facilitates wireless communication with the controller |
| Capacitor (100 μF)        |     1    | -                                                                  | -                    | Removes noise and stabilizes voltage for the Servo. |
| Resistor (5k1 Ω)          |     1    | -                                                                  | -                    | Used as a pull-down resistor for the button. |
| Power Module              |     1    | -                                                                  | Robo Fun             | Provides 5V to the breadbords and Pico. |
| Battery Holder            |     1    | -                                                                  | Optimus Digital      | Powers the Pico and the motor without a computer. | 
| Xbox Controller           |     1    | -                                                                  | Altex                | Enables manual control of the robot. |
| Breadboard ( 830 slots )  |     2    | -                                                                  | Robo Fun             | -                                    |
| Wires (M-M/ M-F)          |   lots   | -                                                                  | Robo Fun             | - |
  
## Software Design
I have opted to use Visual Studio Code since it's the primary IDE for Rust development. Speaking of which, Rust with Embassy(embedded + async) is my primary choice when it comes to the programming language that is used for this project since Rust is very suitable for embedded systems, it is my favorite programming language and a long-time hobby of mine.

The robot's software is divided into two main modes: manual and automatic. In manual mode, it processes Bluetooth inputs from the Xbox controller to move the robot. In automatic mode, it relies on sensor data to avoid obstacles. Additionally, the WiFi module communicates with Spotify's API to stream music seamlessly.

Rust, paired with the Embassy crate, is ideal for handling the robot's concurrent tasks like sensor polling and music streaming. Its safety features, like the Borrow Checker, ensure efficient and error-free code. Let's not forget about its smart and friendly compiler.

## Setup Instructions
| Componenet | Pin Name | Connected To |
| :--------- | :------: | -----------: |
| Bluetooth Module | GND | GND |
|                  | VCC | 3.3V |
|                  | RXD | PIN 1 |
|                  | TXD | PIN 2 |
| DC Motor 1       | +   | PIN 6 |
|                  | -   | GND   |
| DC Motor 2       | +   | PIN 16 |
|                  | -   | GND   |
| DC Motor 3       | +   | PIN 22 |
|                  | -   | GND   |
| DC Motor 4       | +   | PIN 25 |
|                  | -   | GND   |
| Servo            | +   | 5V |
|                  | -   | GND   |
|                  | PWM | PIN 20 |
| Ultrasonic Sensor | + | 5V |
|                   | - | GND |
|                  | ECHO | PIN 29 |
|                  | TRIG | PIN 30 |
| Buzzer           | +    | PIN 24 |
|                  | -    | GND    |


## Final Results
TBD - This section will document the robot's performance, including its speed, responsiveness, and user feedback.

## Conclusions
TBD - The conclusions will reflect on the challenges faced, solutions implemented, and potential improvements.

## Journal
### Day 1 (02.12.2024):
Started the documentation for the project. So far so good right now.

### Day 2 (04.12.2024):
Added the license for the project. Might change it later.

### Day 3 (08.12.2024):
Failed to initialize an Embassy Project. Such a nightmare.

### Day 4 (09.12.2024):
Finally, managed to initialize the library after hours of blood and sweat. I had to search through a four hour stream of a guy trying to solve the same problems as me and ended up finding the solution in a delisted video.

### Day 5 (15.12.2024):
Made the Block Diagram.

### Day 6 (16.12.2024):
Made the circuit for the robot. The cabling is a mess right now.

### Day 7 (17.12.2024):
Updated the documentation with the circuit diagram and added a component table.

## Source Code
TBD

## Resources
[Embassy](https://embassy.dev/)

