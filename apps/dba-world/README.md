# DBA World 3D

Shared Godot 4.7.2 project for DBA World. This directory is the canonical game
source for Android, iOS and future desktop builds; platform hosts must not fork
its scenes or gameplay UI.

## Current scope

- Android `GodotActivity` host inside the existing APK.
- Dedicated 3D DBA World mode launched from the app mode selector.
- Reusable third-person `PlayerController` with collision, smooth acceleration,
  camera orbit, WASD and an analog virtual joystick.
- Shared interaction layer: desktop `E` and mobile `INTERACT` button.
- Outdoor atmosphere driven by the existing trainer world state: accelerated game
  date/season/weather plus real local device time for visual day phase.
- Three connected greybox scenes:
  - `Outdoor` — campus and DBA World Labs entrance;
  - `Office` — engineer workstation, NOC wall and server-room door;
  - `ServerRoom` — racks, `db-primary`, network switch and UPS.
- `PlatformBridge` autoload as the only intended boundary from game scenes to a
  native host.

## Prototype route

```text
Outdoor
  -> main entrance
Office
  -> workstation inspection
  -> server-room door
Server Room
  -> db-primary inspection
  -> SW-01 inspection
  -> back to Office
```

Open `project.godot` with Godot 4.7.2 to run the project directly on desktop.
The current Android module packages this directory as its root assets directory.
See `PORTABILITY.md` and the repository-level `DBA_WORLD_CROSS_PLATFORM.md`
before adding platform-specific functionality.
