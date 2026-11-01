# Eclipse Ashes – Spielkonzept und Baustand

2D-Dark-Fantasy-Soulslike / Metroidvania in HD-Pixelart. Rust/Bevy 0.16 mit eigenem wgpu-Renderer (WebGL2), alles prozedural aus Code wie beim Vorgänger. Dieses Dokument ist die verbindliche Spezifikation; `CLAUDE.md` ist die technische Kurzreferenz.

| | |
|---|---|
| Genre | 2D Dark Fantasy Soulslike / Metroidvania, strikt lineare vertikale Schläuche, keine offene Welt |
| Kampf | schnell und aggressiv: Rolle mit bildgenauen I-Frames, Schild und Parade, Haltungsbruch, Hinrichtungen |
| Waffenkünste | **Aschenkünste** (Kriegsasche) und **Prägungen** an der Waffe, am Schrein bei Corvus tauschbar |
| Grafik | 2D-HD-Pixelart, dynamisches Licht mit Normalmaps, mehrlagige Parallax-Kulisse, Aschepartikel |
| Vorgänger | *Throne of the Drowned*: alle Piraten- und Seemechaniken entfernt. Schiffe sind trockene, versteinerte Wracks im Inland, die Meere sind zu Pech und Asche geworden. Das Meer lebt nur noch als Kulisse und Überlieferung. |

## 1. Baustand (02.11.)

| Bereich | Stand |
|---|---|
| Zone 01 „Die Aschenen Klippen von Calamita“ | **spielbar**: Karte, drei Gegnertypen, Miniboss, Boss mit zwei Phasen, Winden-Abkürzung, Schrein, zwei Kriegsaschen |
| Kampfsystem | **fertig**: Hiebe mit Frame-Daten, Kombos, Rolle, Schild/Parade, Haltung, Hinrichtung, Aschenkünste, Prägungen, Trank |
| Schrein, Tod, Asche | **fertig**: Rasten (Gegner kehren zurück), Katha (Stufen), Corvus (Kunst/Prägung), Aschefleck beim Tod |
| Spielstand | **fertig**: JSON in `localStorage` (`eclipse-ashes`), Fortsetzen im Titel |
| Abschluss | **fertig**: nach Barnaby „Ende des ersten Kapitels“ mit Spielzeit, Stufe, Toden; weiterspielen oder zum Titel |
| Tests | 13 Simulationstests (`src/game/tests.rs`, `cargo test --bin schwarzwasser`): ganzer Weg Start → Nebeltor, Leitern, Abkürzung, Parade und Hinrichtung, Miniboss, Bosssieg bis Abschluss, Tod und Aschefleck, Menüs |
| Zonen 02–15, Bosse 02–15, Minibosse | **spezifiziert** (Abschnitt 5), noch nicht gebaut |
| Kapitän Roderick, Stummer Wanderer | spezifiziert, noch nicht gebaut |

## 2. Steuerung

| Taste | Aktion |
|---|---|
| A D / ←→ | Laufen |
| W S | Leitern klettern |
| Leertaste | Springen (kurz drücken = niedriger Sprung); S + Leertaste fällt durch Planken |
| Umschalt / K | Rolle |
| J | leichter Hieb, in Reichweite eines gebrochenen Gegners: Hinrichtung |
| L | schwerer Hieb |
| Q halten | Schild; höchstens 10 Bilder vor dem Treffer gehoben = Parade |
| C | Aschenkunst |
| R | Aschentrank |
| E | Rasten, Winde drehen, Aufheben |
| Esc | Pause (Weiter, Steuerung, Zum Titel) |

Touch: Stick links (frei setzbar), rechts Hieb, Sprung, Rolle, Schwer, Block, Kunst, E, Trank; oben rechts Menü (`src/ui/touch.rs`).

## 3. Kampfsystem (Werte aus `src/game/fight.rs`)

Frame-Daten bei 60 fps (Bild = 1/60 s). Schaden = Grundschaden 22 · (1 + 0,08 · Kraft) · Faktor.

