'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { cn } from '@/lib/utils';

interface HeatmapCell {
  hour: number;
  day_of_week: number;
  intensity: number;
  session_count: number;
  token_count: number;
}

interface UsageHeatmapProps {
  data: {
    timezone: string;
    heatmap: HeatmapCell[];
  };
  loading?: boolean;
  className?: string;
}

const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
const HOURS = Array.from({ length: 24 }, (_, i) => i);

export function UsageHeatmap({ data, loading = false, className }: UsageHeatmapProps) {
  const getCellData = (hour: number, day: number): HeatmapCell | undefined => {
    return data.heatmap.find(cell => cell.hour === hour && cell.day_of_week === day);
  };

  const getIntensityColor = (intensity: number) => {
    if (intensity === 0) return 'bg-gray-100 dark:bg-gray-800';
    if (intensity < 0.2) return 'bg-green-100 dark:bg-green-900/30';
    if (intensity < 0.4) return 'bg-green-200 dark:bg-green-800/50';
    if (intensity < 0.6) return 'bg-green-300 dark:bg-green-700/60';
    if (intensity < 0.8) return 'bg-green-400 dark:bg-green-600/70';
    return 'bg-green-500 dark:bg-green-500/80';
  };

  const formatTime = (hour: number) => {
    if (hour === 0) return '12 AM';
    if (hour < 12) return `${hour} AM`;
    if (hour === 12) return '12 PM';
    return `${hour - 12} PM`;
  };

  const totalSessions = data.heatmap.reduce((sum, cell) => sum + cell.session_count, 0);
  const totalTokens = data.heatmap.reduce((sum, cell) => sum + cell.token_count, 0);
  const avgIntensity = data.heatmap.reduce((sum, cell) => sum + cell.intensity, 0) / data.heatmap.length;

  if (loading) {
    return (
      <Card className={className}>
        <CardHeader>
          <div className="h-5 bg-muted rounded w-40 mb-2"></div>
          <div className="h-4 bg-muted rounded w-64"></div>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            {Array.from({ length: 7 }, (_, i) => (
              <div key={i} className="flex gap-1">
                <div className="w-8 h-4 bg-muted rounded"></div>
                {Array.from({ length: 24 }, (_, j) => (
                  <div key={j} className="w-4 h-4 bg-muted rounded animate-pulse"></div>
                ))}
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Activity Heatmap</CardTitle>
            <CardDescription>
              Usage patterns by day and hour ({data.timezone})
            </CardDescription>
          </div>
          <div className="flex gap-2">
            <Badge variant="secondary">
              {totalSessions} sessions
            </Badge>
            <Badge variant="outline">
              {(totalTokens / 1000).toFixed(1)}K tokens
            </Badge>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <TooltipProvider>
          <div className="space-y-4">
            {/* Hour labels */}
            <div className="flex">
              <div className="w-10"></div>
              <div className="flex-1 flex">
                {HOURS.filter((_, i) => i % 4 === 0).map(hour => (
                  <div key={hour} className="flex-1 text-center text-xs text-muted-foreground">
                    {formatTime(hour)}
                  </div>
                ))}
              </div>
            </div>

            {/* Heatmap grid */}
            <div className="space-y-1">
              {DAYS.map((day, dayIndex) => (
                <div key={day} className="flex items-center">
                  <div className="w-10 text-xs text-muted-foreground text-right pr-2">
                    {day}
                  </div>
                  <div className="flex gap-1">
                    {HOURS.map(hour => {
                      const cellData = getCellData(hour, dayIndex);
                      const intensity = cellData?.intensity || 0;
                      
                      return (
                        <Tooltip key={`${dayIndex}-${hour}`}>
                          <TooltipTrigger asChild>
                            <div
                              className={cn(
                                'w-3 h-3 rounded-sm cursor-pointer hover:ring-2 hover:ring-primary hover:ring-offset-1 transition-all',
                                getIntensityColor(intensity)
                              )}
                            />
                          </TooltipTrigger>
                          <TooltipContent>
                            <div className="text-sm">
                              <div className="font-medium">{day} {formatTime(hour)}</div>
                              <div>Sessions: {cellData?.session_count || 0}</div>
                              <div>Tokens: {cellData?.token_count.toLocaleString() || 0}</div>
                              <div>Intensity: {(intensity * 100).toFixed(0)}%</div>
                            </div>
                          </TooltipContent>
                        </Tooltip>
                      );
                    })}
                  </div>
                </div>
              ))}
            </div>

            {/* Legend */}
            <div className="flex items-center justify-between pt-4 border-t">
              <div className="text-xs text-muted-foreground">
                Average activity: {(avgIntensity * 100).toFixed(1)}%
              </div>
              <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <span>Less</span>
                <div className="flex gap-1">
                  <div className="w-3 h-3 rounded-sm bg-gray-100 dark:bg-gray-800"></div>
                  <div className="w-3 h-3 rounded-sm bg-green-100 dark:bg-green-900/30"></div>
                  <div className="w-3 h-3 rounded-sm bg-green-200 dark:bg-green-800/50"></div>
                  <div className="w-3 h-3 rounded-sm bg-green-300 dark:bg-green-700/60"></div>
                  <div className="w-3 h-3 rounded-sm bg-green-400 dark:bg-green-600/70"></div>
                  <div className="w-3 h-3 rounded-sm bg-green-500 dark:bg-green-500/80"></div>
                </div>
                <span>More</span>
              </div>
            </div>
          </div>
        </TooltipProvider>
      </CardContent>
    </Card>
  );
}