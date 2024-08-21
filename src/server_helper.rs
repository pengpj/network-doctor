use std::process::Output;

/// pidof 'k3s server'
/// 检查 k3s server 进程是否存在，如果不存在，则输出错误信息
pub(crate) fn check_k3s_server() -> (bool) {
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

fn pidof_cmd(k3s_server: &str) -> Result<Output, bool> {
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