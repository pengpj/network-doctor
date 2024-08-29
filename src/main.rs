mod route_helper;
mod flannel_helper;
mod iptables_helper;
mod curl_helper;
mod cert_helper;
mod server_helper;
mod agent_helper;

fn main() -> () {
    println!("Hello, world!");

    // 读取 flannel 配置
    flannel_helper::flannel_config();

    // 仅在 linux 系统中执行
    #[cfg(target_os = "linux")]
    route_helper::kubernetes_route();

    // 检查 iptables 规则
    iptables_helper::check_iptables();

    if server_helper::check_k3s_server() {
        // 读取证书有效期
        cert_helper::cert_check();
        // curl ping
        curl_helper::curl_ping();
        // check node flannel
        let node_ips = server_helper::check_node_ips();
        for node_ip in node_ips {
            agent_helper::check_master_8472(&node_ip);
        }
    } else if agent_helper::check_k3s_agent() {
        // 读取 master ip 地址
        let master_ip = agent_helper::load_master_ip();
        // curl master ip
        agent_helper::curl_master_ping(&master_ip);
        // 检查 8472 端口
        agent_helper::check_master_8472(&master_ip);
    }
}