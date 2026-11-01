# Graph Report - soulslike  (2026-11-01)

## Corpus Check
- 58 files · ~141,229 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 7, .wgsl 3, .toml 1)

## Summary
- 1359 nodes · 3504 edges · 72 communities (61 shown, 11 thin omitted)
- Extraction: 86% EXTRACTED · 14% INFERRED · 0% AMBIGUOUS · INFERRED: 504 edges (avg confidence: 0.78)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `cf49e580`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Rng
- PropArt
- render.rs
- Game
- Rig
- synth.rs
- audio.rs
- .rows
- Game
- render_event
- tick
- Act
- Frame
- fx.rs
- terrain.rs
- Input
- 5. Bosse
- beasts.rs
- chars.rs
- v
- art.rs
- deploy.yml
- fight.rs
- Canvas
- FS
- shade
- prof.rs
- text.rs
- Atlas
- Level
- Eclipse Ashes – Spielkonzept und Baustand
- PS
- ships.rs
- Rgba
- main
- Grafik von Throne of the Drowned – Analyse und 1:1-Anleitung für ein neues Projekt
- main.rs
- Pat
- gfx/mod.rs
- env.rs
- Biome
- BS
- .hud
- ui.rs
- Color
- Ponytail Help
- touch.rs
- Mat
- side.rs
- CLAUDE.md – Kontext für Throne of the Drowned
- Spielmechanik und Steuerung – Werte und Nachbau-Anleitung
- input.rs
- Noise
- Tex
- Mode
- ponytail-audit/SKILL.md
- Ponytail Gain
- ponytail-review/SKILL.md
- Ponytail
- README.md
- Graphify-Abbild des Codes
- png.rs
- Throne of the Drowned – Spielkonzept
- 2. Werte und Charakter (`src/game/data.rs`)
- Wave
- ponytail-debt/SKILL.md
- Dark Fantasy Pirate RPG – Grafik-Spezifikation
- 22. Anhang: Shader im Wortlaut
- build.sh
- schwarzwasser

## God Nodes (most connected - your core abstractions)
1. `Canvas` - 178 edges
2. `PropArt` - 92 edges
3. `Rgba` - 78 edges
4. `hex()` - 78 edges
5. `Rig` - 72 edges
6. `Frame` - 59 edges
7. `h01()` - 55 edges
8. `p6()` - 52 edges
9. `v()` - 50 edges
10. `V3` - 46 edges

## Surprising Connections (you probably didn't know these)
- `9. Ablauf eines Bildes (Spiel)` --references--> `tick()`  [EXTRACTED]
  docs/MECHANIK_UND_STEUERUNG.md → src/main.rs
- `install_unlock()` --calls--> `window()`  [INFERRED]
  src/audio.rs → crates/art/src/buildings.rs
- `pick()` --calls--> `l()`  [INFERRED]
  crates/art/src/color.rs → src/game/env.rs
- `pickf()` --calls--> `l()`  [INFERRED]
  crates/art/src/terrain.rs → src/game/env.rs
- `Art` --references--> `Spr`  [EXTRACTED]
  src/art.rs → crates/art/src/atlas.rs

## Import Cycles
- 2-file cycle: `src/game/fight.rs -> src/game/mod.rs -> src/game/fight.rs`

## Communities (72 total, 11 thin omitted)

### Community 1 - "PropArt"
Cohesion: 0.10
Nodes (89): main(), door(), el_glow(), figurehead(), figurehead_slot(), forge(), hub_tree(), scale_area() (+81 more)

### Community 2 - "render.rs"
Cohesion: 0.05
Nodes (27): GfxPlugin, ADD, ALB, att(), build(), COMMON, ct(), extract_packed() (+19 more)

### Community 3 - "Game"
Cohesion: 0.16
Nodes (6): Game, new_player(), Save, SAVE_KEY, store_get(), store_set()

### Community 4 - "Rig"
Cohesion: 0.09
Nodes (29): add(), cross(), dir_yaw(), Dot, dot(), frame_from(), Hit, ID (+21 more)

### Community 5 - "synth.rs"
Cohesion: 0.12
Nodes (27): arp(), down(), drum(), Ev, first(), freq(), Inst, Accordion (+19 more)

### Community 6 - "audio.rs"
Cohesion: 0.15
Nodes (5): Audio, buffer(), Graph, install_unlock(), source()

### Community 7 - ".rows"
Cohesion: 0.14
Nodes (6): worley(), bake_chunk(), ChunkBake, mat_at(), open_water_chunk(), TerrainMap

