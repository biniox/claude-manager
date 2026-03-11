// Instance status enum
export enum InstanceStatus {
    Stopped = 'stopped',
    Running = 'running',
    Error = 'error'
}

// Instance configuration
export interface Instance {
    id: string
    name: string
    folder_path: string
    autostart: boolean
    created_at: string // ISO 8601 timestamp
}

// Instance status with output buffer
export interface InstanceStatusInfo {
    id: string
    status: InstanceStatus
    output_buffer?: string
}

// Output event payload from Tauri
export interface OutputEvent {
    instance_id: string
    data: string
}

// Tauri commands return types
export type CreateInstanceResult = Instance
export type GetInstancesResult = Instance[]
export type DeleteInstanceResult = void
export type UpdateInstanceResult = Instance
export type StartInstanceResult = void
export type StopInstanceResult = void
export type RestartInstanceResult = void
export type WriteToTerminalResult = void
export type ResizeTerminalResult = void
export type GetInstanceStatusResult = InstanceStatusInfo
export type GetAutostartInstancesResult = Instance[]
