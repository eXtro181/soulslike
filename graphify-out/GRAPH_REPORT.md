# Graph Report - soulslike  (2026-11-01)

## Corpus Check
- 64 files · ~204,875 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: .wgsl 3, .toml 1, (none) 1)

## Summary
- 2010 nodes · 5614 edges · 76 communities (71 shown, 5 thin omitted)
- Extraction: 87% EXTRACTED · 13% INFERRED · 0% AMBIGUOUS · INFERRED: 742 edges (avg confidence: 0.76)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ffda89f0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- World
- PropArt
- render.rs
- Game
- Rig
- synth.rs
- prof.rs
- .rows
- Game
- Boss
- main.rs
- Act
- Frame
- fx.rs
- terrain.rs
- Input
- data.rs
- beasts.rs
- chars.rs
- v
- art.rs
- deploy.yml
- ship.rs
- Canvas
- Art
- rnd
- Cmd
- text.rs
- Atlas
- Save
- endboss.rs
- save.rs
- ships.rs
- Rgba
- main
- Grafik von Throne of the Drowned – Analyse und 1:1-Anleitung für ein neues Projekt
- .interactables
- Pat
- gfx/mod.rs
- .update_sea_bosses
- Biome
- game/mod.rs
- CK
- ui.rs
- Color
- actors.rs
- PS
- Mat
- seaboss.rs
- CLAUDE.md – Kontext für Throne of the Drowned
- Spielmechanik und Steuerung – Werte und Nachbau-Anleitung
- ES
- Noise
- 6. Spielsysteme
- 9. Technik
- BS
- Rarity
- SS
- AK
- README.md
- WorldView
- png.rs
- Throne of the Drowned – Spielkonzept
- 7. Grafik
- Body
- Save
- Dark Fantasy Pirate RPG – Grafik-Spezifikation
- Hover
- Cut
- 22. Anhang: Shader im Wortlaut
- Technische Umsetzung
- build.sh
- schwarzwasser
- Ingame-Szenen

## God Nodes (most connected - your core abstractions)
1. `Canvas` - 173 edges
2. `World` - 117 edges
3. `Frame` - 115 edges
4. `Game` - 109 edges
5. `PropArt` - 93 edges
6. `Art` - 81 edges
7. `Rgba` - 78 edges
8. `hex()` - 76 edges
9. `Rig` - 72 edges
10. `p6()` - 52 edges

## Surprising Connections (you probably didn't know these)
- `8. Spielstand (`data.rs::Save`, `src/save.rs`)` --references--> `Save`  [EXTRACTED]
  docs/MECHANIK_UND_STEUERUNG.md → src/game/data.rs
- `9. Ablauf eines Bildes (Spiel)` --references--> `tick()`  [EXTRACTED]
  docs/MECHANIK_UND_STEUERUNG.md → src/main.rs
- `dpr()` --calls--> `window()`  [INFERRED]
  src/ui/textinput.rs → crates/art/src/buildings.rs
- `install_unlock()` --calls--> `window()`  [INFERRED]
  src/audio.rs → crates/art/src/buildings.rs
- `field()` --calls--> `window()`  [INFERRED]
  src/ui/textinput.rs → crates/art/src/buildings.rs

## Import Cycles
- 2-file cycle: `src/game/dialog.rs -> src/game/mod.rs -> src/game/dialog.rs`
- 2-file cycle: `src/game/mod.rs -> src/ui/hud.rs -> src/game/mod.rs`
- 3-file cycle: `src/game/dialog.rs -> src/game/mod.rs -> src/ui/hud.rs -> src/game/dialog.rs`

## Communities (76 total, 5 thin omitted)

### Community 0 - "World"
Cohesion: 0.06
Nodes (40): Rng, T, shop_def(), Arena, Collider, Box, Circle, Deco (+32 more)

### Community 1 - "PropArt"
Cohesion: 0.08
Nodes (101): door(), el_glow(), figurehead(), figurehead_slot(), forge(), hub_crown(), hub_tree(), scale_area() (+93 more)

### Community 2 - "render.rs"
Cohesion: 0.06
Nodes (24): ADD, ALB, att(), COMMON, ct(), extract_packed(), finish(), Gpu (+16 more)

