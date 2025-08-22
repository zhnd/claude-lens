'use client'

import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { RefreshCw, Play, Pause } from 'lucide-react'
import { format } from 'date-fns'

interface RefreshControlsProps {
  isPolling: boolean
  lastUpdated: Date | null
  onManualRefresh: () => void
  onTogglePolling: () => void
  loading?: boolean
}

export function RefreshControls({
  isPolling,
  lastUpdated,
  onManualRefresh,
  onTogglePolling,
  loading = false,
}: RefreshControlsProps) {
  return (
    <div className="flex items-center gap-4">
      {/* Last Updated */}
      {lastUpdated && (
        <div className="text-sm text-muted-foreground">
          Last updated: {format(lastUpdated, 'HH:mm:ss')}
        </div>
      )}

      {/* Polling Status */}
      <Badge variant={isPolling ? 'success' : 'secondary'}>
        {isPolling ? 'Auto-refresh ON' : 'Auto-refresh OFF'}
      </Badge>

      {/* Controls */}
      <div className="flex gap-2">
        <Button
          size="sm"
          variant="outline"
          onClick={onManualRefresh}
          disabled={loading}
        >
          <RefreshCw className={`h-4 w-4 mr-2 ${loading ? 'animate-spin' : ''}`} />
          Refresh
        </Button>

        <Button
          size="sm"
          variant={isPolling ? 'secondary' : 'default'}
          onClick={onTogglePolling}
        >
          {isPolling ? (
            <>
              <Pause className="h-4 w-4 mr-2" />
              Stop Auto-refresh
            </>
          ) : (
            <>
              <Play className="h-4 w-4 mr-2" />
              Start Auto-refresh
            </>
          )}
        </Button>
      </div>
    </div>
  )
}