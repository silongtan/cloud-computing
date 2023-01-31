### Notes

* A learning repo for Rust formatting and display
* [Rust AWS Lambda docs](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html)
* Install AWS VS Code plugin and configure it to use your AWS account.
* See GitHub repo here: https://github.com/awslabs/aws-lambda-rust-runtime#deployment


To deploy: `make deploy` which runs: `cargo lambda build --release`

* Test inside of AWS Lambda console
* Test locally with:

```bash
cargo run
```

Result:
```
12 months in a year.
"Christian" "Slater" is the actor's name.
[0: 1, 1: 2, 2: 3]
Dublin: 53.348°N 6.260°W
Oslo: 59.950°N 10.750°E
Vancouver: 49.250°N 123.100°W
```