| Aktion | Ausholen | aktiv | Erholung | Reichweite | Schaden | Haltung | Ausdauer |
|---|---|---|---|---|---|---|---|
| Leichter Hieb | 8 | 4 | 14 | 50 px | ×1,0 (+15 % je Kombostufe, bis 3) | ×0,8 | 15 |
| Schwerer Hieb | 18 | 5 | 22 | 58 px | ×1,9 | ×2,2 | 28 |
| Rolle | – | 24 Bilder, unverwundbar Bild 3–17 | – | 270 px/s abnehmend | – | – | 20 |
| Hinrichtung | Stich nach 27 Bildern | – | 60 Bilder gesamt | 40 px + halbe Breite | ×5 | setzt Haltung zurück | – |

- Erholung eines Hiebs lässt sich mit der Rolle abbrechen; während der Erholung gedrückte Hiebe werden vorgemerkt (Kombo).
- **Schild**: Bewegung 60 px/s, Treffer von vorn kosten 15 % Schaden und 1,6 × Schaden Ausdauer; Ausdauer leer = Deckung gebrochen (0,9 s betäubt).
- **Parade**: Schild höchstens 10 Bilder vor einem parierbaren Treffer gehoben → Gegner gebrochen (1,4 s), Boss +30 % Haltung und 1 s taumelnd. Nicht parierbar: Ankerschwung des Rost-Skeletts, Ankerwurf, Sprungstampfer, Kanonenkugeln, Teer.
- **Haltung**: jeder Treffer füllt sie; nach 1,5 s ohne Treffer sinkt sie (25/s, Boss 20/s nach 2 s). Voll = gebrochen (2,2 s, Boss 2,5 s): Hinrichtung möglich.
- **Treffer-Stopp**: 0,04 s bei Treffern, 0,12 s bei Paraden, 0,18 s bei Hinrichtungen (Spielzeit ×0,15).
- **Spieler**: Leben 90 + 12 · Vitalität, Ausdauer 90 + 8 · Ausdauer (Erholung 55/s nach 0,6 s, mit Schild halb), Glut (FP) 50. Getroffen: 0,4 s unverwundbar, 0,3 s taumeln. Aschentrank: 3 Ladungen, heilt 45 % über die Zeit, 0,9 s.
- **Bewegung**: 150 px/s, Sprung 440 px/s (Schwerkraft 1500), Koyotenzeit 0,1 s, Sprungpuffer 0,12 s, Leitern 120 px/s.

### Aschenkünste (Kriegsasche)

| Kunst | Glut | Ausholen/aktiv/Erholung | Wirkung |
|---|---|---|---|
| Aschenwirbel | 18 | 12/20/18 | Drehhieb, trifft vorn und hinten (2 × 0,7), Haltung ×1,5 |
| Glutstoß | 14 | 6/14/16 | Vorstoß 420 px/s, unverwundbar bis zum Ende des Stoßes, ×1,6 |
| Sturmschild | 12 | 4/36/10 | Konterhaltung: der nächste Treffer wird mit ×2,2 Schaden und ×3 Haltung zurückgeschlagen |

### Prägungen
Gewöhnlich · **Schwer** (+35 % Haltung, +5 % Schaden, +10 % Ausdauerkosten) · **Glut** (Brand: 3 Schaden je ½ s, 3 s) · **Asche** (−5 % Schaden, jeder Treffer gibt 3 Glut zurück).

## 4. Zone 01: Die Aschenen Klippen von Calamita (gebaut)

Vertikaler Schlauch entlang versteinerter Wracks. Karte 52 × 40 Kacheln à 32 px (`src/game/level.rs`, `MAP`), Legende im Code. Ablauf:

1. **Schreinraum** (unten links): Start, Ascheschrein. Rechts eine Leiter nach oben.
2. **Vier Ebenen** nach oben, jeweils über eine Leiter auf der Gegenseite verbunden (Schlauch im Zickzack), mit Planken zum Durchspringen. Auf Ebene 3 liegt die Kriegsasche **Glutstoß**, am Ende von Ebene 2 **Sturmschild**.
3. **Oberste Halle** unter freiem Himmel (die Galeone in der Kulisse): Miniboss **Zertrümmerter Gallions-Schütze**, daneben die **Winde**. Gedreht, rasselt die Kettenleiter hinab bis zum Schrein (Abkürzung, wird gespeichert). Ohne Winde ist der Schacht nur ein Sturz nach unten.
4. **Tunnel** mit Schlick-Krabbler an der Decke → **Nebeltor** → Arena unter freiem Himmel: **Kapitän Barnaby, der Aschen-Vogt**. Das Tor schließt sich hinter dem Spieler, die Kamera steht fest über der Arena.

