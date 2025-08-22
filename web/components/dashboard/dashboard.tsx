'use client'

import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { TimeRangeSelector } from "./time-range-selector"
import { 
  KPICard,
  TokenTrendChart,
  ToolUsagePie,
  UsageHeatmap,
  CostProgressRing
} from "@/components/charts"
import { 
  useDashboardKPIs,
  useTokenTrend,
  useToolUsage,
  useUsageHeatmap,
  useBudgetProgress
} from "@/hooks/use-analytics"
import { 
  Activity, 
  DollarSign, 
  Code2,
  Users,
  AlertTriangle,
  RefreshCw,
  BarChart3
} from 'lucide-react'
import Link from 'next/link'

export function Dashboard() {
  const [timeRange, setTimeRange] = useState('24h')
  
  // Fetch all dashboard data with the selected time range
  const kpis = useDashboardKPIs({ range: timeRange })
  const tokenTrend = useTokenTrend({ range: timeRange })
  const toolUsage = useToolUsage({ range: timeRange })
  const heatmap = useUsageHeatmap({ range: timeRange })
  const budgetProgress = useBudgetProgress()

  const handleRefreshAll = () => {
    kpis.refetch()
    tokenTrend.refetch()
    toolUsage.refetch()
    heatmap.refetch()
    budgetProgress.refetch()
  }

  const hasError = kpis.error || tokenTrend.error || toolUsage.error || heatmap.error || budgetProgress.error
  const isLoading = kpis.loading || tokenTrend.loading || toolUsage.loading || heatmap.loading || budgetProgress.loading

  if (hasError) {
    return (
      <div className="container mx-auto p-6">
        <Card className="border-destructive/50 bg-destructive/5">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-destructive">
              <AlertTriangle className="h-5 w-5" />
              Error Loading Dashboard
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 text-sm">
              {kpis.error && <p>KPIs: {kpis.error}</p>}
              {tokenTrend.error && <p>Token Trend: {tokenTrend.error}</p>}
              {toolUsage.error && <p>Tool Usage: {toolUsage.error}</p>}
              {heatmap.error && <p>Usage Heatmap: {heatmap.error}</p>}
              {budgetProgress.error && <p>Budget Progress: {budgetProgress.error}</p>}
            </div>
            <Button onClick={handleRefreshAll} className="mt-4" size="sm">
              <RefreshCw className="h-4 w-4 mr-2" />
              Retry
            </Button>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-3xl font-bold">Personal Analytics Dashboard</h1>
          <p className="text-muted-foreground">
            Monitor your Claude Code usage, costs, and productivity
          </p>
        </div>
        <div className="flex items-center gap-3">
          <TimeRangeSelector 
            value={timeRange} 
            onChange={setTimeRange}
            disabled={isLoading}
          />
          <Button 
            onClick={handleRefreshAll} 
            variant="outline" 
            size="sm"
            disabled={isLoading}
          >
            <RefreshCw className={`h-4 w-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          <Button asChild size="sm">
            <Link href="/analytics">
              <BarChart3 className="h-4 w-4 mr-2" />
              Advanced Analytics
            </Link>
          </Button>
        </div>
      </div>

      {/* KPI Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <KPICard
          title="Today's Sessions"
          value={kpis.data?.today_sessions ?? 0}
          change={kpis.data?.today_sessions_change}
          changeLabel="from yesterday"
          icon={<Users className="h-4 w-4" />}
          loading={kpis.loading}
        />
        <KPICard
          title="Token Usage"
          value={kpis.data?.total_tokens ?? 0}
          change={kpis.data?.total_tokens_change}
          changeLabel="from previous period"
          icon={<Activity className="h-4 w-4" />}
          loading={kpis.loading}
        />
        <KPICard
          title="Total Cost"
          value={kpis.data ? `$${kpis.data.total_cost.toFixed(2)}` : '$0.00'}
          change={kpis.data?.total_cost_change}
          changeLabel="from previous period"
          icon={<DollarSign className="h-4 w-4" />}
          loading={kpis.loading}
        />
        <KPICard
          title="Lines of Code"
          value={kpis.data?.lines_of_code ?? 0}
          change={kpis.data?.lines_of_code_change}
          changeLabel="from previous period"
          icon={<Code2 className="h-4 w-4" />}
          loading={kpis.loading}
        />
      </div>

      {/* Main Charts Row */}
      <div className="grid gap-6 lg:grid-cols-3">
        <div className="lg:col-span-2">
          <TokenTrendChart
            data={tokenTrend.data?.data_points ?? []}
            range={timeRange}
            loading={tokenTrend.loading}
          />
        </div>
        <div className="lg:col-span-1">
          <CostProgressRing
            data={budgetProgress.data ?? {
              current_month_cost: 0,
              monthly_budget: 500,
              percentage_used: 0,
              days_remaining: 15,
              projected_month_end_cost: 0,
              is_over_budget: false,
              daily_breakdown: []
            }}
            loading={budgetProgress.loading}
          />
        </div>
      </div>

      {/* Secondary Charts Row */}
      <div className="grid gap-6 lg:grid-cols-2">
        <ToolUsagePie
          data={toolUsage.data ?? { total_tool_calls: 0, tools: [] }}
          loading={toolUsage.loading}
        />
        <UsageHeatmap
          data={heatmap.data ?? { timezone: 'UTC', heatmap: [] }}
          loading={heatmap.loading}
        />
      </div>

      {/* Footer */}
      <div className="text-center text-sm text-muted-foreground">
        <p>
          Data refreshes automatically every 30 seconds. 
          <Button 
            variant="link" 
            size="sm" 
            className="px-1 h-auto font-normal"
            asChild
          >
            <Link href="/analytics">
              View detailed analytics →
            </Link>
          </Button>
        </p>
      </div>
    </div>
  )
}