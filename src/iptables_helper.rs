/// 检查主机 iptables 规则
/// -A KUBE-SERVICES -d 10.43.41.84/32 -p tcp -m comment --comment "educcp/jiashi-server:port-1 has no endpoints" -m tcp --dport 443 -j REJECT --reject-with icmp-port-unreachable
/// 检查并输出
pub(crate) fn check_iptables() -> () {
    // 检查 /usr/sbin/iptabls 是否存在
    let iptables = "/usr/sbin/iptables";
    println!("iptables: {}", iptables);
    if !std::path::Path::new(iptables).exists() {
        eprintln!("iptables not found");
        return;
    }

    // 执行 iptables -L -n 检查 iptables 的规则
}