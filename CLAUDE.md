# CLAUDE.md – Kontext für Eclipse Ashes

Stand 01.11. Kurzreferenz, damit neue Sitzungen sofort weiterarbeiten können. Das Spielkonzept mit allen 15 Zonen, Bossen, NPCs, den genauen Kampfwerten und dem Baustand steht in `docs/ECLIPSE_ASHES.md` – bei inhaltlichen Änderungen dort mitpflegen. Bei Änderungen an Architektur, Systemen oder Entscheidungen hier nachtragen (nicht anhängen, sondern den passenden Abschnitt aktualisieren).

## Projekt und Vorgaben
- **Eclipse Ashes**: 2D-Dark-Fantasy-Soulslike / Metroidvania im Browser, HD-Pixelart in Seitenansicht, **Rust/Bevy 0.16** mit eigenem wgpu-Renderer (WebGL2/wasm). Paket heißt weiter `schwarzwasser` (Wasm-Dateien `schwarzwasser*`).
- Entstanden aus *Throne of the Drowned* (`eXtro181/PirateGame`, Historie in diesem Repo bis `ffda89f`). **Keine Piraten- oder Seemechaniken mehr** (kein Schiff, keine offene Welt, keine Seebosse); Schiffe und Meer nur noch als versteinerte Kulisse und Überlieferung.
- **Keine Bild-/Audiodateien**: Grafik, Musik und Klänge entstehen beim Start aus Code (`crates/art`, `src/synth.rs`). In `public/` liegen nur Icons und das Web-Manifest.
- Repo `eXtro181/soulslike`, **live https://extro181.github.io/soulslike/** (GitHub Pages). `.github/workflows/deploy.yml` baut den Release (mit `wasm-opt`) bei Push auf `main` und auf den Standardzweig `claude/adoring-fermi-02zsb6` (solange es kein `main` gibt) und veröffentlicht ihn, ca. 5 min. Testläufe auch live: `…/soulslike/?play&pos=40,11`.
- **Verbindlich vom Nutzer**: Antworten und Spieltexte auf **Deutsch**. Dark Fantasy (gedämpftes Licht, Lichtinseln, Dunst), strikt lineare vertikale Schläuche mit Abkürzungen, alle Gegner in derselben Grafikqualität und mit denselben Animationen wie der Spieler. Grafikstil: `docs/GRAFIK_SPEZIFIKATION.md`.
- **Skills**: Ponytail liegt in `.claude/skills/` (`ponytail`, `-review`, `-audit`, `-debt`, `-gain`, `-help`; aus `DietrichGebert/ponytail` @ `96608e0`, MIT, nur die Skills ohne Hooks/Statusband). Vorgaben dieser Datei gehen vor: Antworten und Berichte bleiben auf Deutsch. Abkürzungen im Code mit `shortcut: <Grenze>, <wann ausbauen>` markieren.
- **Arbeitsweise**: auf einem Zweig arbeiten, pushen, **erst nach Zustimmung nach `main` mergen**. Nie `cargo fmt` (formatiert das ganze Repo um). Keine PRs ohne Auftrag. Commits enden mit den Attributionszeilen der Sitzung.