### Community 3 - "Game"
Cohesion: 0.09
Nodes (7): king_for_zone(), crew(), derived(), item(), entry(), weapon_entries(), Game

### Community 4 - "Rig"
Cohesion: 0.09
Nodes (28): add(), cross(), dir_yaw(), Dot, dot(), frame_from(), Hit, ID (+20 more)

### Community 5 - "synth.rs"
Cohesion: 0.07
Nodes (46): add(), ambience(), arp(), Bq, crossfade(), down(), drum(), env() (+38 more)

### Community 6 - "prof.rs"
Cohesion: 0.05
Nodes (21): Audio, buffer(), Graph, install_unlock(), source(), add(), AUDIO, BAKE (+13 more)

### Community 7 - ".rows"
Cohesion: 0.13
Nodes (16): mix(), shade(), flowers(), worley(), bake_chunk(), ChunkBake, EdgeNoise, face_pixel() (+8 more)

### Community 8 - "Game"
Cohesion: 0.08
Nodes (17): Actor, Enemy, Game, HurtOpt, Player, Target, Boss, EndShip (+9 more)

### Community 9 - "Boss"
Cohesion: 0.13
Nodes (16): AO, AP, Active, Recover, Windup, Atk, Boss, Game (+8 more)

### Community 10 - "main.rs"
Cohesion: 0.07
Nodes (21): new_save(), Options, GfxPlugin, build(), App, copy_opts(), Infos, load_opts() (+13 more)

### Community 11 - "Act"
Cohesion: 0.06
Nodes (28): Act, Attack, Belt1, Belt2, Confirm, Down, Drink, Heavy (+20 more)

### Community 12 - "Frame"
Cohesion: 0.08
Nodes (9): draw_tentacle_bones(), Game, draw_tentacle(), Frame, LightDef, Q, draw(), FLAMES (+1 more)

### Community 13 - "fx.rs"
Cohesion: 0.07
Nodes (30): BLOOD, Bolt, BONE, BurstOpt, Col, DUST, EARTH, EMBER (+22 more)

### Community 14 - "terrain.rs"
Cohesion: 0.15
Nodes (37): bayer(), BANK_PX, biome_pal(), BiomePal, EN, FACE_PX, FACE_ROWS, field() (+29 more)

### Community 15 - "Input"
Cohesion: 0.12
Nodes (19): Input, Menu, MenuOut, Close, LookChanged, None, OptionsChanged, Start (+11 more)

### Community 16 - "data.rs"
Cohesion: 0.08
Nodes (32): 5.1 Vier Könige (`data.rs::KINGS`, `src/game/boss.rs`), Crew, CREW, Derived, el_name(), FIRES, Gift, GIFTS (+24 more)

### Community 17 - "beasts.rs"
Cohesion: 0.17
Nodes (35): abyss_angler(), angler(), basilisk(), beast_frame(), BONE, build(), canon(), chest() (+27 more)

### Community 18 - "chars.rs"
Cohesion: 0.10
Nodes (32): ANIMS, COATS, Ctx, dead_frame(), DIRS, draw_bell(), draw_skulls(), draw_snake_tail() (+24 more)

### Community 19 - "v"
Cohesion: 0.18
Nodes (35): render_oblique(), v(), banner(), banner_pole(), berth(), brazier(), buoy(), burning_wreck() (+27 more)

### Community 20 - "art.rs"
Cohesion: 0.12
Nodes (15): anim_index(), beast_sheet(), BOSS_STRIP, build_malakor(), build_player(), build_sea_boss(), build_ship(), build_title() (+7 more)

### Community 21 - "deploy.yml"
Cohesion: 0.06
Nodes (31): runs-on, steps, concurrency, cancel-in-progress, group, environment, needs, runs-on (+23 more)

### Community 22 - "ship.rs"
Cohesion: 0.31
Nodes (5): GhostShip, Hull, Ship, turn_to(), wrap()

### Community 23 - "Canvas"
Cohesion: 0.16
Nodes (3): Canvas, ChunkOut, MatTex