### Community 8 - "Game"
Cohesion: 0.22
Nodes (5): Game, overlap(), Tgt, Boss, Foe

### Community 9 - "render_event"
Cohesion: 0.16
Nodes (14): add(), ambience(), Bq, crossfade(), env(), exp_decay(), Ft, Band (+6 more)

### Community 11 - "Act"
Cohesion: 0.13
Nodes (15): Act, Art, Attack, Block, Confirm, Down, Drink, Heavy (+7 more)

### Community 12 - "Frame"
Cohesion: 0.10
Nodes (6): MAP, TS, Frame, LightDef, Q, SprOpt

### Community 13 - "fx.rs"
Cohesion: 0.09
Nodes (27): BLOOD, BONE, BurstOpt, Col, DUST, EARTH, EMBER, Fx (+19 more)

### Community 14 - "terrain.rs"
Cohesion: 0.14
Nodes (34): bayer(), BANK_PX, BiomePal, ChunkOut, EN, FACE_PX, FACE_ROWS, field() (+26 more)

### Community 16 - "5. Bosse"
Cohesion: 0.40
Nodes (5): 5.1 Vier Könige (`data.rs::KINGS`, `src/game/boss.rs`), 5.2 Seebosse (`src/game/seaboss.rs`), 5.3 Endkampf (`src/game/endboss.rs`, Ablauf `EP`), 5.4 Strudel, 5. Bosse

### Community 17 - "beasts.rs"
Cohesion: 0.18
Nodes (33): abyss_angler(), angler(), basilisk(), beast_frame(), BONE, build(), canon(), chest() (+25 more)

### Community 18 - "chars.rs"
Cohesion: 0.10
Nodes (32): ANIMS, COATS, Ctx, dead_frame(), DIRS, draw_bell(), draw_skulls(), draw_snake_tail() (+24 more)

### Community 19 - "v"
Cohesion: 0.18
Nodes (35): render_oblique(), v(), banner(), banner_pole(), berth(), brazier(), buoy(), burning_wreck() (+27 more)

### Community 20 - "art.rs"
Cohesion: 0.22
Nodes (9): k(), add_prop(), anim_index(), beast_sheet(), build(), char_ship(), CharSheet, PropDef (+1 more)

### Community 21 - "deploy.yml"
Cohesion: 0.06
Nodes (31): runs-on, steps, concurrency, cancel-in-progress, group, environment, needs, runs-on (+23 more)

### Community 22 - "fight.rs"
Cohesion: 0.12
Nodes (20): AFFS, art_def(), ArtDef, ARTS, Boss, BOSS_HP, BOSS_NAME, BOSS_POST (+12 more)

### Community 23 - "Canvas"
Cohesion: 0.12
Nodes (4): Canvas, glyph(), GLYPHS, MatTex

### Community 24 - "FS"
Cohesion: 0.12
Nodes (18): Foe, foe_def(), FoeDef, FS, Broken, Chase, Dead, Hang (+10 more)

### Community 25 - "shade"
Cohesion: 0.28
Nodes (12): mix(), shade(), flowers(), EdgeNoise, face_pixel(), hill_pixel(), masonry_pixel(), Materials (+4 more)

### Community 26 - "prof.rs"
Cohesion: 0.14
Nodes (11): add(), AUDIO, DRAW, NAMES, on(), PACK, Prof, time() (+3 more)

### Community 27 - "text.rs"
Cohesion: 0.12
Nodes (21): Art, CONTROLS, adv(), Al, C, L, R, BLOOD_L (+13 more)

### Community 28 - "Atlas"
Cohesion: 0.16
Nodes (7): Atlas, encode_canvas(), encode_px(), encode_rows(), Page, PAGE, Spr

### Community 30 - "Eclipse Ashes – Spielkonzept und Baustand"
Cohesion: 0.17
Nodes (12): 1. Baustand (01.11.), 2. Steuerung, 3. Kampfsystem (Werte aus `src/game/fight.rs`), 4. Zone 01: Die Aschenen Klippen von Calamita (gebaut), 5. Weltaufbau: 15 Zonen und 15 Bosse, 6. NPCs und Quests, 7. Grafik, 8. Code-Logik (ursprüngliche Lua-Skizze → Umsetzung) (+4 more)

### Community 31 - "PS"
Cohesion: 0.18
Nodes (11): PS, Art, Attack, Climb, Dead, Drink, Exec, Free (+3 more)

### Community 32 - "ships.rs"
Cohesion: 0.17
Nodes (23): boss_decor(), build(), deck_point(), Design, design(), hull_half(), lateen(), m() (+15 more)

