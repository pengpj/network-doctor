/// 读取 /var/lib/rancher/k3s/agent/etc/cni/net.d/10-flannel.conflist 并打印
pub(crate) fn flannel_config() -> () {
    let flannel_conf = "/var/lib/rancher/k3s/agent/etc/cni/net.d/10-flannel.conflist";
    println!("flannel config: {}", flannel_conf);
    let content = match std::fs::read_to_string(flannel_conf) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("failed to read file: {}", e);
            return;
        }
    };
    // println!("content: {}", content);
}