# async-process 示例代码
这是一个支持 async 的子程序库

Linux 的设计哲学其中之一就是单一责任, 可以使用不同程序使用 pipe 组合完成各种复杂的功能, 如:
```bash
cat word.txt | uniq | wc -l
```
Demo 中做了两个 ping 的简单使用, 一个是对输出绑定到 Reader 中, 然后使用 lines() 遍历, 另外一个是等待结束后, 取得输出的内容.

# Demo 说明
- 更新时间: 2022-05-29
- Rust 版本: rustc 1.63.0-nightly (cd282d7f7 2022-05-18)