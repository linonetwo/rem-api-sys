# LocalCTP Rust

## localctp-sys

使用 Git submodule 加载 `https://github.com/dearleeyoung/LocalCTP`

```sh
git submodule update --recursive && git submodule update --remote
```

然后生成最新版的 CTP 动态链接库。

生成 so 并复制到 `crates/localctp-sys/v_current` 文件夹里：

```sh
chmod +x scripts/update_localctp.sh
./scripts/update_localctp.sh
```

## 运行示例

```sh
cargo run --example ctp_query
```

### `error while loading shared libraries: libthostmduserapi_se.so: cannot open shared object file: No such file or directory`

```sh
sudo ln -s crates/localctp-sys/v_current/libthosttraderapi_se.so /usr/local/lib/
sudo ln -s crates/localctp-sys/v_current/libthostmduserapi_se.so /usr/local/lib/
sudo ldconfig
```
