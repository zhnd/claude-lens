'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  PieChart,
  Pie,
  Cell,
  ResponsiveContainer,
  Tooltip,
  Legend,
} from 'recharts';

interface ToolUsageData {
  tool_name: string;
  usage_count: number;
  success_rate: number;
  avg_duration_ms: number;
  percentage: number;
  color: string;
}

interface ToolUsagePieProps {
  data: {
    total_tool_calls: number;
    tools: ToolUsageData[];
  };
  loading?: boolean;
  className?: string;
}

export function ToolUsagePie({ data, loading = false, className }: ToolUsagePieProps) {
  const formatTooltip = (value: any, name: string, props: any) => {
    const { payload } = props;
    return [
      <div key="tooltip" className="space-y-1">
        <div className="font-medium">{payload.tool_name}</div>
        <div className="text-sm text-muted-foreground">
          {payload.usage_count.toLocaleString()} uses ({Number(value).toFixed(1)}%)
        </div>
        <div className="text-sm text-muted-foreground">
          Success rate: {payload.success_rate.toFixed(1)}%
        </div>
        <div className="text-sm text-muted-foreground">
          Avg duration: {(payload.avg_duration_ms / 1000).toFixed(1)}s
        </div>
      </div>
    ];
  };

  const customLabel = ({ tool_name, percentage }: any) => {
    if (percentage < 5) return ''; // Don't show labels for small slices
    return `${tool_name}\n${percentage.toFixed(1)}%`;
  };

  if (loading) {
    return (
      <Card className={className}>
        <CardHeader>
          <div className="h-5 bg-muted rounded w-32 mb-2"></div>
          <div className="h-4 bg-muted rounded w-48"></div>
        </CardHeader>
        <CardContent>
          <div className="h-80 bg-muted rounded animate-pulse"></div>
        </CardContent>
      </Card>
    );
  }

  const topTools = data.tools.slice(0, 3);
  const totalCalls = data.total_tool_calls;

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Tool Usage Distribution</CardTitle>
            <CardDescription>
              Most frequently used tools
            </CardDescription>
          </div>
          <Badge variant="secondary">
            {totalCalls.toLocaleString()} total calls
          </Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="flex flex-col lg:flex-row gap-6">
          <div className="flex-1">
            <ResponsiveContainer width="100%" height={280}>
              <PieChart>
                <Pie
                  data={data.tools}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={customLabel}
                  outerRadius={100}
                  fill="#8884d8"
                  dataKey="percentage"
                  fontSize={12}
                >
                  {data.tools.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip
                  content={({ active, payload }) => {
                    if (active && payload && payload.length) {
                      return (
                        <div className="bg-card border rounded-lg p-3 shadow-lg">
                          {formatTooltip(payload[0].value, String(payload[0].name || ''), payload[0])}
                        </div>
                      );
                    }
                    return null;
                  }}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
          
          <div className="lg:w-64 space-y-3">
            <h4 className="font-medium text-sm text-muted-foreground">Top Tools</h4>
            {topTools.map((tool, index) => (
              <div key={tool.tool_name} className="flex items-center justify-between p-2 rounded-md border">
                <div className="flex items-center gap-2">
                  <div
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: tool.color }}
                  />
                  <div>
                    <div className="font-medium text-sm">{tool.tool_name}</div>
                    <div className="text-xs text-muted-foreground">
                      {tool.usage_count.toLocaleString()} uses
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="font-medium text-sm">{tool.percentage.toFixed(1)}%</div>
                  <div className="text-xs text-green-600 dark:text-green-400">
                    {tool.success_rate.toFixed(1)}% success
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}