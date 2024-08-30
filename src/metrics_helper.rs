/// curl -k -s -w "%{http_code}" https://172.20.112.199:10250/metrics -o /dev/null
/// unexpect: 000
/// expect: 401
pub(crate) fn curl_metrics(metrics_ip: &str) -> () {
    let sh = format!("curl -k -s -w \"%{{http_code}}\" https://{}:10250/metrics -o /dev/null", metrics_ip);
    println!("command: {}", sh);
    let output = match std::process::Command::new("sh")
        .arg("-c")
        .arg(sh)
        .output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("failed to execute process: {}", e);
            return;
        }
    };
    if !output.status.success() {
        println!("failed to execute process: {}", output.status);
        return;
    }
    let output_str = String::from_utf8(output.stdout);
    println!("response status code: {:?}", output_str);

    if output_str.unwrap().contains("401") {
        println!("curl metrics ip is ok");
    } else {
        eprintln!("curl metrics ip is not ok");
    }
}