### Community 24 - "Art"
Cohesion: 0.14
Nodes (11): Banner, Dialog, draw_legend(), Game, inside(), Ui, w2ui(), world_k() (+3 more)

### Community 25 - "rnd"
Cohesion: 0.12
Nodes (8): Game, Game, Game, ng_k(), rnd(), rr(), Game, Kraken

### Community 26 - "Cmd"
Cohesion: 0.06
Nodes (36): level_cost(), Cmd, Buy, BuyWeapon, Cheat, Close, Debug, DebugGive (+28 more)

### Community 27 - "text.rs"
Cohesion: 0.10
Nodes (28): Art, adv(), Al, C, L, BLOOD, BLOOD_L, BONE (+20 more)

### Community 28 - "Atlas"
Cohesion: 0.12
Nodes (9): Atlas, encode_canvas(), encode_px(), encode_rows(), Page, PAGE, Spr, PropDef (+1 more)

### Community 29 - "Save"
Cohesion: 0.22
Nodes (13): upgrade_cost(), ch(), Choice, ShipUp, crew_talk(), dialog_for(), FIGS, pages() (+5 more)

### Community 30 - "endboss.rs"
Cohesion: 0.08
Nodes (20): BOW, DRAIN_T, END_NAME, EndBoss, EP, Approach, Done, Dormant (+12 more)

### Community 31 - "save.rs"
Cohesion: 0.13
Nodes (15): create(), derive(), ITER, KEY, list(), load_all(), login(), profile_id() (+7 more)

### Community 32 - "ships.rs"
Cohesion: 0.17
Nodes (24): render_yaw(), boss_decor(), build(), deck_point(), Design, design(), hull_half(), lateen() (+16 more)

### Community 33 - "Rgba"
Cohesion: 0.19
Nodes (10): from_f(), from_hsl(), lum(), outline_of(), pal(), Rgb, Rgba, rgbf() (+2 more)

### Community 34 - "main"
Cohesion: 0.21
Nodes (21): main(), big_leaves(), blossom_tree(), branch(), bush(), clump(), crown(), crystal() (+13 more)

### Community 35 - "Grafik von Throne of the Drowned – Analyse und 1:1-Anleitung für ein neues Projekt"
Cohesion: 0.09
Nodes (23): 10. Gelände: Materialien, Paletten, Chunk-Backen, 11. Der Renderer: Zeichenlisten und Vertexformat, 12. Render-Durchgänge, Formate, Mischmodi, 13. G-Puffer-Shader, 14. Licht-Shader (`fs_light`), 15. Bloom und Tiefenschärfe, 16. Endbild (`fs_final`), 17. Stimmungen je Zone (+15 more)

### Community 36 - ".interactables"
Cohesion: 0.26
Nodes (12): Focus, Use, Board, Call, Chest, Enter, Fire, Gate (+4 more)

### Community 37 - "Pat"
Cohesion: 0.11
Nodes (13): Mat, mat(), Pat, Bricks, Deck, Fur, Grain, Plain (+5 more)

### Community 38 - "gfx/mod.rs"
Cohesion: 0.05
Nodes (48): pick(), apply(), Env, env(), l(), lerp(), lerp3(), mix() (+40 more)

### Community 39 - ".update_sea_bosses"
Cohesion: 0.25
Nodes (3): Game, SeaBoss, Slam

### Community 40 - "Biome"
Cohesion: 0.12
Nodes (10): Flat, Biome, Coast, Earth, End, Fire, Frost, Hub (+2 more)

### Community 41 - "game/mod.rs"
Cohesion: 0.14
Nodes (10): Timed, Banner, Whirl, WAKE_END, WAKE_FADE, WAKE_HOP, WAKE_JOLT, WAKE_RISE (+2 more)

### Community 42 - "CK"
Cohesion: 0.12
Nodes (16): BUTTER, CK, Butterfly, Cat, Chicken, Crab, Crow, Firefly (+8 more)

### Community 43 - "ui.rs"
Cohesion: 0.32
Nodes (14): ramp(), anchor(), bar_frame(), br(), brass_rect(), brd(), compass(), emblem() (+6 more)

### Community 44 - "Color"
Cohesion: 0.19
Nodes (10): save_lit(), LIGHT, disc(), R, whirl_debris(), whirl_foam(), whirlpool(), LightEnv (+2 more)

