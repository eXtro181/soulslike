# Spielmechanik und Steuerung – Werte und Nachbau-Anleitung

> **Altdokument (Vorgänger *Throne of the Drowned*).** Eclipse Ashes hat Piraten- und Seemechaniken entfernt; das aktuelle Spiel beschreibt [`ECLIPSE_ASHES.md`](ECLIPSE_ASHES.md). Dieses Dokument bleibt als Referenz für Lore, Figuren und Werte.

Gegenstück zu `docs/GRAFIK_ANLEITUNG.md`: alle Spielregeln von *Throne of the Drowned* mit den exakten Zahlen aus dem Code (Stand Commit `ffda89f`, übernommen aus `eXtro181/PirateGame`). Wer ein neues Projekt auf dieser Grundlage baut, kann Kampf, Werte, Gegner, Bosse, Seefahrt und Steuerung damit 1:1 nachbilden. In Klammern steht jeweils die Fundstelle im Code.

Einheiten: Weltpixel (32 px je Kachel), Sekunden, Bogenmaß. `S = 2.0` ist der Maßstab gegenüber der alten 16-px-Version (`src/game/actors.rs`); Gegnerwerte in `data.rs` werden über `en(..)` bei Tempo, Sichtweite und Reichweite automatisch ×2 genommen.

---

## 1. Steuerung

### 1.1 Tastatur (`src/game/input.rs`, `keys`)
| Aktion (`Act`) | Taste | an Land | auf See |
|---|---|---|---|
| Up/Down/Left/Right | W S A D / Pfeile | laufen (Achse normiert, Totzone 0,15) | ←→ wenden, ↑↓ Segelstufe ±1 (0–3) |
| Attack | J | leichter Hieb | Breitseite Backbord |
| Heavy | L (oder Shift+J) | schwerer Hieb | Breitseite Steuerbord |
| Item | K | Pistole bzw. Gezeitenstoß | Breitseite Steuerbord |
| Roll | Leertaste | Rolle | – |
| Parry | Q | Parieren | – |
| Lock | Tab | Ziel fixieren/lösen (nächstes Ziel bis 320 px) | – |
| Drink | R | Rum trinken | – |
| Belt1 / Belt2 | 1 / 2 | Bombe werfen / Seelenphiole | – |
| Spell / Swap | C / X | Zauber wirken / wechseln | C = Crew-Geschütz |
| Interact | E | sprechen, rasten, Truhe, Tür, an Bord | anlegen (Steg ≤ 120 px) |
| Shanty | F | – | Shanty (40 s, Fahrt ×1,35) |
| Names | Alt (halten) | Namen aller Gegenstände | – |
| Map / Legend / Status / Menu | M / H / I / Esc | Seekarte / Steuerung / Fortschritt / Pause | gleich |
| Confirm | Enter | Menü bestätigen | – |

Eingaben werden je Bild als „gehalten“ und „gerade gedrückt“ geführt; virtuelle (Touch-)Knöpfe laufen über `v_press`/`v_release` in dieselben Mengen. Maus: Hover in Menüs, Klick bestätigt.

### 1.2 Touch (`src/ui/touch.rs`)
- **Stick**: Berührung in der linken 45 % der Fläche (unterhalb y 36) setzt den Ursprung frei; Auslenkung / 34 px, auf Länge 1 begrenzt. Zeichnung: Grundfläche 80×80, Knopf 32×32 (Farbe 232,200,140,170).
- **Knöpfe** (UI-Pixel, Mittelpunkt relativ zu (Breite−44, Höhe−44), Radius, Treffer mit +6 px Toleranz):

