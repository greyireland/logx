# logo

quick init log env

### quickstart



```
cargo add logo

logo.init_prod();// init prod env
info!("hello {:?}", "world");
//write to logs/xxx-20321212.log
//2023-09-16 19:28:30.572+08 0ms INFO hello world

logo.init_debug(); // init debug env
//write to stdout
```