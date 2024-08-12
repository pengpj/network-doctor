mod route_helper;

fn main() -> () {
    println!("Hello, world!");
    // 仅在 linux 系统中执行
    #[cfg(target_os = "linux")]
    route_helper::kubernetes_route();
}