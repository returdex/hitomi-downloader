<p align="center">
  <img src="https://github.com/user-attachments/assets/efd0470a-f5cb-4c1d-a0c3-3f5c39113933" alt="Hitomi Downloader banner" />
</p>

# Hitomi Downloader

A desktop downloader for `hitomi.la` built with Tauri, Vue 3, and Rust.

[简体中文](./README.zh-CN.md)

## Overview

Hitomi Downloader provides a GUI workflow for searching comics, downloading galleries with multiple concurrent tasks, browsing local downloads, and exporting downloaded content to archive formats that are easier to read or manage offline.

The current codebase includes:

- Search by keyword or comic ID / URL
- Multi-task download management with pause, resume, cancel, and progress tracking
- Local library browsing for downloaded comics
- Single-comic export to PDF or CBZ
- Batch export of all local comics to PDF or CBZ
- Optional skipping of files that have already been exported in the same format
- CBZ `ComicInfo.xml` tag translation when the UI language is `zh-CN`
- Configurable download/export directories, directory naming rules, and proxy settings
- Built-in i18n support

## Screenshots

![Search and details](https://github.com/user-attachments/assets/fd93fd2f-db16-43b6-86cf-aa643eb572c8)
![Downloads and local library](https://github.com/user-attachments/assets/81a859f2-2a06-4eca-b45f-4f6555cc62c0)

## How To Use

1. Open the `Search` tab and search by keyword, comic ID, or `hitomi.la` URL.
2. Start a download from the card list, or open the `Comic` tab for detailed information before downloading.
3. Monitor active tasks in the download panel on the right.
4. Open the `Local` tab to review downloaded comics and export them when needed.

### Exporting Local Comics

The `Local` tab supports both per-comic export and batch export:

- `Export PDF`: export one comic as a PDF
- `Export CBZ`: export one comic as a CBZ archive
- `Export All PDF`: export all detected local comics as PDF
- `Export All CBZ`: export all detected local comics as CBZ
- `Skip existing PDF/CBZ`: skip items whose target archive already exists

Batch export shows a final summary with exported, skipped, and failed counts. The skip option is remembered locally for later sessions.

### CBZ Tag Translation

When the UI language is set to `zh-CN`, the app translates comic tags written into `ComicInfo.xml` during CBZ export.

- Only exported CBZ metadata is translated
- Search data and in-app source metadata stay unchanged
- Unknown tags fall back to the original text

The translation dictionary is based on [scooderic/exhentai-tags-chinese-translation](https://github.com/scooderic/exhentai-tags-chinese-translation).

## Configuration

The app can be configured from the settings dialog:

- Download image format: `webp` or `avif`
- Proxy mode: system, direct, or custom host/port
- Download directory
- Export directory
- Directory naming template, with placeholders such as:
  - `{id}`
  - `{title}`
  - `{type}`
  - `{artists}`
  - `{language}`
  - `{language_localname}`

Example directory template:

```text
{type}/{artists}/[{artists}] {title}({id}) - {language}({language_localname})
```

## Development

### Tech Stack

- Frontend: Vue 3, TypeScript, Naive UI, Pinia, UnoCSS
- Desktop shell: Tauri v2
- Backend: Rust

### Project Structure

```text
src/              Vue UI, panes, components, stores, i18n
src/locales/      Locale files
src-tauri/src/    Rust commands, downloader, export, config, and Hitomi client logic
src-tauri/resources/
                  Embedded resources such as the zh-CN tag dictionary
```

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/installation)

### Install

```bash
git clone https://github.com/lanyeeee/hitomi-downloader.git
cd hitomi-downloader
pnpm install
```

### Run In Development

```bash
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

## Contributing

Issues and pull requests are welcome.

- For larger feature ideas, open an issue or discussion first
- Documentation improvements, bug fixes, localization work, and dependency maintenance are all welcome
- Please submit pull requests against the `develop` branch

If you are contributing from a fork, a typical flow is:

1. Fork the repository on GitHub
2. Create a feature branch in your fork
3. Commit your changes
4. Push the branch to your fork
5. Open a pull request from your fork to the upstream `develop` branch

## Notes

False-positive antivirus reports can happen for small unsigned desktop projects. If needed, you can build the application locally from source using the steps above.

## Disclaimer

- This project is intended for learning, research, and communication purposes only
- Users are responsible for their own use of the software
- The author is not responsible for legal issues, losses, or third-party disputes caused by misuse

## Credits

- [Pupil](https://github.com/tom5079/Pupil)
- [scooderic/exhentai-tags-chinese-translation](https://github.com/scooderic/exhentai-tags-chinese-translation)
