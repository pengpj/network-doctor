/// 获取 kubernetes 路由
/// 在 linux 系统中，可以通过 `ip route get 10.43.0.1` 获取网络路由信息
///
pub(crate) fn kubernetes_route() -> () {
    // 执行 ip route get 10.43.0.1
    println!("ip route get 10.43.0.1");
    let output = match std::process::Command::new("ip")
        .arg("route")
        .arg("get")
        .arg("10.43.0.1")
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

    // 10.43.0.1 via 10.21.17.1 dev eth0 src 10.21.17.59 uid 0
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut parts = stdout.split_whitespace();
    // 提取接口
    let _from = parts.next();
    let _via = parts.next();
    let _gateway = parts.next();
    let _dev = parts.next();
    let iface = parts.next();
    println!("target iface: {:?}", iface.unwrap());

}