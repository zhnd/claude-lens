'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { TrendingDown, TrendingUp } from 'lucide-react';

interface KPICardProps {
  title: string;
  value: string | number;
  change?: number;
  changeLabel?: string;
  icon?: React.ReactNode;
  className?: string;
  loading?: boolean;
  trend?: 'up' | 'down' | 'neutral';
}

export function KPICard({
  title,
  value,
  change,
  changeLabel,
  icon,
  className,
  loading = false,
  trend,
}: KPICardProps) {
  const formatValue = (val: string | number) => {
    if (typeof val === 'number') {
      if (val >= 1000000) {
        return `${(val / 1000000).toFixed(1)}M`;
      }
      if (val >= 1000) {
        return `${(val / 1000).toFixed(1)}K`;
      }
      return val.toLocaleString();
    }
    return val;
  };

  const getTrendColor = () => {
    if (!change && !trend) return 'text-muted-foreground';
    
    const actualTrend = trend || (change && change > 0 ? 'up' : 'down');
    
    switch (actualTrend) {
      case 'up':
        return 'text-green-600 dark:text-green-400';
      case 'down':
        return 'text-red-600 dark:text-red-400';
      default:
        return 'text-muted-foreground';
    }
  };

  const getTrendIcon = () => {
    if (!change && !trend) return null;
    
    const actualTrend = trend || (change && change > 0 ? 'up' : 'down');
    
    return actualTrend === 'up' ? (
      <TrendingUp className="h-4 w-4" />
    ) : (
      <TrendingDown className="h-4 w-4" />
    );
  };

  if (loading) {
    return (
      <Card className={cn("animate-pulse", className)}>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">
            <div className="h-4 bg-muted rounded w-20"></div>
          </CardTitle>
          {icon && (
            <div className="h-4 w-4 bg-muted rounded"></div>
          )}
        </CardHeader>
        <CardContent>
          <div className="h-8 bg-muted rounded w-16 mb-2"></div>
          <div className="h-3 bg-muted rounded w-24"></div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={cn("hover:shadow-md transition-shadow", className)}>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium text-muted-foreground">
          {title}
        </CardTitle>
        {icon && (
          <div className="text-muted-foreground">
            {icon}
          </div>
        )}
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold mb-2">
          {formatValue(value)}
        </div>
        {(change !== undefined || changeLabel) && (
          <div className={cn("flex items-center text-xs", getTrendColor())}>
            {getTrendIcon()}
            <span className="ml-1">
              {change !== undefined && (
                <>
                  {change > 0 ? '+' : ''}{change.toFixed(1)}%
                  {changeLabel && ` ${changeLabel}`}
                </>
              )}
              {change === undefined && changeLabel && changeLabel}
            </span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}