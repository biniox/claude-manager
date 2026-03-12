# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Development Commands

### Frontend (Node/pnpm)

- `pnpm dev` - Start Vite dev server (use with `pnpm tauri dev`)
- `pnpm build` - TypeScript compilation + Vite build
- `pnpm lint` - Run ESLint with --max-warnings 0
- `pnpm format` - Run Prettier to format code

### Tauri (Desktop App)

- `pnpm tauri dev` - Run full app in development mode (frontend + backend hot reload)
- `pnpm tauri build` - Build production bundle for current platform
- `pnpm tauri icon <path>` - Update app icon

### Rust Backend

- `cd src-tauri && cargo check` - Quick compile check without binary
- `cd src-tauri && cargo test` - Run Rust tests (if any)
- `cd src-tauri && cargo clippy` - Linter for Rust code

---

## High-Level Architecture

Claude Manager is a Tauri 2 application with a React/TypeScript frontend and Rust backend. The app manages multiple CloudCode CLI instances, each running in its own PTY (pseudo-terminal) for full shell interaction.

### Data Flow

```
┌─────────────┐    invoke()    ┌──────────────────┐    spawn()    ┌──────────────┐
│   React     │ ──────────────> │   Tauri Commands │ ───────────> │  PTY Process │
│ Component   │                │   (commands.rs)  │              │ (claude CLI) │
└─────────────┘                └──────────────────┘              └──────────────┘
       │                                │                                │
       │                         ProcessManager                        │
       │                          (singleton)                          │
       │                                │                                │
       │                          tokio::spawn                         │
       │                         (async task)                          │
       │                                │                                │
       │ <───────────────────── emit() <──────────────────────────────│
       │       (instance-output / instance-terminated events)
       ▼
┌────────────────────┐
│  XtermWrapper      │
│  (xterm.js)        │
│  - listen() events │
│  - terminal.write()│
└────────────────────┘
```

### Key Architectural Decisions

1. **Terminal Output Streaming**: Output is streamed directly from backend to `XtermWrapper` via Tauri events, bypassing the Zustand store. This avoids unnecessary state updates and buffer management complexity.

2. **PTY Handle Separation**: Due to trait conflicts in `portable-pty`, the PTY handles are stored separately:
    - `master_writer: Box<dyn Write + Send>` - for writing user input
    - `master_resize: Box<dyn MasterPty + Send>` - for resizing
    - `child: Box<dyn Child + Send>` - for process control

3. **Process Manager Singleton**: A single `ProcessManager` manages all running instances using a `HashMap<String, ClaudeProcess>`. This is initialized on app startup and ensures proper cleanup on window close.

4. **Event-Driven Status Updates**: Instance status changes (e.g., process termination) are propagated via Tauri events (`instance-terminated`), which the frontend listens to and updates the Zustand store accordingly.

---

## File Structure Highlights

### Frontend (React)

- `src/app/routes/home.tsx` - Main layout with sidebar (instance list) and terminal area
- `src/features/terminal/xterm-wrapper.tsx` - xterm.js integration, handles `instance-output` and `instance-terminated` events directly
- `src/features/instances/store/instances-store.ts` - Zustand store for instance CRUD and process control commands
- `src/lib/tauri-commands.ts` - TypeScript types matching Rust backend (source of truth for types)

### Backend (Rust)

- `src-tauri/src/main.rs` - Entry point, ProcessManager initialization, command registration, window close listener
- `src-tauri/src/commands.rs` - All Tauri commands exposed to frontend (CRUD, process control, terminal I/O, autostart)
- `src-tauri/src/process_manager.rs` - `ProcessManager` singleton, `ClaudeProcess` struct, PTY management
- `src-tauri/src/config.rs` - Configuration persistence (`~/.claude-manager/instances.json`)
- `src-tauri/src/types.rs` - Serde types shared with frontend

---

## TypeScript/Rust Type Synchronization

The frontend types in `src/lib/tauri-commands.ts` must match the Rust types in `src-tauri/src/types.rs`. When changing one, update the other:

- `InstanceStatus` enum: Rust `Stopped`, `Running`, `Error` → TS `'stopped'`, `'running'`, `'error'`
- `Instance` struct: fields map 1:1
- `OutputEvent`, `InstanceStatusInfo`: shared across both languages

---

## Adding New Tauri Commands

When adding a new Tauri command:

1. **Backend** (`src-tauri/src/commands.rs`):

    ```rust
    #[tauri::command]
    async fn my_command(param: String) -> Result<MyResult, InstanceError> {
        // implementation
        Ok(result)
    }
    ```

2. **Main.rs** (`src-tauri/src/main.rs`):
    - Register command: `.invoke_handler(tauri::generate_handler![..., my_command])`

3. **Frontend Types** (`src/lib/tauri-commands.ts`):

    ```typescript
    export type MyCommandResult = MyResult
    ```

4. **Permissions** (`src-tauri/capabilities/migrated.json`):
    - Add command to `allow` list if using Tauri 2 capability system

---

## Tauri Events

Backend → Frontend events used in this app:

| Event                 | Triggered When | Payload                                 |
| --------------------- | -------------- | --------------------------------------- |
| `instance-output`     | PTY emits data | `{ instance_id: string, data: string }` |
| `instance-terminated` | Process exits  | `{ instance_id: string, data: string }` |

To emit events from Rust:

```rust
app.emit_all("instance-output", OutputEvent { instance_id: id.clone(), data: chunk })?;
```

To listen from frontend:

```typescript
await listen<OutputEvent>('instance-output', (event) => {
    // handle event.payload
})
```

---

## Dark Purple Theme

The app uses a dark purple theme with these key colors:

- Background: `#0f0a1a` (very dark purple)
- Foreground: `#f1e6ff` (light purple-white)
- Primary: `#a855f7` (purple)
- Sidebar/panels: `#2d1b3e`, `#3d2352`, `#251a35`

Note: Tailwind CSS v4 uses hex values directly (no oklch due to compatibility issues). The `dark` class is on the `<html>` element.

---

## Common Patterns

### Creating a new Shadcn UI component:

```bash
npx shadcn@latest add <component-name>
```

Then import from `@/components/ui/<component-name>`

### Using the Zustand store:

```typescript
const { instances, createInstance, selectInstance } = useInstancesStore()
await createInstance(name, folderPath, autostart)
```

### Adding a new feature module:

Create under `src/features/<feature>/` with:

- `components/` - UI components
- `store/` - Zustand store (if needed)
- `types/` - TypeScript types
- `index.tsx` - Public exports

---

## Debugging Terminal Issues

If terminal shows black screen or no output:

1. Check backend logs in Tauri console for PTY spawn errors
2. Verify `claude` CLI is in system PATH
3. Check `instance-output` event is being emitted from `process_manager.rs`
4. Check `XtermWrapper` is listening for events and `instanceId` matches
5. There's a `console.log('Terminal output:', data)` in `xterm-wrapper.tsx` for debugging

---

## Configuration File Location

Instance configurations are stored in:

- **macOS/Linux**: `~/.claude-manager/instances.json`
- **Windows**: `%APPDATA%\claude-manager\instances.json`

Use this path to inspect or manually edit instances when debugging.