| Gegner | Leben | Haltung | Schaden | Tempo | Reichweite | Ausholen | Asche | Verhalten |
|---|---|---|---|---|---|---|---|---|
| Aschen-Marodeur | 70 | 60 | 14 | 125 | 40 | 0,32 s | 40 | Draufgänger: Ausfall 260 px/s, 2er-Kombo; steht der Spieler nah an einer Wand, bedrängt er ihn (3er-Kombo, 30 % schneller, kürzeres Ausholen) |
| Rost-Skelett | 220 | 160 | 30 | 48 | 118 | 0,8 s | 140 | schwerer Ankerträger: Ankerschwung nach beiden Seiten, nicht parierbar, unterbrechungsfrei (Superrüstung) |
| Schlick-Krabbler | 50 | 40 | 12 | 90 | 34 | 0,28 s | 30 | hängt kopfüber an der Decke, fällt auf den Spieler herab |
| Zertrümmerter Gallions-Schütze (Miniboss) | 420 | 200 | 24 | 55 | 340 | 0,7 s | 600 | feuert Kanonenkugeln im Bogen, weicht unter 110 px zurück und tritt zu; einmalig |

**Kapitän Barnaby, der Aschen-Vogt** (1000 Leben, Haltung 320, 2000 Asche)
- Phase 1, Anker-Hellebarde: **Ankerschwung** (0,75 s Ausholen, vorn 170/hinten 60 px, 32, parierbar) · **Ankerwurf** (0,6 s, Anker an der Kette 380 px weit und zurück, 26) · **Sprungstampfer** (Ducken 0,5 s, Sprung auf den Spieler 0,6 s, Aufschlag 120 px, 36; wer in der Luft ist, entgeht ihm). Nah: 60 % Schwung, 40 % Stampfer; fern: je 50 % Wurf/Stampfer.
- Phase 2 ab 50 %, **Teer und Glut**: kürzeres Ausholen und Pausen, alle 2,4 s zwei Teerbomben (eine auf den Spieler, eine zufällig), Marke 1 s vorher, Einschlag 44 px · 28; danach 4 s brennender Teer (8 je ½ s). Schwung und Stampfer hinterlassen ebenfalls brennenden Teer.

### Überlieferung (Gegenstände und Orte)
- **Aschentrank**: „Ein Schluck aus dem Krug der Strandwache. Schmeckt nach Rauch und nach einem Meer, an das sich niemand mehr erinnert.“
- **Kriegsasche: Glutstoß**: „Die letzte Glut aus dem Kessel der *Calamita*. Wer sie führt, stürzt voran wie das Schiff, das in die Klippen fuhr, als das Wasser ging.“
- **Kriegsasche: Sturmschild**: „Ein Splitter des Sturmglases, das einst den Throne der Ertrunkenen vor den Wogen warnte. Es warnt noch immer – nur kommen keine Wogen mehr.“
- **Ascheschrein**: „Ein Säbel im Glutbecken. Die Seefahrer schworen hier auf den Thron; jetzt wärmen sich die Verlorenen an dem, was von ihren Schwüren übrig ist.“
- **Kapitän Barnaby**: „Vogt der Klippen, Hüter einer Galeone, die nie wieder auslaufen wird. Er zählt noch immer die Gezeiten – in Asche, die durch seine Finger rinnt.“

## 5. Weltaufbau: 15 Zonen und 15 Bosse

Jede Zone ist ein eigener vertikaler Schlauch mit Schrein, mindestens einer Abkürzung und einem Nebeltor zum Boss. Zonen sind über ihre Ausgänge linear verbunden. Neue Zonen werden als weitere Kachelkarte (`MAP`) mit eigener Legende für Gegner und Boss angelegt.

