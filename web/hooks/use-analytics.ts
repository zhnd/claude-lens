'use client';

import { useState, useEffect } from 'react';
import { 
  apiClient, 
  AnalyticsQuery,
  DashboardKPIs,
  TokenTrendData,
  ToolUsageData,
  UsageHeatmapData,
  ModelCostComparison,
  BudgetProgressData,
  AdvancedToolEfficiency,
  SessionDurationDistribution,
  CodeGenerationStats
} from '@/lib/api';

export function useDashboardKPIs(params: AnalyticsQuery = {}, autoRefresh = true) {
  const [data, setData] = useState<DashboardKPIs | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getDashboardKPIs(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch KPIs');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 30000); // Refresh every 30 seconds
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useTokenTrend(params: AnalyticsQuery = {}, autoRefresh = true) {
  const [data, setData] = useState<TokenTrendData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getTokenTrend(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch token trend');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 60000); // Refresh every minute
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useToolUsage(params: AnalyticsQuery = {}, autoRefresh = true) {
  const [data, setData] = useState<ToolUsageData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getToolUsage(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch tool usage');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 60000); // Refresh every minute
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useUsageHeatmap(params: AnalyticsQuery = {}, autoRefresh = false) {
  const [data, setData] = useState<UsageHeatmapData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getUsageHeatmap(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch usage heatmap');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 300000); // Refresh every 5 minutes
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

// Advanced analytics hooks
export function useModelCostComparison(params: AnalyticsQuery = {}, autoRefresh = false) {
  const [data, setData] = useState<ModelCostComparison | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getModelCostComparison(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch model cost comparison');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 300000); // Refresh every 5 minutes
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useBudgetProgress(params: AnalyticsQuery = {}, autoRefresh = true) {
  const [data, setData] = useState<BudgetProgressData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getBudgetProgress(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch budget progress');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 120000); // Refresh every 2 minutes
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useAdvancedToolEfficiency(params: AnalyticsQuery = {}, autoRefresh = false) {
  const [data, setData] = useState<AdvancedToolEfficiency | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getAdvancedToolEfficiency(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch tool efficiency');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 300000); // Refresh every 5 minutes
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useSessionDurationDistribution(params: AnalyticsQuery = {}, autoRefresh = false) {
  const [data, setData] = useState<SessionDurationDistribution | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getSessionDurationDistribution(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch session duration distribution');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 300000); // Refresh every 5 minutes
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}

export function useCodeGenerationStats(params: AnalyticsQuery = {}, autoRefresh = false) {
  const [data, setData] = useState<CodeGenerationStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiClient.getCodeGenerationStats(params);
      if (response.success && response.data) {
        setData(response.data);
      } else {
        setError(response.error || 'Failed to fetch code generation stats');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    if (autoRefresh) {
      const interval = setInterval(fetchData, 300000); // Refresh every 5 minutes
      return () => clearInterval(interval);
    }
  }, [JSON.stringify(params), autoRefresh]);

  return { data, loading, error, refetch: fetchData };
}