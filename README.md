# Eclipse Ashes

Ein 2D-Dark-Fantasy-Soulslike / Metroidvania im Browser in **HD-Pixelart**: dynamisches Licht mit Normalmaps, mehrlagige Parallax-Kulisse, Aschepartikel. Geschrieben in **Rust mit Bevy 0.16** und eigenem wgpu-Renderer (WebGL2). Alle Grafiken, Musik und Klänge entstehen beim Start aus Code – es gibt keine Bild- oder Audiodateien.

Hervorgegangen aus *Throne of the Drowned* ([`eXtro181/PirateGame`](https://github.com/eXtro181/PirateGame), Historie in diesem Repo): Die Piraten- und Seemechaniken sind entfernt, das Meer ist zu Pech und Asche geworden, die Schiffe liegen versteinert in den Klippen.

## Das Spiel

- Strikt lineare, vertikale Schläuche mit Engstellen, Schleifen und freischaltbaren Abkürzungen – keine offene Welt.
- Schneller, aggressiver Kampf: Rolle mit bildgenauen Unverwundbarkeits-Frames, Schild und Parade, Haltungsbruch und Hinrichtungen.
- **Aschenkünste** (Kriegsasche) und **Prägungen** an der Waffe, am Ascheschrein bei Corvus tauschbar; Katha steigert die Stufen gegen Asche.
- Spielbar: **Zone 01 – Die Aschenen Klippen von Calamita** mit Aschen-Marodeur, Rost-Skelett und Schlick-Krabbler, dem Miniboss *Zertrümmerter Gallions-Schütze* und **Kapitän Barnaby, dem Aschen-Vogt** (zwei Phasen). Geplant: 15 Zonen mit 15 Hauptbossen.

Konzept, alle Zonen, Bosse, Werte und Baustand: [`docs/ECLIPSE_ASHES.md`](docs/ECLIPSE_ASHES.md). Technische Kurzreferenz: [`CLAUDE.md`](CLAUDE.md).

## Steuerung

| Taste | Aktion |
|---|---|
| A D / Pfeile | Laufen |
| W S | Leitern |
| Leertaste | Springen (S + Leertaste: durch Planken fallen) |
| Umschalt / K | Rolle |
| J · L | leichter · schwerer Hieb (J am gebrochenen Gegner: Hinrichtung) |
| Q halten | Schild, kurz vor dem Treffer: Parade |
| C | Aschenkunst |
| R | Aschentrank |
| E | Rasten, Winde, Aufheben |
| Esc | Pause |

iPhone: Touch-Steuerung (Stick links, Knöpfe rechts).

## Bauen

```bash
cargo build --target wasm32-unknown-unknown --profile fast
```
```bash
PROFILE=fast WASM_BINDGEN=/pfad/zu/wasm-bindgen ./build.sh
```
```bash
cd dist && python3 -m http.server 8790
```

`wasm-bindgen` in der Version aus `Cargo.lock` (0.2.129). Testlauf direkt im Spiel: `?play`, Boss-Arena: `?play&pos=40,11`.