## Bauen und testen
- Voraussetzung: `rustup target add wasm32-unknown-unknown`, `wasm-bindgen-cli` 0.2.129 (`cargo install wasm-bindgen-cli --version 0.2.129 --root /tmp/claude-0/wbg`).
- Spiel: `cargo build --target wasm32-unknown-unknown --profile fast`, dann `PROFILE=fast WASM_BINDGEN=/tmp/claude-0/wbg/bin/wasm-bindgen ./build.sh` → `dist/`. Erwartet: genau eine Warnung (`AudioBufferSourceNode::stop` veraltet), sonst warnungsfrei halten.
- Server: in `dist/` `python3 -m http.server 8790` (mit `setsid nohup`; `build.sh` löscht `dist/`, danach den Server neu starten; beim Beenden `pkill -f '^python3 -m http.server'`, sonst trifft `pkill` die eigene Shell). Bildschirmfotos per Playwright + Swiftshader (`require('playwright')`): auf die Konsolenzeile „Welt erzeugt“ warten, dann einige Sekunden, dann `page.screenshot`. Grafik entsteht in ~0,3 s auf einer Atlasseite.
- **Testläufe**: `?play` startet sofort ein neues Spiel, `&pos=x,y` setzt den Spieler auf Kachel (x, y) – z. B. `?play&pos=12,11` (obere Halle mit Miniboss), `?play&pos=40,11` (Boss-Arena, Kampf beginnt sofort), `?play&pos=10,17` (Ebene 1). `&prof` misst die Bildzeiten (`src/prof.rs`).
- **Simulationstests**: `cargo test --bin schwarzwasser` (nativ, erster Lauf ~7 min für Bevy, danach Sekunden). `src/game/tests.rs` spielt Zone 01 ohne Grafik mit virtuellen Tasten (`Input::v_press`/`v_held`) durch: ganzer Weg Start → Nebeltor, Leitern, Abkürzung, Trank, Rolle, Parade und Hinrichtung, Miniboss, Bosssieg bis zum Abschluss-Bildschirm, Tod und Aschefleck, Schrein- und Pause-Menü. Nach Änderungen an Karte oder Kampf ausführen; CI führt sie nicht aus.
- Kunst-Vorschau ohne Spiel: `CARGO_TARGET_DIR=target-art cargo run -q --release -p sw-art --bin preview -- <ordner> <befehl>` (Figuren, Wesen, Schiffe usw. des Vorgängers).
- Code-Graph: `graphify update .` nach größeren Änderungen (siehe `docs/GRAPHIFY.md`).
- **Fallstricke des Renderers**: Texturfelder nie mit einer durch 6 teilbaren Ebenenzahl (WebGL hält sie für Würfel-Felder; `render.rs` rundet auf). Farben werden im Shader linearisiert (hoch 2,2): Kulissenfarben deutlich heller wählen, als sie wirken sollen. Weiche Kreise (`modes::SOFT`) gibt es nur in den Überlagerungen (`f.over`, `f.glow`), nicht in der Weltliste. Gemalte Kulisse mit `flags::GLOW` zeichnen (leuchtet selbst), sonst wird sie wie eine Wand beleuchtet und säuft ab.

## Architektur
### `crates/art` (Bibliothek `sw_art`, prozedurale Kunst)
- Unverändert aus dem Vorgänger: `canvas.rs` (Farbe + Normale + Höhe + Leuchten je Pixel), `atlas.rs`, `color.rs`, `noise.rs`, `chars.rs` (Figuren 64×72 mit allen Animationen: idle, walk, atk, roll, hurt, dead, cast, drink, shoot; Blickrichtung 2 = Seite), `rig.rs`/`beasts.rs` (Kreaturen-Rig), `ships.rs` (Schiffe, für die Wrack-Kulisse), `props.rs`, `structures.rs`, `font.rs`, `ui.rs` u. a.
- **Neu `side.rs`**: Kacheln der Seitenansicht (32 px): `rock(v, top, dark)` Säulenbasalt mit Aschekruste bzw. Rückwand, `plank`, `ladder(chain)`, `winch(wound)`.

