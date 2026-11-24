# iOS 16.x Remote Pairing（DSH Go 实验分支）

## 为什么 idevice_pair 里要选 Lockdown？

[jkcoxson/idevice_pair](https://github.com/jkcoxson/idevice_pair) 写明：**USB Remote pairing 需要 iOS 17.4+**；更低版本只能用 **Lockdown** 导出「USB 信任」记录。

DSH Go 的本机连接 **不能** 导入 Lockdown 文件（`HostID` / 证书那一套）。原生层只接受 **Remote Pairing** plist：`public_key`、`private_key`、`identifier`（可选 `alt_irk`），与 `idevice::remote_pairing::RpPairingFile` 一致。

因此在 **iOS 16.6.1** 上：idevice_pair 的 Lockdown 是正常选项，但 **不是** DSH 的导入格式。

## iOS 16 上生成可导入的 Remote Pairing

Apple 自 **iOS 16.0** 起在 USB 上会多一个连到 `remoted` 的链路；`pymobiledevice3` 可在 **已有 USB Lockdown 信任** 之后，通过 **Remote Pairing（SRP）** 写出 `remote_<udid>.plist`，字段与 DSH 所需格式一致。

### 前置条件

- 电脑：Windows / macOS / Linux，已装 [pymobiledevice3](https://github.com/doronz88/pymobiledevice3) 与 Apple 移动设备驱动（Windows 见 libimobiledevice 驱动）。
- 手机：USB 连接、解锁、对电脑点 **信任**；**开发者模式** 已开（侧载 DSH 后一般在 设置 → 隐私与安全性）。
- 侧载的 DSH Go 已安装；**LocalDevVPN** 配置按主文档准备。

### 推荐命令（实机 16.x，USB）

```bash
# 1) 经典 lockdown 配对（若本机还没有 pair record）
python3 -m pymobiledevice3 lockdown pair

# 2) 通过 remoted 做 Remote Pairing（会写 ~/.pymobiledevice3/remote_<id>.plist）
python3 -m pymobiledevice3 remote pair
```

把生成的 `remote_*.plist` 拷到 iPhone「文件」能打开的位置，在 DSH → 本机连接 → **导入配对凭据** 中选择该文件（不要选 Lockdown 导出的 plist）。

### iOS 17.4+ 的更简单 USB 路线（对照）

同一工具链上还可使用（**多数 16.x 没有该 lockdown 服务**）：

```bash
python3 -m pymobiledevice3 lockdown remotepairing --pair
```

或 idevice_pair GUI：**Remote pairing → Create**（见 [PAIRING-WINDOWS.md](PAIRING-WINDOWS.md)）。

## 与 DSH 实验性 iOS 16 构建的关系

分支 `ios/ios16-phone-control` 已将 Swift / Rust 系统版本门槛降到 **16**，XCTest 路线以 **PhoneRunner + DDI + RP 记录** 为目标；**不** 承诺 CoreDevice 读屏与 17.4+ 相同能力。详见 [../IOS16-EXPERIMENTAL.md](../IOS16-EXPERIMENTAL.md)。

## 仍可能卡住的点（16.x）

| 项 | 说明 |
|----|------|
| PhoneRunner 宿主 | Xcode 预编译 XCTRunner 常为 17+；16.x 可能需从源码编 arm64 宿主（OpenMinis/iCTRL #15）。 |
| DDI | 须与机型 / 16.x 匹配；App 内「准备」不保证每台机成功。 |
| `remote pair` 失败 | 检查开发者模式、USB 线、驱动；勿把 Lockdown 文件当 RP 导入。 |

## 安全

配对 plist 含私钥，不要提交仓库、不要发聊天；导入后由你选择「记住在这台手机上」或「只用这一次」。
