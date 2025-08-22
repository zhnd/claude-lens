'use client'

import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { MetricCard } from "./metric-card"
import { MetricsChart } from "./metrics-chart"
import { ToolUsageChart } from "./tool-usage-chart"
import { SessionsTable } from "./sessions-table"
import { RefreshControls } from "./refresh-controls"
import { useMetricsPolling } from "@/hooks/use-metrics"
import { formatDuration, formatNumber } from "@/lib/utils"
import { 
  Activity, 
  Users, 
  Terminal, 
  Clock,
  AlertTriangle,
  TrendingUp,
  Database,
  Cpu
} from 'lucide-react'

export function Dashboard() {
  const [autoRefresh, setAutoRefresh] = useState(true)
  const {
    overview,
    timeline,
    sessions,
    loading,
    error,
    lastUpdated,
    refresh,
    isPolling,
    startPolling,
    stopPolling,
  } = useMetricsPolling(autoRefresh, 30000) // 30 seconds interval

  const handleTogglePolling = () => {
    if (isPolling) {
      stopPolling()
      setAutoRefresh(false)
    } else {
      startPolling()
      setAutoRefresh(true)
    }
  }

  if (error) {
    return (
      <div className="container mx-auto p-6">
        <Card className="border-red-200 bg-red-50">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-red-600">
              <AlertTriangle className="h-5 w-5" />
              Error Loading Dashboard
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-red-600 mb-4">{error}</p>
            <RefreshControls
              isPolling={isPolling}
              lastUpdated={lastUpdated}
              onManualRefresh={refresh}
              onTogglePolling={handleTogglePolling}
              loading={loading}
            />
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold">Claude Scope Dashboard</h1>
          <p className="text-muted-foreground">
            Monitoring Claude Code sessions and tool usage
          </p>
        </div>
        <RefreshControls
          isPolling={isPolling}
          lastUpdated={lastUpdated}
          onManualRefresh={refresh}
          onTogglePolling={handleTogglePolling}
          loading={loading}
        />
      </div>

      {/* Metrics Overview Cards */}
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        <MetricCard
          title="Total Sessions"
          value={overview?.total_sessions ?? 0}
          description="All recorded sessions"
          icon={Users}
          status="default"
        />
        <MetricCard
          title="Active Sessions"
          value={overview?.active_sessions ?? 0}
          description="Currently running"
          icon={Activity}
          status={overview?.active_sessions ? 'success' : 'default'}
        />
        <MetricCard
          title="Total Commands"
          value={formatNumber(overview?.total_commands ?? 0)}
          description="Commands executed"
          icon={Terminal}
        />
        <MetricCard
          title="Avg Session Duration"
          value={overview ? formatDuration(overview.avg_session_duration) : '0s'}
          description="Average session length"
          icon={Clock}
        />
      </div>

      {/* Charts Row */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Timeline Chart */}
        <MetricsChart
          title="Activity Timeline (24h)"
          data={timeline?.points ?? []}
          type="line"
          height={300}
        />

        {/* Tool Usage */}
        <ToolUsageChart
          title="Top Tools Usage"
          data={overview?.top_tools ?? []}
        />
      </div>

      {/* Recent Activity */}
      {timeline && timeline.points.length > 0 && (
        <div className="grid gap-6 lg:grid-cols-3">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5" />
                Timeline Summary
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Total Points:</span>
                <span className="font-medium">{timeline.summary.total_points}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Average:</span>
                <span className="font-medium">{timeline.summary.avg_value.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Min Value:</span>
                <span className="font-medium">{timeline.summary.min_value.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Max Value:</span>
                <span className="font-medium">{timeline.summary.max_value.toFixed(2)}</span>
              </div>
            </CardContent>
          </Card>

          <Card className="lg:col-span-2">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Database className="h-5 w-5" />
                Recent Activity
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                {overview?.recent_activity.slice(0, 5).map((activity, index) => (
                  <div key={index} className="flex items-center justify-between p-2 rounded border">
                    <div>
                      <span className="font-medium">{activity.name}</span>
                      {Object.keys(activity.labels).length > 0 && (
                        <div className="text-xs text-muted-foreground">
                          {Object.entries(activity.labels).map(([key, value]) => (
                            <span key={key} className="mr-2">{key}={value}</span>
                          ))}
                        </div>
                      )}
                    </div>
                    <div className="text-right">
                      <div className="font-medium">{activity.value}</div>
                      <div className="text-xs text-muted-foreground">
                        {new Date(activity.timestamp).toLocaleTimeString()}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Sessions Table */}
      {sessions && (
        <SessionsTable
          sessions={sessions.sessions}
          title="Recent Sessions"
        />
      )}

      {/* Loading State */}
      {loading && !overview && (
        <Card>
          <CardContent className="p-8 text-center">
            <div className="flex items-center justify-center space-x-2">
              <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-primary"></div>
              <span>Loading dashboard data...</span>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}