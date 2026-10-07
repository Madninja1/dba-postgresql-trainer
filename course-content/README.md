# Course content layout

Course material reserves one directory per UI/content language:

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

`en/` may be empty. Empty locale directories are intentionally kept in Git with `.gitkeep`.

The current DBA-1 question bank is Russian, so its files live under `ru/`. The application UI can already be switched between English and Russian independently of the question-bank language.

The content build script discovers a bundle only when both `topic.json` and `questions.json` exist in the same locale directory. A directory containing only one of those files is rejected as incomplete.

When translated question banks are added later, keep the same course/topic layout, logical topic slug, question keys, answer keys, and source references across languages. Runtime selection between translated question banks can then be added without changing the repository layout.
