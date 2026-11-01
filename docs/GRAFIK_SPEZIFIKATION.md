# Dark Fantasy Pirate RPG – Grafik-Spezifikation

Oct 24, 2026 · @Dennis

## Zweck & Übergabe-Prompt

Dieses Dokument beschreibt den Grafikstil der Referenz-Screenshots (High-Fidelity-Pixelart, Top-Down-3/4-Sicht, Action-RPG) so präzise, dass Opus 5.5 ihn nachbauen kann — umgebaut auf **Dark Fantasy Piraten**. Bild 4 (Panorama mit Weltenbaum) wird zum **Hauptmenü**.

Übergabe-Prompt (vor das Dokument setzen):

```
Du bist Senior Technical Artist. Setze ausschließlich die GRAFIK um, die diese
Spezifikation beschreibt: Top-Down-3/4-Pixelart (16x16-Tiles, gerendert in 640x360,
ganzzahlig hochskaliert), Setting Dark Fantasy Piraten.

WICHTIG: Keine Spielmechaniken. Kein Spieler-Controller, keine Steuerung, kein Kampf,
keine KI, keine Lebenspunkte-Logik, keine Quests, keine Kollisionen, kein Inventar.
Figuren und Kreaturen werden nur statisch platziert oder spielen Idle-Animationen ab.
HUD-Elemente sind reine Optik mit festen Beispielwerten.
Menüeinträge zeigen nur ihren visuellen Hover-Zustand, sie lösen keine Aktionen aus.

Reihenfolge: Hauptmenü-Panorama (Abschnitt 4), dann Hub-Szene (5.1).
Assets als Platzhalter im beschriebenen Stil, später 1:1 durch echte Sprites ersetzbar.
```

## Stilanalyse der Referenz

Der Stil ist moderne „HD-Pixelart“: klare Pixelraster, aber weiche Farbverläufe über viele Abstufungen, kombiniert mit echten Licht- und Partikeleffekten, die nicht an das Pixelraster gebunden sind.

| Merkmal | Referenz (exakt) | Umsetzung |
| --- | --- | --- |
| Perspektive | Top-Down 3/4 (ca. 60° Neigung), Fronten von Klippen/Häusern sichtbar | Y-Sortierung: Objekte mit größerem Fuß-Y zeichnen über anderen |
| Interne Auflösung | ca. 640×360, Ausgabe 1280×720 (Faktor 2) bzw. 1920×1080 (Faktor 3) | Nearest-Neighbor, Pixel-Snapping der Kamera |
| Tile-Größe | 16×16 px (Gras, Wege, Klippen-Kanten) | Autotiling (47er-Blob oder Dual-Grid) |
| Figuren | ca. 16×32 px, Kopf \~⅓ der Höhe (Chibi-nah) | 4- oder 8-Richtungs-Sprites, 6–8 Frames Laufzyklus |
| Outlines | Keine schwarzen Outlines; Konturen durch dunkleren Farbton desselben Materials (Selective Outline) | Pro Material 1 Konturfarbe, \~35 % dunkler, Farbton Richtung Blau/Violett verschoben |
| Schattierung | 4–6 Stufen pro Material, Licht von oben-links | Hue-Shifting: Lichter wärmer (gelb), Schatten kühler (blau/violett) |
| Schatten | Weiche, halbtransparente Ellipsen unter Figuren/Bäumen (ca. 30–40 % Deckkraft) | Separater Schatten-Layer, Multiply |
| Vegetation | Große Bäume aus „Blob-Clustern“ (Blütenbüschel), hängende Ranken/Glyzinien | Sprites mit Wind-Sway per Vertex-Shader (Sinus, 1–2 px Amplitude) |
| Wasser | Türkis, flächig, mit hellen Glanz-Hexagon-Flecken, durchscheinende Steine am Grund | Shader: Grundfarbe + scrollende Noise-Highlights + Uferkanten-Schaumlinie |
| Klippen | Vertikale Säulenbasalt-Optik, 2–4 Tiles hoch, Gras-Lippe oben | Eigene Klippen-Tiles mit Höhenebenen (Z-Level) |
| Licht/Atmo | Leichter Bloom, Lichtpartikel, Farbgrading pro Biom (z. B. Rotnebel in Bild 8) | Post-Processing: Bloom, Vignette, Color-Grading-LUT pro Zone |
| Menü-Panorama (Bild 4) | Seitenansicht, gröbere Pixel (Faktor \~4), stark gestaffelte Ebenen, Atmosphärenperspektive | Parallax-Ebenen, siehe Abschnitt 4 |

