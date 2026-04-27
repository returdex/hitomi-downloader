<p align="center">
  <img src="https://github.com/user-attachments/assets/efd0470a-f5cb-4c1d-a0c3-3f5c39113933" alt="Hitomi Downloader banner" />
</p>

# Hitomi Downloader

一个基于 Tauri、Vue 3 和 Rust 构建的 `hitomi.la` 桌面下载器。

[English](./README.md)

## 项目简介

Hitomi Downloader 提供了从搜索、下载、管理到导出的完整桌面端流程，适合将漫画内容下载到本地后继续整理、归档和离线阅读。

当前代码已经支持：

- 通过关键词搜索漫画
- 通过漫画 ID 或 `hitomi.la` 链接直接定位作品
- 多任务下载，支持暂停、继续、取消和进度展示
- 浏览本地已下载漫画
- 单本导出为 PDF 或 CBZ
- 一键批量导出本地全部漫画为 PDF 或 CBZ
- 跳过已经存在同格式导出文件的项目
- 当界面语言为 `zh-CN` 时，为 CBZ 的 `ComicInfo.xml` 写入中文标签翻译
- 自定义下载目录、导出目录、目录命名规则和代理配置
- 内置多语言支持

## 界面预览

![搜索与详情](https://github.com/user-attachments/assets/fd93fd2f-db16-43b6-86cf-aa643eb572c8)
![下载与本地库存](https://github.com/user-attachments/assets/81a859f2-2a06-4eca-b45f-4f6555cc62c0)

## 使用说明

1. 在 `Search` 页签中输入关键词、漫画 ID 或 `hitomi.la` 链接进行搜索。
2. 在搜索结果卡片上直接开始下载，或进入 `Comic` 页签查看详情后再下载。
3. 右侧下载面板会显示当前任务状态和速度。
4. 在 `Local` 页签中查看本地已下载漫画，并执行导出操作。

### 本地导出

`Local` 页签目前支持单本导出和批量导出：

- `Export PDF`：将单本漫画导出为 PDF
- `Export CBZ`：将单本漫画导出为 CBZ
- `Export All PDF`：将本地漫画全部导出为 PDF
- `Export All CBZ`：将本地漫画全部导出为 CBZ
- `Skip existing PDF/CBZ`：如果目标文件已存在，则跳过该项目

批量导出结束后，界面会显示导出成功、跳过和失败的数量统计。跳过同格式文件的开关会保存在本地，方便下次继续使用。

### CBZ 标签翻译

当界面语言设置为 `zh-CN` 时，程序会在导出 CBZ 时翻译写入 `ComicInfo.xml` 的标签内容。

- 只会翻译 CBZ 元数据中的标签
- 不会改动程序内部数据或搜索使用的原始标签
- 没有匹配翻译的标签会保留原文

当前翻译词典来自 [scooderic/exhentai-tags-chinese-translation](https://github.com/scooderic/exhentai-tags-chinese-translation)。

## 配置说明

可以在设置对话框中调整以下内容：

- 下载图片格式：`webp` 或 `avif`
- 代理模式：系统代理、直连、自定义代理
- 下载目录
- 导出目录
- 目录命名模板

目录模板目前支持这些占位符：

- `{id}`
- `{title}`
- `{type}`
- `{artists}`
- `{language}`
- `{language_localname}`

示例：

```text
{type}/{artists}/[{artists}] {title}({id}) - {language}({language_localname})
```

## 开发说明

### 技术栈

- 前端：Vue 3、TypeScript、Naive UI、Pinia、UnoCSS
- 桌面容器：Tauri v2
- 后端：Rust

### 目录结构

```text
src/                  Vue 界面、页签、组件、状态管理、多语言
src/locales/          多语言文案
src-tauri/src/        Rust 命令、下载逻辑、导出逻辑、配置和 Hitomi 客户端
src-tauri/resources/  内置资源，例如中文标签翻译词典
```

### 环境要求

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/installation)

### 安装依赖

```bash
git clone https://github.com/lanyeeee/hitomi-downloader.git
cd hitomi-downloader
pnpm install
```

### 开发运行

```bash
pnpm tauri dev
```

### 构建

```bash
pnpm tauri build
```

## 贡献说明

欢迎提交 issue 和 pull request。

- 较大的新功能建议先开 issue 或 discussion 讨论
- 文档改进、Bug 修复、翻译补充、依赖升级都欢迎提交
- Pull Request 请提交到 `develop` 分支

如果你是从自己的 fork 发起贡献，推荐流程如下：

1. 在 GitHub 上 fork 原仓库
2. 在你的 fork 中创建功能分支
3. 提交代码改动
4. 推送到你的 fork
5. 从你的 fork 向上游仓库的 `develop` 分支发起 Pull Request

## 说明

未签名的个人桌面项目有时会遇到杀毒软件误报。如果你介意这一点，可以按上面的步骤自行从源码构建。

## 免责声明

- 本项目仅供学习、研究与交流使用
- 使用本工具所带来的风险由使用者自行承担
- 对因不当使用导致的法律问题、损失或第三方纠纷，作者不承担责任

## 致谢

- [Pupil](https://github.com/tom5079/Pupil)
- [scooderic/exhentai-tags-chinese-translation](https://github.com/scooderic/exhentai-tags-chinese-translation)