### Community 33 - "Rgba"
Cohesion: 0.23
Nodes (14): from_f(), from_hsl(), lum(), outline_of(), pal(), pick(), ramp_n(), Rgb (+6 more)

### Community 34 - "main"
Cohesion: 0.24
Nodes (22): hub_crown(), big_leaves(), blossom_tree(), branch(), bush(), clump(), crown(), dead_tree() (+14 more)

### Community 35 - "Grafik von Throne of the Drowned – Analyse und 1:1-Anleitung für ein neues Projekt"
Cohesion: 0.09
Nodes (23): 10. Gelände: Materialien, Paletten, Chunk-Backen, 11. Der Renderer: Zeichenlisten und Vertexformat, 12. Render-Durchgänge, Formate, Mischmodi, 13. G-Puffer-Shader, 14. Licht-Shader (`fs_light`), 15. Bloom und Tiefenschärfe, 16. Endbild (`fs_final`), 17. Stimmungen je Zone (+15 more)

### Community 37 - "Pat"
Cohesion: 0.11
Nodes (13): Mat, mat(), Pat, Bricks, Deck, Fur, Grain, Plain (+5 more)

### Community 38 - "gfx/mod.rs"
Cohesion: 0.10
Nodes (22): ATLAS, ATLAS_PX, CHUNK, CHUNK_SLOTS, FLASH, FOG, GfxInit, GHOST (+14 more)

### Community 39 - "env.rs"
Cohesion: 0.33
Nodes (8): apply(), ash(), Env, l(), n3(), GLight, Globals, lin()

### Community 40 - "Biome"
Cohesion: 0.12
Nodes (12): Flat, Biome, Coast, Earth, End, Fire, Frost, Hub (+4 more)

### Community 41 - "BS"
Cohesion: 0.20
Nodes (10): BS, Broken, Dead, Dormant, Idle, Intro, Slam, Sweep (+2 more)

### Community 43 - "ui.rs"
Cohesion: 0.30
Nodes (15): ramp(), anchor(), bar_frame(), br(), brass_rect(), brd(), compass(), emblem() (+7 more)

### Community 44 - "Color"
Cohesion: 0.19
Nodes (10): save_lit(), LIGHT, disc(), R, whirl_debris(), whirl_foam(), whirlpool(), LightEnv (+2 more)

### Community 45 - "Ponytail Help"
Cohesion: 0.22
Nodes (8): Band (Claude Code), Configure Default Mode, Deactivate, Levels, More, Ponytail Help, Skills, Update

### Community 46 - "touch.rs"
Cohesion: 0.43
Nodes (3): Btn, layout(), TouchPad

### Community 47 - "Mat"
Cohesion: 0.11
Nodes (17): Mat, Abyss, Ash, Cobble, Dirt, Grass, Hex, Ice (+9 more)

### Community 48 - "side.rs"
Cohesion: 0.48
Nodes (6): flat_ev(), ladder(), plank(), rock(), T, winch()

### Community 49 - "CLAUDE.md – Kontext für Throne of the Drowned"
Cohesion: 0.22
Nodes (9): Altdokumente (Vorgänger), Architektur, Bauen und testen, CLAUDE.md – Kontext für Eclipse Ashes, `crates/art` (Bibliothek `sw_art`, prozedurale Kunst), Konventionen, Offene Punkte, Projekt und Vorgaben (+1 more)

### Community 50 - "Spielmechanik und Steuerung – Werte und Nachbau-Anleitung"
Cohesion: 0.14
Nodes (14): 10. Nachbau-Reihenfolge für ein neues Projekt, 1.1 Tastatur (`src/game/input.rs`, `keys`), 1.2 Touch (`src/ui/touch.rs`), 1. Steuerung, 3.1 Tod, Feuer, Perlen (`src/game/mod.rs`), 3. Kampf des Spielers (`src/game/actors.rs`, `player_logic`), 4.1 KI-Ablauf, 4.2 Gegnertabelle (Rohwerte; Tempo, Sicht, Reichweite im Spiel ×2) (+6 more)

### Community 52 - "Noise"
Cohesion: 0.26
Nodes (4): BAYER, fade(), grad2(), Noise

### Community 53 - "Tex"
Cohesion: 0.29
Nodes (7): Tex, AtlasCol, AtlasNrm, ChunkCol, ChunkNrm, Mask, Noise

### Community 54 - "Mode"
Cohesion: 0.33
Nodes (6): Mode, Controls, Pause, Play, Shrine, Title

