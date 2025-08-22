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

interface ModelCostItem {
  model_name: string;
  cost_per_session: number;
  total_sessions: number;
  total_cost: number;
  avg_input_tokens: number;
  avg_output_tokens: number;
  efficiency_score: number;
  color: string;
}

interface ModelCostChartProps {
  data: {
    models: ModelCostItem[];
    total_cost: number;
    period: string;
  };
  loading?: boolean;
  className?: string;
}

export function ModelCostChart({ data, loading = false, className }: ModelCostChartProps) {
  const formatTooltip = (value: any, name: string, props: any) => {
    const { payload } = props;
    return [
      <div key="tooltip" className="space-y-1">
        <div className="font-medium">{payload.model_name}</div>
        <div className="text-sm text-muted-foreground">
          Total Cost: ${payload.total_cost.toFixed(2)}
        </div>
        <div className="text-sm text-muted-foreground">
          Per Session: ${payload.cost_per_session.toFixed(2)}
        </div>
        <div className="text-sm text-muted-foreground">
          Sessions: {payload.total_sessions}
        </div>
        <div className="text-sm text-muted-foreground">
          Efficiency: ${payload.efficiency_score.toFixed(3)}/token
        </div>
      </div>
    ];
  };

  const formatModelName = (name: string) => {
    return name.split('-').slice(0, -1).join('-'); // Remove version part for display
  };

  if (loading) {
    return (
      <Card className={className}>
        <CardHeader>
          <div className="h-5 bg-muted rounded w-40 mb-2"></div>
          <div className="h-4 bg-muted rounded w-64"></div>
        </CardHeader>
        <CardContent>
          <div className="h-80 bg-muted rounded animate-pulse"></div>
        </CardContent>
      </Card>
    );
  }

  const mostExpensive = data.models.reduce((prev, current) => 
    prev.total_cost > current.total_cost ? prev : current
  );

  const mostEfficient = data.models.reduce((prev, current) => 
    prev.efficiency_score < current.efficiency_score ? prev : current
  );

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Model Cost Comparison</CardTitle>
            <CardDescription>
              Total spending by model ({data.period})
            </CardDescription>
          </div>
          <Badge variant="secondary">
            ${data.total_cost.toFixed(2)} total
          </Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-6">
          <ResponsiveContainer width="100%" height={300}>
            <BarChart
              data={data.models}
              margin={{
                top: 20,
                right: 30,
                left: 20,
                bottom: 60,
              }}
            >
              <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
              <XAxis
                dataKey="model_name"
                tickFormatter={formatModelName}
                className="text-xs"
                stroke="currentColor"
                angle={-45}
                textAnchor="end"
                height={80}
              />
              <YAxis
                tickFormatter={(value) => `$${value.toFixed(0)}`}
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
              <Bar dataKey="total_cost" radius={[4, 4, 0, 0]}>
                {data.models.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={entry.color} />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="p-4 rounded-lg border">
              <div className="flex items-center justify-between mb-2">
                <h4 className="font-medium text-sm">Highest Spend</h4>
                <div
                  className="w-3 h-3 rounded-full"
                  style={{ backgroundColor: mostExpensive.color }}
                />
              </div>
              <div className="text-lg font-bold">{formatModelName(mostExpensive.model_name)}</div>
              <div className="text-sm text-muted-foreground">
                ${mostExpensive.total_cost.toFixed(2)} total
              </div>
              <div className="text-sm text-muted-foreground">
                {mostExpensive.total_sessions} sessions
              </div>
            </div>

            <div className="p-4 rounded-lg border">
              <div className="flex items-center justify-between mb-2">
                <h4 className="font-medium text-sm">Most Efficient</h4>
                <div
                  className="w-3 h-3 rounded-full"
                  style={{ backgroundColor: mostEfficient.color }}
                />
              </div>
              <div className="text-lg font-bold">{formatModelName(mostEfficient.model_name)}</div>
              <div className="text-sm text-muted-foreground">
                ${mostEfficient.efficiency_score.toFixed(3)} per token
              </div>
              <div className="text-sm text-muted-foreground">
                ${mostEfficient.cost_per_session.toFixed(2)} per session
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}