| # | Zone | Aufbau | Feinde | Miniboss | Hauptboss |
|---|---|---|---|---|---|
| 01 | Die Aschenen Klippen von Calamita | vertikaler Schlauch an versteinerten Wracks | Aschen-Marodeur, Rost-Skelett, Schlick-Krabbler | Zertrümmerter Gallions-Schütze | **Kapitän Barnaby, der Aschen-Vogt** (P1 Anker-Hellebarde, P2 Teer-Explosionen) |
| 02 | Pfad der Verdörrten Pilger | enge Bergpässe, einstürzende Hängebrücken | Buß-Bettler, Krähen-Anbeter, Stein-Speerwerfer | Klagende Mutter | **Der Verfallene Hohepriester Malphas** (P1 Zauber, P2 Asche-Flügel-Bestie) |
| 03 | Versunkene Katakomben von Ostraka | Labyrinth unter dem einstigen Meeresspiegel | Teer-Geister, Knochen-Krabben, Bleich-Wurm | Der Ertrunkenen-Henker | **Tiefen-Schrecken Vaelis** (P1 Tentakelschläge, P2 Flut aus schwarzem Pech) |
| 04 | Festung der Verrosteten Kette | steile Innenhöfe, Aufzüge, eisernes Gitternetz | Eisen-Garde, Ketten-Hunde, Armbrust-Befehlshaber | Panzer-Ritter von Ostraka | **Kommandant Vlaus & Das Eiserne Gebet** (Duo: Schildritter und Großschwert-Krieger) |
| 05 | Tal der Versteinerten Flotte | Schlucht, gestrandete Kriegsschiffe als Brücken und Tunnel | Mast-Späher, Verweste Segler, Pulver-Ghule | Kanonen-Scheusal | **Admiral Dagon, Das Entstellte Steuerrad** (Bestie aus Holz und Eisen) |
| 06 | Die Aschene Stadt Loth-Veritas | gotische Dächer, Gassen, bröckelnde Kathedralen | Inquisitions-Horde, Flammen-Gelehrte, Gargoyles | Schatten-Meuchelmörder | **Bischof Aurelius der Glühende** (P1 Heilige Flamme, P2 Glut-Aura) |
| 07 | Unterirdischer Teer-Sumpf | Schlamm verlangsamt, Giftdämpfe | Sumpf-Laucher, Gift-Egel, Moos-Skelette | Schleim-Verschlinger | **Die Fäulnis-Mutter Hydra** (mehrköpfiger Geschosskampf) |
| 08 | Gießerei der Verlassenen Kronen | Erzschmelze, flüssiges Metall als Falle | Erz-Golems, Schmiede-Untote, Glut-Käfer | Der Gussmeister | **Der Geschmolzene Monolith** (P1 langsame Keule, P2 Magma-Wellen) |
| 09 | Der Schreiende Wald aus Holz-Relikten | Baum-Schläuche aus versteinertem Schiffsholz | Wurzeltrolle, Harz-Hexen, Schatten-Wölfe | Uralter Treant-Stumpf | **Königin Silvia der Bleichen Geister** (Tanzkampf mit zwei Klingen) |
| 10 | Gefängnis der Ertrunkenen Seelen | gestapelte Zellentrakte, Falltüren | Wärter mit Peitschen, Kettensägner, Wahnsinnige Insassen | Folterknecht Ignaz | **Der Kettensammler** (zieht die Arenawände zusammen) |
| 11 | Frost-Klippen des Toten Meeres | rutschige Eishöhlen, gefrorene Wracks | Eis-Zombies, Frost-Harpyien, Gletscherspringer | Polar-Bestie | **Prinz Joram der Gefrorene Erbe** (harter, schneller Schwertkampf) |
| 12 | Der Aschen-Schlund (Abstieg) | reiner Schacht nach unten, Sprungpassagen | Flug-Asche-Dämonen, Sturz-Skelette, Wand-Würmer | Abgrund-Wächter | **Der Aschen-Ur-Dämon** (riesiger Hintergrund-Boss mit Armschlägen) |
| 13 | Kathedrale der Schwarzen Sonne | Hallen mit Buntglas, Spiegelrätsel | Sonnen-Fanatiker, Licht-Ritter, Hohe Priesterinnen | Paladin des Erloschenen Lichtes | **Erz-Inquisitor Lucian** (P1 Rapier-Paraden, P2 Teleport-Schattenklone) |
| 14 | Die Versteinerten Docks des Throns | Vorkammer, Docks ohne Wasser, skelettierte Flotten | Elite-Wachen des Throns, Letzte Offiziere | Champion der Ertrunkenen | **Der Schatten von „Throne of the Drowned“** (geisterhafter Admiral, alte Wassermagie als Asche) |
| 15 | Der Thron der Verbliebenen Asche | ein verfallener Thronsaal über dem Aschen-Ozean, nur Arena | – | – | **Gott-Kaiser Alden, Der Erloschene Monarch** (3 Phasen: Schwert, Asche-Magie, Raserei) |