### Community 45 - "actors.rs"
Cohesion: 0.15
Nodes (13): GroundItem, OnHit, Damage, Explode, Shard, Splash, Owner, Enemy (+5 more)

### Community 46 - "PS"
Cohesion: 0.14
Nodes (14): PS, Attack, Cast, Dead, Drink, Free, Hurt, Parry (+6 more)

### Community 47 - "Mat"
Cohesion: 0.10
Nodes (17): Mat, Abyss, Ash, Cobble, Dirt, Grass, Hex, Ice (+9 more)

### Community 48 - "seaboss.rs"
Cohesion: 0.22
Nodes (7): Companion, CrewGuns, KRAKEN_COOL, boss_pose(), sea_king(), SEA_KINGS, SeaKing

### Community 49 - "CLAUDE.md – Kontext für Throne of the Drowned"
Cohesion: 0.18
Nodes (11): Architektur, Bauen und testen, CLAUDE.md – Kontext für Throne of the Drowned, `crates/art` (Bibliothek `sw_art`, prozedurale Kunst), Debug-URL-Parameter (`?debug&start=<feuer-id>`), Grafik, Konventionen, Offene Punkte (+3 more)

### Community 50 - "Spielmechanik und Steuerung – Werte und Nachbau-Anleitung"
Cohesion: 0.11
Nodes (19): 10. Nachbau-Reihenfolge für ein neues Projekt, 1.1 Tastatur (`src/game/input.rs`, `keys`), 1.2 Touch (`src/ui/touch.rs`), 1. Steuerung, 2.1 Herkunft, 2.2 Geschenke (`GIFTS`, `new_save`), 2.3 Abgeleitete Werte (`derived`), 2.4 Waffen (+11 more)

### Community 51 - "ES"
Cohesion: 0.20
Nodes (10): ES, Chase, Dead, Hurt, Idle, Lurk, Recover, Stagger (+2 more)

### Community 52 - "Noise"
Cohesion: 0.33
Nodes (3): fade(), grad2(), Noise

### Community 53 - "6. Spielsysteme"
Cohesion: 0.25
Nodes (8): 6.1 Charaktererstellung, 6.2 Werte und Stufen, 6.3 Kampf, 6.4 Säbelfeuer, Tod und Perlen, 6.5 Gegenstände, 6.6 Seefahrt, 6.7 Fortschritt, Neues Spiel+, Cheats, 6. Spielsysteme

### Community 54 - "9. Technik"
Cohesion: 0.25
Nodes (8): 9.1 Aufbau, 9.2 Ablauf eines Bildes, 9.3 Leistung und Laden, 9.4 Spielstände, 9.5 Steuerung, 9.6 Bauen und Veröffentlichen, 9.7 Wichtige technische Regeln, 9. Technik

### Community 55 - "BS"
Cohesion: 0.25
Nodes (8): BS, Attack, Dead, Dormant, Idle, Intro, Stagger, Transform

### Community 56 - "Rarity"
Cohesion: 0.22
Nodes (8): 7. Gegenstände, Läden, Fortschritt, Item, ITEMS, Rarity, Common, Rare, Uncommon, Unique

### Community 57 - "SS"
Cohesion: 0.25
Nodes (8): SS, Dead, Dormant, Fight, Morph, Rise, Ship, Sink

### Community 58 - "AK"
Cohesion: 0.13
Nodes (14): AK, Combo, Cone, Dash, Gust, Lash, Leap, Nova (+6 more)

### Community 59 - "README.md"
Cohesion: 0.14
Nodes (8): Benutzen, Graphify-Abbild des Codes, Stand, Wichtige Knotenpunkte (Einstieg), Bauen, Das Spiel, Steuerung, Throne of the Drowned

### Community 61 - "png.rs"
Cohesion: 0.53
Nodes (4): chunk(), crc32(), encode(), save()

### Community 62 - "Throne of the Drowned – Spielkonzept"
Cohesion: 0.12
Nodes (16): 10. Offene Punkte und Ideen, 1. Überblick, 2. Geschichte und Spielablauf, 3.1 Karte, 3.2 Inseln, 3.3 Glutwacht im Detail, 3.4 Seegebiete, 3. Die Welt (+8 more)

