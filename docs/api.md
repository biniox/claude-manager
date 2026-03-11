# Dokumentacja API

Dokument opisuje API między frontendem a backendem Claude Manager.

## Spis treści

- [Komendy Tauri](#komendy-tauri)
- [Eventy](#eventy)
- [Typy](#typy)

## Komendy Tauri

Te komendy są wywoływane z frontendu przy użyciu `invoke()` z `@tauri-apps/api/core`.

### Zarządzanie instancjami

#### `create_instance`

Tworzy nową konfigurację instancji.

**Parametry:**

```typescript
{
    name: string // Nazwa wyświetlana dla instancji
    folder_path: string // Ścieżka do katalogu projektu
    autostart: boolean // Czy automatycznie uruchamiać tę instancję
}
```

**Zwraca:**

```typescript
{
    id: string // Unikalne UUID
    name: string
    folder_path: string
    autostart: boolean
    created_at: string // Znacznik czasu ISO 8601
}
```

**Błędy:**

- `InstanceError` - Jeśli ścieżka folderu nie istnieje lub tworzenie instancji nie powiodło się

---

#### `get_instances`

Pobiera wszystkie skonfigurowane instancje.

**Parametry:** Brak

**Zwraca:** `Instance[]`

---

#### `delete_instance_command`

Usuwa konfigurację instancji i zatrzymuje jej proces jeśli jest uruchomiony.

**Parametry:**

```typescript
{
    id: string // ID instancji do usunięcia
}
```

**Zwraca:** `void`

---

#### `update_instance_command`

Aktualizuje istniejącą konfigurację instancji.

**Parametry:**

```typescript
{
  id: string;              // ID instancji do aktualizacji
  name?: string;           // Nowa nazwa (opcjonalne)
  folder_path?: string;    // Nowa ścieżka folderu (opcjonalne)
  autostart?: boolean;     // Nowe ustawienie autostart (opcjonalne)
}
```

**Zwraca:** `Instance` - Zaktualizowana instancja

---

### Kontrola procesów

#### `start_instance`

Uruchamia instancję CloudCode w określonym folderze.

**Parametry:**

```typescript
{
    id: string // ID instancji do uruchomienia
}
```

**Zwraca:** `void`

**Błędy:**

- `InstanceError::AlreadyRunning` - Jeśli instancja jest już uruchomiona
- `InstanceError::NotFound` - Jeśli instancja nie istnieje

---

#### `stop_instance`

Zatrzymuje uruchomioną instancję CloudCode.

**Parametry:**

```typescript
{
    id: string // ID instancji do zatrzymania
}
```

**Zwraca:** `void`

---

#### `restart_instance`

Restartuje uruchomioną instancję (zatrzymuje i ponownie uruchamia).

**Parametry:**

```typescript
{
    id: string // ID instancji do restartu
}
```

**Zwraca:** `void`

---

### Wejście/wyjście terminala

#### `write_to_terminal`

Wysyła dane wejściowe do PTY terminala instancji.

**Parametry:**

```typescript
{
    id: string // ID instancji
    input: string // Tekst do wpisania (zawiera specjalne klawisze)
}
```

**Zwraca:** `void`

---

#### `resize_terminal`

Zmienia rozmiar PTY terminala.

**Parametry:**

```typescript
{
    id: string // ID instancji
    rows: number // Liczba wierszy
    cols: number // Liczba kolumn
}
```

**Zwraca:** `void`

---

#### `get_instance_status`

Pobiera aktualny status instancji.

**Parametry:**

```typescript
{
    id: string // ID instancji
}
```

**Zwraca:**

```typescript
{
  id: string;
  status: InstanceStatus;  // "running" | "stopped" | "error"
  output_buffer?: string;
}
```

---

#### `get_autostart_instances_command`

Pobiera wszystkie instancje skonfigurowane do autostartu.

**Parametry:** Brak

**Zwraca:** `Instance[]` - Tylko instancje z `autostart: true`

---

#### `start_autostart_instances`

Uruchamia wszystkie instancje skonfigurowane do autostartu.

**Parametry:** Brak

**Zwraca:** `void`

---

## Eventy

Te eventy są emitowane z backendu i nasłuchiwane przez frontend przy użyciu `listen()` z `@tauri-apps/api/event`.

### `instance-output`

Emitowany gdy terminal produkuje wyjście.

**Payload:**

```typescript
{
    instance_id: string
    data: string // Surowe wyjście z PTY
}
```

**Użycie:**

```typescript
const unlisten = await listen<OutputEvent>('instance-output', (event) => {
    console.log('Otrzymano wyjście:', event.payload.data)
})
```

---

### `instance-terminated`

Emitowany gdy proces instancji się kończy.

**Payload:**

```typescript
{
    instance_id: string
    data: string // "[Proces zakończony]"
}
```

---

## Typy

### InstanceStatus

```typescript
enum InstanceStatus {
    Stopped = 'stopped',
    Running = 'running',
    Error = 'error'
}
```

### Instance

```typescript
interface Instance {
    id: string // UUID
    name: string // Nazwa wyświetlana
    folder_path: string // Bezwzględna ścieżka do folderu projektu
    autostart: boolean // Automatyczne uruchamianie przy starcie aplikacji
    created_at: string // Znacznik czasu ISO 8601
}
```

### OutputEvent

```typescript
interface OutputEvent {
    instance_id: string
    data: string
}
```

### InstanceStatusInfo

```typescript
interface InstanceStatusInfo {
    id: string
    status: InstanceStatus
    output_buffer?: string
}
```
