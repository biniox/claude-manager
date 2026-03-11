import { useEffect } from 'react'
import {
    CreateInstanceDialog,
    InstanceCard,
    InstanceControls,
    useInstancesStore
} from '@/features/instances'
import { XtermWrapper } from '@/features/terminal'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { Trash2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { InstanceStatus } from '@/features/instances/types'

export function HomePage() {
    const {
        instances,
        selectedInstanceId,
        selectInstance,
        loadInstances,
        deleteInstance,
        instanceStatuses
    } = useInstancesStore()

    // Load instances on mount
    useEffect(() => {
        loadInstances()
    }, [loadInstances])

    const selectedInstance = instances.find(
        (inst) => inst.id === selectedInstanceId
    )
    const selectedStatus = selectedInstanceId
        ? (instanceStatuses.get(selectedInstanceId) ?? InstanceStatus.Stopped)
        : InstanceStatus.Stopped

    const handleDeleteInstance = async (instanceId: string) => {
        if (confirm('Are you sure you want to delete this instance?')) {
            await deleteInstance(instanceId)
            if (selectedInstanceId === instanceId) {
                selectInstance(null)
            }
        }
    }

    return (
        <div className="flex h-screen overflow-hidden">
            {/* Sidebar */}
            <div className="w-64 flex flex-col border-r bg-background">
                <div className="p-4 space-y-4">
                    <div className="flex items-center justify-between">
                        <h1 className="text-lg font-semibold">
                            Claude Manager
                        </h1>
                    </div>
                    <CreateInstanceDialog />
                </div>

                <Separator />

                <ScrollArea className="flex-1">
                    <div className="p-2 space-y-1">
                        {instances.length === 0 ? (
                            <div className="px-3 py-4 text-sm text-muted-foreground text-center">
                                No instances yet.
                                <br />
                                Create one to get started.
                            </div>
                        ) : (
                            instances.map((instance) => {
                                const status =
                                    instanceStatuses.get(instance.id) ??
                                    InstanceStatus.Stopped
                                const isSelected =
                                    instance.id === selectedInstanceId

                                return (
                                    <div
                                        key={instance.id}
                                        className="space-y-1"
                                    >
                                        <InstanceCard
                                            name={instance.name}
                                            status={status}
                                            isSelected={isSelected}
                                            onClick={() =>
                                                selectInstance(instance.id)
                                            }
                                        />
                                        {isSelected && (
                                            <div className="flex items-center justify-between px-3 py-1">
                                                <InstanceControls
                                                    instanceId={instance.id}
                                                    status={status}
                                                />
                                                <Button
                                                    size="sm"
                                                    variant="ghost"
                                                    onClick={(e) => {
                                                        e.stopPropagation()
                                                        handleDeleteInstance(
                                                            instance.id
                                                        )
                                                    }}
                                                    className="h-7 w-7 p-0 text-muted-foreground hover:text-destructive"
                                                >
                                                    <Trash2 className="w-3.5 h-3.5" />
                                                </Button>
                                            </div>
                                        )}
                                    </div>
                                )
                            })
                        )}
                    </div>
                </ScrollArea>

                <Separator />

                <div className="p-3 text-xs text-muted-foreground">
                    {instances.length} instance
                    {instances.length !== 1 ? 's' : ''}
                </div>
            </div>

            {/* Terminal Area */}
            <div className="flex-1 flex flex-col min-w-0">
                {selectedInstance && (
                    <div className="border-b px-4 py-2 flex items-center justify-between bg-muted/50">
                        <div className="flex items-center gap-2">
                            <h2 className="font-medium">
                                {selectedInstance.name}
                            </h2>
                            <span className="text-xs text-muted-foreground">
                                {selectedInstance.folder_path}
                            </span>
                        </div>
                        <InstanceControls
                            instanceId={selectedInstance.id}
                            status={selectedStatus}
                        />
                    </div>
                )}
                <XtermWrapper instanceId={selectedInstanceId} />
            </div>
        </div>
    )
}

// Necessary for react router to lazy load.
export const Component = HomePage