**Bauvorgaben für jeden Boss** (wie Barnaby): Zustandsautomat mit Ausholen/aktiv/Erholung je Angriff, gewichtete Auswahl nach Abstand, Haltung mit Bruch und Hinrichtung, Phasenwechsel mit Banner, Flächenangriffe mit Marke ≥ 0,6 s vorher, jede Attacke ist entweder parierbar oder klar als nicht parierbar erkennbar. Figuren aus `crates/art` (`chars::preset` oder Kreaturen-Rig), alle mit denselben Animationen wie der Spieler.

## 6. NPCs und Quests

| NPC | Rolle | Stand |
|---|---|---|
| **Katha die blinde Seherin** | steigert Stufen gegen Asche (Vitalität, Ausdauer, Kraft; Kosten 80 · 1,12^(Stufe−1)) | am Schrein, als Menüpunkt |
| **Corvus der Waffenschmied** | bindet Kriegsasche und wechselt die Prägung | am Schrein, als Menüpunkt |
| **Kapitän Roderick** (aus dem Vorgänger) | trauert um das untergegangene Meer; Rache-Quests, die ihn verzehren | geplant |
| **Der stumme Wanderer** | hilft als Ruf-Phantom bei schweren Bossen, wenn seine Quest verfolgt wird | geplant |

## 7. Grafik

- Seitenansicht ohne Diorama-Kippung und Tiefenschärfe; Licht-Pass mit Punktlichtern und Normalmaps aus dem Vorgänger (`src/gfx`).
- Kacheln: Säulenbasalt mit Aschekruste und glimmenden Funken, dunkle Rückwand in Höhlen, versteinerte Planken, Holz- und Kettenleitern, Winde (`crates/art/src/side.rs`).
- Parallax (hinten → vorn): Himmel in 6 harten Farbstufen mit verfinsterter Sonne (Glutkranz), ferne Klippen (Faktor 0,12), mittlere Klippen (0,3), die verkohlten Galeonen aus dem Vorgänger mit Glutlicht (0,35), nahe Klippen (0,5), Dunst- und Aschepartikel.
- Lichtinseln: Schrein, Fackeln, Laterne am Gürtel des Spielers, Glut der Galeone, brennender Teer.
- Stimmung `env::ash()`: roter Dunst, kaum Sonne, entsättigt.

## 8. Code-Logik (ursprüngliche Lua-Skizze → Umsetzung)

| Lua-Skizze | Rust |
|---|---|
| `Zone = { id="Z01_PortCalamita", pirate_gameplay=false }` | `level::MAP` + `Level::new()` (Selbstprüfung der Karte), keine Schiffs- oder Seesysteme mehr im Code |
| `p:SetCameraLock(true)` | `Game::update_camera`: im Bosskampf steht die Kamera über der Arena |
| `Parallax:Start("AshClouds", 0.3)` | `Level::draw_back` (Klippen, Galeone, Dunst) + `Fx::ambient(…, 5, …)` (Asche und Glut) |
| `OnShortcutUnlocked()` | `Game::interact` (Winde): `lv.shortcut = true`, Banner „ABKÜRZUNG GEÖFFNET“, gespeichert |
