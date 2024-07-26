### Bitcoin Protocol Entwicklung - Woche 3: Mine deinen ersten Block

## Überblick
In dieser Herausforderung besteht deine Aufgabe darin, den Mining-Prozess eines Blocks zu simulieren, der die Validierung und Aufnahme von Transaktionen aus einem gegebenen Satz von Transaktionen umfasst. Das Repository enthält einen Ordner namens `mempool`, der JSON-Dateien enthält. Diese Dateien repräsentieren einzelne Transaktionen. Dein Ziel ist es, erfolgreich einen Block zu minen, indem du einige dieser Transaktionen gemäß den unten aufgeführten spezifischen Anforderungen einfügst.

> [!NOTE] 
> Es sind nur grundlegende Validierungsprüfungen erforderlich. Die Signaturvalidierung ist optional und liegt außerhalb des Umfangs dieser Aufgabe.

## Ziel
Dein Hauptziel ist es, ein Skript zu schreiben, das eine Reihe von Transaktionen verarbeitet, validiert und dann in einen Block mined. Die Ausgabe deines Skripts sollte eine Datei namens `out.txt` sein, die einem spezifischen Format folgt.

Platziere deine Lösung im entsprechenden Verzeichnis basierend auf deiner gewählten Programmiersprache:
- [bash](./bash/solution.sh)
- [javascript](./javascript/index.js)
- [python](./python/main.py)
- [rust](./rust/src/main.rs)

## Anforderungen
### Eingabe
- Du erhältst einen Ordner namens `mempool`, der mehrere JSON-Dateien enthält. Jede Datei repräsentiert eine Transaktion, die alle notwendigen Informationen zur Transaktion enthält.

### Ausgabe
Dein Skript muss eine Ausgabedatei namens `out.txt` erzeugen, die die folgende Struktur hat:
- Erste Zeile: Der Blockheader.
- Zweite Zeile: Die serialisierte Coinbase-Transaktion.
- Folgende Zeilen: Die Transaktions-IDs (txids) der im Block geminten Transaktionen in der Reihenfolge. Die erste txid sollte die der Coinbase-Transaktion sein.

### Schwierigkeitstarget
Das Schwierigkeitstarget ist `0000ffff00000000000000000000000000000000000000000000000000000000`. Dies ist der Wert, den der Blockhash unterschreiten muss, damit der Block erfolgreich gemint wird.

### Vorheriger Block Hash
Du kannst jeden Wert für den vorherigen Blockhash verwenden, solange er das Schwierigkeitstarget erfüllt.

## Ausführung
Um deine Lösung lokal zu testen:
- Kommentiere die Zeile aus, die deiner Sprache in [run.sh](./run.sh) entspricht.
- Führe [`local.sh`](./local.sh) aus.

Wenn dein Code funktioniert, wirst du den Test erfolgreich abschließen.

## Bewertungskriterien
Deine Einreichung wird basierend auf folgenden Kriterien bewertet:
- **Autograder**: Dein Code muss den Autograder [Test-Skript](./test/sanity-checks.spec.ts) bestehen.
- **Erklärende Kommentare**: Füge Kommentare hinzu, die jeden Schritt deines Codes erklären.
- **Codequalität**: Dein Code sollte gut organisiert, kommentiert und den Best Practices entsprechen.

### Plagiat-Politik
Unser Plagiatserkennungs-Checker identifiziert gründlich alle Fälle von Kopieren oder Betrug. Die Teilnehmer müssen ihre Lösungen im vorgesehenen Repository veröffentlichen, das privat und nur für die Einzelperson und den Administrator zugänglich ist. Lösungen sollten nicht öffentlich oder mit Kollegen geteilt werden. Im Falle von Plagiaten werden beide beteiligten Parteien direkt disqualifiziert, um Fairness und Integrität zu wahren.

### KI-Nutzungs-Haftungsausschluss
Du kannst KI-Tools wie ChatGPT verwenden, um Informationen zu sammeln und alternative Ansätze zu erkunden, aber vermeide es, ausschließlich auf KI für vollständige Lösungen zu setzen. Überprüfe und validiere alle erhaltenen Einblicke und wahre ein Gleichgewicht zwischen KI-Unterstützung und eigenständigem Problemlösen.

## Warum diese Einschränkungen?
Diese Regeln sollen dein Verständnis der technischen Aspekte von Bitcoin verbessern. Durch das Abschließen dieser Aufgabe erhältst du praktische Erfahrungen mit der Technologie, die die Vertrauenslosigkeit von Bitcoin sichert und aufrechterhält. Diese Herausforderung testet nicht nur deine Fähigkeit, funktionale Bitcoin-Anwendungen zu entwickeln, sondern fördert auch eine tiefe Auseinandersetzung mit den Kernelementen der Bitcoin-Technologie.