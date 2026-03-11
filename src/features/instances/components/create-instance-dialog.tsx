import { useState } from 'react'
import { Plus } from 'lucide-react'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { useInstancesStore } from '@/features/instances/store/instances-store'

export function CreateInstanceDialog() {
    const [open, setOpen] = useState(false)
    const [name, setName] = useState('')
    const [folderPath, setFolderPath] = useState('')
    const [autostart, setAutostart] = useState(false)
    const createInstance = useInstancesStore((state) => state.createInstance)

    const handleSelectFolder = async () => {
        const selected = await openDialog({
            directory: true,
            multiple: false,
            title: 'Select Project Folder'
        })
        if (selected) {
            setFolderPath(selected)
            // Auto-generate name from folder if not set
            if (!name) {
                const folderName =
                    selected.split('/').pop() || selected.split('\\').pop()
                setName(folderName || 'New Instance')
            }
        }
    }

    const handleCreate = async () => {
        if (!name.trim() || !folderPath.trim()) {
            return
        }

        try {
            await createInstance(name.trim(), folderPath.trim(), autostart)
            setName('')
            setFolderPath('')
            setAutostart(false)
            setOpen(false)
        } catch (error) {
            console.error('Failed to create instance:', error)
            alert(
                'Failed to create instance. Please check the folder path and try again.'
            )
        }
    }

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button size="sm" className="w-full gap-2">
                    <Plus className="w-4 h-4" />
                    New Instance
                </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-md">
                <DialogHeader>
                    <DialogTitle>Create New Instance</DialogTitle>
                    <DialogDescription>
                        Create a new Claude instance for managing a project
                        folder.
                    </DialogDescription>
                </DialogHeader>
                <div className="space-y-4 py-4">
                    <div className="space-y-2">
                        <label htmlFor="name" className="text-sm font-medium">
                            Instance Name
                        </label>
                        <Input
                            id="name"
                            placeholder="My Project"
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                        />
                    </div>
                    <div className="space-y-2">
                        <label htmlFor="folder" className="text-sm font-medium">
                            Project Folder
                        </label>
                        <div className="flex gap-2">
                            <Input
                                id="folder"
                                placeholder="/path/to/project"
                                value={folderPath}
                                onChange={(e) => setFolderPath(e.target.value)}
                                readOnly
                                className="flex-1"
                            />
                            <Button
                                type="button"
                                variant="outline"
                                onClick={handleSelectFolder}
                            >
                                Browse
                            </Button>
                        </div>
                    </div>
                    <div className="flex items-center justify-between">
                        <div className="space-y-0.5">
                            <label
                                htmlFor="autostart"
                                className="text-sm font-medium cursor-pointer"
                            >
                                Autostart
                            </label>
                            <p className="text-xs text-muted-foreground">
                                Automatically start this instance when the app
                                opens
                            </p>
                        </div>
                        <Switch
                            id="autostart"
                            checked={autostart}
                            onCheckedChange={setAutostart}
                        />
                    </div>
                </div>
                <DialogFooter>
                    <Button variant="outline" onClick={() => setOpen(false)}>
                        Cancel
                    </Button>
                    <Button
                        onClick={handleCreate}
                        disabled={!name.trim() || !folderPath.trim()}
                    >
                        Create Instance
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}
