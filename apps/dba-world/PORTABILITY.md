# DBA World portability boundary

DBA World is a single Godot project. Android and iOS must not have separate
copies of scenes, HUD, assets or gameplay interaction logic.

## Shared in Godot

- outdoor / office / server-room scenes;
- player controller and camera;
- 3D models, materials, animation, particles and audio;
- in-world HUD, terminal, monitoring and interaction UI;
- presentation of incidents and infrastructure state.

Godot scripts must not import or assume Android/iOS APIs. They use the
`PlatformBridge` autoload when communication with the native application is
needed.

## Platform shell

The native application is responsible for:

- launching and closing the Godot runtime;
- lifecycle and orientation integration;
- passing persisted trainer/world state to the game;
- receiving game actions that must be handled by the native/shared core;
- signing, store integration and platform permissions.

Android currently implements this boundary in `DbaWorldGodotActivity`,
`DbaWorldHostPlugin` and `rememberDbaWorldLauncher`. The runtime plugin exposes
JSON-compatible world/environment snapshots to `PlatformBridge`; a future iOS
host should expose the same contract while reusing `apps/dba-world` unchanged.

## State contract direction

Do not let scenes read Android SharedPreferences, iOS UserDefaults or platform
files directly. The intended contract is JSON-compatible data, for example:

```json
{
  "schema": 1,
  "clock": { "day": 1, "minuteOfDay": 540 },
  "player": { "credits": 150, "rank": "JUNIOR_OPERATOR" },
  "incident": { "nodeId": "db-primary", "severity": "CRITICAL" }
}
```

The concrete native transport can differ per platform; the payload and Godot
consumer stay the same.
