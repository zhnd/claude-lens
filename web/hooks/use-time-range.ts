'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface TimeRangeContextType {
  timeRange: string;
  setTimeRange: (range: string) => void;
}

const TimeRangeContext = createContext<TimeRangeContextType | undefined>(undefined);

export function TimeRangeProvider({ children }: { children: ReactNode }) {
  const [timeRange, setTimeRangeState] = useState('24h');

  // Persist to localStorage
  useEffect(() => {
    const saved = localStorage.getItem('claude-lens-time-range');
    if (saved) {
      setTimeRangeState(saved);
    }
  }, []);

  const setTimeRange = (range: string) => {
    setTimeRangeState(range);
    localStorage.setItem('claude-lens-time-range', range);
  };

  return React.createElement(
    TimeRangeContext.Provider,
    { value: { timeRange, setTimeRange } },
    children
  );
}

export function useTimeRange() {
  const context = useContext(TimeRangeContext);
  if (context === undefined) {
    throw new Error('useTimeRange must be used within a TimeRangeProvider');
  }
  return context;
}