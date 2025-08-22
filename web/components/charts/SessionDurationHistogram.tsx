'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from 'recharts';

interface DurationBucket {
  min_minutes: number;
  max_minutes: number;
  session_count: number;
  percentage: number;
  label: string;
}

interface SessionDurationHistogramProps {
  data: {
    total_sessions: number;
    avg_duration_minutes: number;
    median_duration_minutes: number;
    distribution_buckets: DurationBucket[];
  };
  loading?: boolean;
  className?: string;
}

const BUCKET_COLORS = [
  '#8b5cf6',
  '#06b6d4',
  '#10b981',
  '#f59e0b',
  '#ef4444',
  '#6b7280',
];

export function SessionDurationHistogram({ data, loading = false, className }: SessionDurationHistogramProps) {
  const formatTooltip = (value: any, name: string, props: any) => {
    const { payload } = props;
    return [
      <div key="tooltip" className="space-y-1">
        <div className="font-medium">{payload.label}</div>
        <div className="text-sm text-muted-foreground">
          {payload.session_count} sessions ({payload.percentage.toFixed(1)}%)
        </div>
      </div>
    ];
  };

  if (loading) {
    return (
      <Card className={className}>
        <CardHeader>
          <div className="h-5 bg-muted rounded w-48 mb-2"></div>
          <div className="h-4 bg-muted rounded w-64"></div>
        </CardHeader>
        <CardContent>
          <div className="h-80 bg-muted rounded animate-pulse"></div>
        </CardContent>
      </Card>
    );
  }

  const maxSessions = Math.max(...data.distribution_buckets.map(b => b.session_count));
  const mostCommonBucket = data.distribution_buckets.reduce((prev, current) => 
    prev.session_count > current.session_count ? prev : current
  );

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Session Duration Distribution</CardTitle>
            <CardDescription>
              How long users spend in sessions
            </CardDescription>
          </div>
          <Badge variant="secondary">
            {data.total_sessions} total sessions
          </Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-6">
          <ResponsiveContainer width="100%" height={300}>
            <BarChart
              data={data.distribution_buckets}
              margin={{
                top: 20,
                right: 30,
                left: 20,
                bottom: 5,
              }}
            >
              <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
              <XAxis
                dataKey="label"
                className="text-xs"
                stroke="currentColor"
              />
              <YAxis
                className="text-xs"
                stroke="currentColor"
              />
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
              <Bar dataKey="session_count" radius={[4, 4, 0, 0]}>
                {data.distribution_buckets.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={BUCKET_COLORS[index % BUCKET_COLORS.length]} />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="p-4 rounded-lg border text-center">
              <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                {data.avg_duration_minutes.toFixed(1)}
              </div>
              <div className="text-sm text-muted-foreground">
                Average Duration (min)
              </div>
            </div>

            <div className="p-4 rounded-lg border text-center">
              <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                {data.median_duration_minutes.toFixed(1)}
              </div>
              <div className="text-sm text-muted-foreground">
                Median Duration (min)
              </div>
            </div>

            <div className="p-4 rounded-lg border text-center">
              <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
                {mostCommonBucket.label}
              </div>
              <div className="text-sm text-muted-foreground">
                Most Common Duration
              </div>
            </div>
          </div>

          {/* Distribution breakdown */}
          <div className="space-y-2">
            <h4 className="font-medium text-sm text-muted-foreground">Duration Breakdown</h4>
            {data.distribution_buckets.map((bucket, index) => (
              <div key={bucket.label} className="flex items-center justify-between p-2 rounded-md bg-muted/50">
                <div className="flex items-center gap-3">
                  <div
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: BUCKET_COLORS[index % BUCKET_COLORS.length] }}
                  />
                  <span className="text-sm font-medium">{bucket.label}</span>
                </div>
                <div className="text-right">
                  <div className="font-medium text-sm">{bucket.session_count} sessions</div>
                  <div className="text-xs text-muted-foreground">{bucket.percentage.toFixed(1)}%</div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}