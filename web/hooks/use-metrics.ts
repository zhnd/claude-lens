'use client'

import { useState, useCallback } from 'react'
import { apiClient, MetricsOverview, TimelineData, SessionsResponse } from '@/lib/api'
import { usePolling } from './use-polling'

export function useMetrics() {
  const [overview, setOverview] = useState<MetricsOverview | null>(null)
  const [timeline, setTimeline] = useState<TimelineData | null>(null)
  const [sessions, setSessions] = useState<SessionsResponse | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null)

  const fetchOverview = useCallback(async () => {
    try {
      const response = await apiClient.getMetricsOverview()
      if (response.success && response.data) {
        setOverview(response.data)
        setError(null)
      } else {
        setError(response.error || 'Failed to fetch overview')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error')
    }
  }, [])

  const fetchTimeline = useCallback(async (range: string = '24h') => {
    try {
      const response = await apiClient.getMetricsTimeline(range)
      if (response.success && response.data) {
        setTimeline(response.data)
        setError(null)
      } else {
        setError(response.error || 'Failed to fetch timeline')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error')
    }
  }, [])

  const fetchSessions = useCallback(async (limit: number = 10) => {
    try {
      const response = await apiClient.getSessions(limit, 0)
      if (response.success && response.data) {
        setSessions(response.data)
        setError(null)
      } else {
        setError(response.error || 'Failed to fetch sessions')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error')
    }
  }, [])

  const fetchAll = useCallback(async () => {
    setLoading(true)
    setError(null)
    
    try {
      await Promise.all([
        fetchOverview(),
        fetchTimeline(),
        fetchSessions(),
      ])
      setLastUpdated(new Date())
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch data')
    } finally {
      setLoading(false)
    }
  }, [fetchOverview, fetchTimeline, fetchSessions])

  const refresh = useCallback(async () => {
    await fetchAll()
  }, [fetchAll])

  return {
    overview,
    timeline,
    sessions,
    loading,
    error,
    lastUpdated,
    refresh,
    fetchOverview,
    fetchTimeline,
    fetchSessions,
    fetchAll,
  }
}

export function useMetricsPolling(enabled: boolean = true, interval: number = 30000) {
  const metrics = useMetrics()

  const pollingControls = usePolling(
    metrics.fetchAll,
    { interval, enabled, immediate: true }
  )

  return {
    ...metrics,
    ...pollingControls,
  }
}