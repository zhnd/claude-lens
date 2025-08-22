export interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
  timestamp: string
}

export interface MetricPoint {
  timestamp: string
  name: string
  value: number
  labels: Record<string, string>
}

export interface MetricsOverview {
  total_sessions: number
  active_sessions: number
  total_commands: number
  avg_session_duration: number
  top_tools: ToolUsage[]
  recent_activity: MetricPoint[]
}

export interface ToolUsage {
  name: string
  count: number
  percentage: number
}

export interface TimelineData {
  range: string
  points: MetricPoint[]
  summary: TimelineSummary
}

export interface TimelineSummary {
  total_points: number
  avg_value: number
  min_value: number
  max_value: number
}

export interface SessionData {
  id: string
  user_id: string
  start_time: string
  end_time: string | null
  duration_seconds: number | null
  command_count: number
  tool_usage: SessionToolUsage[]
  status: 'Active' | 'Completed' | 'Terminated'
}

export interface SessionToolUsage {
  tool_name: string
  usage_count: number
}

export interface SessionsResponse {
  sessions: SessionData[]
  total_count: number
  page_info: PageInfo
}

export interface PageInfo {
  has_next: boolean
  has_prev: boolean
  current_page: number
  total_pages: number
}

// Dashboard Analytics Types
export interface DashboardKPIs {
  today_sessions: number
  today_sessions_change: number
  total_tokens: number
  total_tokens_change: number
  total_cost: number
  total_cost_change: number
  lines_of_code: number
  lines_of_code_change: number
  period: string
}

export interface TokenTrendPoint {
  timestamp: string
  input_tokens: number
  output_tokens: number
  cache_creation_tokens: number
  cache_read_tokens: number
  total_tokens: number
}

export interface TokenTrendData {
  range: string
  data_points: TokenTrendPoint[]
}

export interface ToolUsageStats {
  tool_name: string
  usage_count: number
  success_rate: number
  avg_duration_ms: number
  percentage: number
  color: string
}

export interface ToolUsageData {
  total_tool_calls: number
  tools: ToolUsageStats[]
}

export interface HeatmapCell {
  hour: number
  day_of_week: number
  intensity: number
  session_count: number
  token_count: number
}

export interface UsageHeatmapData {
  timezone: string
  heatmap: HeatmapCell[]
}

// Advanced Analytics Types
export interface ModelCostComparisonItem {
  model_name: string
  cost_per_session: number
  total_sessions: number
  total_cost: number
  avg_input_tokens: number
  avg_output_tokens: number
  efficiency_score: number
  color: string
}

export interface ModelCostComparison {
  models: ModelCostComparisonItem[]
  total_cost: number
  period: string
}

export interface DailyCostBreakdown {
  date: string
  cost: number
  sessions: number
  tokens: number
}

export interface BudgetProgressData {
  current_month_cost: number
  monthly_budget: number
  percentage_used: number
  days_remaining: number
  projected_month_end_cost: number
  is_over_budget: boolean
  daily_breakdown: DailyCostBreakdown[]
}

export interface TrendDirection {
  type: 'Increasing' | 'Decreasing' | 'Stable'
  percentage?: number
}

export interface AdvancedToolStats {
  tool_name: string
  usage_count: number
  success_rate: number
  avg_duration_ms: number
  median_duration_ms: number
  efficiency_score: number
  time_saved_estimate_hours: number
  cost_per_use: number
  trend: TrendDirection
}

export interface EfficiencyTimePoint {
  timestamp: string
  overall_score: number
  top_tool_score: number
}

export interface AdvancedToolEfficiency {
  overall_efficiency_score: number
  tools: AdvancedToolStats[]
  efficiency_over_time: EfficiencyTimePoint[]
}

export interface DurationBucket {
  min_minutes: number
  max_minutes: number
  session_count: number
  percentage: number
  label: string
}

export interface DurationTimePoint {
  timestamp: string
  avg_duration_minutes: number
  session_count: number
}

export interface SessionDurationDistribution {
  total_sessions: number
  avg_duration_minutes: number
  median_duration_minutes: number
  distribution_buckets: DurationBucket[]
  duration_over_time: DurationTimePoint[]
}

export interface LanguageStats {
  language: string
  file_count: number
  line_count: number
  percentage: number
  color: string
}

export interface GenerationTimePoint {
  timestamp: string
  files_generated: number
  lines_generated: number
}