## Umbau auf Dark Fantasy Piraten

Die Struktur der Referenz bleibt (Hub um ein heiliges Zentrum, Wasser, Klippen, Stadt, Dschungel, Korruptionszone), aber Sättigung sinkt um ca. 40 %, Helligkeit um ca. 30 %, und die Akzentfarbe wechselt von Pastell-Rosa zu **geisterhaftem Cyan-Grün** (Seelenlicht) plus **Blutrot**.

### Hauptpalette (32 Farben, Auszug)

| Rolle | Hex | Verwendung |
| --- | --- | --- |
| Tiefsee-Schwarz | #0B1016 | Tiefste Schatten, Himmel-Nacht |
| Sturmblau dunkel | #16232E | Nachthimmel |
| Sturmblau mittel | #24394A | Wolkenschatten |
| Nebelgrau-Blau | #4A6170 | Wolken, Fernebene |
| Nebel hell | #8FA3A8 | Wolkenkanten, Gischt |
| Totholz dunkel | #2A1E1A | Schiffsrümpfe, Stege-Schatten |
| Teerholz | #4A3326 | Planken, Masten |
| Altholz hell | #7A5A3E | Holz-Lichtkanten |
| Moosgrün dunkel | #1E2B22 | Gras-Schatten |
| Sumpfgrün | #34473A | Grasfläche |
| Fahlgrün | #5C7454 | Gras-Licht |
| Basalt dunkel | #1F2226 | Klippen-Schatten |
| Basalt | #3A3F45 | Klippen |
| Knochenweiß | #D8CFB8 | Knochen, Segel, UI-Text |
| Seelenlicht | #5FF2C8 | Magie, Geisterflammen, Kristalle |
| Seelenlicht hell | #B8FFE8 | Glow-Kern, Bloom |
| Laternengold | #E8A443 | Laternen, Gold, HUD-Rahmen |
| Glut | #C2502E | Feuer, Schmiede |
| Blutrot | #7A1A22 | Gesundheit, Korruption, Fahnen |
| Korallen-Violett | #5A2E5C | Tote Korallen, Fluchbäume |

**Wasserpalette (dunkelblau, nie schwarz):** Tiefe #0E2140 · Mittel #173763 · Fläche #22508A · Glanz #4F86C0 · Schaum/Gischt #A9C8E6. Seelenlicht-Reflexe (#5FF2C8) liegen additiv darüber.

### Motiv-Mapping (Referenz → Piraten)

