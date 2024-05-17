# LocalCTP Rust

## localctp-sys

使用 Git submodule 加载 `https://github.com/dearleeyoung/LocalCTP`

```sh
git submodule init && git submodule update --recursive && git submodule update --remote
```

然后生成最新版的 CTP 动态链接库。

生成 so 并复制到 `crates/localctp-sys/thirdparty/LocalCTP/v_current` 文件夹里：

```sh
chmod +x scripts/build_localctp.sh && ./scripts/build_localctp.sh
```

## 运行示例

```sh
cargo run --example ctp_query
```

### `error while loading shared libraries: libthostmduserapi_se.so: cannot open shared object file: No such file or directory`

```sh
sudo ln -s crates/localctp-sys/thirdparty/LocalCTP/v_current/libthosttraderapi_se.so /usr/local/lib/
sudo ln -s crates/localctp-sys/thirdparty/LocalCTP/v_current/libthostmduserapi_se.so /usr/local/lib/
sudo ldconfig
```

如果还有问题，尝试用绝对路径。以下绝对路径仅供参考，需要自己修改：

```sh
rm /usr/local/lib/libthostmduserapi_se.so /usr/local/lib/libthosttraderapi_se.so
sudo ln -s /root/localctp-sys/crates/localctp-sys/thirdparty/LocalCTP/v_current/libthosttraderapi_se.so /usr/local/lib/
sudo ln -s /root/localctp-sys/crates/localctp-sys/thirdparty/LocalCTP/v_current/libthostmduserapi_se.so /usr/local/lib/
sudo ldconfig
```

### Order successfully inserted. 但实际上没有下单成功

可能因为 `Rejected due to instrument not found`，可以把 `LocalCTP/TestLocalCTP/instrument.csv` 拖到项目根目录。

## 排查绑定问题

由于 LocalCTP 开源，可以直接在其源码内加入 Log 方法

```c++
void LogDebug(const std::string& msg)
{
    std::cout << msg << std::endl; // Replace this with your actual logging mechanism.
}
LogDebug("Entering ReqAuthenticate with request ID: " + std::to_string(nRequestID));
```

然后构建并运行示例

```sh
./scripts/build_localctp.sh && cargo run --example ctp_query
```

目前建议注释掉 LocalCTP/buildLinux.sh 里的 `buildFunc 6.3.19` 等行，只保留构建所需版本（例如 `6.7.0`）的行，以提高构建速度。

## 发布

```sh
cargo test && cargo publish -p localctp-sys --registry crates-io
```
