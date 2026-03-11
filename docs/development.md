# Przewodnik Dewelopera

Dokument opisuje konfigurację środowiska deweloperskiego i współpracę z projektem Claude Manager.

## Wymagania wstępne

- **Node.js** 18+ z **pnpm**
- **Rust** - stabilne narzędzia
- **Claude CLI** (`claude`) dostępne w PATH
- **Git** do zarządzania wersjami

## Konfiguracja

### 1. Klonowanie repozytorium

```bash
git clone https://github.com/your-username/claude-manager.git
cd claude-manager
```

### 2. Instalacja zależności

```bash
# Instalacja zależności frontendu
pnpm install
```

### 3. Uruchomienie serwera deweloperskiego

```bash
# Uruchamia zarówno serwer deweloperski Vite, jak i Tauri w trybie watch
pnpm tauri dev
```

To spowoduje:

- Uruchomienie serwera deweloperskiego Vite na porcie 5173
- Budowanie i uruchamianie Tauri w trybie obserwacji
- Włączony hot reload dla zmian we frontendzie
- Automatyczna przebudowa przy zmianach w Rust

## Struktura projektu

```
claude-manager/
├── src/                          # Frontend source
│   ├── app/                       # React Router app
│   ├── components/                # Współdzielone komponenty UI
│   ├── features/                  # Moduły funkcjonalności
│   │   ├── instances/             # Zarządzanie instancjami
│   │   └── terminal/             # Wrapper terminala
│   └── lib/                      # Narzędzia
├── src-tauri/                    # Backend source
│   ├── src/                       # Rust source
│   ├── capabilities/               # Uprawnienia Tauri
│   ├── Cargo.toml                 # Zależności Rust
│   └── tauri.conf.json          # Konfiguracja Tauri
├── docs/                         # Dokumentacja
├── package.json                  # Zależności frontendu
└── tauri.conf.json              # Konfiguracja Tauri
```

## Dodawanie nowej funkcjonalności

### Frontend (React + TypeScript)

#### 1. Utwórz katalog funkcjonalności

```bash
mkdir -p src/features/moja-funkcja/{components,store,types}
```

#### 2. Utwórz komponenty

```typescript
// src/features/moja-funkcja/components/MojKomponent.tsx
import { FC } from 'react';

export const MojKomponent: FC = () => {
  return <div>Moja funkcjonalność</div>;
};
```

#### 3. Utwórz store (jeśli potrzebny)

```typescript
// src/features/moja-funkcja/store/moj-store.ts
import { create } from 'zustand'

interface MojStan {
    wartosc: string
    setWartosc: (wartosc: string) => void
}

export const uzywajMojStore = create<MojStan>((set) => ({
    wartosc: '',
    setWartosc: (wartosc) => set({ wartosc })
}))
```

#### 4. Utwórz typy

```typescript
// src/features/moja-funkcja/types/index.ts
export interface MojTyp {
    id: string
    nazwa: string
}
```

#### 5. Eksportuj z indeksu funkcjonalności

```typescript
// src/features/moja-funkcja/index.tsx
export { MojKomponent } from './components/MojKomponent'
export { uzywajMojStore } from './store/moj-store'
export type { MojTyp } from './types'
```

### Backend (Rust)

#### 1. Dodaj komendę do `commands.rs`

```rust
#[tauri::command]
pub async fn moja_komenda(param: String) -> Result<String, String> {
    Ok(format!("Otrzymano: {}", param))
}
```

#### 2. Zarejestruj komendę w `main.rs`

```rust
.invoke_handler(tauri::generate_handler![
    // ...istniejące komendy...
    commands::moja_komenda,
])
```

#### 3. Dodaj typy TypeScript

```typescript
// src/lib/tauri-commands.ts
export interface ParametryMojejKomendy {
    param: string
}

export type WynikMojejKomendy = string
```

#### 4. Wywołaj z frontendu

```typescript
import { invoke } from '@tauri-apps/api/core'

const wynik = await invoke<WynikMojejKomendy>('moja_komenda', {
    param: 'cześć'
})
```