| an Land | dx, dy, r | auf See | dx, dy, r |
|---|---|---|---|
| Hieb | 0, 0, 22 | Backbord | −64, 0, 20 |
| Schwer | −50, 10, 17 | Steuerbord | 0, −8, 22 |
| Rolle | −24, −46, 17 | Anlegen | −40, −56, 16 |
| Parade | 12, −54, 15 | Shanty | 6, −64, 16 |
| E | −86, −14, 15 | | |
| Rum | −66, −50, 14 | | |
| Zauber | −42, −86, 14 | | |
| Schuss | −4, −92, 14 | | |
| Bombe | −110, −52, 12 | | |
| Phiole | −84, −86, 12 | | |
| Ziel | 20, −94, 12 | | |

Oben rechts immer: Menü, Karte, „Zauber ↻“ (x = Breite − 30 − i·46, y 16, r 11).
- Auf See mit Stick: Kurs dreht auf die Stickrichtung (`turn_to`, max 1,6 rad/s × Tempo-Ausbau), Segel mindestens Stufe 2.
- `src/ui/textinput.rs` legt echte DOM-Eingabefelder über den Canvas, damit die iPhone-Tastatur erscheint.

---

## 2. Werte und Charakter (`src/game/data.rs`)

### 2.1 Herkunft
| id | Name | Waffe | Nebenhand | Vit/Aus/Kra/Ges/Tie | Stufe |
|---|---|---|---|---|---|
| matrose | Deckmatrose | cutlass | pistol | 6/6/6/6/4 | 6 |
| kanonier | Kanonier | hammer | – | 7/5/9/3/3 | 6 |
| duellant | Duellant | rapier | – | 5/7/4/9/3 | 6 |
| schmuggler | Schmuggler | dagger | pistol | 5/7/4/8/4 | 6 |
| sumpfkind | Sumpfkind | fetish | tide | 4/6/3/5/10 | 6 |
| treibgut | Treibgut | club | – | 3/3/3/3/3 | 1 |

Sonderregeln: Schmuggler +20 % Perlen von Gegnern, Kanonier +30 % Breitseitenschaden.

### 2.2 Geschenke (`GIFTS`, `new_save`)
`rum` +1 Rumflasche (Start 3 → 4) · `coin` +15 % Perlen · `chart` alle Inseln außer dem Ende aufgedeckt · `bombs` 5 Bomben · `pearl` Start mit 1 Perle · `none`.
Startausrüstung: 3 Rum, 1 Phiole, 8 Kugeln bei Pistole, Zauber Kettenblitz, letztes Feuer `kueste`.

### 2.3 Abgeleitete Werte (`derived`)
- Leben = round(60 + 9·Vit [+30 Frostmantel])
- Ausdauer = round(55 + 7·Aus [+20 Muschelhorn])
- Geist = round(40 + 6·Tie [+20 Sturmglas] [+15 Eisglas] [+10 Voodoopuppe])
- Zauberschaden = round((18 + 4,5·Tie) [×1,1 Eisglas] [×1,1 Voodoopuppe])
- Waffenschaden = round(Basis · (0,7 + 0,06·Skalenwert) · (1 + 0,12·Waffenstufe) [×1,1 Ascheherz])
- Nebenhand = round(Basis · (0,75 + 0,05·Ges)), Gezeitenstoß mit Tie statt Ges
- Rüstungsfaktor = [0,88 Ring] · [0,92 Moortinktur]
- Ausdauer-Erholung = [×1,3 Mantel des Lotsen] · [×1,2 Rabenfeder]
- Stufe kaufen: `level_cost(l) = round(120 · 1,1^l)` Perlen, +1 auf einen Wert.
- Waffe härten (Schmied): `upgrade_cost(l) = round(600 · 1,6^l)` Perlen und l+1 Gezeitensplitter.

