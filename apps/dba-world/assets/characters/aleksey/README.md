# Aleksey character asset

This directory is the canonical art source slot for the DBA World protagonist.
Gameplay and camera logic must not be moved here; they remain in
`res://scripts/player/player_controller.gd`.

## Runtime model

The authored character model is expected at:

```text
res://assets/characters/aleksey/models/aleksey.glb
```

`res://scenes/player/aleksey_visual.tscn` loads that model at runtime. Until the
GLB is present, the game intentionally displays a neutral capsule marker rather
than reconstructing Aleksey from Godot primitive body parts.

## Authoring target

The approved visual direction is the Aleksey turnaround/model sheet created for
DBA World:

- male, 21 years old;
- approximately 182 cm tall;
- slim, reasonably fit build;
- light-brown / dark-blond tousled hair;
- light gray hoodie;
- charcoal jacket;
- dark cargo trousers;
- practical gray sneakers;
- understated wrist device;
- grounded near-future 2045 style, not armor and not exaggerated cyberpunk.

The in-game model should preserve the silhouette and proportions of that sheet.

## Coordinate and import contract

Author and export the model so that:

- feet rest on `Y = 0`;
- character height is approximately `1.82` Godot units;
- character is centered around `X = 0`;
- forward direction is Godot `-Z`;
- scale is `1.0` on import;
- transforms are applied before export;
- the GLB root imports as `Node3D`;
- all required materials/textures are included or stored under this directory.

Do not add Android/iOS-specific resources or scripts to the character asset.

## Recommended mobile budget

Initial target for the main player character:

- LOD0: about 25k-40k triangles;
- LOD1: about 12k-20k triangles;
- LOD2: about 5k-10k triangles;
- textures: 1K-2K depending on material importance;
- one humanoid skeleton;
- as few material slots as practical.

These are art targets, not hard runtime invariants.

## Suggested model structure

The exact imported node names are not currently required by gameplay, but the
source asset should stay logically separable:

```text
Aleksey
├── Skeleton3D
│   └── Body / skinned character mesh
├── Hair
├── Hoodie / Jacket
├── CargoPants
├── Sneakers
└── WristDevice
```

Clothing may be authored as one skinned mesh if that is better for mobile
rendering. Logical separation is more important in the source file than forcing
many runtime draw calls.

## Animation contract (next stage)

The first authored animation set should eventually provide:

- `idle`;
- `walk`;
- `run`;
- `interact`.

Animation wiring belongs in the player visual/controller layer once the real
rigged GLB exists. Do not encode gameplay state into the model file.

## Directory layout

```text
aleksey/
├── README.md
├── models/
│   └── aleksey.glb
├── textures/
├── animations/
└── reference/
```

The `reference/` directory is for local character/model sheets and modeling
references. Reference art is not required at runtime.
