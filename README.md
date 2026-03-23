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
* [🏗️ Architecture and Ecosystem](#-architecture-and-ecosystem)
* [🌟 Star History](#-star-history)
* [🗺️ Roadmap](#️-roadmap)
* [🚀 Download Setup](#-download-setup)
  * [App Market](#app-market)
  * [Installation Package](#installation-package)
  * [Package Manager](#package-manager)
  * [Docker Hosting](#docker-hosting)
  * [Unraid Hosting](#unraid-hosting)
  * [TrueNAS Hosting](#TrueNAS-hosting)
  * [Insider Preview](#insider-preview)
* [🏘️ Community](#️-community)
* [🛠️ Development Guide](#️-development-guide)
* [❓ FAQ](#-faq)
  * [How does Wellspring store data?](#how-does-wellspring-store-data)
  * [Does it support data synchronization through a third-party sync disk?](#does-it-support-data-synchronization-through-a-third-party-sync-disk)
  * [Is Wellspring open source?](#is-wellspring-open-source)
  * [How to upgrade to a new version?](#how-to-upgrade-to-a-new-version)
  * [What if some blocks (such as paragraph blocks in list items) cannot find the block icon?](#what-if-some-blocks-such-as-paragraph-blocks-in-list-items-cannot-find-the-block-icon)
  * [What should I do if the data repo key is lost?](#what-should-i-do-if-the-data-repo-key-is-lost)
  * [Do I need to pay for it?](#do-i-need-to-pay-for-it)
* [🙏 Acknowledgement](#-acknowledgement)
  * [Contributors](#contributors)

---

## 💡 Introduction

Wellspring is a privacy-first personal knowledge management system, support fine-grained block-level reference and Markdown
WYSIWYG.

## 🔮 Features

Most features are free, even for commercial use.

* Content block
  * Block-level reference and two-way links
  * Custom attributes
  * SQL query embed
  * Protocol `siyuan://`
* Editor
  * Block-style
  * Markdown WYSIWYG
  * List outline
  * Block zoom-in
  * Million-word large document editing
  * Mathematical formulas, charts, flowcharts, Gantt charts, timing charts, staffs, etc.
  * Web clipping
  * PDF Annotation link
* Export
  * Block ref and embed
  * Standard Markdown with assets
  * PDF, Word and HTML
  * Copy to WeChat MP, Zhihu and Yuque
* Database
  * Table view
* Flashcard spaced repetition
* AI writing and Q/A chat via OpenAI API
* Tesseract OCR 
* Multi-tab, drag and drop to split screen
* Template snippet
* JavaScript/CSS snippet
* Android/iOS/HarmonyOS App
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

## 🚀 Download Setup

It is recommended to give priority to installing through the application market on the desktop and mobile, so that you can upgrade the version with one click in the future.

### Installation Package

* [GitHub](https://github.com/sourdusk/wellspring/releases)

### Package Manager

#### `siyuan`

[![Packaging status](https://repology.org/badge/vertical-allrepos/siyuan.svg)](https://repology.org/project/siyuan/versions)

#### `siyuan-note`

[![Packaging status](https://repology.org/badge/vertical-allrepos/siyuan-note.svg)](https://repology.org/project/siyuan-note/versions)

## 🛠️ Development Guide

See [Development Guide](https://github.com/sourdusk/wellspring/blob/master/.github/CONTRIBUTING.md).

## ❓ FAQ

### How does Wellspring store data?

The data is saved in the workspace folder, in the workspace data folder:

* `assets` is used to save all inserted assets
* `emojis` is used to save emoji images
* `snippets` is used to save code snippets
* `storage` is used to save query conditions, layouts and flashcards, etc.
* `templates` is used to save template snippets
* `widgets` is used to save widgets
* `plugins` is used to save plugins
* `public` is used to save public data
* The rest of the folders are the notebook folders created by the user, files with the suffix of `.sy` in the notebook folder are used to save the document data, and the data format is JSON

### Does it support data synchronization through file synchronization?

Due to the fact that the backend for wellspring is a sql database, file synchronization tools like onedrive and syncthing have the capability to break the program.

Although file synchronization is not supported, there's an S3 and WebDAV sync feature.

In addition, you can also consider manually exporting and importing data to achieve data synchronization:

* Desktop: <kbd>Settings</kbd> - <kbd>Export</kbd> - <kbd>Export Data</kbd> / <kbd>Import Data</kbd>
* Mobile: <kbd>Right column</kbd> - <kbd>About</kbd> - <kbd>Export Data</kbd> / <kbd>Import Data</kbd>

### Is Wellspring open source?

Wellspring is completely open source, and contributions are welcome:

For more details, please refer to [Development Guide](https://github.com/sourdusk/wellspring/blob/master/.github/CONTRIBUTING.md).

### How to upgrade to a new version?

* If it is installed through the installation package on the desktop, you can open the option of <kbd>Settings</kbd> - <kbd>About</kbd> - <kbd>Automatically download update installation package</kbd>, so that Wellspring will automatically download The latest version of the installation package and prompts to install
* If it is installed by manual installation package, please download the installation package again to install

### What if some blocks (such as paragraph blocks in list items) cannot find the block icon?

The first sub-block under the list item is the block icon omitted. You can move the cursor into this block and trigger its block menu with <kbd>Ctrl+/</kbd> .

### What should I do if the data repo key is lost?

* If the data repo key is correctly initialized on multiple devices before, the key is the same on all devices and can be set in <kbd>Settings</kbd> - <kbd>About</kbd> - <kbd>Data repo key</kbd> - <kbd>Copy key string</kbd> retrieve
* If it has not been configured correctly before (for example, the keys on multiple devices are inconsistent) or all devices are unavailable and the key string cannot be obtained, you can reset the key by following the steps below:

  1. Manually back up the data, you can use <kbd>Export Data</kbd> or directly copy the <kbd>workspace/data/</kbd> folder on the file system
  2. <kbd>Settings</kbd> - <kbd>About</kbd> - <kbd>Data rep key</kbd> - <kbd>Reset data repo</kbd>
  3. Reinitialize the data repo key. After initializing the key on one device, other devices import the key
  4. The cloud uses the new synchronization directory, the old synchronization directory is no longer available and can be deleted
  5. The existing cloud snapshots are no longer available and can be deleted

### Do I need to pay for it?

All features are free.

## 🙏 Acknowledgement

The birth of Wellspring is inseparable from many open source projects and contributors, please refer to the project source code kernel/go.mod, app/package.json and project homepage.

The growth of Wellspring is inseparable from user feedback and promotion, thank you for everyone's help to Wellspring ❤️

### Contributors

Welcome to join us and contribute code to Wellspring together.

<a href="https://github.com/sourdusk/wellspring/graphs/contributors">
   <img src="https://contrib.rocks/image?repo=sourdusk/wellspring" />
</a>
