# Course content layout

[Русская версия](README_RU.md)

Course content is localized by directory. Each logical topic may provide one or more locale variants:

```text
course-content/
└── <course>/
    └── <topic>/
        ├── en/
        │   ├── topic.json
        │   └── questions.json
        └── ru/
            ├── topic.json
            └── questions.json
```

A locale directory may be empty. Empty directories can be reserved in Git with `.gitkeep` and are ignored by the content registry until both `topic.json` and `questions.json` are present.

## Runtime locale selection

Locale-aware content selection is implemented for both Android and TUI.

When the UI language changes, the trainer selects the matching content variant for every logical topic and synchronizes it into the existing SQLite records. Logical IDs stay stable, so sessions and statistics are not reset merely because the displayed language changes.

The selection order for each topic is:

1. exact requested locale, for example `en-us`;
2. base language, for example `en`;
3. the project fallback locale `ru`;
4. the first available locale for that topic.

This means the current empty `en/` directory is valid: an English UI still uses the existing Russian question bank until an English translation is added.

Locale directory names must be lowercase codes such as `en`, `ru` or `pt-br`.

## Translation compatibility rules

A translated variant is not an independent question bank. It is a localized representation of the same logical topic.

Across locales for the same `<course>/<topic>`, keep these values identical:

- `topic.course`;
- `topic.slug`;
- `topic.sort_order`;
- question keys;
- question types;
- answer keys;
- which answers are correct;
- source keys and source kinds.

The following fields may be translated:

- topic title and description;
- question text;
- answer text;
- explanation;
- source module, section and locator text;
- source URL when a localized documentation URL is appropriate.

The built-in content loader validates localized variants before selecting one. If a translation changes logical question/answer identity or correctness, content loading fails instead of silently corrupting session/statistics semantics.

## Current content

The current DBA-1 question bank is Russian and lives under `ru/`.

```text
course-content/
└── dba-1/
    └── 01-tools-install/
        ├── en/
        │   └── .gitkeep
        └── ru/
            ├── topic.json
            └── questions.json
```

Adding a translation requires only adding the matching files to the reserved locale directory while preserving the logical keys described above. No SQLite schema migration is required for a new translation.
