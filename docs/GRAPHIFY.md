# Graphify-Abbild des Codes

Das Verzeichnis `graphify-out/` enthält den vollständigen Code-Graphen von *Throne of the Drowned*, erzeugt mit [Graphify](https://pypi.org/project/graphifyy/) (`graphifyy` 0.9.100, reine AST-Auswertung, ohne KI-Kosten). Er zeigt alle Dateien, Typen, Funktionen und Methoden mit ihren Aufrufen, Importen und Verweisen, gruppiert in Gemeinschaften (Communities).

| Datei | Inhalt |
|---|---|
| `graphify-out/graph.json` | der Graph: Knoten (`id`, `label`, `source_file`, `source_location`, `community`) und Kanten (`calls`, `contains`, `imports_from`, `references`, `method` … mit `confidence`) |
| `graphify-out/graph.html` | interaktive Ansicht im Browser (Datei direkt öffnen) |
| `graphify-out/GRAPH_REPORT.md` | Bericht: Zentrale Knoten („God Nodes“), überraschende Verbindungen, Import-Zyklen, alle Gemeinschaften, Wissenslücken, Fragevorschläge |
| `graphify-out/manifest.json` | Hash je Quelldatei (zeigt, ob der Graph veraltet ist) |

## Stand
- Lokal erzeugt aus diesem Repo, Commit-Stand des Spielcodes `ffda89f` (identisch mit `eXtro181/PirateGame` `main`): rund 2 010 Knoten, 5 610 Kanten, 76 Gemeinschaften (Code und Anleitungen). Die WGSL-Shader (`src/gfx/shaders/*.wgsl`) und TOML-Dateien versteht Graphify nicht; sie stehen im Wortlaut in `docs/GRAFIK_ANLEITUNG.md` (Anhang).
- Zum Vergleich der gehostete Graphify-Index von `eXtro181/PirateGame` (Build `c69d885c`, Commit `ffda89f`, Pipeline `production-v1`): 1 507 Knoten, 4 894 Kanten, 55 Gemeinschaften. Er beschreibt denselben Code mit einem anderen Extraktor; sobald `eXtro181/soulslike` im Graphify-Arbeitsbereich indiziert ist, liefert der MCP-Server (`query_graph`, `graphify_callers`, `graphify_impact` …) dasselbe für dieses Repo.

## Benutzen
```bash
python3 -m venv .venv-graphify && .venv-graphify/bin/pip install graphifyy==0.9.100
```
```bash
.venv-graphify/bin/graphify update .
```
`update` liest alle Codedateien neu und schreibt `graph.json`, `graph.html` und `GRAPH_REPORT.md` (nach jeder größeren Codeänderung ausführen und mit einchecken). Abfragen ohne Browser:
```bash
.venv-graphify/bin/graphify explain "Game"
```
```bash
.venv-graphify/bin/graphify path "tick()" "Frame"
```
```bash
.venv-graphify/bin/graphify query "Wie entsteht ein Boden-Chunk?"
```

## Wichtige Knotenpunkte (Einstieg)
- `src/main.rs` `tick()` / `setup()` – Bildschleife und Start
- `src/game/mod.rs` `Game` – Spielablauf; `src/game/actors.rs` Spieler/Gegner
- `src/gfx/mod.rs` `Frame` – Zeichenlisten (meistverbundener Typ), `src/gfx/render.rs` Renderer
- `src/art.rs` `Art` – Atlanten; `crates/art/src/canvas.rs` `Canvas`, `rig.rs` `Rig`, `terrain.rs`
- `src/world/mod.rs` `World`, `src/world/view.rs` Streaming

Der Ordner `graphify-out/cache/` wird nicht eingecheckt (`.gitignore`), er entsteht beim nächsten `update` neu.
