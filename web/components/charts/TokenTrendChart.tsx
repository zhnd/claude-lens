'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { format } from 'date-fns';

interface TokenTrendPoint {
  timestamp: string;
  input_tokens: number;
  output_tokens: number;
  cache_creation_tokens: number;
  cache_read_tokens: number;
  total_tokens: number;
}

interface TokenTrendChartProps {
  data: TokenTrendPoint[];
  range: string;
  loading?: boolean;
  className?: string;
}

export function TokenTrendChart({ data, range, loading = false, className }: TokenTrendChartProps) {
  const formatXAxis = (tickItem: string) => {
    const date = new Date(tickItem);
    switch (range) {
      case '24h':
        return format(date, 'HH:mm');
      case '7d':
        return format(date, 'MMM dd');
      case '30d':
        return format(date, 'MMM dd');
      default:
        return format(date, 'MMM dd');
    }
  };

  const formatTooltip = (value: any, name: string) => {
    const labels: Record<string, string> = {
      input_tokens: 'Input Tokens',
      output_tokens: 'Output Tokens',
      cache_creation_tokens: 'Cache Creation',
      cache_read_tokens: 'Cache Read',
      total_tokens: 'Total Tokens',
    };
    return [Number(value).toLocaleString(), labels[name] || name];
  };

  const formatTooltipLabel = (label: string) => {
    return format(new Date(label), 'PPpp');
  };

  if (loading) {
    return (
      <Card className={className}>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <div className="h-5 bg-muted rounded w-32 mb-2"></div>
              <div className="h-4 bg-muted rounded w-48"></div>
            </div>
            <div className="flex gap-2">
              <div className="h-6 bg-muted rounded w-16"></div>
              <div className="h-6 bg-muted rounded w-16"></div>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="h-80 bg-muted rounded animate-pulse"></div>
        </CardContent>
      </Card>
    );
  }

  const totalTokens = data.reduce((sum, point) => sum + point.total_tokens, 0);
  const avgTokensPerPoint = data.length > 0 ? Math.round(totalTokens / data.length) : 0;

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Token Usage Trend</CardTitle>
            <CardDescription>
              Token consumption over time ({range})
            </CardDescription>
          </div>
          <div className="flex gap-2">
            <Badge variant="secondary">
              Total: {totalTokens.toLocaleString()}
            </Badge>
            <Badge variant="outline">
              Avg: {avgTokensPerPoint.toLocaleString()}
            </Badge>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={320}>
          <LineChart
            data={data}
            margin={{
              top: 5,
              right: 30,
              left: 20,
              bottom: 5,
            }}
          >
            <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
            <XAxis
              dataKey="timestamp"
              tickFormatter={formatXAxis}
              className="text-xs"
              stroke="currentColor"
            />
            <YAxis
              tickFormatter={(value) => value.toLocaleString()}
              className="text-xs"
              stroke="currentColor"
            />
            <Tooltip
              formatter={formatTooltip}
              labelFormatter={formatTooltipLabel}
              contentStyle={{
                backgroundColor: 'hsl(var(--card))',
                border: '1px solid hsl(var(--border))',
                borderRadius: '8px',
                fontSize: '14px',
              }}
            />
            <Legend />
            <Line
              type="monotone"
              dataKey="input_tokens"
              stroke="#8b5cf6"
              strokeWidth={2}
              dot={false}
              name="Input Tokens"
            />
            <Line
              type="monotone"
              dataKey="output_tokens"
              stroke="#06b6d4"
              strokeWidth={2}
              dot={false}
              name="Output Tokens"
            />
            <Line
              type="monotone"
              dataKey="cache_creation_tokens"
              stroke="#10b981"
              strokeWidth={2}
              dot={false}
              name="Cache Creation"
            />
            <Line
              type="monotone"
              dataKey="cache_read_tokens"
              stroke="#f59e0b"
              strokeWidth={2}
              dot={false}
              name="Cache Read"
            />
          </LineChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  );
}