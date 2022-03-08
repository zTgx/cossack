### How to run tests

* assert!(expression) - panics if expression evaluates to false.
* assert_eq!(left, right) and assert_ne!(left, right) - testing left and right expressions for equality and inequality respectively

---
* 运行所有tests
```
cargo test
```

* 显示输出
```
cargo test -- --show-output
```

* 运行指定 test
```
cargo test <function_name>
```