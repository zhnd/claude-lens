'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
} from 'recharts'
import { format } from 'date-fns'
import { MetricPoint } from '@/lib/api'

interface MetricsChartProps {
  title: string
  data: MetricPoint[]
  type?: 'line' | 'bar' | 'pie'
  height?: number
}

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884d8']

export function MetricsChart({ 
  title, 
  data, 
  type = 'line', 
  height = 300 
}: MetricsChartProps) {
  const formatXAxisLabel = (dateString: string) => {
    return format(new Date(dateString), 'HH:mm')
  }

  const formatTooltipLabel = (dateString: string) => {
    return format(new Date(dateString), 'MMM dd, HH:mm:ss')
  }

  const renderChart = () => {
    switch (type) {
      case 'bar':
        return (
          <BarChart data={data}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis 
              dataKey="timestamp" 
              tickFormatter={formatXAxisLabel}
              fontSize={12}
            />
            <YAxis fontSize={12} />
            <Tooltip 
              labelFormatter={formatTooltipLabel}
              formatter={(value: number) => [value, 'Value']}
            />
            <Bar dataKey="value" fill="#8884d8" />
          </BarChart>
        )
      
      case 'pie':
        const pieData = data.reduce((acc, item) => {
          const existingItem = acc.find(a => a.name === item.name)
          if (existingItem) {
            existingItem.value += item.value
          } else {
            acc.push({ name: item.name, value: item.value })
          }
          return acc
        }, [] as { name: string; value: number }[])

        return (
          <PieChart>
            <Pie
              data={pieData}
              cx="50%"
              cy="50%"
              labelLine={false}
              label={({ name, percent }) => `${name} ${(percent * 100).toFixed(0)}%`}
              outerRadius={80}
              fill="#8884d8"
              dataKey="value"
            >
              {pieData.map((entry, index) => (
                <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
              ))}
            </Pie>
            <Tooltip />
          </PieChart>
        )
      
      default: // line
        return (
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis 
              dataKey="timestamp" 
              tickFormatter={formatXAxisLabel}
              fontSize={12}
            />
            <YAxis fontSize={12} />
            <Tooltip 
              labelFormatter={formatTooltipLabel}
              formatter={(value: number) => [value, 'Value']}
            />
            <Line 
              type="monotone" 
              dataKey="value" 
              stroke="#8884d8" 
              strokeWidth={2}
              dot={{ r: 3 }}
            />
          </LineChart>
        )
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">{title}</CardTitle>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={height}>
          {renderChart()}
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}