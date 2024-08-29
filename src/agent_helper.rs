/// 读取 /etc/systemd/system/k3s.service
pub(crate) fn load_master_ip() -> String {
    let k3s_service = "/etc/systemd/system/k3s.service";
    println!("k3s service: {}", k3s_service);
    let content = match std::fs::read_to_string(k3s_service) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("failed to read file: {}", e);
            return "".to_string();
        }
    };
    // 文本中 '--server=https://192.168.200.110:6443' 为 master ip 地址
    let master_ip = content.split("--server=").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[1].split("'").collect::<Vec<&str>>()[0];
    println!("master ip: {}", master_ip);
    return master_ip.to_string();
}

pub(crate) fn check_k3s_agent() -> bool {
    let k3s_agent = "k3s agent";
    let output = match crate::server_helper::pidof_cmd(k3s_agent) {
        Ok(value) => value,
        Err(value) => return value,
    };
    if output.status.success() {
        println!("k3s agent is running");
        return true;
    } else {
        println!("k3s agent is not running");
    }
    return false;
}

pub(crate) fn curl_master_ping(master_ip: &str) -> () {

    // curl -k https://master_ip:6443/ping
    let url = format!("https://{}:6443/ping", master_ip);
    println!("command: curl -k {}", url);
    let output = match std::process::Command::new("curl")
        .arg("-k")
        .arg(url)
        .output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("failed to execute process: {}", e);
            return;
        }
    };
    println!("output: {}", String::from_utf8_lossy(&output.stdout));

    // if output stdout contains "pong", ok
    if String::from_utf8_lossy(&output.stdout).contains("pong") {
        println!("curl master ip is ok");
    } else {
        eprintln!("curl master ip is not ok");
    }

    check_master_8472(master_ip);
}

pub(crate) fn check_master_8472(master_ip: &str) -> () {
    // udp master ip 8472
    // nc -vz -u master_ip 8472
    println!("command: nc -vz -u {} 8472", master_ip);
    let output = match std::process::Command::new("nc")
        .arg("-vz")
        .arg("-u")
        .arg(master_ip)
        .arg("8472")
        .output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("failed to execute process: {}", e);
            return;
        }
    };
    println!("output: {}", String::from_utf8_lossy(&output.stdout));

    // if output stdout contains "succeeded", ok
    if String::from_utf8_lossy(&output.stdout).contains("succeeded") {
        println!("nc {} ip is ok", master_ip);
    } else {
        eprintln!("nc {} ip is not ok, flannel network is not ok", master_ip);
    }
}