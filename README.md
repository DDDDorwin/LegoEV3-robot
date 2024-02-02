# Robosistance: A LEGO EV3 Security Robot

This project is a comprehensive security solution utilizing the versatile LEGO EV3 expansionsset. Our robot, Robosistance, combines innovative software and hardware development to create a security robot capable of performing a variety of tasks.

For more detailed information about our project, please refer to our documentation available on Google Docs:

[View Detailed Project Documentation](https://docs.google.com/file/d/1D9C4a-TLAHeQpdVU-6WqZQHNaGy3qEXd/preview)

## Project Overview

Robosistance is designed to enhance security measures through automation and robotics. The project is divided into three main components:

- **Frontend**: The user interface for monitoring and controlling Robosistance.
- **Backend**: The server-side logic that processes data and manages commands sent to and from the robot.
- **Robot Development**: The core of Robosistance, developed on the LEGO EV3 platform, includes motion control, obstacle detection, and other security features.

## Features

- Real-time video surveillance.
- Automated patrol routes.
- Obstacle avoidance and navigation.
- Remote control and monitoring via a web interface.
- Alert system for security breaches.

## Build Instructions

### Backend
#### Toolchain setup
##### Rust
Install the nightly toolchain.

`rustup toolchain install nightly`

Set the toolchain for the project.
**NOTE: This must be done in the `/robosistance` folder**

`rustup override set nightly`

##### MongoDB

Add a `.env` file to the robosistance folder with `MONGOURI=?` where `?` should be the database URI.

#### Building
Error checking.

`cargo check`

Format source files.

`cargo fmt`

Build.

`cargo build`

To build and run.

`cargo run`

### Frontend
Go to the frontend folder.

`cd frontend`

Install necessary npm modules.

`npm install`

Start development server to see what it currently looks like.

`npm run dev`

To create a dist for use with the backend

`npm run build`

## Names
- Mai Anh Duc - maiduc22
- Leon Hejdenberg Philip - leonpx
- Linus Olofsson - kax-y
- Oskar Pettersson Löfstedt - 00oskpet
- Nguyen Anh Quan - anhquan410
- Lifang Zheng - DDDDorwin 
- Leonora Stiernborg - leonorastiernborg
- Hoang Van Tien - tienchuotbk

## IDE
We chose Visual Studio Code as our IDE.

## Communication
We have chosen to use mostly Discord for communication.

## Review process
Pull requests without sufficient reviews will be force-merged after two days.
