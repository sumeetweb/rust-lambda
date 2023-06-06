# rust-lambda

## Build and Deploy

AWS CLI and cargo lambda must be installed before running this command. Please check below link for installing cargo lambda.
https://www.cargo-lambda.info/guide/installation.html

Build and deploy lambda as a function url:
```bash
cargo lambda build --arm64 && cargo lambda deploy new-lambda-project --enable-function-url
```

Note: Deploying as a function url is different from deploying at API Gateway. Some headers might not work as expected while using function URL. 
