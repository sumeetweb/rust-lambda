
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use jsonwebtokens_cognito::KeySet;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request. You will only see this message if your authentication token is correct.");

    let keyset = KeySet::new("ap-south-1", "ap-south-1_uU90KbaQr");
    // Error handling
    if keyset.is_err() {
        let resp : Response<Body> = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(String::from("Error: Loading JWT Keys").into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    let keyset = keyset.ok().unwrap();

    let verifier = keyset.new_id_token_verifier(&["78dd765hhdjrutde2vf7g1v73d"]).build()?;
    // Get the token from the request header Authorization and verify it
    let token_str = event.headers().get("Authorization").unwrap().to_str().unwrap();
    let verify_res = keyset.verify(&token_str, &verifier).await;
    // Error handling
    if verify_res.is_err() {
        let resp : Response<Body> = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(String::from("Error: Auth Token").into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    verify_res.ok().unwrap();

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
