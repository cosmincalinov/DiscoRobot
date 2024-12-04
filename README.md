# DiscoRobot
## Table of Contents
[1. Introduction](#introduction)\
[2. Overview](#overview)\
[3. Hardware Design](#hardware-design)\
[4. Software Design](#software-design)\
[5. Final Results](#final-results)\
[6. Conclusions](#conclusions)\
[7. Journal](#journal)\
[8. Source Code](#source-code)\
[9. Resources](#resources)

## Introduction
This robot is designed to entertain by combining mobility and music. It can be controlled via Bluetooth using an Xbox PC controller, and it connects to Spotify over WiFi to play songs. In automatic mode, it navigates the room while avoiding obstacles using its sensors.

Its purpose is to entertain people, adding fun and humor with a selection of songs.

This robot can be used as a moveable speaker when you want extra ambiance around the house or at parties. It can also be used as a toy for children.

For future work, I would like to incorporate a vacuum feature similar to a Roomba's. This would combine functionality with fun.

## Overview
TBD - The block diagram will illustrate the robot's main hardware components, communication pathways (Bluetooth, WiFi), and software interactions.

## Hardware Design
- Raspberry Pi Pico WH: Acts as the main microcontroller.
- PIR Sensor: Detects motion for automatic navigation.
- Infrared Obstacle Senzor: Helps avoid obstacles in auto mode.
- Buzzer
- Xbox PC Controller: Enables manual control of the robot.
- Bluetooth Module: Facilitates wireless communication with the controller.
  
## Software Design
I have opted to use Visual Studio Code since it's the primary IDE for Rust development. Speaking of which, Rust with Embassy(embedded + async) is my primary choice when it comes to the programming language that is used for this project since Rust is very suitable for embedded systems, it is my favorite programming language and a long-time hobby of mine.

The robot's software is divided into two main modes: manual and automatic. In manual mode, it processes Bluetooth inputs from the Xbox controller to move the robot. In automatic mode, it relies on sensor data to avoid obstacles. Additionally, the WiFi module communicates with Spotify's API to stream music seamlessly.

Rust, paired with the Embassy crate, is ideal for handling the robot's concurrent tasks like sensor polling and music streaming. Its safety features, like the Borrow Checker, ensure efficient and error-free code. Let's not forget about its smart and friendly compiler.

## Final Results
TBD - This section will document the robot's performance, including its speed, responsiveness, and user feedback.

## Conclusions
TBD - The conclusions will reflect on the challenges faced, solutions implemented, and potential improvements.

## Journal
### Day 1 (02.12.2024):
Started the documentation for the project. So far so good right now.

### Day 2 (04.12.2024):
Added the license for the project. Might change it later.

## Source Code
TBD

## Resources
[Embassy](https://embassy.dev/)

