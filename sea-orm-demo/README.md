# SeaORM crate 示例代码

SeaORM 是目前 rust 比较成熟的 ORM 库, 是 ActiveRecord 模式,遵循约定优于配置的原则.

支持主流数据库,可以和主流 web 框架集成.


# SeaORM Demo笔记
- Entity 的设计需要遵循 SeaORM 的约定, 具体查看  src/post/ 目录下的代码.
- 有 sea-orm-cli 这个工具从数据库表传建 Entity , demo 中没有使用, 也可以在代码中自动创建表.
- cli 也可以管理维护 migration , 使用 sea-orm-cli 还是手动维护,取决于开发者自身. demo 中没有使用.

# Demo 说明
- 更新时间: 2022-05-30
- Rust 版本: rustc 1.63.0-nightly (cd282d7f7 2022-05-18)

