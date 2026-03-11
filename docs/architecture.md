# Dokumentacja Architektury

Dokument opisuje architekturę Claude Manager.

## Architektura wysokiego poziomu

```
┌─────────────────────────────────────────────────────────────┐
│                      Claude Manager                      │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React + TypeScript)    Backend (Rust)     │
│  ┌────────────────────────────┐  ┌─────────────────────┐ │
│  │      React Components     │  │   Tauri Commands  │ │
│  │  - Home                 │  │   - CRUD          │ │
│  │  - InstanceCard         │  │   - Process Ctrl   │ │
│  │  - InstanceControls      │  │   - Terminal I/O   │ │
│  │  - CreateInstanceDialog  │  │                     │ │
│  └────────────────────────────┘  └─────────────────────┘ │
│              │                         │                   │
│        ┌─────┴─────┐       ┌─────────┴───────┐      │
│        │   Store   │       │  Process Manager  │      │
│        │ Zustand   │       │  (instancje map)  │      │
│        └───────────┘       └─────────┬───────┘      │
│                                     │                  │
│                              ┌────────┴────────┐       │
│                              │  portable-pty   │       │
│                              └─────────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

## Komponenty

### Frontend

#### Komponenty React

Zlokalizowane w `src/features/`:

```
src/features/
├── instances/
│   ├── components/
│   │   ├── instance-card.tsx       # Element instancji w pasku bocznym
│   │   ├── instance-controls.tsx  # Przyciski Start/Stop/Restart
│   │   └── create-instance-dialog.tsx
│   ├── store/
│   │   └── instances-store.ts      # Store Zustand
│   └── types/
│       └── index.ts
└── terminal/
    └── xterm-wrapper.tsx          # Integracja xterm.js
```

#### Zarządzanie stanem

**Store Zustand** (`instances-store.ts`):

```typescript
interface InstancesState {
    // Stan
    instances: Instance[]
    selectedInstanceId: string | null
    instanceStatuses: Map<string, InstanceStatus>
    isLoading: boolean

    // Akcje
    loadInstances: () => Promise<void>
    createInstance: (name, folder, autostart) => Promise<Instance>
    deleteInstance: (id) => Promise<void>
    updateInstance: (id, updates) => Promise<void>
    selectInstance: (id) => void
    startInstance: (id) => Promise<void>
    stopInstance: (id) => Promise<void>
    restartInstance: (id) => Promise<void>
    updateStatus: (id, status) => void
}
```

### Backend

#### Struktura modułów

```
src-tauri/src/
├── main.rs              # Punkt wejścia, konfiguracja pluginów
├── commands.rs          # Obsługa komend Tauri
├── config.rs            # Trwałość konfiguracji
├── process_manager.rs    # Zarządzanie cyklem życia procesów
├── terminal_io.rs        # Abstrakcja wejścia/wyjścia PTY
└── types.rs             # Wspólne typy
```

#### Menedżer procesów

`ProcessManager` jest singletonem zarządzającym wszystkie uruchomione instancje:

```rust
pub struct ProcessManager {
    processes: Arc<Mutex<HashMap<String, ClaudeProcess>>>,
    app_handle: AppHandle,
}

impl ProcessManager {
    pub fn start_instance(&self, instance: &Instance) -> Result<(), InstanceError>;
    pub fn stop_instance(&self, id: &str) -> Result<(), InstanceError>;
    pub fn restart_instance(&self, instance: &Instance) -> Result<(), InstanceError>;
    pub fn write_to_instance(&self, id: &str, input: &str) -> Result<(), InstanceError>;
    pub fn resize_instance(&self, id: &str, rows: u16, cols: u16) -> Result<(), InstanceError>;
}
```

#### Proces Claude

Każda uruchomiona instancja posiada `ClaudeProcess`:

```rust
pub struct ClaudeProcess {
    pub instance_id: String,
    pub master_writer: Box<dyn Write + Send>,    // Zapis do PTY
    pub master_resize: Box<dyn MasterPty + Send>, // Zmiana rozmiaru PTY
    pub child: Box<dyn Child + Send>,          // Uchwyt procesu
    pub status: InstanceStatus,
}
```

## Przepływ danych

### Uruchamianie instancji

```
Użytkownik klika "Start"
       ↓
Frontend: invoke('start_instance', { id })
       ↓
Backend: ProcessManager::start_instance()
       ↓
Backend: portable_pty::openpty()
       ↓
Backend: child = pty.slave.spawn_command("claude")
       ↓
Backend: tokio::spawn(zadanie odczytu)
       ↓
Backend: emit("instance-output")  ←──┐
       ↓                              │
Frontend: otrzymuje wyjście         │
       ↓                              │
Frontend: xterm.write(data) ◄───────┘
```

### Przepływ wejścia terminala

```
Użytkownik wpisuje w terminalu
       ↓
xterm: onData(data)
       ↓
Frontend: invoke('write_to_terminal', { id, input })
       ↓
Backend: ClaudeProcess::write(input)
       ↓
Backend: PTY writer.write_all(input)
       ↓
proces claude otrzymuje wejście
```

### Zakończenie instancji

```
Proces się kończy (EOF na odczycie PTY)
       ↓
Backend: aktualizuje status na Stopped
       ↓
Backend: emit("instance-terminated")
       ↓
Frontend: aktualizuje UI
```

## Konfiguracja

### Lokalizacja pliku

- **macOS/Linux**: `~/.claude-manager/instances.json`
- **Windows**: `%APPDATA%\claude-manager\instances.json`

### Schemat

```json
{
    "instances": [
        {
            "id": "uuid",
            "name": "Nazwa Projektu",
            "folder_path": "/ścieżka/do/projektu",
            "autostart": true,
            "created_at": "2024-01-01T00:00:00Z"
        }
    ]
}
```

## Uwagi międzyplatformowe

### Implementacja PTY

Używanie karty `portable-pty` do obsługi PTY na wielu platformach:

- **Linux/macOS**: Używa `forkpty`
- **Windows**: Używa ConPTY

### Obsługa ścieżek

- Backend używa crate `dirs` do uzyskania katalogu konfiguracji niezależnego od platformy
- Frontend używa `@tauri-apps/plugin-dialog` do wyboru folderu

## Uwagi dotyczące bezpieczeństwa

1. **Wstrzykiwanie komend**: Wszystkie komendy shella są uruchamiane przez PTY, nie przez interpretację przez shell
2. **Walidacja ścieżek**: Ścieżki folderów są walidowane przed utworzeniem instancji
3. **Czyszczenie procesów**: Wszystkie procesy są zatrzymywane przy wyjściu z aplikacji

## Uwagi dotyczące wydajności

1. **Buforowanie wyjścia**: Wyjście terminala jest strumieniowane natychmiast, nie buforowane
2. **Wejście/wyjście asynchroniczne**: Wszystkie operacje I/O używają runtime asynchroniczny tokio
3. **Debouncing UI**: Aktualizacje UI są grupowane w pętli zdarzeń

## Planowane ulepszenia

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
