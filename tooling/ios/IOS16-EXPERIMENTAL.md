# iOS 16 实验性手机操控（fork: p1ayer222）

面向 [#1887](https://github.com/ccch1mneyyy/dsh-Go/issues/1887)：**iPhone / iPad OS 16.x**（含 **16.6.1**）上尝试 XCTest 自运行路线。

## 开发方式（本 fork）

- **本机**：只改代码、提交推送；不在这边跑 `cargo` / `xcodebuild` / `flutter build`。
- **CI**：推送到 `ios/**` 后由 GitHub Actions 完成校验与打包：
  - `phone-protocol`（ubuntu）：`build-phone-self-runner.sh --check`、协议测试
  - `ipa`（macos）：`check-archive.sh` → artifact **`ios-unsigned-ipa`** / `DSH-Go-unsigned.ipa`
- 手动触发：**Actions → “iOS unsigned IPA only” → Run workflow**，分支选 `ios/ios16-phone-control`。

## 本 fork 相对上游改了什么

- Swift：`buildAvailable` / 原生 client 门槛 **16.0+**（此前人为锁 17）。
- Rust `phone-self-runner`：接受 **ProductVersion major ≥ 16** 的会话与 DDI 版本解析。
- idevice 补丁：`run_rsd` 不再在 **&lt; 17** 时直接拒绝，改为 **&lt; 16**。

## 没有自动解决的事

| 项 | 说明 |
|---|---|
| **Remote Pairing 生成** | 上游 `PAIRING-WINDOWS.md` 写明 USB Remote Pairing 下限约 **iOS 17.4**。16.6.1 上**可能没有**官方文档化的 RP 导出路径；须自备已有效的 RP plist 或等上游方案。 |
| **PhoneRunner 宿主** | 新版 Xcode 预编译 XCTRunner 宿主常为 **iOS 17+**。16 上需参考 [OpenMinis/iCTRL PR #15](https://github.com/OpenMinis/iCTRL/pull/15)：**从源码编 arm64 宿主**，`IPHONEOS_DEPLOYMENT_TARGET` / Mach-O 与嵌套 `.xctest` 版本一致（勿只改 plist）。用 `tooling/ios/prepare-phone-self-runner.sh` 在 **真机 16.x** 上安装已签名 helper。 |
| **DDI** | 须与 **board/chip + 16.x** 匹配的 Personalized 镜像；App 内「准备」仍走固定 catalog，**不保证**每一台 16.x 都能自动挂载成功。可先在 Mac 上手动挂好 DDI 再测。 |
| **CoreDevice 读屏** | 完整 CoreDevice 能力仍建议 **17.4+**；16.x 目标以 **XCTest 增强**为主，不承诺与 17 同能力。 |

## 建议验证顺序（16.6.1）

1. 开发者模式、LocalDevVPN、已签名主 App。
2. 安装 **16 兼容**的 `PhoneRunnerUITests-Runner`（见上）。
3. 导入 RP 记录（若无法生成，本条即卡点）。
4. 设置里打开 XCTest 增强 → 连接检查 → 读屏 / 点按。

未在真机 16.6.1 上跑通前，本分支标记为 **实验性**，不声称与上游等价。

