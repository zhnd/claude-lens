'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { SessionData } from "@/lib/api"
import { formatDuration } from "@/lib/utils"
import { format } from 'date-fns'
import { Clock, User, Terminal } from 'lucide-react'

interface SessionsTableProps {
  sessions: SessionData[]
  title?: string
}

export function SessionsTable({ sessions, title = "Recent Sessions" }: SessionsTableProps) {
  const getStatusVariant = (status: SessionData['status']) => {
    switch (status) {
      case 'Active':
        return 'success'
      case 'Completed':
        return 'default'
      case 'Terminated':
        return 'destructive'
      default:
        return 'secondary'
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Terminal className="h-5 w-5" />
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {sessions.length === 0 ? (
            <div className="text-center text-muted-foreground py-8">
              No sessions found
            </div>
          ) : (
            sessions.map((session) => (
              <div
                key={session.id}
                className="flex items-center justify-between p-4 border rounded-lg hover:bg-muted/50 transition-colors"
              >
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <User className="h-4 w-4 text-muted-foreground" />
                    <span className="font-medium">{session.user_id}</span>
                    <Badge variant={getStatusVariant(session.status)}>
                      {session.status}
                    </Badge>
                  </div>
                  
                  <div className="flex items-center gap-4 text-sm text-muted-foreground">
                    <div className="flex items-center gap-1">
                      <Clock className="h-3 w-3" />
                      <span>Started: {format(new Date(session.start_time), 'MMM dd, HH:mm')}</span>
                    </div>
                    
                    {session.duration_seconds && (
                      <div className="flex items-center gap-1">
                        <span>Duration: {formatDuration(session.duration_seconds)}</span>
                      </div>
                    )}
                    
                    <div className="flex items-center gap-1">
                      <Terminal className="h-3 w-3" />
                      <span>{session.command_count} commands</span>
                    </div>
                  </div>
                  
                  {session.tool_usage.length > 0 && (
                    <div className="flex gap-1 flex-wrap mt-2">
                      {session.tool_usage.slice(0, 3).map((tool) => (
                        <Badge key={tool.tool_name} variant="outline" className="text-xs">
                          {tool.tool_name}: {tool.usage_count}
                        </Badge>
                      ))}
                      {session.tool_usage.length > 3 && (
                        <Badge variant="outline" className="text-xs">
                          +{session.tool_usage.length - 3} more
                        </Badge>
                      )}
                    </div>
                  )}
                </div>
                
                <div className="text-xs text-muted-foreground">
                  ID: {session.id.slice(0, 8)}...
                </div>
              </div>
            ))
          )}
        </div>
      </CardContent>
    </Card>
  )
}