### Community 55 - "ponytail-audit/SKILL.md"
Cohesion: 0.40
Nodes (4): 1. Map first, 2. Look for, 3. Check before you report, 4. Output

### Community 56 - "Ponytail Gain"
Cohesion: 0.40
Nodes (4): Boundaries, Honesty boundary, Ponytail Gain, Scoreboard

### Community 57 - "ponytail-review/SKILL.md"
Cohesion: 0.40
Nodes (4): 1. Understand first, 2. Look for, 3. Check before you report, 4. Output

### Community 58 - "Ponytail"
Cohesion: 0.40
Nodes (4): Before you write, Levels, Ponytail, The smallest complete change

### Community 59 - "README.md"
Cohesion: 0.22
Nodes (4): Bauen, Das Spiel, Eclipse Ashes, Steuerung

### Community 60 - "Graphify-Abbild des Codes"
Cohesion: 0.40
Nodes (4): Benutzen, Graphify-Abbild des Codes, Stand, Wichtige Knotenpunkte (Einstieg)

### Community 61 - "png.rs"
Cohesion: 0.53
Nodes (4): chunk(), crc32(), encode(), save()

### Community 62 - "Throne of the Drowned – Spielkonzept"
Cohesion: 0.05
Nodes (38): 10. Offene Punkte und Ideen, 1. Überblick, 2. Geschichte und Spielablauf, 3.1 Karte, 3.2 Inseln, 3.3 Glutwacht im Detail, 3.4 Seegebiete, 3. Die Welt (+30 more)

### Community 63 - "2. Werte und Charakter (`src/game/data.rs`)"
Cohesion: 0.40
Nodes (5): 2.1 Herkunft, 2.2 Geschenke (`GIFTS`, `new_save`), 2.3 Abgeleitete Werte (`derived`), 2.4 Waffen, 2. Werte und Charakter (`src/game/data.rs`)

### Community 64 - "Wave"
Cohesion: 0.40
Nodes (5): Wave, Saw, Sine, Square, Tri

### Community 65 - "ponytail-debt/SKILL.md"
Cohesion: 0.50
Nodes (3): Boundaries, Output, Scan

### Community 66 - "Dark Fantasy Pirate RPG – Grafik-Spezifikation"
Cohesion: 0.08
Nodes (24): 5.1 Hub: Bassin des Seelenbaums (Bild 1 / 9), 5.2 Gezeiten-Grotte (Bild 2 / 3), 5.3 Hafenstadt (Bild 5), 5.4 Verfluchter Mangrovensumpf (Bild 6 / 7), 5.5 Blutgezeiten (Bild 8), Bildinhalt neu interpretiert, Code-Struktur, Dark Fantasy Pirate RPG – Grafik-Spezifikation (+16 more)

### Community 69 - "22. Anhang: Shader im Wortlaut"
Cohesion: 0.50
Nodes (4): 22. Anhang: Shader im Wortlaut, `common.wgsl`, `post.wgsl`, `scene.wgsl`

## Knowledge Gaps
- **125 isolated node(s):** `schwarzwasser`, `sw-art`, `Player`, `Npc`, `Crew` (+120 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 488 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Canvas` connect `Canvas` to `ships.rs`, `Rgba`, `PropArt`, `main`, `Rig`, `.rows`, `ui.rs`, `Color`, `terrain.rs`, `side.rs`, `beasts.rs`, `chars.rs`, `v`, `art.rs`, `shade`, `Atlas`?**
  _High betweenness centrality (0.344) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Canvas` (e.g. with `Rgba` and `wreck_flame()`) actually correct?**
  _`Canvas` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `schwarzwasser`, `sw-art`, `Player` to the rest of the system?**
  _125 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PropArt` be split into smaller, more focused modules?**
  _Cohesion score 0.0993939393939394 - nodes in this community are weakly interconnected._
- **Why does `Frame` connect `Frame` to `Game`, `main.rs`, `gfx/mod.rs`, `env.rs`, `.hud`, `tick`, `fx.rs`, `touch.rs`, `art.rs`, `text.rs`, `Atlas`, `Level`?**
  _High betweenness centrality (0.224) - this node is a cross-community bridge._
- **Are the 8 inferred relationships involving `PropArt` (e.g. with `figurehead()` and `figurehead_slot()`) actually correct?**
  _`PropArt` has 8 INFERRED edges - model-reasoned connections that need verification._
- **Should `render.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05070422535211268 - nodes in this community are weakly interconnected._