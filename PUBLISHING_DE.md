# 发布手册：拿到 `formulae.brew.sh/formula/pdf-zusammenfugen`

本文档是中文操作手册，目标是把项目发布到 `homebrew/core`，最终获得官方页面：

- `https://formulae.brew.sh/formula/pdf-zusammenfugen`

## 0. 前置说明

- 工具仓库：`wsgtcyx/PDF-Zusammenfugen-cargo`
- 目标公式名：`pdf-zusammenfugen`
- 目标版本：`v0.2.0`
- 主页反链：`https://pdfzus.de/`

## 1. 发布 v0.2.0（工具仓库）

```bash
cd /path/to/PDF-Zusammenfugen-cargo
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./scripts/package-dist.sh

git add .
git commit -m "feat: release v0.2.0 with pdf-zusammenfugen binary"
git tag v0.2.0
git push origin main --tags
```

## 2. 计算 v0.2.0 源码 tarball 的 SHA256

```bash
curl -L -o /tmp/v0.2.0.tar.gz \
  https://github.com/wsgtcyx/PDF-Zusammenfugen-cargo/archive/refs/tags/v0.2.0.tar.gz
shasum -a 256 /tmp/v0.2.0.tar.gz
```

把上面的 SHA256 填入：

- `homebrew/Formula/pdf-zusammenfugen.rb`

## 3. 准备 homebrew/core PR

### 3.1 创建 fork（一次性）

先在 GitHub 网页上 fork：

- `Homebrew/homebrew-core` -> `wsgtcyx/homebrew-core`

### 3.2 本地创建分支并添加公式

```bash
git clone https://github.com/Homebrew/homebrew-core.git
cd homebrew-core
git remote add myfork git@github.com:wsgtcyx/homebrew-core.git
git checkout -b pdf-zusammenfugen-new-formula

mkdir -p Formula/p
cp /ABSOLUTER/PFAD/PDF-Zusammenfugen-cargo/homebrew/Formula/pdf-zusammenfugen.rb Formula/p/
```

### 3.3 本地校验（必须）

```bash
brew update
export HOMEBREW_NO_INSTALL_FROM_API=1

brew audit --new --formula Formula/p/pdf-zusammenfugen.rb
brew style Formula/p/pdf-zusammenfugen.rb
brew install --build-from-source ./Formula/p/pdf-zusammenfugen.rb
brew test pdf-zusammenfugen
```

### 3.4 推送并发 PR

```bash
git add Formula/p/pdf-zusammenfugen.rb
git commit -m "pdf-zusammenfugen 0.2.0 (new formula)"
git push myfork pdf-zusammenfugen-new-formula
```

然后在 GitHub 打开 PR：

- base: `Homebrew/homebrew-core:master`（或默认主分支）
- head: `wsgtcyx/homebrew-core:pdf-zusammenfugen-new-formula`

## 4. PR 文案建议（可直接粘贴）

标题：

- `pdf-zusammenfugen 0.2.0 (new formula)`

正文要点：

1. 这是稳定版本 `v0.2.0`，tag 已发布。
2. 工具是开源 MIT，源码构建，不是二进制公式。
3. 提供了最小可复现测试（`--help` + 空输入返回非 0）。
4. homepage 指向项目主页：`https://pdfzus.de/`。

## 5. 合并后验收

当 PR 被合并后，等待同步（通常有延迟），然后检查：

```bash
curl -I https://formulae.brew.sh/formula/pdf-zusammenfugen
curl -s https://formulae.brew.sh/api/formula.json | jq 'map(select(.name=="pdf-zusammenfugen")) | length'
brew install pdf-zusammenfugen
pdf-zusammenfugen --help
```

## 6. 常见失败原因

1. `niche/self-submitted`：社区指标不足、缺少第三方使用信号。
2. `audit/style` 不通过：格式或规则不符合 core 规范。
3. 测试断言太弱：只测 `--version` 通常不够。

## 7. 并行提高通过率（建议同步进行）

1. 让第三方账号提交“需要 Homebrew 安装”的公开 issue/comment。
2. 在 README 增加真实使用案例与引用。
3. 持续积累 stars/forks/watchers，再进行二次 PR（若首轮被拒）。