### 2.4 Waffen
| key | Name | Schaden | Skala | Tempo | Ausdauer | Reichweite | Schwer × | Parier × | Preis |
|---|---|---|---|---|---|---|---|---|---|
| cutlass | Entermesser | 17 | Kra | 1,0 | 14 | 1,6 | 2,0 | 1,0 | 0 |
| hammer | Ankerhammer | 30 | Kra | 0,66 | 24 | 1,9 | 1,9 | 1,0 | 2400 |
| rapier | Rapier | 13 | Ges | 1,3 | 10 | 1,9 | 1,8 | 1,6 | 1800 |
| dagger | Dolch | 11 | Ges | 1,45 | 9 | 1,3 | 1,7 | 1,0 | 900 |
| fetish | Muschelfetisch | 9 | Tie | 1,1 | 12 | 1,4 | 1,6 | 1,0 | 1500 |
| club | Morscher Knüppel | 10 | Kra | 0,95 | 13 | 1,5 | 1,8 | 1,0 | 0 |

Nebenhand: Steinschlosspistole 34 Schaden / 10 Ausdauer (verbraucht 1 Kugel), Gezeitenstoß 24 / 18.
Zauber: Kettenblitz 22 Geist, Feuerball 18 Geist.

---

## 3. Kampf des Spielers (`src/game/actors.rs`, `player_logic`)

Zustände `PS`: Free, Roll, Attack, Parry, Hurt, Drink, Cast, Shoot, Throw, Sit, Talk, Dead, Ship.

| Mechanik | Wert |
|---|---|
| Laufen | (60 + 0,8·Ges) · S px/s; verlangsamt (Frost) ×0,6 für `slow_t` |
| Laufanimation | Bild = Zeit·10; Stand = Zeit·2,5 |
| Ausdauer-Erholung | 42/s · Erholungsfaktor; außerhalb von Free ×0,4; keine während der Rolle; 0,7 s Pause nach jedem Verbrauch |
| Ausdauer-Regel | Aktion geht, solange Ausdauer > 0 (darf ins Minus fallen) |
| Rolle | 18 Ausdauer, Dauer 0,42 s, Unverwundbar 0–0,32 s, Tempo 150·S·(1 − 0,6·t/0,42) |
| Leichter Hieb | Ausdauer der Waffe, Dauer 0,48 s / Tempo |
| Schwerer Hieb | Ausdauer ×1,6, Dauer 0,85 s / Tempo, Schaden × Schwer-Faktor, Wucht doppelt |
| Treffermoment | bei 38 % der Dauer, 10 px Ausfallschritt; Reichweite (Reichweite·14 + 6)·S, Bogen ±1,15 rad (schwer ±1,3); Bosse +½ Trefferradius |
| Kombo | ab 55 % der Dauer wird der nächste Hieb vorgemerkt; Kombo-Zähler 0→1→2, Schaden ×(1 + 0,1·Kombo); Schaden ×(0,9…1,1) Zufall |
| Bildfolge Hieb | Bild 0 < 25 %, 1 < 40 %, 2 < 55 %, 3 < 75 %, sonst 4; Ausrichtung bis 30 % auf Ziel bzw. Stick |
| Parieren | 10 Ausdauer, Fenster 0,2 s × Parier-Faktor (Rapier 0,32 s), Zustand 0,45 s; erfolgreich = Angreifer taumelt, „Pariert! Jetzt zustechen.“ |
| Getroffen werden | Schaden · Rüstung · NG-Faktor, danach 0,45 s unverwundbar, Zustand Hurt 0,35 s (beim Trinken nur bei schweren Treffern) |
| Rum | 1 Flasche, Wirkung bei 0,5 s: heilt 45 % des Lebens über die Zeit (max. 90 %/s), Zustand 0,95 s, langsam gehen mit 40 px/s möglich |
| Seelenphiole | wie Rum, füllt den Geist ganz (nur wenn nicht voll) |
| Zauber | Wirkung bei 0,3 s, Zustand 0,65 s |
| Kettenblitz | erstes Ziel bis 280 px im Kegel ±0,9 rad, springt bis 4 Ziele weit (≤ 160 px), Schaden je Sprung ×0,8 |
| Feuerball | Schaden ×1,3, Geschwindigkeit (340, 280), Radius 10, Lebensdauer 1,4 s, explodiert |
| Pistole | Schuss bei 0,18 s, Kugel (600, 520) px/s, r 6, 0,7 s; Zustand 0,5 s |
| Gezeitenstoß | Welle (300, 240) px/s, r 12, 1,1 s, durchschlagend |
| Bombe | Wurf bei 0,2 s auf 140×110 px voraus, Flugzeit 0,75 s, vz 220, Schwerkraft 600, Explosion 70 Schaden |
| Herunterspringen | 0,15 s gegen eine Kante drücken → genau eine Ebene tiefer (`jump_target`), Sprung 0,42 s |
| Zielerfassung | Tab: nächstes Ziel bis 320 px; Hiebe, Zauber und Schüsse richten sich darauf aus |

