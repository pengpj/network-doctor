/// timeout 3 curl -v -k https://10.43.0.1:443/ping
/// expect: 200 OK
/// body: pong
pub(crate) fn curl_ping() -> () {
    let url = "https://10.43.0.1:443/ping";
    exec_curl(url);

    let url = "https://127.0.0.1:6443/ping";
    exec_curl(url);
}

fn exec_curl(url: &str) {
    let output = match std::process::Command::new("timeout")
        .arg("3")
        .arg("curl")
        .arg("-v")
        .arg("-k")
        .arg(url)
        .output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("failed to execute process: {}", e);
            return;
        }
    };
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("failed: {}", stderr);
        return;
    }
}