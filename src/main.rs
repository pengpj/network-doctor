mod route_helper;
mod flannel_helper;
mod iptables_helper;
mod curl_helper;
mod cert_helper;
mod server_helper;
mod agent_helper;
mod metrics_helper;

fn main() -> () {
    println!("< check k3s node >");

    // 读取 flannel 配置
    println!("### check flannel config");
    flannel_helper::flannel_config();

    // 仅在 linux 系统中执行
    #[cfg(target_os = "linux")]
    println!("#### route ");
    #[cfg(target_os = "linux")]
    route_helper::kubernetes_route();

    // 检查 iptables规则
    println!("#### iptables ");
    iptables_helper::check_iptables();

    if server_helper::check_k3s_server() {
        println!("#### tls cert");
        // 读取证书有效期
        cert_helper::cert_check();
        // curl ping
        println!("#### api server ping");
        curl_helper::curl_ping();
        // check node flannel
        println!("#### check components");
        let node_ips = server_helper::check_node_ips();
        for node_ip in node_ips {
            agent_helper::check_master_8472(&node_ip);
            metrics_helper::curl_metrics(&node_ip);
        }
    } else if agent_helper::check_k3s_agent() {
        // 读取 master ip 地址
        let master_ip = agent_helper::load_master_ip();
        // curl master ip
        println!("#### api server ping");
        agent_helper::curl_master_ping(&master_ip);
        println!("#### flannel vxlan udp network");
        // 检查 8472 端口
        agent_helper::check_master_8472(&master_ip);
    }
}