### 3.1 Tod, Feuer, Perlen (`src/game/mod.rs`)
- Tod: Banner „DU BIST ERTRUNKEN“, alle Perlen werden zum **Perlenfleck** an der letzten sicheren Stelle (`last_safe`: freier Boden, kein Abgrund). Stirbt man erneut, bevor man ihn aufhebt, ist er verloren. Mit dem Cheat „Unendlich Perlen“ gibt es keinen Fleck.
- Wiederbelebung am letzten Säbelfeuer: volles Leben/Ausdauer/Geist, Rum und Phiolen voll, alle Gegner neu, nicht besiegte Bosse zurückgesetzt, Schiff am letzten Steg mit vollem Rumpf, Spielstand gespeichert.
- 11 Säbelfeuer: `kueste hain asche asche2 frost frost2 raben raben2 mangroven mangroven2 ende`. Rasten heilt, füllt auf, belebt Gegner, speichert; Reisen zwischen entzündeten Feuern.
- Perlen von Gegnern: Basis × [1,2 Schmuggler] × [1,15 Glücksdublone] × [1,15 Treibholz-Amulett] × (1 + 0,25·NG).

---

## 4. Gegner (`data.rs::enemies`, KI in `actors.rs::enemy_logic`)

### 4.1 KI-Ablauf
Zustände `ES`: Idle → Chase → Windup → Strike → Recover, dazu Lurk (Mimics, wachen bei < 52 px auf), Hurt (0,3 s), Stagger (1,4 s), Dead.
- Sieht den Spieler bei Abstand < Sichtweite und freier Sichtlinie (`line_clear`). Leine: > 520 px vom Startpunkt → zurück. Ohne Sicht 3 s weiterjagen.
- Angriff, wenn Abstand < Reichweite + 8 und Abklingzeit 0; Fernkämpfer schießen zwischen 80 px und Reichweite und weichen unter 120 px zurück.
- Ausholen = `windup`, danach Schlag (0,18 s), Erholung = `recover`, danach Abklingzeit 0,4–1,0 s.
- Sonderarten: `aoe` = Bodenschlag mit Radius (y ×1,3), schwer; `charge` = Ansturm 260 px/s für 0,5 s; `ranged` mit Salven (`volley`, Fächer 0,6 rad); `burn` = Brand; `chill` = Spieler 2 s verlangsamt.
- Schwarm: Gegner stoßen sich unter 28 px voneinander ab (Faktor 0,6).
- Treffer: Gleichgewicht (`poise`) sinkt um die Wucht; ≤ 0 → Hurt und volles Gleichgewicht. Taumelnde Gegner (nach Parade) nehmen ×2,6 Schaden (Kritischer Stich). Gegner mit Gleichgewicht < 50 werden zurückgestoßen.
- NG+: Leben × (1 + 0,5·NG), Schaden × (1 + 0,3·NG). Nur Gegner im Umkreis der Kamera (±1500 × ±1000 px) werden aktualisiert.

