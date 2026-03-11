# Changelog

Wszystkie istotne zmiany w Claude Manager będą dokumentowane w tym pliku.

Format oparty na [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
a ten projekt stosuje [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-03-11

### Dodano

- **Ciemny motyw z fioletową kolorystyką** - Piękny ciemny motyw z fioletowymi akcentami
- **Zarządzanie wieloma instancjami** - Tworzenie, odczyt, aktualizacja, usuwanie instancji
- **Emulacja terminala PTY** - Pełna emulacja terminala z interaktywną obsługą shella przy użyciu xterm.js
- **Kontrola procesów** - Uruchamianie, zatrzymywanie i restartowanie instancji jednym kliknięciem
- **Strumieniowanie wyjścia w czasie rzeczywistym** - Przesyłanie wyjścia terminala w czasie rzeczywistym przez eventy Tauri
- **Wsparcie autostartu** - Konfiguracja instancji do automatycznego uruchamiania przy starcie aplikacji
- **Wybór folderu** - Zintegrowany wybór folderu do tworzenia instancji
- **Trwała konfiguracja** - Instancje zapisywane w `~/.claude-manager/instances.json`

### Backend (Rust)

- Komendy Tauri dla operacji CRUD na instancjach
- Menedżer procesów z obsługą PTY przy użyciu `portable-pty`
- Strumieniowanie wyjścia przez eventy do frontendu
- Automatyczne czyszczenie procesów przy zamknięciu aplikacji
- Wsparcie wielu platform (macOS, Linux, Windows)

### Frontend (React + TypeScript)

- Store Zustand do centralnego zarządzania stanem
- Integracja xterm.js z dodatkiem fit do emulacji terminala
- Komponenty UI Shadcn (dialog, input, switch, scroll-area, separator)

### Dokumentacja

- README z przeglądem projektu i przewodnikiem użycia
- Dokumentacja API wszystkich komend Tauri i eventów
- Dokumentacja architektury z diagramami przepływu danych
- Przewodnik dewelopera z instrukcjami konfiguracji
- CHANGELOG do śledzenia historii wersji

### Naprawiono

- Poprawiona obsługa wyjścia terminala bezpośrednio do komponentu xterm
- Zoptymalizowane zarządzanie stanem aplikacji

---

## Wersjonowanie

Projekt stosuje semantyczne wersjonowanie w formacie `MAJOR.MINOR.PATCH`:

- **MAJOR**: Zmiany łamiąco lub nowe główne funkcje
- **MINOR**: Nowe funkcje (kompatybilne wstecznie)
- **PATCH**: Poprawki błędów i drobne ulepszenia

## Rozwój

Najnowsze zmiany rozwojowe znajdziesz w [repozytorium GitHub](https://github.com/your-username/claude-manager).

### Planowane funkcje

- [ ] Wiele terminali na instancję (zakładki)
- [ ] Szukanie i historia terminala
- [ ] Grupowanie/foldery instancji
- [ ] Szybkie akcje z zasobnika systemowego
- [ ] Import/eksport konfiguracji instancji
- [ ] Niestandardowe zmienne środowiskowe na instancję
- [ ] Monitorowanie CPU/pamięci na instancję
- [ ] Skróty klawiszowe
- [ ] Personalizacja motywów
- [ ] Szablony instancji
