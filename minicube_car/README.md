# Car Generator

A Rust Actix web server that generates a car based on a given set of parameters.

## Usage

Clone this repository and run the following commands:
``` cargo run ```
After the server is running, you can access the API at http://localhost:8080/{MAKE}/{MODEL}
To request specific car information you can use the following query parameters:
make: The make of the car
model: The model of the car