### 4.2 Gegnertabelle (Rohwerte; Tempo, Sicht, Reichweite im Spiel ×2)
| key | Name | Leben | Schaden | Tempo | Sicht | Reichw. | Ausholen | Erholen | Perlen | Gleichgew. | Besonderes |
|---|---|---|---|---|---|---|---|---|---|---|---|
| skeleton | Ertrunkener Matrose | 60 | 16 | 38 | 110 | 22 | 0,5 | 0,6 | 60 | 20 | |
| skelBrute | Ankerträger | 150 | 32 | 28 | 100 | 28 | 0,95 | 1,0 | 180 | 60 | aoe 52 |
| skelGunner | Musketen-Gerippe | 45 | 20 | 30 | 150 | 130 | 0,9 | 1,2 | 80 | 10 | Fernkampf |
| ashSkeleton | Aschegänger | 80 | 20 | 42 | 120 | 22 | 0,45 | 0,5 | 110 | 25 | Brand |
| drowned | Tangertrunkener | 95 | 22 | 34 | 110 | 30 | 0,6 | 0,7 | 120 | 30 | |
| drownedIce | Frostertrunkener | 95 | 22 | 34 | 110 | 30 | 0,6 | 0,7 | 120 | 30 | Kälte |
| stormSailor | Sturmmatrose | 85 | 19 | 46 | 120 | 26 | 0,4 | 0,5 | 120 | 20 | |
| crab | Strandkrabbe | 35 | 10 | 52 | 80 | 16 | 0,35 | 0,5 | 30 | 5 | Rig |
| crabWarrior | Krabbenkrieger | 170 | 28 | 32 | 100 | 26 | 0,75 | 0,8 | 220 | 70 | Rig |
| shell | Muschelkriecher | 70 | 18 | 26 | 90 | 18 | 0,6 | 0,6 | 90 | 40 | Ansturm |
| livingChest | Lebendiger Schatzrechner | 90 | 22 | 44 | 120 | 22 | 0,4 | 0,6 | 260 | 40 | Mimic |
| mimicChest | Schatz-Mimic-Rechner | 120 | 26 | 40 | 120 | 24 | 0,45 | 0,6 | 320 | 50 | Mimic |
| skelkraken | Skelett-Kraken | 420 | 30 | 30 | 120 | 36 | 0,9 | 1,0 | 900 | 120 | aoe 68 |
| cannonAngler | Kanonen-Angel-Fisch-Dämon | 120 | 26 | 24 | 170 | 150 | 1,1 | 1,4 | 240 | 60 | Kanone |
| cannonEater | Kanonen-Fresser-Dämon | 140 | 28 | 30 | 110 | 26 | 0,7 | 0,8 | 260 | 60 | Ansturm |
| stormHarpy | Sturm-Wind-Harpye-Konstrukt | 70 | 18 | 62 | 140 | 22 | 0,4 | 0,6 | 160 | 15 | |
| wyvern | Küsten-Wyvern | 520 | 34 | 50 | 150 | 34 | 0,8 | 1,0 | 1400 | 140 | Ansturm |
| orcCaptain | Ork-Schlachter-Kapitän | 560 | 36 | 30 | 130 | 34 | 0,9 | 1,0 | 1400 | 160 | aoe 72 |
| bellConstruct | Uhrwerk-Glocken-Konstrukt | 600 | 34 | 24 | 130 | 40 | 1,1 | 1,1 | 1500 | 200 | aoe 88 |
| abyssPirate | Tiefen-Abgrund-Pirat | 100 | 22 | 36 | 120 | 26 | 0,55 | 0,6 | 180 | 35 | |
| voodoo | Voodoo-Mutanten-Kapitän | 160 | 22 | 30 | 160 | 140 | 0,9 | 1,0 | 420 | 60 | Seelenschuss ×3 |
| turtleWorm | Schild-Kröten-Festungs-Wurm | 720 | 30 | 18 | 170 | 160 | 1,2 | 1,3 | 1800 | 260 | Kanone ×3 |
| trueMimic | Mimic-Kapitäns wahre Gestalt | 650 | 38 | 34 | 130 | 40 | 0,9 | 0,9 | 1800 | 200 | aoe 80 |

