use std::process::Output;
use crate::agent_helper;

/// pidof 'k3s server'
/// 检查 k3s server 进程是否存在，如果不存在，则输出错误信息
pub(crate) fn check_k3s_server() -> bool {
    let k3s_server = "k3s server";
    let output = match pidof_cmd(k3s_server) {
        Ok(value) => value,
        Err(value) => return value,
    };
    if output.status.success() {
        println!("k3s server is running");
        return true;
    } else {
        println!("k3s server is not running");
    }

    let k3s_agent = "k3s agent";
    let output = match pidof_cmd(k3s_agent) {
        Ok(value) => value,
        Err(value) => return value,
    };
    if output.status.success() {
        println!("k3s agent is running");
        return false;
    } else {
        println!("k3s agent is not running");
    }
    return false;
}

pub(crate) fn pidof_cmd(k3s_server: &str) -> Result<Output, bool> {
    let output = match std::process::Command::new("pidof")
        .arg(k3s_server)
        .output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("failed to execute process: {}", e);
            return Err(false);
        }
    };
    Ok(output)
}

/// kubectl get nodes -o custom-columns=IP:.status.addresses[0].address --no-headers
/// 获取所有 node 的 IP 地址
pub(crate) fn check_node_ips() -> Vec<String> {
    let output = match std::process::Command::new("kubectl")
        .arg("get")
        .arg("nodes")
        .arg("-o")
        .arg("custom-columns=IP:.status.addresses[0].address")
        .arg("--no-headers")
        .output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("failed to execute process: {}", e);
            return Vec::new();
        }
    };
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("node ips: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("failed: {}", stderr);
        return Vec::new();
    }

    // 按行分割，获取每个 node 的 IP 地址
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.split('\n').collect();
    let mut node_ips = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        println!("node ip: {}", line);
        node_ips.push(line.to_string());
    }

    node_ips


}