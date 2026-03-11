import { useEffect, useRef, useState } from 'react'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import 'xterm/css/xterm.css'
import { listen } from '@tauri-apps/api/event'
import { OutputEvent } from '@/lib/tauri-commands'
import { invoke } from '@tauri-apps/api/core'
import { setupInstanceTerminationListener } from '@/features/instances/store/instances-store'

interface XtermWrapperProps {
    instanceId: string | null
}

export function XtermWrapper({ instanceId }: XtermWrapperProps) {
    const terminalRef = useRef<HTMLDivElement>(null)
    const terminalRefCurrent = useRef<Terminal | null>(null)
    const fitAddonRef = useRef<FitAddon | null>(null)
    const [isReady, setIsReady] = useState(false)
    const unlistenRef = useRef<(() => void) | null>(null)
    const unlistenTerminateRef = useRef<(() => void) | null>(null)

    // Initialize terminal
    useEffect(() => {
        if (!terminalRef.current || terminalRefCurrent.current) return

        const terminal = new Terminal({
            cursorBlink: true,
            fontSize: 14,
            fontFamily: 'Menlo, Monaco, "Courier New", monospace',
            theme: {
                background: '#0f0a1a',
                foreground: '#f1e6ff',
                cursor: '#a855f7',
                black: '#1a1225',
                red: '#ef4444',
                green: '#22c55e',
                yellow: '#eab308',
                blue: '#3b82f6',
                magenta: '#a855f7',
                cyan: '#06b6d4',
                white: '#f1e6ff',
                brightBlack: '#251a35',
                brightRed: '#f87171',
                brightGreen: '#4ade80',
                brightYellow: '#facc15',
                brightBlue: '#60a5fa',
                brightMagenta: '#c084fc',
                brightCyan: '#22d3ee',
                brightWhite: '#ffffff'
            },
            scrollback: 10000,
            convertEol: true
        })

        const fitAddon = new FitAddon()
        terminal.loadAddon(fitAddon)

        terminal.open(terminalRef.current)
        fitAddon.fit()

        terminalRefCurrent.current = terminal
        fitAddonRef.current = fitAddon

        // Handle user input
        terminal.onData((data) => {
            if (instanceId) {
                invoke('write_to_terminal', {
                    id: instanceId,
                    input: data
                }).catch((err) => {
                    console.error('Failed to write to terminal:', err)
                })
            }
        })

        // Handle resize
        const handleResize = () => {
            if (fitAddonRef.current && terminalRefCurrent.current) {
                fitAddonRef.current.fit()

                // Send new dimensions to backend
                const dims = terminalRefCurrent.current
                invoke('resize_terminal', {
                    id: instanceId,
                    rows: dims.rows,
                    cols: dims.cols
                }).catch(console.error)
            }
        }

        window.addEventListener('resize', handleResize)
        setIsReady(true)

        return () => {
            window.removeEventListener('resize', handleResize)
            terminal.dispose()
            terminalRefCurrent.current = null
            fitAddonRef.current = null
        }
    }, [instanceId])

    // Setup event listener for terminal output
    useEffect(() => {
        if (!isReady || !instanceId) return

        const setupListener = async () => {
            // Output listener
            const unlisten = await listen<OutputEvent>(
                'instance-output',
                (event) => {
                    const terminal = terminalRefCurrent.current
                    if (terminal && event.payload.instance_id === instanceId) {
                        console.log('Terminal output:', event.payload.data)
                        terminal.write(event.payload.data)
                    }
                }
            )
            unlistenRef.current = unlisten

            // Termination listener
            const unlistenTerminate = await setupInstanceTerminationListener()
            unlistenTerminateRef.current = unlistenTerminate
        }

        setupListener()

        return () => {
            if (unlistenRef.current) {
                unlistenRef.current()
            }
            if (unlistenTerminateRef.current) {
                unlistenTerminateRef.current()
            }
        }
    }, [isReady, instanceId])

    // Clear terminal when instance changes
    useEffect(() => {
        const terminal = terminalRefCurrent.current
        if (terminal && isReady) {
            terminal.clear()
        }
    }, [instanceId, isReady])

    // Focus terminal on mount and when instance changes
    useEffect(() => {
        const terminal = terminalRefCurrent.current
        if (terminal && isReady && instanceId) {
            // Small delay to ensure terminal is fully rendered
            setTimeout(() => {
                terminal.focus()
            }, 100)
        }
    }, [isReady, instanceId])

    return (
        <div className="flex-1 h-full bg-[#0f0a1a] overflow-hidden">
            {instanceId ? (
                <div ref={terminalRef} className="h-full w-full" />
            ) : (
                <div className="flex items-center justify-center h-full text-gray-500">
                    <div className="text-center">
                        <p className="text-lg mb-2">No instance selected</p>
                        <p className="text-sm">
                            Create or select an instance from the sidebar
                        </p>
                    </div>
                </div>
            )}
        </div>
    )
}
