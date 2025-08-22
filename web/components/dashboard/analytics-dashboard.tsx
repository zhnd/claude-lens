'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { 
  ResponsiveContainer, 
  LineChart, 
  Line, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  Legend,
  AreaChart,
  Area,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
  ComposedChart
} from 'recharts';
import { 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  Activity, 
  GitCommit, 
  GitPullRequest,
  Code,
  Users,
  Clock,
  Target,
  Zap,
  AlertTriangle
} from 'lucide-react';

interface AnalyticsDashboardProps {
  timeRange: string;
  userFilter?: string;
  organizationFilter?: string;
}

interface ProductivityMetrics {
  total_commits: number;
  total_pull_requests: number;
  total_lines_added: number;
  total_lines_removed: number;
  files_changed: number;
  active_repositories: string[];
  productivity_trend: ProductivityPoint[];
  top_contributors: ContributorStats[];
}

interface ProductivityPoint {
  timestamp: string;
  commits: number;
  pull_requests: number;
  lines_added: number;
  lines_removed: number;
}

interface ContributorStats {
  user_email: string;
  commits: number;
  pull_requests: number;
  lines_added: number;
  lines_removed: number;
}

interface CostAnalytics {
  total_cost_usd: number;
  total_input_tokens: number;
  total_output_tokens: number;
  total_cache_creation_tokens: number;
  total_cache_read_tokens: number;
  average_cost_per_session: number;
  cost_trend: CostPoint[];
  model_breakdown: ModelCostBreakdown[];
  top_users_by_cost: UserCostStats[];
}

interface CostPoint {
  timestamp: string;
  cost_usd: number;
  input_tokens: number;
  output_tokens: number;
  cache_creation_tokens: number;
  cache_read_tokens: number;
}

interface ModelCostBreakdown {
  model_name: string;
  total_cost_usd: number;
  input_tokens: number;
  output_tokens: number;
  sessions: number;
  percentage_of_total: number;
}

interface UserCostStats {
  user_email: string;
  total_cost_usd: number;
  total_tokens: number;
  sessions: number;
  avg_cost_per_session: number;
}

interface EfficiencyMetrics {
  tokens_per_commit: number;
  cost_per_commit: number;
  tokens_per_line_of_code: number;
  cost_per_line_of_code: number;
  session_productivity_score: number;
  tool_efficiency: ToolEfficiencyStats[];
  time_to_productivity: TimeToProductivityPoint[];
}

interface ToolEfficiencyStats {
  tool_name: string;
  usage_count: number;
  success_rate: number;
  avg_duration_ms: number;
  productivity_correlation: number;
}

interface TimeToProductivityPoint {
  timestamp: string;
  session_start_to_first_commit_minutes: number;
  session_start_to_first_edit_minutes: number;
}

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884d8', '#82ca9d'];

