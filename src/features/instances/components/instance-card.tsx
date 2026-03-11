import { CheckCircle2, Circle, XCircle } from 'lucide-react'
import { InstanceStatus } from '@/features/instances/types'
import { cn } from '@/lib/utils'

interface InstanceCardProps {
    name: string
    status: InstanceStatus
    isSelected: boolean
    onClick: () => void
}

const statusConfig = {
    [InstanceStatus.Running]: {
        icon: CheckCircle2,
        className: 'text-green-500',
        bgClass: 'bg-green-500/10'
    },
    [InstanceStatus.Stopped]: {
        icon: Circle,
        className: 'text-gray-400',
        bgClass: 'bg-gray-500/10'
    },
    [InstanceStatus.Error]: {
        icon: XCircle,
        className: 'text-red-500',
        bgClass: 'bg-red-500/10'
    }
}

export function InstanceCard({
    name,
    status,
    isSelected,
    onClick
}: InstanceCardProps) {
    const config = statusConfig[status]
    const StatusIcon = config.icon

    return (
        <button
            onClick={onClick}
            className={cn(
                'w-full text-left px-3 py-2.5 rounded-lg transition-all duration-200 flex items-center gap-2',
                isSelected
                    ? 'bg-primary text-primary-foreground'
                    : 'hover:bg-accent hover:text-accent-foreground'
            )}
        >
            <StatusIcon className={cn('w-4 h-4', config.className)} />
            <span className="text-sm font-medium truncate flex-1">{name}</span>
        </button>
    )
}