| Referenz | Dark-Fantasy-Piraten |
| --- | --- |
| Rosa Weltenbaum mit Glyzinien | **Ertrunkener Seelenbaum**: schwarzer, knorriger Baum aus versteinertem Treibholz, statt Blüten hängende Netze, Algen und Ketten mit Seelenlicht-Laternen; Krone als blasser, cyan glimmender Nebel |
| Steinbecken mit blauen Kristallen | Bassin aus bemoostem Basalt und Schiffsplanken, Kristalle = cyan glühende Seelen-Steine, Wasser dunkelblau (#173763) mit cyan Leuchtpartikeln |
| Dorf mit Schmiede, Bänken | Piratenlager: Schmiede mit Kanonenrohren, Rum-Fässer, Taverne aus Schiffsrumpf, Galgen, Seekarten-Tisch |
| Bunte Bäume (Lila, Blau, Rot) | Tote Mangroven, Fluchweiden (Korallen-Violett), Seetang-Palmen (Petrol), Glutpalmen (Rost-Rot) |
| Wasserfall-Klippen | Basaltkliffe mit Wrackteilen; Wasserfall fällt in eine Gezeiten-Grotte, Gischt cyan beleuchtet |
| Teal-Dächer der Stadt | Hafenstadt mit dunklen Schieferdächern (#2B3A40), Fachwerk aus Teerholz, Laternen in Fenstern, Taue und Netze zwischen Häusern |
| Dschungel mit Pflanzen-Gegnern | Verfluchter Mangrovensumpf, Gegner: Krabbenkrieger, Seetang-Ertrunkene, Muschel-Wesen |
| Rote Korruptionszone | **Blutgezeiten**: rot-violetter Nebel, Korallenwucherungen, Knochen, glühende Seeigel-Laternen |

## Hauptmenü (Bild 4)

Das Menü ist ein animiertes Pixel-Panorama in 480×270 px, 4-fach skaliert auf 1920×1080. Komposition wie Bild 4: Horizont auf ca. 62 % Bildhöhe, großes Wolkenband oben, Berge links, Seelenbaum auf Felsinsel rechts der Mitte (Goldener Schnitt, x ≈ 63 %), Festung links unten, Meer rechts.

### Bildinhalt neu interpretiert

- **Himmel:** Gewitternacht statt Mittagsblau. Verlauf oben #0B1016 → Horizont #24394A, 6 harte Farbstufen (keine weichen Verläufe, Banding ist gewollt).
- **Wolken:** Gleiche Form wie Referenz (große, zerfranste Bänder mit weißen Kanten), aber Fläche #24394A, Kante #4A6170, Glanzkante #8FA3A8. Gelegentlich Blitz hinter Wolken (Wolkenkanten blitzen 2 Frames #B8FFE8 auf).
- **Mond:** Blasser Vollmond hinter dem Wolkenband, x ≈ 22 %, y ≈ 18 %, Halo als 3 konzentrische Ringe mit 20/10/5 % Deckkraft.
- **Berge links:** Zackige Basaltküste statt runder Hügel, Farben #16232E/#24394A, Atmosphärennebel davor.
- **Seelenbaum (Hauptmotiv):** Felsinsel wie Referenz (Pyramidenform), aber aus Basalt mit Wrackteilen und einer Treppe aus Schiffsrümpfen. Baum: schwarzer Stamm, Krone als blasser Nebelball (#8FA3A8 mit cyan Glow). Die goldenen Ringe der Referenz werden zu **3 rotierenden Seelenlicht-Runenringen** (#5FF2C8), die sich langsam drehen (Ellipsen, 1 Umdrehung / 12 s).
- **Festung links unten:** Die Ringstadt wird zur Piratenfestung „Schwarzwasser“ mit Hafenmauer, Leuchtturm (Lichtkegel schwenkt, 8 s Periode) und vereinzelten Fenster-Lichtern (#E8A443, flackernd).
- **Flüsse:** Dunkelblaues, spiegelndes Wasser (#173763, Glanz #4F86C0) mit cyan Lichtpunkten.
- **Meer rechts:** Dunkelblaues Meer (Horizont #22508A, nah #0E2140) mit bewegten Wellenlinien, ein **Geisterschiff** mit zerrissenen Segeln (Knochenweiß, 60 % Deckkraft) treibt langsam von rechts nach links (Durchlauf 90 s).
- **Vordergrund:** Dunkle Silhouetten von Mangroven und ein Steg unten rechts mit Hütten, fast schwarz (#0B1016), rahmen das Bild.

### Parallax-Ebenen (hinten → vorne)

| # | Ebene | Scroll-Faktor | Animation |
| --- | --- | --- | --- |
| 0 | Himmelsverlauf | 0.00 | Statisch, Blitz-Overlay |
| 1 | Mond + Sterne | 0.02 | Sterne funkeln (zufällig 1 px an/aus) |
| 2 | Ferne Wolken | 0.05 | Driften 2 px/s nach links |
| 3 | Nahe Wolken | 0.10 | Driften 5 px/s, Endlos-Wrap |
| 4 | Berge + Meer-Horizont | 0.15 | Geisterschiff |
| 5 | Seelenbaum-Insel | 0.25 | Runenringe, Partikel, Glow-Puls 4 s |
| 6 | Ebene, Festung, Flüsse | 0.40 | Leuchtturm, Fensterlichter, Wasserglitzern |
| 7 | Nebelband | 0.50 | Sinus-Verzerrung, Alpha 30–50 % |
| 8 | Vordergrund-Silhouetten | 0.80 | Leichtes Wiegen |
| 9 | Regen/Asche-Partikel + Vignette | 1.00 | Diagonaler Regen, 120 Partikel |

Der Parallax-Effekt entsteht allein durch eine langsame automatische Kamerafahrt (Sinus, 30 s Periode, max. ±12 px Versatz bei Ebene 8).

### Menü-UI

- **Logo** oben links (x 6 %, y 10 %): Spieltitel in Pixel-Serifenschrift, Knochenweiß mit 1 px Blutrot-Schlagschatten, darüber ein gekreuzter Säbel-und-Anker-Emblem.
- **Menüeinträge** links, vertikal, Abstand 14 px intern: Neues Spiel · Fortsetzen · Optionen · Credits · Beenden.
- **Auswahl-Zustand:** Eintrag wird Laternengold, links erscheint ein animierter Kompasspfeil (4 Frames), Text schiebt 4 px nach rechts (Ease-out, 120 ms). Inaktiv: Knochenweiß 70 %.
- **Hintergrund der Einträge:** Halbtransparentes Pergament-Band (#0B1016 bei 55 %) mit zerfranstem Rand.
- **Unten rechts:** Versionsnummer, 50 % Deckkraft. Unten Mitte: „Drücke eine Taste“ pulsierend (Alpha 40–100 %, 1.5 s) beim ersten Start.
- **Audio-Hinweis:** Wellenrauschen, Knarren von Holz, entferntes Donnergrollen synchron zum Blitz.

## Ingame-Szenen

Jede Szene übernimmt das Layout ihrer Referenz und tauscht Materialien, Licht und Requisiten.

### 5.1 Hub: Bassin des Seelenbaums (Bild 1 / 9)

- **Layout:** Zentrales Bassin (ca. 34×22 Tiles) in Kleeblattform, Baum mittig oben, Krone überragt den Bildschirm oben (bewusst angeschnitten). Unten mittig eine halbrunde Steinplattform, auf der eine Figur als Bildmotiv steht.
- **Ring aus Wegen:** Erdige Pfade (#4A3326 / #2A1E1A, nass glänzend) umrunden das Bassin; Wiese dazwischen in Sumpfgrün mit vereinzelten weißen Blüten → ersetzt durch **blasse Knochenpilze** (2×2 px, Knochenweiß).
- **Bassin-Rand:** Weiße Steinbögen werden zu verwitterten Basaltbögen mit eingelassenen Seelen-Steinen (cyan, 6×8 px, pulsierender Glow, 3 s). Drei Wächter-Statuen (links, rechts, Mitte vorn) als kapuzentragende Ertrunkene mit Laterne.
- **Bambuszaun:** wird zu Zaun aus Rudern und Treibholzpfählen mit Tauwerk.
- **Links oben:** Schmiede mit Glutofen (animiert, 6 Frames, Funkenpartikel), Amboss, Kanonenrohre im Regal.
- **Rechts oben:** Taverne/Lagerplatz mit Tischen aus Fässern, Piraten-Figuren (über einer schwebt ein Laternengold-Symbol, rein dekorativ), Fahne in Blutrot mit Totenkopf (Wind-Animation 4 Frames).
- **Bäume am Rand:** Fluchweiden (Korallen-Violett) und Seetang-Palmen (Petrol #1F4A4C) statt bunter Laubbäume.
- **Atmosphäre:** Leichter Bodennebel, Glühwürmchen (cyan, 40 Partikel), Color-Grading kühl.

### 5.2 Gezeiten-Grotte (Bild 2 / 3)

- Basalt-Säulenklippen 3–4 Tiles hoch mit Gras-Lippe oben. Wasserfall: 3 Tiles breit, animierte vertikale Streifen (scrollende Textur, 60 px/s), unten Gischtwolke mit cyan Lichtpartikeln.
- Wasser in Stufen: Pool oben (dunkelblau #173763), Absätze mit Schaumlinien, Trittsteine als Wrackbalken.
- Requisiten: Gesunkene Galionsfigur halb im Wasser (statt Steinstatue), Truhen, Kisten mit Seepocken.

### 5.3 Hafenstadt (Bild 5)

- Fachwerkhäuser 2–3 Stockwerke, Dächer aus schwarzem Schiefer in Schuppenmuster (wie die Teal-Dachziegel der Referenz, 4×3 px pro Schuppe), Rundbogenfenster mit warmem Licht und Blumenkästen → **Kästen mit Kräutern und Fischgräten**.
- Kopfsteinpflaster nass, reflektierende Pfützen. NPCs: Matrosen, Wachen mit Dreispitz, Händler.

### 5.4 Verfluchter Mangrovensumpf (Bild 6 / 7)

- Dichtes Blätterdach als Vordergrund-Overlay (Ebene über allen Figuren, 70 % Deckkraft).
- Kreaturen-Sprites mit dekorativem Balken über dem Kopf (16×2 px, blau → hier **blutrot**). Kreaturen-Designs (nur Aussehen, Idle-Animation): Krabbenkrieger, Ertrunkene mit Seetang.
- Flussüberquerung über eingestürzte Steinbrücke → zerbrochener Holzsteg mit Seilen.

### 5.5 Blutgezeiten (Bild 8)

- Basalt-Hexagonboden mit Blutrot-Adern, Nebel rot-violett (Overlay #7A1A22, 35 %, Multiply + additive Glow-Punkte).
- Glühende Laternenpilze (#E8A443 → #C2502E), Fluchbäume mit violetten Korallenbüscheln.
- Abgrund an Klippenkanten: nach unten in Nebel ausfadender Verlauf. Seelenblume (rechts) als Deko-Objekt: weiß-cyan, pulsierend.

## HUD & UI

Das HUD der Referenz bleibt in Position und Größe, nur Materialien wechseln: aus grünem Gras-Rahmen wird **gealtertes Messing mit Tauwerk**.

Alle HUD-Elemente sind reine Optik mit festen Beispielwerten (z. B. Leben 70 %, Zähler 3); keine Logik dahinter.

| Element | Position (bei 640×360) | Referenz | Piraten-Version |
| --- | --- | --- | --- |
| Lebensbalken | x 8, y 6, 140×8 px | Grün, dunkler Rahmen, geschwungener Haken links | Blutrot #7A1A22 → Glanzlinie #C2502E, Rahmen Messing #E8A443/#7A5A3E, links Haken als Ankerform |
| Ausdauer-Pips | unter dem Balken, 4 Segmente à 8×3 px | Gelbe Striche | Laternengold, leere Segmente #2A1E1A |
| Waffen-Slots | 2 Kreise à 22 px, überlappend unter dem Balken | Grüne Kreise mit Schwert/Bogen | Messing-Bullaugen mit Säbel und Steinschlosspistole, aktiver Slot cyan Glow |
| Verbrauchsgut | unten links, Kreis 24 px | Orangene Frucht mit Zähler | Rumflasche oder Seelenlicht-Phiole, Zähler in Knochenweiß, 1 px Schatten |
| Schnellslots | über dem Verbrauchsgut, 2 Kreise 16 px | Trank + Pfeile mit Zahl | Bomben + Kugeln |
| Gegner-HP | über Gegnerkopf, 16×2 px | Blau | Blutrot, fester Beispielwert |
| Interaktion | Sprechblase über Objekt | Blaue Blase mit „…“ | Pergament-Blase, Laternengold-Rand |

Schrift: Pixel-Font mit 5–7 px Versalhöhe, leichte Serifen. Alle UI-Elemente auf ganzzahligen Pixelkoordinaten, keine Skalierung außer dem globalen Faktor.

## Technische Umsetzung

Engine-neutral formuliert; passt für Godot 4, Unity (URP 2D) oder Web (PixiJS/PlayCanvas). Die Engine ist noch offen — bitte vor dem Start festlegen.

### Rendering-Pipeline

1. Szene in ein Low-Res-Render-Target (640×360, Menü 480×270) zeichnen, Filter: Nearest.
2. Kamera auf ganze Pixel snappen; Sub-Pixel-Bewegung nur für Partikel und Licht.
3. Licht-Pass: 2D-Punktlichter (Laternen, Seelensteine, Glut) additiv in eigenen Buffer, Ambient pro Zone (Hub #4A6170 bei 60 %).
4. Post-Processing in Ausgabeauflösung: Bloom (Threshold 0.75, nur Seelenlicht/Feuer), Color-Grading-LUT pro Zone, Vignette 25 %, leichtes Film-Grain 3 %.
5. Ganzzahliges Hochskalieren in das Fenster, Letterboxing bei krummen Faktoren.

### Ebenen & Sortierung

| Layer | Inhalt | Sortierung |
| --- | --- | --- |
| Ground | Wasser, Gras, Wege (Tilemap) | keine |
| Ground-Deko | Blumen, Pilze, Steine | keine |
| Shadows | Ellipsen-Schatten | Multiply |
| Entities | Figuren, Kreaturen, Bäume, Häuser | Y-Sort nach Fußpunkt |
| Overhead | Baumkronen, Dächer, Blätterdach | über Entities, fade bei Überlappung |
| FX | Partikel, Lichter, Nebel | additiv / alpha |
| UI | HUD | Bildschirmraum |

### Shader

- **Wasser:** Grundfarbe #22508A, zur Mitte hin #0E2140 + zwei gegenläufig scrollende Voronoi-Noises für die hexagonalen Glanzflecken (Schwelle 0.85 → #4F86C0, quantisiert auf Pixelraster); Uferschaum über Distance-Field der Tilemap-Kante; Grund durchscheinend über Alpha 60 %.
- **Wind-Sway:** Vertex-Offset x = sin(t·1.5 + worldX·0.1) · 1.5 px · (1 − uv.y), danach auf Pixel runden.
- **Glow-Puls:** Emission = base · (0.7 + 0.3 · sin(t · 2π / Periode)).
- **Nebel:** Zwei Layer Perlin-Noise, langsam scrollend, Alpha quantisiert in 4 Stufen (Pixel-Look).
- **Rim-Light: 1 px Randlicht auf Figuren in der Lichtfarbe naher Laternen oder Seelensteine**.

### Partikel (Richtwerte)

| System | Anzahl | Farbe | Verhalten |
| --- | --- | --- | --- |
| Glühwürmchen/Seelenfunken | 40 | #5FF2C8 | Langsames Wandern, Alpha-Puls |
| Schmiedefunken | 8/s | #E8A443 → #C2502E | Aufwärts, Schwerkraft, 0.6 s Lebensdauer |
| Regen (Menü) | 120 | #8FA3A8, 40 % | Diagonal 20°, 1×4 px |
| Blutnebel-Asche | 60 | #C2502E | Aufsteigend, flackernd |
| Gischt Wasserfall | 25 | #B8FFE8 | Radial nach oben, 0.8 s |

### Code-Struktur

```
/src
  /core        App.ts, SceneManager.ts, Camera.ts (Pixel-Snap, automatische Fahrt)
  /render      LowResTarget.ts, LightPass.ts, PostFX.ts, Palette.ts
  /scenes      MainMenu.ts, HubSoulTree.ts, TideGrotto.ts, HarborTown.ts,
               MangroveSwamp.ts, BloodTides.ts   (reine Schaubilder)
  /menu        ParallaxLayer.ts, MenuVisual.ts, RuneRings.ts, GhostShip.ts
  /world       Tilemap.ts, Autotile.ts, YSort.ts, Overhead.ts
  /props       StaticSprite.ts, IdleAnimation.ts (Figuren, Kreaturen, Deko)
  /fx          Particles.ts, Water.shader, Fog.shader, Sway.shader
  /ui          HudVisual.ts, PixelFont.ts
/assets
  /tiles /sprites /menu /ui /palettes (palette.hex, 32 Farben)
```

### Reihenfolge für Opus 5.5

1. Render-Pipeline + Palette + Pixel-Kamera.
2. Hauptmenü mit allen 10 Parallax-Ebenen (Platzhalter aus Formen in Palettenfarben).
3. Hub-Szene mit Tilemap, Y-Sort, Wasser-Shader, statisch platzierten Figuren.
4. HUD.
5. Weitere Szenen und Kreaturen-Sprites mit Idle-Animation.

Offene Frage: Welche Engine und welches Zielformat (PC, Web, Mobile)?