„Rig“ = Kreatur aus `crates/art/src/beasts.rs`, sonst Figur aus `chars.rs`. Alle Gegner haben dieselben Animationen wie der Spieler (idle, walk, atk, hurt, dead, ggf. shoot/cast).

---

## 5. Bosse

### 5.1 Vier Könige (`data.rs::KINGS`, `src/game/boss.rs`)
| key | Insel | Name | 2. Gestalt | Leben | Schaden |
|---|---|---|---|---|---|
| fire | asche | Admiral Brandvogt, der Glutkönig | Dämonen-Kapitän-Mantikor | 1300 | 30 |
| ice | frost | Königin Ysra Frostweh | Abgrund-Gorgonen-Königin | 1250 | 28 |
| wind | raben | Kapitän Rabensturm, der Sturmkönig | Uhrwerk-Sturm-Monster | 1200 | 27 |
| earth | mangroven | Morgrim Korallenhand, König der Tiefe | Kraken-Basilisk-Chimäre | 1450 | 32 |

Je 4000 Perlen (+25 % je NG), Galionsfigur, 2 Gezeitensplitter, öffnet das Tor der Insel.
- Zustände `BS`: Dormant → Intro → Idle ⇄ Attack → (bei 50 % Leben) Transform → Phase 2 → Dead; Stagger nach Parade (Schaden ×2,2).
- Angriff = Windup → Active → Recover. Schaden in Phase 2 ×1,15. Kombo-Folgehiebe: Ausholen 0,32 s (schnell 0,22), in Phase 2 ×0,8; nach der Kombo 0,7 s Erholung.
- Angriffsarten `AK`: Combo, Sweep, Dash, Slam, Cone, Volley, Rain, Nova, Gust, Spikes, Spin, Lash, Leap. Gewichtete Auswahl je Element/Phase (`s` = Maßstab):

| Element | Phase 1 (Gewicht) | Phase 2 (Gewicht) |
|---|---|---|
| fire | Combo×2 (3), Dash 120 (2), Volley 3 (2), Rain 4 (1), Nova 46 (1) | Dash 150 (3), Cone 90 (3), Nova 56 (1), Rain 6 (2) |
| ice | Combo×3 schnell (3), Volley 5 (2), Rain 5 (2), Nova 50 langsam (1) | Sweep 70 (3), Volley 7 (2), Nova 64 langsam (2), Rain 7 (2) |
| wind | Combo×2 (3), Gust (2), Rain 6 Blitz (2), Dash 140 (2) | Dash 170 (3), Volley 7 (3), Gust (2), Rain 8 Blitz (2) |
| earth | Slam 30 (3), Spikes 7 (2), Combo×2 schwer (2), Volley 3 (1) | Slam 40 doppelt (3), Spikes 9 (3), Volley 4 (1), Nova 70 (2) |

- Element-Treffer: Eis verlangsamt 1,5 s, Feuer setzt Brand, Erde zieht 15 Ausdauer ab.

### 5.2 Seebosse (`src/game/seaboss.rs`)
| el | Ungeheuer | Piratenschiff (ab 50 %) | Leben | Perlen |
|---|---|---|---|---|
| fire | Tiamat, Mutter der Glut | Die Glutkrone | 1300 | 5000 |
| ice | Krallgard, der Eishummer | Die Frostwitwe | 1300 | 5000 |
| wind | Sturmzunge, die Seeschlange | Die Rabenbraut | 1300 | 5000 |
| earth | Der Abgrundangler | Die Tiefenlaterne | 1300 | 5000 |

Bei 50 % wird das Ungeheuer zum Schiff (Leben auf genau 50 % gesetzt, Blitz), danach Wrack; erbeutete Wracks brennen im Ring der Glutwacht. Abklingzeiten der Angriffe 2,0–3,4 s, Bodenschläge (Slam) r 70, 20 Schaden; Breitseiten nur bei seitlicher Lage (|Seite| > 0,7) und < 360 px.

