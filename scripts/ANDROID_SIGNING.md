# Local Android signing

The release build is signed locally. GitHub Actions and repository secrets are not used.

The script reads:

```bash
DBA_TRAINER_KEYSTORE
DBA_TRAINER_KEY_ALIAS
```

Passwords are intentionally not stored in the repository. `apksigner` asks for them interactively.

Example:

```bash
export DBA_TRAINER_KEYSTORE="$HOME/.config/dba-trainer/dba-trainer-release.jks"
export DBA_TRAINER_KEY_ALIAS="dba-trainer"

./scripts/build-release-local.sh 0.2.0
```

If the keystore is in the repository root and is ignored by Git, the default path is:

```text
./dba-trainer-release.jks
```

Do not commit the keystore.
