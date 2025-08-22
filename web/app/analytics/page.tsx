'use client'

import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { TimeRangeSelector } from "@/components/dashboard/time-range-selector"
import { 
  ModelCostChart,
  SessionDurationHistogram,
  CostProgressRing
} from "@/components/charts"
import { 
  useModelCostComparison,
  useBudgetProgress,
  useAdvancedToolEfficiency,
  useSessionDurationDistribution,
  useCodeGenerationStats
} from "@/hooks/use-analytics"
import { 
  BarChart3,
  PieChart,
  TrendingUp,
  Clock,
  Code2,
  ArrowLeft,
  RefreshCw,
  AlertTriangle,
  DollarSign
} from 'lucide-react'
import Link from 'next/link'
import { cn } from '@/lib/utils'

export default function AnalyticsPage() {
  const [timeRange, setTimeRange] = useState('30d')
  
  // Fetch all advanced analytics data
  const modelCosts = useModelCostComparison({ range: timeRange })
  const budgetProgress = useBudgetProgress()
  const toolEfficiency = useAdvancedToolEfficiency({ range: timeRange })
  const sessionDuration = useSessionDurationDistribution({ range: timeRange })
  const codeGenStats = useCodeGenerationStats({ range: timeRange })

  const handleRefreshAll = () => {
    modelCosts.refetch()
    budgetProgress.refetch()
    toolEfficiency.refetch()
    sessionDuration.refetch()
    codeGenStats.refetch()
  }

  const hasError = modelCosts.error || budgetProgress.error || toolEfficiency.error || 
                   sessionDuration.error || codeGenStats.error
  const isLoading = modelCosts.loading || budgetProgress.loading || toolEfficiency.loading || 
                    sessionDuration.loading || codeGenStats.loading

  if (hasError) {
    return (
      <div className="container mx-auto p-6">
        <Card className="border-destructive/50 bg-destructive/5">
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-destructive">
              <AlertTriangle className="h-5 w-5" />
              Error Loading Analytics
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 text-sm">
              {modelCosts.error && <p>Model Costs: {modelCosts.error}</p>}
              {budgetProgress.error && <p>Budget Progress: {budgetProgress.error}</p>}
              {toolEfficiency.error && <p>Tool Efficiency: {toolEfficiency.error}</p>}
              {sessionDuration.error && <p>Session Duration: {sessionDuration.error}</p>}
              {codeGenStats.error && <p>Code Generation: {codeGenStats.error}</p>}
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
        <div className="flex items-center gap-4">
          <Button variant="outline" size="sm" asChild>
            <Link href="/">
              <ArrowLeft className="h-4 w-4 mr-2" />
              Back to Dashboard
            </Link>
          </Button>
          <div>
            <h1 className="text-3xl font-bold">Advanced Analytics</h1>
            <p className="text-muted-foreground">
              Deep insights into your Claude Code usage patterns and efficiency
            </p>
          </div>
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
        </div>
      </div>

      {/* Cost Analysis Section */}
      <div className="space-y-4">
        <div className="flex items-center gap-2">
          <DollarSign className="h-5 w-5 text-muted-foreground" />
          <h2 className="text-xl font-semibold">Cost Analysis</h2>
        </div>
        
        <div className="grid gap-6 lg:grid-cols-3">
          <div className="lg:col-span-2">
            <ModelCostChart
              data={modelCosts.data ?? { models: [], total_cost: 0, period: timeRange }}
              loading={modelCosts.loading}
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
      </div>

      {/* Usage Patterns Section */}
      <div className="space-y-4">
        <div className="flex items-center gap-2">
          <Clock className="h-5 w-5 text-muted-foreground" />
          <h2 className="text-xl font-semibold">Usage Patterns</h2>
        </div>
        
        <div className="grid gap-6 lg:grid-cols-2">
          <SessionDurationHistogram
            data={sessionDuration.data ?? {
              total_sessions: 0,
              avg_duration_minutes: 0,
              median_duration_minutes: 0,
              distribution_buckets: []
            }}
            loading={sessionDuration.loading}
          />
          
          {/* Tool Efficiency Summary */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5" />
                Tool Efficiency Overview
              </CardTitle>
            </CardHeader>
            <CardContent>
              {toolEfficiency.loading ? (
                <div className="space-y-3">
                  {Array.from({ length: 4 }, (_, i) => (
                    <div key={i} className="h-16 bg-muted rounded animate-pulse" />
                  ))}
                </div>
              ) : (
                <div className="space-y-4">
                  <div className="text-center p-4 rounded-lg bg-muted/50">
                    <div className="text-2xl font-bold text-primary">
                      {toolEfficiency.data?.overall_efficiency_score.toFixed(1) ?? '0.0'}
                    </div>
                    <div className="text-sm text-muted-foreground">Overall Efficiency Score</div>
                  </div>
                  
                  <div className="space-y-2">
                    {toolEfficiency.data?.tools.slice(0, 4).map((tool, index) => (
                      <div key={tool.tool_name} className="flex items-center justify-between p-3 rounded-md border">
                        <div>
                          <div className="font-medium">{tool.tool_name}</div>
                          <div className="text-sm text-muted-foreground">
                            {tool.usage_count} uses • {tool.success_rate.toFixed(1)}% success
                          </div>
                        </div>
                        <div className="text-right">
                          <Badge 
                            variant={tool.efficiency_score > 8 ? "default" : 
                                   tool.efficiency_score > 6 ? "secondary" : "outline"}
                          >
                            {tool.efficiency_score.toFixed(1)}
                          </Badge>
                          <div className="text-xs text-muted-foreground mt-1">
                            {tool.time_saved_estimate_hours.toFixed(1)}h saved
                          </div>
                        </div>
                      </div>
                    )) ?? []}
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Code Generation Section */}
      <div className="space-y-4">
        <div className="flex items-center gap-2">
          <Code2 className="h-5 w-5 text-muted-foreground" />
          <h2 className="text-xl font-semibold">Code Generation Statistics</h2>
        </div>
        
        <div className="grid gap-6 lg:grid-cols-2">
          {/* Code Generation Summary */}
          <Card>
            <CardHeader>
              <CardTitle>Generation Overview</CardTitle>
            </CardHeader>
            <CardContent>
              {codeGenStats.loading ? (
                <div className="space-y-3">
                  {Array.from({ length: 4 }, (_, i) => (
                    <div key={i} className="h-8 bg-muted rounded animate-pulse" />
                  ))}
                </div>
              ) : (
                <div className="space-y-6">
                  <div className="grid grid-cols-2 gap-4">
                    <div className="text-center p-3 rounded-lg bg-muted/50">
                      <div className="text-lg font-bold">
                        {codeGenStats.data?.total_code_files_generated ?? 0}
                      </div>
                      <div className="text-xs text-muted-foreground">Files Generated</div>
                    </div>
                    <div className="text-center p-3 rounded-lg bg-muted/50">
                      <div className="text-lg font-bold">
                        {codeGenStats.data?.total_lines_generated.toLocaleString() ?? '0'}
                      </div>
                      <div className="text-xs text-muted-foreground">Lines of Code</div>
                    </div>
                  </div>
                  
                  <div className="text-center p-3 rounded-lg bg-muted/50">
                    <div className="text-lg font-bold">
                      {codeGenStats.data?.avg_lines_per_file.toFixed(1) ?? '0.0'}
                    </div>
                    <div className="text-xs text-muted-foreground">Avg Lines per File</div>
                  </div>
                  
                  {codeGenStats.data?.code_quality_metrics && (
                    <div>
                      <h4 className="font-medium mb-2">Quality Metrics</h4>
                      <div className="space-y-2 text-sm">
                        <div className="flex justify-between">
                          <span>Avg File Size:</span>
                          <span>{codeGenStats.data.code_quality_metrics.avg_file_size_kb.toFixed(1)} KB</span>
                        </div>
                        <div className="flex justify-between">
                          <span>Complexity Score:</span>
                          <span>{codeGenStats.data.code_quality_metrics.avg_complexity_score.toFixed(1)}/10</span>
                        </div>
                        <div className="flex justify-between">
                          <span>Estimated Bug Rate:</span>
                          <span>{(codeGenStats.data.code_quality_metrics.estimated_bug_rate * 100).toFixed(2)}%</span>
                        </div>
                        <div className="flex justify-between">
                          <span>Readability Score:</span>
                          <span>{codeGenStats.data.code_quality_metrics.readability_score.toFixed(1)}/10</span>
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </CardContent>
          </Card>
          
          {/* Language Distribution */}
          <Card>
            <CardHeader>
              <CardTitle>Language Distribution</CardTitle>
            </CardHeader>
            <CardContent>
              {codeGenStats.loading ? (
                <div className="space-y-3">
                  {Array.from({ length: 5 }, (_, i) => (
                    <div key={i} className="h-12 bg-muted rounded animate-pulse" />
                  ))}
                </div>
              ) : (
                <div className="space-y-3">
                  {codeGenStats.data?.most_generated_languages.map((lang, index) => (
                    <div key={lang.language} className="flex items-center justify-between p-3 rounded-md border">
                      <div className="flex items-center gap-3">
                        <div
                          className="w-4 h-4 rounded-full"
                          style={{ backgroundColor: lang.color }}
                        />
                        <div>
                          <div className="font-medium">{lang.language}</div>
                          <div className="text-sm text-muted-foreground">
                            {lang.file_count} files
                          </div>
                        </div>
                      </div>
                      <div className="text-right">
                        <div className="font-medium">{lang.percentage.toFixed(1)}%</div>
                        <div className="text-sm text-muted-foreground">
                          {lang.line_count.toLocaleString()} lines
                        </div>
                      </div>
                    </div>
                  )) ?? []}
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}