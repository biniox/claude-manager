import { create } from 'zustand'
import type { Instance, OutputEvent } from '@/lib/tauri-commands'
import { InstanceStatus } from '@/lib/tauri-commands'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

interface InstancesState {
    // State
    instances: Instance[]
    selectedInstanceId: string | null
    instanceStatuses: Map<string, InstanceStatus>
    isLoading: boolean

    // Actions
    loadInstances: () => Promise<void>
    createInstance: (
        name: string,
        folderPath: string,
        autostart: boolean
    ) => Promise<Instance>
    updateInstance: (
        id: string,
        name?: string,
        folderPath?: string,
        autostart?: boolean
    ) => Promise<void>
    deleteInstance: (id: string) => Promise<void>
    selectInstance: (id: string | null) => void
    startInstance: (id: string) => Promise<void>
    stopInstance: (id: string) => Promise<void>
    restartInstance: (id: string) => Promise<void>
    updateStatus: (instanceId: string, status: InstanceStatus) => void
}

export const useInstancesStore = create<InstancesState>((set) => ({
    // Initial state
    instances: [],
    selectedInstanceId: null,
    instanceStatuses: new Map(),
    isLoading: false,

    // Load all instances
    loadInstances: async () => {
        set({ isLoading: true })
        try {
            const instances = await invoke<Instance[]>('get_instances')
            set({ instances, isLoading: false })

            // Get status for all instances
            const statuses = new Map<string, InstanceStatus>()
            for (const instance of instances) {
                const statusInfo = await invoke<{ status: InstanceStatus }>(
                    'get_instance_status',
                    { id: instance.id }
                )
                statuses.set(instance.id, statusInfo.status)
            }
            set({ instanceStatuses: statuses })
        } catch (error) {
            console.error('Failed to load instances:', error)
            set({ isLoading: false })
        }
    },

    // Create a new instance
    createInstance: async (
        name: string,
        folderPath: string,
        autostart: boolean
    ) => {
        const instance = await invoke<Instance>('create_instance', {
            name,
            folderPath,
            autostart
        })

        set((state) => ({
            instances: [...state.instances, instance],
            instanceStatuses: new Map(state.instanceStatuses).set(
                instance.id,
                InstanceStatus.Stopped
            )
        }))

        return instance
    },

    // Update an existing instance
    updateInstance: async (
        id: string,
        name?: string,
        folderPath?: string,
        autostart?: boolean
    ) => {
        const instance = await invoke<Instance>('update_instance_command', {
            id,
            name,
            folder_path: folderPath,
            autostart
        })

        set((state) => ({
            instances: state.instances.map((inst) =>
                inst.id === id ? instance : inst
            )
        }))
    },

    // Delete an instance
    deleteInstance: async (id: string) => {
        await invoke('delete_instance_command', { id })

        set((state) => ({
            instances: state.instances.filter((inst) => inst.id !== id),
            selectedInstanceId:
                state.selectedInstanceId === id
                    ? null
                    : state.selectedInstanceId,
            instanceStatuses: (() => {
                const newStatuses = new Map(state.instanceStatuses)
                newStatuses.delete(id)
                return newStatuses
            })()
        }))
    },

    // Select an instance
    selectInstance: (id: string | null) => {
        set({ selectedInstanceId: id })
    },

    // Start an instance
    startInstance: async (id: string) => {
        await invoke('start_instance', { id })
        set((state) => ({
            instanceStatuses: new Map(state.instanceStatuses).set(
                id,
                InstanceStatus.Running
            )
        }))
    },

    // Stop an instance
    stopInstance: async (id: string) => {
        await invoke('stop_instance', { id })
        set((state) => ({
            instanceStatuses: new Map(state.instanceStatuses).set(
                id,
                InstanceStatus.Stopped
            )
        }))
    },

    // Restart an instance
    restartInstance: async (id: string) => {
        await invoke('restart_instance', { id })
        set((state) => ({
            instanceStatuses: new Map(state.instanceStatuses).set(
                id,
                InstanceStatus.Running
            )
        }))
    },

    // Update instance status (called from termination event listener)
    updateStatus: (instanceId: string, status: InstanceStatus) => {
        set((state) => ({
            instanceStatuses: new Map(state.instanceStatuses).set(
                instanceId,
                status
            )
        }))
    }
}))

// Setup event listener for instance termination (called from xterm-wrapper)
export function setupInstanceTerminationListener() {
    return listen<OutputEvent>('instance-terminated', (event) => {
        const { instance_id } = event.payload
        useInstancesStore
            .getState()
            .updateStatus(instance_id, InstanceStatus.Stopped)
    })
}
