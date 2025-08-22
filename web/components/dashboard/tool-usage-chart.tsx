'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ToolUsage } from "@/lib/api"
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

interface ToolUsageChartProps {
  data: ToolUsage[]
  title?: string
}

export function ToolUsageChart({ data, title = "Tool Usage" }: ToolUsageChartProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {/* Bar Chart */}
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={data}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="name" fontSize={12} />
                <YAxis fontSize={12} />
                <Tooltip 
                  formatter={(value: number) => [value, 'Count']}
                />
                <Bar dataKey="count" fill="#8884d8" />
              </BarChart>
            </ResponsiveContainer>
          </div>
          
          {/* Tool Usage List */}
          <div className="space-y-2">
            {data.map((tool, index) => (
              <div key={tool.name} className="flex items-center justify-between p-2 rounded-lg border">
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center text-sm font-medium">
                    {index + 1}
                  </div>
                  <span className="font-medium">{tool.name}</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-sm text-muted-foreground">{tool.count} uses</span>
                  <Badge variant="secondary">{tool.percentage.toFixed(1)}%</Badge>
                </div>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}