# DBA Trainer

[Русская версия](README_RU.md)

DBA Trainer is an offline training application for database administration and programming topics. A shared Rust core powers both the terminal interface and the Android application.

The current course focus is PostgreSQL DBA training. The repository is structured so additional learning tracks can be added later, including SQL, Rust, and Python.

## Current release

Current application version: **0.2.0**.

Prebuilt packages are kept in the repository under `releases/<version>/` when they are published. The intended release set is:

```text
releases/
└── v0.2.0/
    ├── dba-trainer-android-v0.2.0.apk
    ├── dba-trainer-tui-linux-x86_64-v0.2.0.tar.gz
    └── dba-trainer-tui-windows-x86_64-v0.2.0.zip
```

The Android package is an installable APK. The Linux and Windows packages contain terminal binaries; there is no separate desktop GUI installer.

## Features

- fully offline operation;
- no account, backend, or network service required;
- SQLite persistence;
- quizzes by topic;
- general quizzes scoped by top-level course, for example `DBA-1`, `DBA-2`, and later course families;
- 20, 50, or all available questions;
- single-choice and multiple-choice questions;
- answer feedback with explanation and source;
- resumable active sessions;
- explicit session cancellation;
- overall, course, topic, and quiz-mode statistics;
- statistics reset;
- English and Russian UI;
- English is the default UI language;
- the selected UI language is remembered independently by Android and TUI;
- runtime content locale selection with fallback when a translation is missing.

## Content languages

Course content is stored per locale:

```text
course-content/
└── dba-1/
    └── 01-tools-install/
        ├── en/
        │   ├── topic.json
        │   └── questions.json
        └── ru/
            ├── topic.json
            └── questions.json
```

A locale directory may be empty. A locale becomes usable only when both `topic.json` and `questions.json` are present.

At runtime the application tries to use the selected locale. If it is unavailable, content falls back to another available locale according to the content loader rules. This means the UI can be English while the current question bank still falls back to Russian.

Translations must preserve the same logical identity across languages: course code, topic slug, question keys, answer keys, question types, correctness flags, and source identity. This keeps statistics and saved sessions stable when the language changes.

See [course-content/README.md](course-content/README.md) for the content format and translation rules.

## Architecture

```text
Android / Jetpack Compose        TUI / Ratatui
            │                         │
            └──────────┬──────────────┘
                       │
                  application
                       │
        ┌──────────────┼──────────────┐
        │              │              │
      domain         content     storage-sqlite
                                      │
                                    SQLite

Android
  └── Kotlin / Compose
       └── UniFFI
            └── mobile-ffi
                 └── shared Rust application stack
```

The UI layers do not issue SQL directly. Business rules live in Rust and are reused by both front ends.

## Repository layout

```text
.
├── apps/
│   ├── android/            Android application
│   └── tui/                terminal application
├── course-content/         versioned training content
├── crates/
│   ├── application/        use cases and repository contracts
│   ├── content/            content loading and validation
│   ├── domain/             domain models
│   ├── mobile-ffi/         UniFFI Android bridge
│   └── storage-sqlite/     SQLite implementation
├── releases/               prebuilt packages committed to the repository
└── tools/
    └── uniffi-bindgen/     Kotlin binding generator wrapper
```

## Build the TUI

Development run:

```bash
cargo run -p dba-trainer-tui
```

Optimized native build:

```bash
cargo build --release -p dba-trainer-tui
```

The executable is created under `target/release/` for the current platform.

## Build Android

Android requires the Android SDK, Android NDK, Rust Android targets, and `cargo-ndk`.

Build the Rust shared libraries:

```bash
cargo ndk \
  -t arm64-v8a \
  -t x86_64 \
  -o apps/android/app/src/main/jniLibs \
  build \
  --release \
  -p dba-trainer-mobile-ffi
```

Regenerate Kotlin UniFFI bindings whenever the exported Rust FFI changes:

```bash
cargo run \
  -p uniffi-bindgen \
  -- \
  generate \
  target/aarch64-linux-android/release/libdba_trainer_mobile_ffi.so \
  --language kotlin \
  --out-dir apps/android/app/src/main/java \
  --no-format
```

Build a debug APK:

```bash
cd apps/android
./gradlew assembleDebug
```

For a public APK, build and sign a release package locally with the long-lived project keystore. The keystore and its passwords must never be committed to Git.

## Release model

This project does not require GitHub Actions or repository secrets for releases. Release artifacts are built locally, verified, and then copied into `releases/<version>/` before the release commit/tag is created.

Before committing a release:

```bash
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
```

Then verify the Android build and test the resulting APK on a device.

## Privacy

DBA Trainer is designed to work offline. Training progress and statistics are stored locally in SQLite. No user account or remote backend is required.

## License

The software source code is licensed under the [Apache License 2.0](LICENSE).

See [NOTICE](NOTICE) for copyright information and the separate treatment of third-party/course material. Content under `course-content/` is not automatically relicensed under Apache-2.0 unless explicitly stated.

Third-party project and product names are used only to identify the technologies being studied. DBA Trainer is an independent project.