### 5.3 Endkampf (`src/game/endboss.rs`, Ablauf `EP`)
Approach → Rise (7 s, der Fliegende Holländer steigt auf) → Ship (2600 Leben, Breitseiten, XXL-Feuer, Donnermörser; unter 60 % erscheint Malakors roter Kraken, 900 Leben) → Drain (9 s, das Meer läuft über den Rand) → Seabed: **Malakor** zu Fuß (2400 Leben, 32 Schaden, vier Schwerter, Combo/Spin/Lash/Leap/Geysire) → bei 0 Leben „MALAKOR FÄLLT“ → **Knochenkraken** (2800 Leben; Spikes 10, Sweep 110, Volley 18 rundum, Leap r 80 schwer) → Done (20 000 Perlen, Säbelfeuer „ende“ erscheint, `save.end_won`, `save.end_naval`).

### 5.4 Strudel
Öffnet, wenn alle 4 Galionsfiguren auf den Thronen sitzen und alle 4 Seebosse besiegt sind (`whirl_ready`). Acht Krakenarme; ab 250 px packen sie das Schiff (`Cut::Whirl`: 1 s Griff mit Wackeln, dann Spirale) → `arrive_end`.

---

## 6. Seefahrt (`src/game/ship.rs`, `crew.rs`)
| Mechanik | Wert |
|---|---|
| Rumpf | 120 + 24·Verteidigung |
| Zielfahrt | Segelstufe (0–3) · 60 px/s · [1,35 Shanty] · (1 + 0,06·Tempo); Annäherung mit Faktor 0,8/s |
| Wenden (Tasten) | ax · (0,7 + v/120) · (1 + 0,04·Tempo) rad/s |
| Wenden (Stick) | auf Stickrichtung, max 1,6·(1 + 0,04·Tempo) rad/s, Segel ≥ 2 |
| Auflaufen | Tempo ×0,3; über 40 px/s 4 Schaden |
| Breitseite | Schaden 40 · [1,3 Kanonier] · (1 + 0,12·Angriff); 3 + Angriff/3 Kugeln im Abstand 20 px, 340 px/s quer + Schiffsfahrt, 0,9 s, r 8; Abklingzeit je Seite 1,6 s |
| Schaden am Schiff | × (1 − 0,04·Verteidigung) × NG-Faktor; 0 Rumpf = Tod |
| Shanty (F) | 40 s, Fahrt ×1,35 |
| Ausbauten | Tempo/Verteidigung/Angriff je 0–10, Kosten `round(1500 · 1,45^Stufe)` |
| Schiff rufen | steigt am Steg in 1,8 s aus der Tiefe (Alpha über 1,6 s) |
| Geisterschiffe | jagen unter 520 px, Breitseite 14 Schaden bei < 340 px und seitlicher Lage, Abklingzeit 3,2 s, Wendegeschwindigkeit 0,9 rad/s |
| Haie | 60 Leben, jagen unter 320 px, Biss bei < 36 px, Abklingzeit 2,5 s |
| Kraken (See) | 900 Leben, wacht unter 520 px auf, Armschlag alle 3,2 s, Treffer im Umkreis 60 px |

Crew-Geschütze (Taste C, `crew.rs`; Angriffsfaktor 1 + 0,12·Angriff): 1 Mörser (Bogenschuss, 55 Schaden, Explosion) · 2 Kettenkugeln (14 Schaden je Kugel, Fächer) · 3 Flammenwerfer (3 Schaden je Flamme, Kegel bei nahen Zielen) · 4 Kraken-Begleiter (acht grüne Tentakel um das Ziel, 45·(1 + 0,08·Angriff) je Schlag, Abklingzeit 45 s). Crew: Kanonierin Asra (Glut), Navigatorin Lene (Frost), Bootsmann Kalo (Sturm), Zimmermann Ode (Tiefe).

