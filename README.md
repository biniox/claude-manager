# Claude Manager

Aplikacja desktopowa do zarządzania wieloma instancjami [CloudCode](https://github.com/anthropics/cloudcode). Zbudowana z Tauri, React i TypeScript.

![Claude Manager](docs/screenshot.png)

## Funkcje

- **Zarządzanie wieloma instancjami** - Twórz, zarządzaj i przełączaj między wieloma instancjami CloudCode
- **Terminal PTY** - Pełna emulacja terminala z interaktywną obsługą shella
- **Kontrola procesów** - Uruchamiaj, zatrzymuj i restartuj instancje jednym kliknięciem
- **Wyjście w czasie rzeczywistym** - Streamuj wyjście terminala w czasie rzeczywistym
- **Autostart** - Konfiguruj instancje do automatycznego uruchamiania przy starcie aplikacji
- **Trwała konfiguracja** - Twoje instancje są zapisywane i przywracane między sesjami
- **Ciemny motyw** - Piękny ciemny motyw z fioletowymi akcentami

## Tech Stack

### Frontend

- **React 19** - biblioteka UI
- **TypeScript** - bezpieczeństwo typów
- **Zustand** - zarządzanie stanem
- **Tailwind CSS 4** - style
- **xterm.js** - emulacja terminala
- **Radix UI** - podstawy komponentów

### Backend

- **Tauri 2** - framework desktopowy
- **Rust** - język backendu
- **portable-pty** - obsługa PTY dla wielu platform
- **tokio** - runtime asynchroniczny

## Instalacja

### Wymagania wstępne

- Node.js 18+ z pnpm
- Rust toolchain (dla Tauri)
- Claude CLI (`claude`) zainstalowany i dostępny w PATH

### Rozwój

```bash
# Sklonuj repozytorium
git clone https://github.com/your-username/claude-manager.git
cd claude-manager

# Zainstaluj zależności
pnpm install

# Uruchom w trybie deweloperskim
pnpm tauri dev
```

### Budowanie

```bash
# Zbuduj dla produkcji
pnpm tauri build
```

## Użycie

### Tworzenie instancji

1. Kliknij przycisk "Nowa instancja" w pasku bocznym
2. Wpisz nazwę dla swojej instancji
3. Kliknij "Przeglądaj", aby wybrać folder projektu
4. Opcjonalnie włącz "Autostart", aby uruchamiać automatycznie
5. Kliknij "Utwórz instancję"

### Zarządzanie instancjami

- **Start**: Kliknij przycisk "Start", aby uruchomić instancję Claude
- **Stop**: Kliknij "Stop", aby zatrzymać instancję
- **Restart**: Kliknij "Restart", aby zatrzymać i ponownie uruchomić
- **Usuń**: Kliknij ikonę kosza, aby usunąć instancję

### Interakcja z terminalem

- Kliknij na instancję w pasku bocznym, aby ją wybrać
- Wpisuj komendy bezpośrednio w terminalu
- Używaj standardowych skrótów terminalowych (Ctrl+C itp.)

## Konfiguracja

Instancje są zapisywane w:

- **macOS/Linux**: `~/.claude-manager/instances.json`
- **Windows**: `%APPDATA%\claude-manager\instances.json`

Format:

```json
{
    "instances": [
        {
            "id": "uuid",
            "name": "Mój Projekt",
            "folder_path": "/sciezka/do/projektu",
            "autostart": true,
            "created_at": "2024-01-01T00:00:00Z"
        }
    ]
}
```

## Architektura

```
claude-manager/
├── src/                          # Frontend source
│   ├── app/                       # React Router app
│   │   ├── routes/                # Komponenty stron
│   │   └── router.tsx            # Konfiguracja routera
│   ├── components/                # Wielokrotnego użytku komponenty UI
│   │   └── ui/                   # Komponenty Shadcn
│   ├── features/                  # Moduły funkcjonalności
│   │   ├── instances/             # Zarządzanie instancjami
│   │   │   ├── components/        # UI instancji
│   │   │   ├── store/            # Store Zustand
│   │   │   └── types/            # Typy TypeScript
│   │   └── terminal/             # Emulacja terminala
│   └── lib/                      # Narzędzia
├── src-tauri/                    # Backend source
│   ├── src/
│   │   ├── main.rs               # Punkt wejścia
│   │   ├── commands.rs           # Komendy Tauri
│   │   ├── config.rs             # Konfiguracja
│   │   ├── process_manager.rs     # Zarządzanie procesami
│   │   ├── terminal_io.rs         # Wejście/wyjście PTY
│   │   └── types.rs              # Wspólne typy
│   └── Cargo.toml               # Zależności Rust
└── docs/                         # Dokumentacja
    ├── api.md                    # Dokumentacja API
    ├── architecture.md            # Szczegóły architektury
    └── development.md            # Przewodnik dewelopera
```

## API

### Frontend → Backend (Komendy Tauri)

| Komenda                   | Parametry                                   | Zwraca               | Opis                        |
| ------------------------- | ------------------------------------------- | -------------------- | --------------------------- |
| `create_instance`         | `name`, `folder_path`, `autostart`          | `Instance`           | Utwórz nową instancję       |
| `get_instances`           | -                                           | `Instance[]`         | Pobierz wszystkie instancje |
| `delete_instance_command` | `id`                                        | `void`               | Usuń instancję              |
| `update_instance_command` | `id`, `name?`, `folder_path?`, `autostart?` | `Instance`           | Aktualizuj instancję        |
| `start_instance`          | `id`                                        | `void`               | Uruchom instancję           |
| `stop_instance`           | `id`                                        | `void`               | Zatrzymaj instancję         |
| `restart_instance`        | `id`                                        | `void`               | Restartuj instancję         |
| `write_to_terminal`       | `id`, `input`                               | `void`               | Zapisz do terminala         |
| `resize_terminal`         | `id`, `rows`, `cols`                        | `void`               | Zmień rozmiar terminala     |
| `get_instance_status`     | `id`                                        | `InstanceStatusInfo` | Pobierz status instancji    |

### Backend → Frontend (Events)

| Event                 | Payload                 | Opis                  |
| --------------------- | ----------------------- | --------------------- |
| `instance-output`     | `{ instance_id, data }` | Wyjście terminala     |
| `instance-terminated` | `{ instance_id, data }` | Zakończenie instancji |

## Współpraca

Współpraca jest mile widziana! Zachęcamy do zgłaszania Pull Request.

1. Fork repozytorium
2. Utwórz branch funkcjonalności (`git checkout -b feature/AmazingFeature`)
3. Zatwierdź zmiany (`git commit -m 'Dodaj wspaniałą funkcję'`)
4. Wypchnij do brancha (`git push origin feature/AmazingFeature`)
5. Otwórz Pull Request

## Licencja

Ten projekt jest licencjonowany na licencji MIT - zobacz plik [LICENSE](LICENSE) dla szczegółów.

## Podziękowania

- [Tauri](https://tauri.app/) - Framework desktopowy
- [CloudCode](https://github.com/anthropics/cloudcode) - CLI Claude
- [xterm.js](https://xtermjs.org/) - Emulator terminala
- [Radix UI](https://www.radix-ui.com/) - Podstawy komponentów
- [Tailwind CSS](https://tailwindcss.com/) - Framework CSS

## Wsparcie

W razie problemów, pytań lub propozycji współpracy, odwiedź [repozytorium GitHub](https://github.com/your-username/claude-manager/issues).
