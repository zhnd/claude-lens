'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { cn } from '@/lib/utils';
import { AlertTriangle, TrendingUp, TrendingDown, Calendar } from 'lucide-react';

interface BudgetProgressData {
  current_month_cost: number;
  monthly_budget: number;
  percentage_used: number;
  days_remaining: number;
  projected_month_end_cost: number;
  is_over_budget: boolean;
  daily_breakdown: Array<{
    date: string;
    cost: number;
    sessions: number;
    tokens: number;
  }>;
}

interface CostProgressRingProps {
  data: BudgetProgressData;
  loading?: boolean;
  className?: string;
}

export function CostProgressRing({ data, loading = false, className }: CostProgressRingProps) {
  const {
    current_month_cost,
    monthly_budget,
    percentage_used,
    days_remaining,
    projected_month_end_cost,
    is_over_budget,
    daily_breakdown,
  } = data;

  const dailyAvg = daily_breakdown.length > 0 
    ? daily_breakdown.reduce((sum, day) => sum + day.cost, 0) / daily_breakdown.length
    : 0;

  const projectedVsBudget = ((projected_month_end_cost - monthly_budget) / monthly_budget) * 100;
  
  const getStatusColor = () => {
    if (is_over_budget || percentage_used > 90) return 'text-red-600 dark:text-red-400';
    if (percentage_used > 75) return 'text-amber-600 dark:text-amber-400';
    return 'text-green-600 dark:text-green-400';
  };

  const getProgressColor = () => {
    if (is_over_budget || percentage_used > 90) return 'bg-red-500';
    if (percentage_used > 75) return 'bg-amber-500';
    return 'bg-green-500';
  };

  const getStatusIcon = () => {
    if (is_over_budget || projectedVsBudget > 10) {
      return <AlertTriangle className="h-4 w-4" />;
    }
    if (projectedVsBudget > 0) {
      return <TrendingUp className="h-4 w-4" />;
    }
    return <TrendingDown className="h-4 w-4" />;
  };

  if (loading) {
    return (
      <Card className={className}>
        <CardHeader>
          <div className="h-5 bg-muted rounded w-32 mb-2"></div>
          <div className="h-4 bg-muted rounded w-48"></div>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="flex justify-center">
            <div className="w-40 h-40 bg-muted rounded-full animate-pulse"></div>
          </div>
          <div className="space-y-3">
            <div className="h-4 bg-muted rounded w-full"></div>
            <div className="h-4 bg-muted rounded w-3/4"></div>
            <div className="h-4 bg-muted rounded w-2/3"></div>
          </div>
        </CardContent>
      </Card>
    );
  }

  // Create a simple circular progress implementation since we don't have a circular progress component
  const radius = 70;
  const strokeWidth = 8;
  const normalizedRadius = radius - strokeWidth * 2;
  const circumference = normalizedRadius * 2 * Math.PI;
  const strokeDasharray = `${circumference} ${circumference}`;
  const strokeDashoffset = circumference - (percentage_used / 100) * circumference;

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Budget Progress</CardTitle>
            <CardDescription>
              Monthly spending vs budget
            </CardDescription>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <Calendar className="h-4 w-4" />
            <span>{days_remaining} days left</span>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Circular Progress */}
        <div className="flex justify-center">
          <div className="relative">
            <svg
              height={radius * 2}
              width={radius * 2}
              className="transform -rotate-90"
            >
              <circle
                stroke="currentColor"
                fill="transparent"
                strokeWidth={strokeWidth}
                r={normalizedRadius}
                cx={radius}
                cy={radius}
                className="text-muted stroke-current opacity-20"
              />
              <circle
                stroke="currentColor"
                fill="transparent"
                strokeWidth={strokeWidth}
                strokeDasharray={strokeDasharray}
                style={{ strokeDashoffset }}
                r={normalizedRadius}
                cx={radius}
                cy={radius}
                className={cn(
                  "transition-all duration-500 ease-in-out",
                  percentage_used > 90 ? "text-red-500" :
                  percentage_used > 75 ? "text-amber-500" : "text-green-500"
                )}
                strokeLinecap="round"
              />
            </svg>
            <div className="absolute inset-0 flex flex-col items-center justify-center">
              <span className="text-2xl font-bold">{percentage_used.toFixed(0)}%</span>
              <span className="text-sm text-muted-foreground">used</span>
            </div>
          </div>
        </div>

        {/* Budget Details */}
        <div className="space-y-4">
          <div className="flex justify-between items-center">
            <span className="text-sm font-medium">Current Spend</span>
            <span className="font-bold">${current_month_cost.toFixed(2)}</span>
          </div>
          
          <div className="flex justify-between items-center">
            <span className="text-sm font-medium">Monthly Budget</span>
            <span className="text-muted-foreground">${monthly_budget.toFixed(2)}</span>
          </div>

          <div className="flex justify-between items-center">
            <span className="text-sm font-medium">Remaining</span>
            <span className={cn("font-medium", getStatusColor())}>
              ${(monthly_budget - current_month_cost).toFixed(2)}
            </span>
          </div>

          <div className="pt-2 border-t space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium flex items-center gap-2">
                {getStatusIcon()}
                Projected Spend
              </span>
              <span className={cn("font-bold", getStatusColor())}>
                ${projected_month_end_cost.toFixed(2)}
              </span>
            </div>
            
            {projectedVsBudget !== 0 && (
              <div className="text-xs text-muted-foreground">
                {projectedVsBudget > 0 ? '+' : ''}
                {projectedVsBudget.toFixed(1)}% vs budget
              </div>
            )}
          </div>

          <div className="pt-2 border-t">
            <div className="flex justify-between items-center mb-2">
              <span className="text-sm font-medium">Daily Average</span>
              <span className="text-sm">${dailyAvg.toFixed(2)}</span>
            </div>
            <Progress 
              value={(dailyAvg / (monthly_budget / 30)) * 100} 
              className="h-2"
            />
          </div>
        </div>

        {/* Status Badges */}
        <div className="flex gap-2">
          {is_over_budget && (
            <Badge variant="destructive" className="text-xs">
              Over Budget
            </Badge>
          )}
          {percentage_used > 90 && !is_over_budget && (
            <Badge variant="secondary" className="text-xs bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-100">
              Budget Warning
            </Badge>
          )}
          {percentage_used <= 75 && (
            <Badge variant="secondary" className="text-xs bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100">
              On Track
            </Badge>
          )}
        </div>
      </CardContent>
    </Card>
  );
}