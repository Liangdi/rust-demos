# systemstat-demo
获取系统信息如 cpu,内存,磁盘,网络等情况可以满足某些监控的场景需求.

systemstat 是存 rust 写的使用 /proc 下的文件来获取信息(win 使用 winapi)

# Demo 说明
- 更新时间: 2022-05-27
- Rust 版本: rustc 1.63.0-nightly (cd282d7f7 2022-05-18)
- 目前代码主要是 crate example ， 主要是验证可用