### `src/`
- `main.rs`: Bevy-App, Eingabe (Tastatur, Maus, Touch), ruft `Game::update/draw/ui`, Audio. Testlauf-Parameter `?play&pos=`.
- `art.rs`: baut den Atlas beim Start: Schrift, Logo „ECLIPSE ASHES“, Kacheln (`s:rock0..2`, `s:top0..2`, `s:back0..2`, `s:plank0..2`, `s:ladder`, `s:chain`, `s:winch0/1`), Requisiten (`saberfire` als Schrein, `torch`, `skullpile`, `barrels`, `crates`, `cannon`, `anchorDeco`, `swordGrave0..2`, `candles0..2`), verkohlte Galeonen `hw:fire`/`hw:earth` (Kulisse), Figurenbögen (`ashSkeleton`, `skelBrute`, `barnaby` = Ork-Kapitän ×1,5, `gunner` = Musketen-Skelett ×1,4, `crab`, Spieler).
- `gfx/`: Renderer aus dem Vorgänger (G-Puffer, Licht-Pass mit Punktlichtern und Normalen, Bloom, Grading, UI). In der Seitenansicht werden Kippung, Tiefenschärfe und Sonnenschatten in `Game::draw` abgeschaltet. Boden-Chunks, Wasser und Spiegelungen sind noch vorhanden, aber ungenutzt (`#[allow(dead_code)]` in `main.rs`).
- `game/mod.rs`: `Game` (Modi Titel/Spiel/Pause/Steuerung/Schrein), `Save` (JSON in `localStorage` unter `eclipse-ashes`, Felder mit `#[serde(default)]`), Zonen-Aufbau aus der Karte (`reset_zone`), Schrein (Rasten, Katha, Corvus), Tod und Aschefleck, Winde, Nebeltor und Bossbeginn, Kamera (im Bosskampf fest über der Arena), Zeichnen von Dingen und Figuren.
- `game/level.rs`: Karte `MAP` (52 × 40, Legende im Code, Selbstprüfung in `Level::new`), Kollision (`Body`, `step`: Fels, Planken und Leiterköpfe von oben, durchfallen mit `drop`), Kacheln zeichnen, Parallax-Kulisse (`draw_back`: Himmel in 6 Stufen, verfinsterte Sonne als Pixelscheibe, Klippen in drei Tiefen, Galeonen).
- `game/fight.rs`: Kampf mit Frame-Daten (`F` = 1/60 s): Spieler-Zustände `PS`, `LIGHT`/`HEAVY`, Rolle, Schild/Parade, Aschenkünste `ARTS`, Prägungen `AFFS`, Hinrichtung; Gegner `Kind` (Marauder, Grabber, Crawler, Gunner) mit `FoeDef` und KI `FS`; Barnaby (`Boss`, `BS`) mit Ankerschwung, Ankerwurf, Sprungstampfer und Teerbomben in Phase 2; Kanonenkugeln `update_shots`.
- `game/hud.rs`: HUD, Boss-/Miniboss-Leisten, Haltungsbalken, Banner, Menüs (Titel, Pause, Steuerung, Schrein, Abschluss `Mode::End`). Menüs nehmen Bestätigungen erst ab dem zweiten Bild an (`ui_frames`), die Pause öffnet in `ui()` – sonst schließt dieselbe Taste sie im selben Bild wieder.
- `game/env.rs` (Stimmung `ash()`), `fx.rs` (Partikel, Hiebbögen, Ringe; Asche über `ambient(…, 5, …)`), `input.rs` (Aktionen `Act`).
- `ui/text.rs` (Pixelschrift), `ui/touch.rs` (Touch-Stick und Knöpfe), `synth.rs`/`audio.rs` (Musik „glut“, „boss“, „titel“; SFX), `prof.rs`.

## Konventionen
- Weltkoordinaten in Pixeln, 32 px je Kachel, y nach unten; Körper stehen auf ihrem Fuß (`Body.y` = Unterkante). UI in UI-Pixeln (kürzere Bildschirmseite ≈ 360).
- Zeichenreihenfolge über den Sortierschlüssel von `world_q`: Himmel −10, Klippen/Galeonen −9…−8, Rückwand −1, Fels 0, Planken/Leitern 0,1, Deko 0,3–0,5, Gegner 1, Boss 1,5, Spieler 2.
- Neue Zone: eigene Karte mit derselben Legende, Gegner als neue `Kind` mit `FoeDef`, Figur in `art.rs` registrieren; Boss als Zustandsautomat nach dem Muster von Barnaby (siehe Bauvorgaben in `docs/ECLIPSE_ASHES.md`).
- Spielstand: neue Felder immer mit Default; alte Stände müssen laden.

## Altdokumente (Vorgänger)
`docs/SPIELKONZEPT.md` und `docs/MECHANIK_UND_STEUERUNG.md` beschreiben *Throne of the Drowned* (Referenz für Lore und Werte), `docs/GRAFIK_ANLEITUNG.md` den Renderer und die Kunst-Bibliothek (gilt weiter, bis auf Kippung und Draufsicht), `docs/GRAPHIFY.md` den Code-Graphen.

## Offene Punkte
- Zonen 02–15, Bosse 02–15 und ihre Minibosse, NPCs Kapitän Roderick und Stummer Wanderer (spezifiziert in `docs/ECLIPSE_ASHES.md`).
- Eigene Figuren für Spieler mit Schild und für Barnaby mit Anker-Hellebarde (derzeit Figurenbögen des Vorgängers: Entermesser, Ork-Kapitän).
- Auf echter GPU und iPhone feinjustieren (Helligkeit, Bloom, Leistung), Balancing.