export interface CodeQualityMetrics {
  avg_file_size_kb: number
  avg_complexity_score: number
  estimated_bug_rate: number
  readability_score: number
}

export interface CodeGenerationStats {
  total_code_files_generated: number
  total_lines_generated: number
  avg_lines_per_file: number
  most_generated_languages: LanguageStats[]
  generation_over_time: GenerationTimePoint[]
  code_quality_metrics: CodeQualityMetrics
}

// Query parameters
export interface AnalyticsQuery {
  start_time?: string
  end_time?: string
  user_email?: string
  organization_id?: string
  range?: string
}

const API_BASE = process.env.NODE_ENV === 'production' ? '/api' : 'http://localhost:3000/api'

export class ApiClient {
  private async request<T>(endpoint: string): Promise<ApiResponse<T>> {
    try {
      const response = await fetch(`${API_BASE}${endpoint}`)
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      return await response.json()
    } catch (error) {
      return {
        success: false,
        data: null,
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: new Date().toISOString()
      }
    }
  }

  private buildQuery(params: AnalyticsQuery): string {
    const searchParams = new URLSearchParams()
    if (params.start_time) searchParams.append('start_time', params.start_time)
    if (params.end_time) searchParams.append('end_time', params.end_time)
    if (params.user_email) searchParams.append('user_email', params.user_email)
    if (params.organization_id) searchParams.append('organization_id', params.organization_id)
    if (params.range) searchParams.append('range', params.range)
    return searchParams.toString()
  }

  async getHealth(): Promise<ApiResponse<{ status: string; version: string }>> {
    return this.request('/health')
  }

  async getMetricsOverview(): Promise<ApiResponse<MetricsOverview>> {
    return this.request('/metrics/overview')
  }

  async getMetricsTimeline(range: string = '24h', metricName?: string): Promise<ApiResponse<TimelineData>> {
    const params = new URLSearchParams({ range })
    if (metricName) params.append('metric_name', metricName)
    return this.request(`/metrics/timeline?${params}`)
  }

  async getSessions(limit: number = 20, offset: number = 0): Promise<ApiResponse<SessionsResponse>> {
    const params = new URLSearchParams({ 
      limit: limit.toString(), 
      offset: offset.toString() 
    })
    return this.request(`/sessions?${params}`)
  }

  async getSession(id: string): Promise<ApiResponse<SessionData>> {
    return this.request(`/sessions/${id}`)
  }

  async getSessionMetrics(id: string): Promise<ApiResponse<MetricPoint[]>> {
    return this.request(`/sessions/${id}/metrics`)
  }

  // Dashboard Analytics Methods
  async getDashboardKPIs(params: AnalyticsQuery = {}): Promise<ApiResponse<DashboardKPIs>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/dashboard/kpis${query ? '?' + query : ''}`)
  }

  async getTokenTrend(params: AnalyticsQuery = {}): Promise<ApiResponse<TokenTrendData>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/dashboard/token-trend${query ? '?' + query : ''}`)
  }

  async getToolUsage(params: AnalyticsQuery = {}): Promise<ApiResponse<ToolUsageData>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/dashboard/tool-usage${query ? '?' + query : ''}`)
  }

  async getUsageHeatmap(params: AnalyticsQuery = {}): Promise<ApiResponse<UsageHeatmapData>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/dashboard/usage-heatmap${query ? '?' + query : ''}`)
  }

  // Advanced Analytics Methods
  async getModelCostComparison(params: AnalyticsQuery = {}): Promise<ApiResponse<ModelCostComparison>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/advanced/model-costs${query ? '?' + query : ''}`)
  }

  async getBudgetProgress(params: AnalyticsQuery = {}): Promise<ApiResponse<BudgetProgressData>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/advanced/budget-progress${query ? '?' + query : ''}`)
  }

  async getAdvancedToolEfficiency(params: AnalyticsQuery = {}): Promise<ApiResponse<AdvancedToolEfficiency>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/advanced/tool-efficiency${query ? '?' + query : ''}`)
  }

  async getSessionDurationDistribution(params: AnalyticsQuery = {}): Promise<ApiResponse<SessionDurationDistribution>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/advanced/session-duration${query ? '?' + query : ''}`)
  }

  async getCodeGenerationStats(params: AnalyticsQuery = {}): Promise<ApiResponse<CodeGenerationStats>> {
    const query = this.buildQuery(params)
    return this.request(`/analytics/advanced/code-generation${query ? '?' + query : ''}`)
  }
}

export const apiClient = new ApiClient()