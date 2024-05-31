# 盛立 CTP 柜台接口 Rust

## rem-super-api-sys

使用方法见[./crates/rem-super-api-sys/README.md](./crates/rem-super-api-sys/README.md)。

## 构建

将柜台的 SDK 复制到 `crates/rem-super-api-sys/thirdparty/rem_super_api/lib` 文件夹里：

```sh
chmod +x scripts/build_localctp.sh && ./scripts/build_localctp.sh
```

## 运行示例

```sh
cargo run --example ctp_query
```

### `error while loading shared libraries: libthostmduserapi_se.so: cannot open shared object file: No such file or directory`

```sh
sudo ln -s crates/rem-super-api-sys/thirdparty/rem_super_api/lib/libthosttraderapi_se.so /usr/local/lib/
sudo ldconfig
```

如果还有问题，尝试用绝对路径。以下绝对路径仅供参考，需要自己修改：

```sh
rm /usr/local/lib/libthosttraderapi_se_local.so
sudo ln -s /root/rem-super-api-sys/crates/rem-super-api-sys/thirdparty/rem_super_api/lib/libthosttraderapi_se_local.so /usr/local/lib/
sudo ldconfig
```

### Order successfully inserted. 但实际上没有下单成功

可能因为 `Rejected due to instrument not found`，可以把 `LocalCTP/TestLocalCTP/instrument.csv` 拖到项目根目录。

## 发布

```sh
cargo test && cargo publish -p rem-super-api-sys --registry crates-io
```
