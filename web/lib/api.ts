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
}

export const apiClient = new ApiClient()