---

## 7. Gegenstände, Läden, Fortschritt
- Seltenheitsfarben: gewöhnlich (216,207,184), ungewöhnlich (95,242,200), selten (232,164,67), einzigartig (220,82,64). Gegenstände am Boden schweben mit Schimmer in dieser Farbe.
- Alle 25 Gegenstände mit Texten: `data.rs::ITEMS` (Tabelle auch in `docs/SPIELKONZEPT.md` 6.5).
- Läden (`SHOPS`, Gegenstand · Preis · einmalig):
  - Glutgrund, Schmiedin Ilka: Ascheherz 5000 · Glutrum 7000 · Folio der Glut 6000 · Bomben 260 (wiederholbar)
  - Reifstein, Pelzhändlerin Wendla: Frostmantel 4500 · Eisglas 4000 · Phiole 2500 (wiederholbar)
  - Rabenwacht, Kartenhändler Orrin: Sturmkompass 6000 · Rabenfeder 3500 · Muschelhorn 2500 · Treibholz-Amulett 3200 · Kugeln 130 (wiederholbar)
  - Moorhain, Kräuterhexe Mabel: Moortinktur 4000 · Voodoopuppe 5000 · Phiole 2800 · Bomben 300
- NG+ (`save.ng`): bei der Laternenmagd nach dem Sieg; Könige, Seebosse, Throne, Endkampf zurück; Stufe, Ausrüstung, Schiff, Crew, Feuer, Schätze bleiben.
- Cheats: „Unendlich Perlen“ (`INF_PEARLS` = 999 999 999, alter Stand wird gemerkt). Debug-Menü und URL-Parameter: siehe `CLAUDE.md`.

## 8. Spielstand (`data.rs::Save`, `src/save.rs`)
- JSON mit `camelCase`, Version 4, alle Felder `#[serde(default)]`; `end_won` heißt im Spielstand weiter `megalodon`. `migrate` repariert alte Stände (Kartenverschiebung um 80×60 Kacheln bei < v4, Grenzen für Aussehen, gültige Feuer).
- Profile in `localStorage`, verschlüsselt mit PBKDF2-SHA256 (150 000 Runden) + AES-256-GCM.
- Speichern beim Rasten, Reisen, nach Bossen und bei der Wiederbelebung.

## 9. Ablauf eines Bildes (Spiel)
`main.rs::tick` → Eingabe (Tastatur, Maus, Touch) → `Game::update` (Menüs/Dialoge, Spieler oder Schiff, Gegner im Umkreis, Bosse, NPCs, Tiere, Leben, Geschosse, Schiff/See, Zwischensequenzen, Timer) → `Env::apply` (Licht/Stimmung je Zone) → Welt streamen und zeichnen (`world/view.rs`) → Figuren (`draw.rs`), Effekte (`fx.rs`), HUD/Menüs (`ui/`) → Renderer (`gfx/render.rs`) → Audio.

## 10. Nachbau-Reihenfolge für ein neues Projekt
1. Eingabe-Schicht wie in 1.1/1.2 (Aktionen statt Tasten, Touch als virtuelle Aktionen).
2. Werte und Formeln aus 2 als reine Datenmodule (`data.rs`) – zuerst testbar ohne Grafik.
3. Spieler-Zustandsautomat aus 3 mit genau diesen Zeiten; danach Gegner-KI aus 4.
4. Bosse (5.1) mit gewichteter Angriffsauswahl, dann Seefahrt (6) und Seebosse (5.2), zuletzt Endkampf (5.3).
5. Tod/Feuer/Perlen (3.1), Läden und Fortschritt (7), Spielstand (8).
6. Grafik nach `docs/GRAFIK_ANLEITUNG.md`, Inhalte und Welt nach `docs/SPIELKONZEPT.md`; Code-Zusammenhänge im Graphify-Abbild `graphify-out/` (siehe `docs/GRAPHIFY.md`).
