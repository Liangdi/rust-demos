# Rust Demo

记录一些 rust crate 的 demo 代码, 做点笔记,注释,方便后续查询使用

# 基本规则
- 记录代码的最后更新时间
- 使用编码所在时间,对应包的最新发布版本,保证代码的一定实效性
- crate 发布新版本后,如果 demo 有更新,则记录更新日志以及简要笔记
  
# Demo 列表
## 数据库
- [sqlx-demo](./sqlx-demo/) SQLx 数据库链接操作库，非 ORM
- [sea-orm-demo](./sea-orm-demo/) SeaORM 框架, ActiveRecord 模式的 ORM 框架

## 系统
- [systemstat-demo](./systemstat-demo/) systemstat 获取系统信息，比如 cpu，内存，磁盘，网络等.
- [async-process-demo](./async-process-demo/) async-process 支持 async 的子程序库,很方便的创建子程序并且获取输出以及控制输入.

# License
- 文档:CC BY-NC
- 代码: MIT License