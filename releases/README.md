# Prebuilt releases

[Русская версия](README_RU.md)

This directory intentionally stores prebuilt DBA Trainer packages in Git.

Each published version should use its own directory:

```text
releases/
└── v0.2.0/
    ├── dba-trainer-android-v0.2.0.apk
    ├── dba-trainer-tui-linux-x86_64-v0.2.0.tar.gz
    └── dba-trainer-tui-windows-x86_64-v0.2.0.zip
```

Release binaries are build artifacts, not source-of-truth files. They must be rebuilt from the matching Git tag and tested before being committed here.

Android release APKs must be signed with the same long-lived project keystore so that future versions can update an already installed application. The keystore and passwords must never be committed to this repository.
