mod route_helper;
mod flannel_helper;
mod iptables_helper;
mod curl_helper;

fn main() -> () {
    println!("Hello, world!");

    // 读取 flannel 配置
    flannel_helper::flannel_config();

    // 仅在 linux 系统中执行
    #[cfg(target_os = "linux")]
    route_helper::kubernetes_route();

    // curl ping
    curl_helper::curl_ping();
}