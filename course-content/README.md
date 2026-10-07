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

## Course codes

Course codes are not limited to DBA courses. They use the form:

```text
<track>-<positive-number>
```

The track starts with a lowercase ASCII letter and may contain lowercase ASCII letters, digits and single hyphens. Examples:

```text
dba-1
rust-1
sql-1
python-1
postgresql-dba-2
```

The final numeric component must be greater than zero. This keeps course identity generic so new learning tracks can be added without changing the content schema or application code.

## Topic order and source notes

Each `topic.json` contains two numeric metadata fields in addition to `sort_order`:

```json
{
  "notes_part": 3,
  "topic_number": 3,
  "sort_order": 30
}
```

- `notes_part` identifies the source notes/conspect part the topic belongs to;
- `topic_number` is the human-facing topic number inside the course;
- `sort_order` remains the technical sorting value.

Both `notes_part` and `topic_number` must be positive. `topic_number` must be unique inside one course. They are logical metadata and therefore must stay identical across locale variants of the same topic.

For the current `rust-1` course, the 15 source note files map one-to-one to topic numbers `1` through `15`.

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
- `topic.notes_part`;
- `topic.topic_number`;
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
