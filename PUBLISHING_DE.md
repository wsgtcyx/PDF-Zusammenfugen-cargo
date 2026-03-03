# 从零到发布：PDF-Zusammenfugen Cargo + Homebrew Tap

本文档展示完整发布流程，涉及以下两个仓库：

- 工具仓库：`wsgtcyx/PDF-Zusammenfugen-cargo`
- Tap 仓库：`wsgtcyx/PDF-Zusammenfugen-homebrew`

## 1. 初始化仓库并推送

```bash
cd PDF-Zusammenfugen-cargo-from-zero
git init
git add .
git commit -m "feat: initial rust pdf merge cli"
git branch -M main
git remote add origin git@github.com:wsgtcyx/PDF-Zusammenfugen-cargo.git
git push -u origin main
```

## 2. 确认版本并打 Tag

- 在 `Cargo.toml` 中更新版本号（例如 `0.1.0`）
- Tag 规范：`v<version>`

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: release v0.1.0"
git tag v0.1.0
git push origin main --tags
```

## 3. 构建并生成发布产物

```bash
./scripts/package-dist.sh
```

产物会输出到 `dist/`：

- Release 二进制压缩包（`*.tar.gz`）
- Source 包（`*.crate`）
- `checksums.txt`（SHA256 校验）

## 4. 创建 GitHub Release 并上传产物

1. 进入 GitHub -> `PDF-Zusammenfugen-cargo` -> Releases -> "Draft a new release"
2. 选择 Tag `v0.1.0`
3. 上传 `dist/` 里的产物
4. 发布 Release

## 5. 准备 Homebrew Formula

本项目内已提供 Formula 模板：

- `homebrew/Formula/pdf-zusammenfugen-cargo.rb`

发布前需要更新两个值：

1. `url`：改为该 Tag 对应的源码 tarball 链接
2. `sha256`：使用该 `url` 对应文件的真实 SHA256

计算 SHA256：

```bash
curl -L -o /tmp/v0.1.0.tar.gz \
  https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo/archive/refs/tags/v0.1.0.tar.gz
shasum -a 256 /tmp/v0.1.0.tar.gz
```

## 6. 把 Formula 推送到 Tap 仓库

```bash
git clone git@github.com:wsgtcyx/PDF-Zusammenfugen-homebrew.git
cd PDF-Zusammenfugen-homebrew
mkdir -p Formula
cp /ABSOLUTER/PFAD/PDF-Zusammenfugen-cargo-from-zero/homebrew/Formula/pdf-zusammenfugen-cargo.rb Formula/
git add Formula/pdf-zusammenfugen-cargo.rb
git commit -m "feat: add pdf-zusammenfugen-cargo formula"
git push origin main
```

## 7. 验证用户安装路径

```bash
brew tap wsgtcyx/pdfzus https://github.com/wsgtcyx/PDF-Zusammenfugen-homebrew.git
brew install wsgtcyx/pdfzus/pdf-zusammenfugen-cargo
pdf-zusammenfugen-cargo --help
```

## 8. 发布前本地端到端检查

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

Homebrew 本地验证（当前推荐方式：临时 Tap）：

```bash
brew tap-new wsgtcyx/pdfzus-local
TAP_DIR="$(brew --repository)/Library/Taps/wsgtcyx/homebrew-pdfzus-local"
cp homebrew/Formula/pdf-zusammenfugen-cargo.rb "$TAP_DIR/Formula/"

brew install --build-from-source wsgtcyx/pdfzus-local/pdf-zusammenfugen-cargo
brew test wsgtcyx/pdfzus-local/pdf-zusammenfugen-cargo

brew uninstall --force wsgtcyx/pdfzus-local/pdf-zusammenfugen-cargo
brew untap wsgtcyx/pdfzus-local
```

## 9. 常见问题与排查

### SHA 不匹配

- 原因：`url` 和 `sha256` 不是同一个文件计算出来的
- 解决：重新 `curl -L` 下载后再计算一次 SHA256

### Tag 找不到

- 原因：本地打了 tag，但没推到远端
- 解决：`git push origin --tags`

### Homebrew 使用旧缓存

```bash
brew uninstall pdf-zusammenfugen-cargo || true
brew cleanup
rm -rf "$(brew --cache)/downloads"/*pdf-zusammenfugen* || true
```

## 10. 反链检查

反链刻意只放在 README/仓库简介中。

检查项：

1. 打开 GitHub README
2. 能看到锚文本 `PDF zusammenfuegen online`
3. 链接目标是你的首页
4. 可选：把同样文案写进 GitHub 仓库描述
