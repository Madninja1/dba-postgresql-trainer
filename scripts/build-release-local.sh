#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-0.2.0}"
KEYSTORE="${DBA_TRAINER_KEYSTORE:-./dba-trainer-release.jks}"
KEY_ALIAS="${DBA_TRAINER_KEY_ALIAS:-dba-trainer}"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

RELEASE_DIR="$ROOT/releases/v${VERSION}"
mkdir -p "$RELEASE_DIR"

echo "== DBA Trainer v${VERSION}: checks =="
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets

echo "== Android native libraries =="
cargo ndk \
  -t arm64-v8a \
  -t x86_64 \
  -o apps/android/app/src/main/jniLibs \
  build \
  --release \
  -p dba-trainer-mobile-ffi

echo "== UniFFI Kotlin bindings =="
cargo run \
  -p uniffi-bindgen \
  -- \
  generate \
  target/aarch64-linux-android/release/libdba_trainer_mobile_ffi.so \
  --language kotlin \
  --out-dir apps/android/app/src/main/java \
  --no-format

echo "== Android release APK =="
(
  cd apps/android
  ./gradlew clean assembleRelease
)

UNSIGNED_APK="$ROOT/apps/android/app/build/outputs/apk/release/app-release-unsigned.apk"
if [[ ! -f "$UNSIGNED_APK" ]]; then
  echo "ERROR: unsigned release APK not found:"
  echo "  $UNSIGNED_APK"
  echo
  echo "Check apps/android/app/build/outputs/apk/release/"
  exit 1
fi

if [[ ! -f "$KEYSTORE" ]]; then
  echo "ERROR: Android keystore not found:"
  echo "  $KEYSTORE"
  echo
  echo "Set DBA_TRAINER_KEYSTORE=/absolute/path/to/dba-trainer-release.jks"
  exit 1
fi

ANDROID_SDK="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Android/Sdk}}"
BUILD_TOOLS_ROOT="$ANDROID_SDK/build-tools"

if [[ ! -d "$BUILD_TOOLS_ROOT" ]]; then
  echo "ERROR: Android build-tools directory not found:"
  echo "  $BUILD_TOOLS_ROOT"
  exit 1
fi

BUILD_TOOLS_DIR="$(find "$BUILD_TOOLS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)"
ZIPALIGN="$BUILD_TOOLS_DIR/zipalign"
APKSIGNER="$BUILD_TOOLS_DIR/apksigner"

if [[ ! -x "$ZIPALIGN" || ! -x "$APKSIGNER" ]]; then
  echo "ERROR: zipalign/apksigner not found in:"
  echo "  $BUILD_TOOLS_DIR"
  exit 1
fi

ALIGNED_APK="$RELEASE_DIR/.dba-trainer-android-v${VERSION}-aligned.apk"
SIGNED_APK="$RELEASE_DIR/dba-trainer-android-v${VERSION}.apk"

"$ZIPALIGN" -f -p 4 "$UNSIGNED_APK" "$ALIGNED_APK"

echo
echo "apksigner will now ask for your keystore password."
"$APKSIGNER" sign \
  --ks "$KEYSTORE" \
  --ks-key-alias "$KEY_ALIAS" \
  --out "$SIGNED_APK" \
  "$ALIGNED_APK"

rm -f "$ALIGNED_APK"

echo "== Verify Android signature =="
"$APKSIGNER" verify --verbose --print-certs "$SIGNED_APK"

echo "== Linux x86_64 TUI =="
cargo build --release -p dba-trainer-tui

TMP_LINUX="$(mktemp -d)"
cp "$ROOT/target/release/dba-trainer-tui" "$TMP_LINUX/dba-trainer-tui"
[[ -f "$ROOT/LICENSE" ]] && cp "$ROOT/LICENSE" "$TMP_LINUX/LICENSE"
[[ -f "$ROOT/README.md" ]] && cp "$ROOT/README.md" "$TMP_LINUX/README.md"
[[ -f "$ROOT/README_RU.md" ]] && cp "$ROOT/README_RU.md" "$TMP_LINUX/README_RU.md"

tar -C "$TMP_LINUX" \
  -czf "$RELEASE_DIR/dba-trainer-tui-linux-x86_64-v${VERSION}.tar.gz" \
  .
rm -rf "$TMP_LINUX"

echo "== Windows x86_64 TUI =="
WINDOWS_TARGET="x86_64-pc-windows-gnu"

if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
  echo "ERROR: MinGW-w64 is not installed."
  echo "Ubuntu/Debian:"
  echo "  sudo apt install mingw-w64 zip"
  exit 1
fi

if ! rustup target list --installed | grep -qx "$WINDOWS_TARGET"; then
  rustup target add "$WINDOWS_TARGET"
fi

export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar

cargo build \
  --release \
  --target "$WINDOWS_TARGET" \
  -p dba-trainer-tui

TMP_WINDOWS="$(mktemp -d)"
cp \
  "$ROOT/target/$WINDOWS_TARGET/release/dba-trainer-tui.exe" \
  "$TMP_WINDOWS/dba-trainer-tui.exe"
[[ -f "$ROOT/LICENSE" ]] && cp "$ROOT/LICENSE" "$TMP_WINDOWS/LICENSE"
[[ -f "$ROOT/README.md" ]] && cp "$ROOT/README.md" "$TMP_WINDOWS/README.md"
[[ -f "$ROOT/README_RU.md" ]] && cp "$ROOT/README_RU.md" "$TMP_WINDOWS/README_RU.md"

(
  cd "$TMP_WINDOWS"
  zip -q -r \
    "$RELEASE_DIR/dba-trainer-tui-windows-x86_64-v${VERSION}.zip" \
    .
)
rm -rf "$TMP_WINDOWS"

echo
echo "== Release files =="
ls -lh "$RELEASE_DIR"
echo
echo "Done: $RELEASE_DIR"
