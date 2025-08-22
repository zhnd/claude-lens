'use client';

import { Button } from '@/components/ui/button';
import { Calendar, Clock } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface TimeRange {
  value: string;
  label: string;
  description: string;
}

const TIME_RANGES: TimeRange[] = [
  { value: '24h', label: '24H', description: 'Last 24 hours' },
  { value: '7d', label: '7D', description: 'Last 7 days' },
  { value: '30d', label: '30D', description: 'Last 30 days' },
];

interface TimeRangeSelectorProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
  className?: string;
}

export function TimeRangeSelector({ 
  value, 
  onChange, 
  disabled = false,
  className 
}: TimeRangeSelectorProps) {
  return (
    <div className={cn('flex items-center gap-1', className)}>
      <div className="flex items-center gap-1 text-muted-foreground mr-2">
        <Clock className="h-4 w-4" />
        <span className="text-sm font-medium">Period:</span>
      </div>
      <div className="flex rounded-md border p-1">
        {TIME_RANGES.map((range) => (
          <Button
            key={range.value}
            variant={value === range.value ? "default" : "ghost"}
            size="sm"
            onClick={() => onChange(range.value)}
            disabled={disabled}
            className={cn(
              "h-7 px-3 text-xs font-medium transition-all",
              value === range.value && "shadow-sm"
            )}
            title={range.description}
          >
            {range.label}
          </Button>
        ))}
      </div>
    </div>
  );
}