## Styl kodu

### Frontend

- Używaj **komponenty funkcyjne** z hooks
- Podążaj za wzorcami komponentów **shadcn/ui**
- Używaj **Tailwind CSS** do stylowania
- Pisz **komentarze JSDoc** dla publicznych API

```typescript
/**
 * Renderuje kartę wyświetlającą informacje o instancji
 * @param props - Właściwości komponentu
 */
export const InstanceCard: FC<InstanceCardProps> = ({
    nazwa,
    status,
    jestWybrana
}) => {
    // ...
}
```

### Backend

- Podążaj za konwencjami nazewniczymi **Rust**
- Używaj `Result<T, E>` dla funkcji które mogą zwrócić błąd
- Pisz **dokumentację** dla publicznych elementów

```rust
/// Tworzy nową konfigurację instancji
///
/// # Argumenty
///
/// * `nazwa` - Nazwa wyświetlana dla instancji
/// * `sciezka_folderu` - Ścieżka do katalogu projektu
/// * `autostart` - Czy automatycznie uruchamiać tę instancję
///
/// # Zwraca
///
/// Zwraca utworzoną instancję lub błąd
pub async fn utworz_instancje(
    nazwa: String,
    sciezka_folderu: String,
    autostart: bool,
) -> Result<Instance, InstanceError> {
    // ...
}
```

## Testowanie

### Frontend

```bash
# Uruchom linter
pnpm lint

# Formatuj kod
pnpm format
```

### Backend

```bash
# Sprawdź kod Rust
cd src-tauri
cargo check

# Uruchom testy (kiedy dodane)
cargo test

# Formatuj kod
cargo fmt
```

## Debugowanie

### Frontend

1. Otwórz **DevTools** (F12 lub Cmd+Option+I)
2. Sprawdź **Konsolę** pod kątem błędów
3. Użyj **React DevTools** do inspekcji komponentów
4. Ustaw punkty przerwania w kodzie źródłowym

### Backend

1. Ustaw zmienną środowiskową `RUST_LOG=debug`
2. Sprawdź terminal pod kątem logów wyjściowych
3. Użyj makra `dbg!()` do szybkiego debugowania

## Częste zadania

### Dodawanie nowego komponentu UI

1. Dodaj komponent shadcn:

```bash
npx shadcn@latest add nazwa-komponentu
```

2. Użyj w swoim kodzie:

```typescript
import { Button } from '@/components/ui/button';

<Button onClick={handleKlik}>Kliknij mnie</Button>
```

### Dodawanie nowego pluginu Tauri

1. Zainstaluj plugin:

```bash
cargo add tauri-plugin-nazwa
pnpm add @tauri-apps/plugin-nazwa
```

2. Zarejestruj w `main.rs`:

```rust
.plugin(plugin_init())
```

3. Dodaj do `tauri.conf.json` capabilities jeśli potrzebne

## Budowanie

### Budowa deweloperska

```bash
pnpm tauri dev
```

### Budowa produkcyjna

```bash
pnpm tauri build
```

To utworzy instalatory specyficzne dla platform w `src-tauri/target/release/bundle/`.

### Konfiguracja budowania

Edytuj `src-tauri/tauri.conf.json` dla:

- Nazwy i wersji aplikacji
- Ustawień okna
- Asocjacji plików
- Konfiguracji zasobnika systemowego

## Lista sprawdzeń przed wydaniem

- [ ] Zaktualizuj wersję w `package.json`
- [ ] Zaktualizuj wersję w `src-tauri/Cargo.toml`
- [ ] Zaktualizuj `CHANGELOG.md`
- [ ] Uruchom testy
- [ ] Zbuduj artefakty produkcyjne
- [ ] Przetestuj na wszystkich platformach docelowych
- [ ] Utwórz tag Git
- [ ] Opracuj wydanie GitHub

## Zasoby

- [Dokumentacja Tauri](https://tauri.app/)
- [Dokumentacja React](https://react.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [xterm.js](https://xtermjs.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