export function AnalyticsDashboard({ timeRange, userFilter, organizationFilter }: AnalyticsDashboardProps) {
  const [productivityData, setProductivityData] = useState<ProductivityMetrics | null>(null);
  const [costData, setCostData] = useState<CostAnalytics | null>(null);
  const [efficiencyData, setEfficiencyData] = useState<EfficiencyMetrics | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedTab, setSelectedTab] = useState<'productivity' | 'costs' | 'efficiency'>('productivity');

  useEffect(() => {
    fetchAnalyticsData();
  }, [timeRange, userFilter, organizationFilter]);

  const fetchAnalyticsData = async () => {
    setLoading(true);
    try {
      const params = new URLSearchParams({
        range: timeRange,
        ...(userFilter && { user_email: userFilter }),
        ...(organizationFilter && { organization_id: organizationFilter })
      });

      const [productivityRes, costRes, efficiencyRes] = await Promise.all([
        fetch(`/api/analytics/productivity?${params}`),
        fetch(`/api/analytics/costs?${params}`),
        fetch(`/api/analytics/efficiency?${params}`)
      ]);

      const [productivity, costs, efficiency] = await Promise.all([
        productivityRes.json(),
        costRes.json(),
        efficiencyRes.json()
      ]);

      if (productivity.success) setProductivityData(productivity.data);
      if (costs.success) setCostData(costs.data);
      if (efficiency.success) setEfficiencyData(efficiency.data);
    } catch (error) {
      console.error('Failed to fetch analytics data:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (value: number) => `$${value.toFixed(2)}`;
  const formatNumber = (value: number) => value.toLocaleString();
  const formatTokens = (value: number) => `${(value / 1000).toFixed(1)}K`;

  const TrendIndicator = ({ value, label }: { value: number; label: string }) => {
    const isPositive = value > 0;
    const Icon = isPositive ? TrendingUp : TrendingDown;
    const color = isPositive ? 'text-green-600' : 'text-red-600';
    
    return (
      <div className={`flex items-center space-x-1 ${color}`}>
        <Icon className="h-4 w-4" />
        <span className="text-sm font-medium">{Math.abs(value).toFixed(1)}%</span>
        <span className="text-xs text-gray-500">{label}</span>
      </div>
    );
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Tab Navigation */}
      <div className="flex space-x-1 bg-gray-100 p-1 rounded-lg">
        {[
          { key: 'productivity', label: 'Productivity', icon: GitCommit },
          { key: 'costs', label: 'Costs & Tokens', icon: DollarSign },
          { key: 'efficiency', label: 'Efficiency', icon: Zap }
        ].map(({ key, label, icon: Icon }) => (
          <button
            key={key}
            onClick={() => setSelectedTab(key as any)}
            className={`flex items-center space-x-2 px-3 py-2 rounded-md transition-colors ${
              selectedTab === key 
                ? 'bg-white text-blue-600 shadow-sm' 
                : 'text-gray-600 hover:text-gray-900'
            }`}
          >
            <Icon className="h-4 w-4" />
            <span className="font-medium">{label}</span>
          </button>
        ))}
      </div>

      {/* Productivity Dashboard */}
      {selectedTab === 'productivity' && productivityData && (
        <div className="space-y-6">
          {/* Productivity KPIs */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Total Commits</CardTitle>
                <GitCommit className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatNumber(productivityData.total_commits)}</div>
                <TrendIndicator value={12.5} label="vs last period" />
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Pull Requests</CardTitle>
                <GitPullRequest className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatNumber(productivityData.total_pull_requests)}</div>
                <TrendIndicator value={8.3} label="vs last period" />
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Lines Added</CardTitle>
                <Code className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatNumber(productivityData.total_lines_added)}</div>
                <div className="text-xs text-muted-foreground">
                  -{formatNumber(productivityData.total_lines_removed)} removed
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Files Changed</CardTitle>
                <Activity className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatNumber(productivityData.files_changed)}</div>
                <div className="text-xs text-muted-foreground">
                  {productivityData.active_repositories.length} repositories
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Productivity Trend Chart */}
          <Card>
            <CardHeader>
              <CardTitle>Productivity Trend</CardTitle>
              <CardDescription>Commits, PRs, and lines of code over time</CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={300}>
                <ComposedChart data={productivityData.productivity_trend}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis 
                    dataKey="timestamp" 
                    tickFormatter={(value) => new Date(value).toLocaleDateString()}
                  />
                  <YAxis yAxisId="commits" orientation="left" />
                  <YAxis yAxisId="lines" orientation="right" />
                  <Tooltip 
                    labelFormatter={(value) => new Date(value).toLocaleString()}
                    formatter={(value: number, name: string) => [formatNumber(value), name]}
                  />
                  <Legend />
                  <Bar yAxisId="commits" dataKey="commits" fill="#0088FE" name="Commits" />
                  <Bar yAxisId="commits" dataKey="pull_requests" fill="#00C49F" name="Pull Requests" />
                  <Line yAxisId="lines" type="monotone" dataKey="lines_added" stroke="#FFBB28" name="Lines Added" />
                  <Line yAxisId="lines" type="monotone" dataKey="lines_removed" stroke="#FF8042" name="Lines Removed" />
                </ComposedChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>

          {/* Top Contributors */}
          <Card>
            <CardHeader>
              <CardTitle>Top Contributors</CardTitle>
              <CardDescription>Most productive team members</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {productivityData.top_contributors.map((contributor, index) => (
                  <div key={contributor.user_email} className="flex items-center justify-between">
                    <div className="flex items-center space-x-3">
                      <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                        <span className="text-sm font-medium text-blue-600">#{index + 1}</span>
                      </div>
                      <div>
                        <div className="font-medium">{contributor.user_email}</div>
                        <div className="text-sm text-muted-foreground">
                          {contributor.commits} commits • {contributor.pull_requests} PRs
                        </div>
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="text-sm font-medium">
                        +{formatNumber(contributor.lines_added)} lines
                      </div>
                      <div className="text-xs text-muted-foreground">
                        -{formatNumber(contributor.lines_removed)} removed
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Cost Dashboard */}
      {selectedTab === 'costs' && costData && (
        <div className="space-y-6">
          {/* Cost KPIs */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Total Cost</CardTitle>
                <DollarSign className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatCurrency(costData.total_cost_usd)}</div>
                <TrendIndicator value={15.2} label="vs last period" />
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Input Tokens</CardTitle>
                <Activity className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatTokens(costData.total_input_tokens)}</div>
                <div className="text-xs text-muted-foreground">
                  +{formatTokens(costData.total_cache_read_tokens)} cache read
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Output Tokens</CardTitle>
                <Activity className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatTokens(costData.total_output_tokens)}</div>
                <div className="text-xs text-muted-foreground">
                  +{formatTokens(costData.total_cache_creation_tokens)} cache creation
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Avg Cost/Session</CardTitle>
                <Target className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatCurrency(costData.average_cost_per_session)}</div>
                <TrendIndicator value={-3.7} label="efficiency gain" />
              </CardContent>
            </Card>
          </div>

          {/* Cost Trend Chart */}
          <Card>
            <CardHeader>
              <CardTitle>Cost & Token Usage Trend</CardTitle>
              <CardDescription>Cost and token consumption over time</CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={300}>
                <ComposedChart data={costData.cost_trend}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis 
                    dataKey="timestamp" 
                    tickFormatter={(value) => new Date(value).toLocaleDateString()}
                  />
                  <YAxis yAxisId="cost" orientation="left" />
                  <YAxis yAxisId="tokens" orientation="right" />
                  <Tooltip 
                    labelFormatter={(value) => new Date(value).toLocaleString()}
                    formatter={(value: number, name: string) => {
                      if (name.includes('cost')) return [formatCurrency(value), name];
                      return [formatNumber(value), name];
                    }}
                  />
                  <Legend />
                  <Area 
                    yAxisId="cost" 
                    type="monotone" 
                    dataKey="cost_usd" 
                    stackId="1"
                    stroke="#0088FE" 
                    fill="#0088FE" 
                    name="Cost (USD)"
                    fillOpacity={0.6}
                  />
                  <Line yAxisId="tokens" type="monotone" dataKey="input_tokens" stroke="#00C49F" name="Input Tokens" />
                  <Line yAxisId="tokens" type="monotone" dataKey="output_tokens" stroke="#FFBB28" name="Output Tokens" />
                </ComposedChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>

          {/* Model Breakdown & Top Users */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <Card>
              <CardHeader>
                <CardTitle>Model Cost Breakdown</CardTitle>
                <CardDescription>Cost distribution by AI model</CardDescription>
              </CardHeader>
              <CardContent>
                <ResponsiveContainer width="100%" height={250}>
                  <PieChart>
                    <Pie
                      data={costData.model_breakdown}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ model_name, percentage_of_total }) => 
                        `${model_name.split('-')[1]} (${percentage_of_total.toFixed(1)}%)`
                      }
                      outerRadius={80}
                      fill="#8884d8"
                      dataKey="total_cost_usd"
                    >
                      {costData.model_breakdown.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                      ))}
                    </Pie>
                    <Tooltip formatter={(value: number) => [formatCurrency(value), 'Cost']} />
                  </PieChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Top Users by Cost</CardTitle>
                <CardDescription>Highest cost contributors</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  {costData.top_users_by_cost.map((user, index) => (
                    <div key={user.user_email} className="flex items-center justify-between">
                      <div className="flex items-center space-x-3">
                        <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
                          <span className="text-sm font-medium text-green-600">#{index + 1}</span>
                        </div>
                        <div>
                          <div className="font-medium">{user.user_email}</div>
                          <div className="text-sm text-muted-foreground">
                            {user.sessions} sessions • {formatTokens(user.total_tokens)} tokens
                          </div>
                        </div>
                      </div>
                      <div className="text-right">
                        <div className="text-sm font-medium">
                          {formatCurrency(user.total_cost_usd)}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {formatCurrency(user.avg_cost_per_session)}/session
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      )}

      {/* Efficiency Dashboard */}
      {selectedTab === 'efficiency' && efficiencyData && (
        <div className="space-y-6">
          {/* Efficiency KPIs */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Tokens/Commit</CardTitle>
                <GitCommit className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatNumber(efficiencyData.tokens_per_commit)}</div>
                <TrendIndicator value={-8.5} label="more efficient" />
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Cost/Commit</CardTitle>
                <DollarSign className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatCurrency(efficiencyData.cost_per_commit)}</div>
                <TrendIndicator value={-12.1} label="cost reduction" />
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Productivity Score</CardTitle>
                <Target className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{efficiencyData.session_productivity_score}/10</div>
                <div className="text-xs text-muted-foreground">
                  AI-assisted efficiency rating
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">Cost/Line</CardTitle>
                <Code className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{formatCurrency(efficiencyData.cost_per_line_of_code)}</div>
                <div className="text-xs text-muted-foreground">
                  {efficiencyData.tokens_per_line_of_code.toFixed(0)} tokens/line
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Tool Efficiency */}
          <Card>
            <CardHeader>
              <CardTitle>Tool Efficiency Analysis</CardTitle>
              <CardDescription>Performance and success rates by tool</CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={300}>
                <BarChart data={efficiencyData.tool_efficiency}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="tool_name" />
                  <YAxis yAxisId="usage" orientation="left" />
                  <YAxis yAxisId="rate" orientation="right" domain={[0, 100]} />
                  <Tooltip 
                    formatter={(value: number, name: string) => {
                      if (name.includes('rate')) return [`${value.toFixed(1)}%`, name];
                      return [formatNumber(value), name];
                    }}
                  />
                  <Legend />
                  <Bar yAxisId="usage" dataKey="usage_count" fill="#0088FE" name="Usage Count" />
                  <Line yAxisId="rate" type="monotone" dataKey="success_rate" stroke="#00C49F" name="Success Rate %" />
                </BarChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>

          {/* Time to Productivity */}
          <Card>
            <CardHeader>
              <CardTitle>Time to Productivity</CardTitle>
              <CardDescription>How quickly users become productive in sessions</CardDescription>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={250}>
                <AreaChart data={efficiencyData.time_to_productivity}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis 
                    dataKey="timestamp" 
                    tickFormatter={(value) => new Date(value).toLocaleDateString()}
                  />
                  <YAxis />
                  <Tooltip 
                    labelFormatter={(value) => new Date(value).toLocaleString()}
                    formatter={(value: number, name: string) => [`${value.toFixed(1)} min`, name]}
                  />
                  <Legend />
                  <Area 
                    type="monotone" 
                    dataKey="session_start_to_first_edit_minutes" 
                    stackId="1"
                    stroke="#0088FE" 
                    fill="#0088FE" 
                    name="Time to First Edit"
                  />
                  <Area 
                    type="monotone" 
                    dataKey="session_start_to_first_commit_minutes" 
                    stackId="1"
                    stroke="#00C49F" 
                    fill="#00C49F" 
                    name="Time to First Commit"
                  />
                </AreaChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}