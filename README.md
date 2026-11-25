# Music-Player(测试版)

> 使用 Deepseek & Qwen ai 开发
> 目前仍然处在开发阶段

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust&logoColor=white)
![GTK4](https://img.shields.io/badge/GTK-4-5294F2?logo=gtk&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue)
![Status](https://img.shields.io/badge/status-WIP-yellow)

只是一个跑在 [niri] 上的轻量级本地音乐播放器。基于 `gtk4-layer-shell` 实现悬浮面板，

## Features

- 扫描本地音乐目录，递归读取嵌套子目录
- 基于 `rodio` 的播放 / 暂停 / 上一首(未实现) / 下一首(未实现)
- 以 layer-shell 悬浮在 niri 顶层，支持透明背景与圆角
- 支持纯 CSS 定制外观
- TOML 配置：音乐目录、边距、层级(仍在改善)

## Requirements

### Runtime dependencies

| Package | Purpose |
| --------- | --------- |
| `gtk4` | UI toolkit |
| `adwaita-icon-theme` | 图标主题（缺失会导致按钮图标显示为方框） |
| `alsa-lib` | rodio / cpal 音频后端 |
| `glib2` / `pango` / `cairo` | GTK4 依赖 |

### Build dependencies

```bash
sudo pacman -S --needed base-devel rustup pkgconf git
rustup default stable

```

### Compositor

需要支持 `zwlr_layer_shell_v1` 协议的合成器。已测试：**niri**。
理论上 wlroots 系（sway / hyprland）也能跑，但未验证。

## Getting Started

### Build from source

```bash
git clone https://github.com/alittleshark-dev/Music-Player.git
cd niri-music-player
cargo build --release
./target/release/niri-music-player

```

### Configuration

首次运行会在 `~/.config/niri-music-player/config.toml` 生成默认配置：

```toml
[music]
dir = "/home/username/Music"

[layout]
top = 20
right = 20
layer = "top"          # background | bottom | top | overlay
exclusive_zone = -1    # -1 = 悬浮不占位；填正数则让 niri 预留空间

```

## Project Structure

```text
src/
├── main.rs        # 入口，layer-shell 窗口初始化
├── player.rs      # rodio 播放控制
└── load_conf.rs   # 配置读写

~/.config/music-player/
├── config.toml    # 主要配置文件
└── style.css      # 样式表
```

## Known Issues

- [ ] 图标依赖系统 icon theme，未装 `adwaita-icon-theme` 时显示错误
- [ ] 标题目前显示文件名而非 ID3 元数据（计划用 `lofty` 读取）
- [ ] 无播放进度条与音量控制

## Roadmap

- [ ] 读取 ID3 标签，显示真实歌名 / 艺术家
- [ ] 播放列表 UI（`GtkListBox`）
- [ ] 进度条与拖拽 seek
- [ ] 封面显示
- [ ] MPRIS 支持，可被 `playerctl` 控制

## Contributing

Issues 和 PR 都欢迎。提 bug 时请附上：

- 发行版与 niri 版本
- `cargo build` 的完整报错
- 运行时终端输出

本项目遵循 GPL协议 开源

---
Copyright © 2026 alittleshark-dev
