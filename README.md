# hello-lambda-rs

My version of the AWS SAM [hello tutorial example](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started-hello-world.html) except using Rust.

Borrows from [Mitch Gollub's Rust and Lambda](https://mitchgollub.com/rust-and-aws-lambda/).

## Note

Currently the local testing isn't working, but it works when deployed onto AWS.

Using the url report during deployment:

`curl https://xxxxxxxx.execute-api.yyyyyyyyy.amazonaws.com/Prod/hello/bert`


## Installations

Install AWS SAM as per https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html

Install Rust (including rustup and cargo) using instructions at https://www.rust-lang.org/tools/install

To build the project for the `x86_64-unknown-linux-musl` target environment that it will deployed into, you will also need to install:
* musl-tools (on Linux: `sudo apt install musl-tools`)
* the pre-built `std` Rust package for musl. Use `rustup target add x86_64-unknown-linux-musl`

## Develop

Use `make` to compile without packaging.

Use `sam build` to build and package for deployment.


## Deploy

Run `sam deploy --guided` to deploy the lambda into your AWS account (as shown in [Deploy your application to the AWS Cloud](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started-hello-world.html#serverless-getting-started-hello-world-deploy)).  Use the stack name such as `hello-lambda-rs`.

Clean up your resources by running `aws cloudformation delete-stack --stack-name hello-lambda-rs --region <region>` (as shown in [Clean up](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started-hello-world.html#serverless-getting-started-hello-world-cleanup)).

SAM creates an S3 bucket during deployment that isn't removed. You probably want to keep this if you intend to use SAM again in the future, but you may wish to delete experimental deployments within it.

