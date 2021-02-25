use lamedh_runtime::{Context, Error};
use log::{debug, info};
use serde::Deserialize;
use serde_json::Value;
use std::env;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct TestData {
    name: String,
    code: u32,
    tags: Option<String>,
    lang: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "rust_lambda_call_aws_cli=debug");
    env_logger::init();

    lamedh_runtime::run(lamedh_runtime::handler_fn(handler)).await?;

    Ok(())
}

async fn handler(_: Value, _: Context) -> Result<(), Error> {
    debug!("handler start");

    let output = Command::new("aws")
        .args(&[
            "s3api",
            "select-object-content",
            "--bucket=testdata-xxxx",
            "--key=test_data.json",
            "--input-serialization",
            r#"{"JSON":{"Type":"LINES"}}"#,
            "--output-serialization",
            r#"{"JSON":{"RecordDelimiter":"\n"}}"#,
            "--expression",
            "SELECT * FROM s3object s LIMIT 5",
            "--expression-type=SQL",
            "/tmp/output.json",
        ])
        .output()
        .await?;

    debug!("{:?}", output);

    if Some(0) != output.status.code() {
        panic!("{:?}", output);
    }

    let contents = tokio::fs::read("/tmp/output.json").await?;
    for line in String::from_utf8(contents)?.lines() {
        let d: TestData = serde_json::from_str(line)?;
        info!("{:?}", d);
    }

    Ok(())
}
