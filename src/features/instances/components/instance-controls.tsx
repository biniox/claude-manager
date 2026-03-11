import { Play, Square, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { InstanceStatus } from '@/features/instances/types'
import { useInstancesStore } from '@/features/instances/store/instances-store'

interface InstanceControlsProps {
    instanceId: string
    status: InstanceStatus
}

export function InstanceControls({
    instanceId,
    status
}: InstanceControlsProps) {
    const { startInstance, stopInstance, restartInstance } = useInstancesStore()

    const isRunning = status === InstanceStatus.Running
    const isStopped = status === InstanceStatus.Stopped

    return (
        <div className="flex items-center gap-2">
            {isStopped ? (
                <Button
                    size="sm"
                    variant="default"
                    onClick={() => startInstance(instanceId)}
                    className="gap-1.5"
                >
                    <Play className="w-4 h-4" />
                    Start
                </Button>
            ) : isRunning ? (
                <Button
                    size="sm"
                    variant="destructive"
                    onClick={() => stopInstance(instanceId)}
                    className="gap-1.5"
                >
                    <Square className="w-4 h-4" />
                    Stop
                </Button>
            ) : null}

            {isRunning && (
                <Button
                    size="sm"
                    variant="outline"
                    onClick={() => restartInstance(instanceId)}
                    className="gap-1.5"
                >
                    <RefreshCw className="w-4 h-4" />
                    Restart
                </Button>
            )}
        </div>
    )
}
