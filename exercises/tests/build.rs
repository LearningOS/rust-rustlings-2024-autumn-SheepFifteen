//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
//!
use std::env;
use std::time::SystemTime;

use std::env;
use std::time::SystemTime;

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
                    // 格式化环境变量命令，不包含时间戳
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    // 打印命令让 Cargo 执行
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
    // if let Some(_) = env::var("CARGO_FEATURE_PASS").ok() {
    //     println!("cargo:rustc-cfg=pass");
    //     return;
    // }

    // let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
    //     Ok(n) => n.as_secs(),
    //     Err(_) => panic!("there is error with systemtime"),
    // };

    // println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}
