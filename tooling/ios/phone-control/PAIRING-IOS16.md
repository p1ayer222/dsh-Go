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
# WSL2：走 Windows Apple Mobile Device Service 时，裸跑 pymobiledevice3 常在
# SavePairRecord 处得到 Number:183（记录已写入 Linux 缓存，但 Windows usbmux 不收）。
# 用仓库包装脚本代替（与下面命令等价，仅忽略 183）：
uvx --managed-python --with pymobiledevice3 python tooling/ios/phone-control/pmd3_wsl.py lockdown pair
uvx --managed-python --with pymobiledevice3 python tooling/ios/phone-control/pmd3_wsl.py lockdown remotepairing --pair
# 生成的 remote_<udid>.plist 在 ~/.local/share/pymobiledevice3/

# 1) USB lockdown 信任（必须先成功；失败时不要跑 remote pair）
python3 -m pymobiledevice3 lockdown pair
# 原生 Windows/macOS 若仍见 183：装/重装 Apple 移动设备支持，换线/USB 口，或 iTunes/3uTools 先配对。

# 2) iOS 16 在 USB 上生成 RP（不要用 bare `remote pair`，那是 Bonjour/Apple TV）
python3 -m pymobiledevice3 lockdown remotepairing --pair
# 或（需已 lockdown paired + 开发者模式）：USB 虚拟网卡出现后
python3 -m pymobiledevice3 bonjour rsd
# 再查 ~/.pymobiledevice3/ 或文档中的 remote_<udid>.plist
```

把生成的 `remote_*.plist` 拷到 iPhone「文件」能打开的位置（**扩展名必须是 `.plist`**；微信里请用「用其他应用打开 → 存储到“文件”」），在 DSH → 本机连接 → **导入配对凭据** 中选「记住」后再点文件。

**检查仍失败时**：若 App 报 `ERR_PHONE_PAIRING_REJECTED`（「手机不认这份配对凭据」），通常是 **本机 remoted 在 pair-verify 时拒绝了记录**，不是没导入、也**与 App 内 DSH 引擎（Node）无关**。用 USB 再跑一遍 `lockdown remotepairing --pair`，导入**新文件**；检查失败快照里若只有 `vpnReachable` 没有 `pairingVerified`，也是同一类问题。

**一直 REJECTED 时（16.6 常见）**：USB 写出的记录默认是「**电脑当主机**」连 remoted；DSH 把同一份 plist 装回手机，经 LocalDevVPN 自连。这在 **17.4+** 更易过；**16.x 上 remoted 可能仍拒绝**。可试：① 手机 **LocalDevVPN 已连接** 时重做 USB `remotepairing --pair` 并再导入；② DSH 内 **清除凭据** 后再导入；③ 仍不行时，实验分支只能先走 **Mac USB `tooling/ios/phone-runner.sh`**，本机连接视为未验证。

**若点文件没反应**：先确认 plist 含 `identifier`、`public_key`、`private_key`（pymobiledevice3 `lockdown remotepairing --pair` 完整跑完）；文件名带 `.plist`；不要从「预览」里点，改走「文件」App 里选；重装/更新 DSH 后 `asCopy` 导入已修复微信来源不可选问题。

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