### Community 63 - "7. Grafik"
Cohesion: 0.33
Nodes (6): 7.1 Vorgaben, 7.2 Stil: HD-2D (wie Final Fantasy Resonance), 7.3 Prozedurale Kunst (alles aus Code), 7.4 Zonenstimmung, 7.5 Titelbild, 7. Grafik

### Community 64 - "Body"
Cohesion: 0.25
Nodes (9): 4.1 KI-Ablauf, 4.2 Gegnertabelle (Rohwerte; Tempo, Sicht, Reichweite im Spiel ×2), 4. Gegner (`data.rs::enemies`, KI in `actors.rs::enemy_logic`), Body, Beast, Sheet, en(), enemies() (+1 more)

### Community 65 - "Save"
Cohesion: 0.14
Nodes (8): LookIdx, migrate(), ng_dmg(), pearl_str(), Save, Stain, Stats, Summary

### Community 66 - "Dark Fantasy Pirate RPG – Grafik-Spezifikation"
Cohesion: 0.18
Nodes (11): Bildinhalt neu interpretiert, Dark Fantasy Pirate RPG – Grafik-Spezifikation, Hauptmenü (Bild 4), Hauptpalette (32 Farben, Auszug), HUD & UI, Menü-UI, Motiv-Mapping (Referenz → Piraten), Parallax-Ebenen (hinten → vorne) (+3 more)

### Community 67 - "Hover"
Cohesion: 0.20
Nodes (9): Npc, Hover, Item, None, Npc, Ship, T, Item (+1 more)

### Community 68 - "Cut"
Cohesion: 0.40
Nodes (5): Cut, Door, Travel, Wake, Whirl

### Community 69 - "22. Anhang: Shader im Wortlaut"
Cohesion: 0.50
Nodes (4): 22. Anhang: Shader im Wortlaut, `common.wgsl`, `post.wgsl`, `scene.wgsl`

### Community 70 - "Technische Umsetzung"
Cohesion: 0.29
Nodes (7): Code-Struktur, Ebenen & Sortierung, Partikel (Richtwerte), Reihenfolge für Opus 5.5, Rendering-Pipeline, Shader, Technische Umsetzung

### Community 75 - "Ingame-Szenen"
Cohesion: 0.33
Nodes (6): 5.1 Hub: Bassin des Seelenbaums (Bild 1 / 9), 5.2 Gezeiten-Grotte (Bild 2 / 3), 5.3 Hafenstadt (Bild 5), 5.4 Verfluchter Mangrovensumpf (Bild 6 / 7), 5.5 Blutgezeiten (Bild 8), Ingame-Szenen

## Knowledge Gaps
- **253 isolated node(s):** `schwarzwasser`, `sw-art`, `Player`, `Npc`, `Crew` (+248 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 658 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Canvas` connect `Canvas` to `ships.rs`, `Rgba`, `PropArt`, `main`, `Rig`, `.rows`, `ui.rs`, `Color`, `terrain.rs`, `beasts.rs`, `chars.rs`, `v`, `art.rs`, `Atlas`?**
  _High betweenness centrality (0.177) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Canvas` (e.g. with `Rgba` and `wreck_flame()`) actually correct?**
  _`Canvas` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `schwarzwasser`, `sw-art`, `Player` to the rest of the system?**
  _253 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `World` be split into smaller, more focused modules?**
  _Cohesion score 0.05856299212598425 - nodes in this community are weakly interconnected._
- **Why does `World` connect `World` to `render.rs`, `Game`, `Hover`, `.rows`, `Biome`, `game/mod.rs`, `main.rs`, `Noise`, `WorldView`?**
  _High betweenness centrality (0.133) - this node is a cross-community bridge._
- **Are the 11 inferred relationships involving `World` (e.g. with `.run()` and `.bake_into()`) actually correct?**
  _`World` has 11 INFERRED edges - model-reasoned connections that need verification._
- **Should `PropArt` be split into smaller, more focused modules?**
  _Cohesion score 0.08140929535232384 - nodes in this community are weakly interconnected._