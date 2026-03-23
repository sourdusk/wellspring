<p align="center">
<img alt="Wellspring" src="app/stage/wellspring-logo.svg" width="128">
<br>
<em>Knowledge, from the source.</em>
<br><br>

<p align="center">
<a href="README_zh_CN.md">中文</a> | <a href="README_ja_JP.md">日本語</a> | <a href="README_tr_TR.md">Türkçe</a>
</p>

---

## Table of Contents

* [💡 Introduction](#-introduction)
* [🔮 Features](#-features)
* [🏗️ Architecture and Ecosystem](#️-architecture-and-ecosystem)
* [🌟 Star History](#-star-history)
* [🗺️ Roadmap](#️-roadmap)
* [🚀 Download & Setup](#-download--setup)
  * [Installation Package](#installation-package)
* [🛠️ Development Guide](#️-development-guide)
* [❓ FAQ](#-faq)
  * [How does Wellspring store data?](#how-does-wellspring-store-data)
  * [Does it support file-based synchronization?](#does-it-support-file-based-synchronization)
  * [Is Wellspring open source?](#is-wellspring-open-source)
  * [How do I upgrade to a new version?](#how-do-i-upgrade-to-a-new-version)
  * [How do I access the block menu for blocks without a visible icon?](#how-do-i-access-the-block-menu-for-blocks-without-a-visible-icon)
  * [What do I do if I lose my data repo key?](#what-do-i-do-if-i-lose-my-data-repo-key)
  * [Do I need to pay for it?](#do-i-need-to-pay-for-it)
* [🙏 Acknowledgements](#-acknowledgements)
  * [Contributors](#contributors)

---

## 💡 Introduction

Wellspring is a privacy-first personal knowledge management system with fine-grained block-level referencing and Markdown WYSIWYG editing.

## 🔮 Features

All features are free, including for commercial use.

* Content blocks
  * Block-level references and bidirectional links
  * Custom attributes
  * SQL query embeds
  * Protocol `siyuan://`
* Editor
  * Block-based editing
  * Markdown WYSIWYG
  * List outline
  * Block zoom-in
  * Large document editing (1M+ words)
  * Math formulas, charts, flowcharts, Gantt charts, sequence diagrams, sheet music, and more
  * Web clipping
  * PDF annotation linking
* Export
  * Block references and embeds
  * Standard Markdown with assets
  * PDF, Word, and HTML
* Database
  * Table view
* Flashcard spaced repetition
* AI writing and Q&A via OpenAI API
* Tesseract OCR
* Multi-tab with drag-and-drop split screen
* Template snippets
* JavaScript/CSS snippets
* Android/iOS/HarmonyOS apps
* Docker deployment
* [API](https://github.com/sourdusk/wellspring/blob/master/API.md)
* Community marketplace

## 🏗️ Architecture and Ecosystem

| Project                                                  | Description           | Forks                                                                           | Stars                                                                                | 
|----------------------------------------------------------|-----------------------|---------------------------------------------------------------------------------|--------------------------------------------------------------------------------------|
| [lute](https://github.com/88250/lute)                    | Editor engine         | ![GitHub forks](https://img.shields.io/github/forks/88250/lute)                 | ![GitHub Repo stars](https://img.shields.io/github/stars/88250/lute)                 |
| [chrome](https://github.com/siyuan-note/siyuan-chrome)   | Chrome/Edge extension | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/siyuan-chrome)  | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/siyuan-chrome)  |
| [bazaar](https://github.com/siyuan-note/bazaar)          | Community marketplace | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/bazaar)         | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/bazaar)         |
| [dejavu](https://github.com/siyuan-note/dejavu)          | Data repo             | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/dejavu)         | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/dejavu)         |
| [petal](https://github.com/siyuan-note/petal)            | Plugin API            | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/petal)          | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/petal)          |
| [android](https://github.com/siyuan-note/siyuan-android) | Android App           | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/siyuan-android) | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/siyuan-android) |
| [ios](https://github.com/siyuan-note/siyuan-ios)         | iOS App               | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/siyuan-ios)     | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/siyuan-ios)     |
| [harmony](https://github.com/siyuan-note/siyuan-harmony) | HarmonyOS App         | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/siyuan-harmony) | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/siyuan-harmony) |
| [riff](https://github.com/siyuan-note/riff)              | Spaced repetition     | ![GitHub forks](https://img.shields.io/github/forks/siyuan-note/riff)           | ![GitHub Repo stars](https://img.shields.io/github/stars/siyuan-note/riff)           |

## 🌟 Star History

<a href="https://star-history.com/#siyuan-note/siyuan&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=sourdusk/wellspring&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=sourdusk/wellspring&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=sourdusk/wellspring&type=Date" />
 </picture>
</a>

## 🗺️ Roadmap

* [Wellspring changelog](CHANGELOG.md)

## 🚀 Download & Setup

### Installation Package

* [GitHub Releases](https://github.com/sourdusk/wellspring/releases)

## 🛠️ Development Guide

See the [Development Guide](https://github.com/sourdusk/wellspring/blob/master/.github/CONTRIBUTING.md).

## ❓ FAQ

### How does Wellspring store data?

Data is stored in the workspace folder under `workspace/data/`:

* `assets` — inserted files and images
* `emojis` — custom emoji images
* `snippets` — code snippets
* `storage` — query conditions, layouts, flashcards, etc.
* `templates` — template snippets
* `widgets` — widgets
* `plugins` — plugins
* `public` — public data
* All other folders are user-created notebooks. Document files use the `.sy` extension and are stored as JSON.

### Does it support file-based synchronization?

No. Wellspring uses a SQLite database internally, so file-sync tools like OneDrive or Syncthing can corrupt the data.

Instead, use the built-in S3 or WebDAV sync. You can also manually export and import data:

* Desktop: <kbd>Settings</kbd> → <kbd>Export</kbd> → <kbd>Export Data</kbd> / <kbd>Import Data</kbd>
* Mobile: <kbd>Right column</kbd> → <kbd>About</kbd> → <kbd>Export Data</kbd> / <kbd>Import Data</kbd>

### Is Wellspring open source?

Yes, fully open source. Contributions are welcome — see the [Development Guide](https://github.com/sourdusk/wellspring/blob/master/.github/CONTRIBUTING.md).

### How do I upgrade to a new version?

* **Desktop (auto-update):** Enable <kbd>Settings</kbd> → <kbd>About</kbd> → <kbd>Automatically download update installation package</kbd>. Wellspring will download new versions and prompt you to install.
* **Manual install:** Download the latest installation package from [GitHub Releases](https://github.com/sourdusk/wellspring/releases) and install it over the existing version.

### How do I access the block menu for blocks without a visible icon?

The first sub-block inside a list item doesn't display a block icon. Place your cursor in the block and press <kbd>Ctrl+/</kbd> to open its block menu.

### What do I do if I lose my data repo key?

* If the key was previously set up on multiple devices, it's the same everywhere. Retrieve it from any device via <kbd>Settings</kbd> → <kbd>About</kbd> → <kbd>Data repo key</kbd> → <kbd>Copy key string</kbd>.
* If the key can't be recovered from any device, reset it:

  1. Back up your data first — use <kbd>Export Data</kbd> or copy the `workspace/data/` folder directly
  2. Go to <kbd>Settings</kbd> → <kbd>About</kbd> → <kbd>Data repo key</kbd> → <kbd>Reset data repo</kbd>
  3. Initialize a new key on one device, then import it on your other devices
  4. Create a new cloud sync directory (the old one is no longer usable and can be deleted)
  5. Existing cloud snapshots are no longer usable and can be deleted

### Do I need to pay for it?

No. All features are free.

## 🙏 Acknowledgements

Wellspring is built on the work of many open-source projects and contributors. See `kernel/go.mod`, `app/package.json`, and the project homepage for details.

Thank you to everyone who has contributed feedback and helped spread the word ❤️

### Contributors

Contributions are welcome! See the [Development Guide](https://github.com/sourdusk/wellspring/blob/master/.github/CONTRIBUTING.md) to get started.

<a href="https://github.com/sourdusk/wellspring/graphs/contributors">
   <img src="https://contrib.rocks/image?repo=sourdusk/wellspring" />
</a>
