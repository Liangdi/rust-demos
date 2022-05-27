# SQLx crate 示例代码

SQLx 是一个存 rust 写的 sql 库, 支持 async , 支持主流数据.

和 java 比较的话, jdbc 是接近的产物 , SQLx 不是 orm , 基于 SQLx 的 orm 库有 sea-orm 和 ormx 等.

如果要选择一个直接使用 sql 操作数据库的 crate, 那么 SQLx 是个不错的选择.

# SQLx 使用笔记
- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/master/sqlx-cli/README.md) 是 SQLx 提供的离线数据库维护工具, demo 中未使用
- 使用 query! 和 query_as! 两个宏,需要设定环境变量 DATABASE_URL, 可以使用 .env 实现
- sqlx::query_as 自动绑定 struct ,使用 #[derive(sqlx::FromRow)] 对 struct 修饰

# Demo 说明
- 更新时间: 2022-05-27
- Rust 版本: rustc 1.63.0-nightly (cd282d7f7 2022-05-18)

