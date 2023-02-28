# Rust based Timer

A rust based timer cli tool using Tokio async functionalities.

## Installation

To run the timer, you'll need to have Rust installed on your machine. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

Once Rust is installed, clone this repository and navigate to the project directory in your terminal. Then, run the following command to build the timer:
``` cargo build --release ```

## Usage

To use the timer, run the following command from the project directory:
``` cargo run --release ```

This will start the timer and display a prompt for you to enter the duration of the timer in seconds. Once you enter the duration, the timer will start counting down. When the timer reaches zero, a message will be displayed indicating that the timer has ended.

You also have the option to choose a progress bar based timer which will show the current timer progress. An example of the progress bar timer is shown below:



## Configuration

The timer can be configured to play a sound when the timer ends. To enable sound, set the `SOUND_ENABLED` environment variable to `true`. By default, sound is disabled.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
