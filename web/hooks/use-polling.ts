'use client'

import { useEffect, useCallback, useRef } from 'react'

interface UsePollingOptions {
  interval?: number
  immediate?: boolean
  enabled?: boolean
}

export function usePolling(
  callback: () => void | Promise<void>,
  { interval = 30000, immediate = true, enabled = true }: UsePollingOptions = {}
) {
  const intervalRef = useRef<NodeJS.Timeout | null>(null)
  const callbackRef = useRef(callback)

  // Update callback ref when callback changes
  useEffect(() => {
    callbackRef.current = callback
  }, [callback])

  const startPolling = useCallback(() => {
    if (!enabled) return

    if (intervalRef.current) {
      clearInterval(intervalRef.current)
    }

    intervalRef.current = setInterval(async () => {
      await callbackRef.current()
    }, interval)

    // Call immediately if requested
    if (immediate) {
      callbackRef.current()
    }
  }, [interval, immediate, enabled])

  const stopPolling = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current)
      intervalRef.current = null
    }
  }, [])

  const restartPolling = useCallback(() => {
    stopPolling()
    startPolling()
  }, [stopPolling, startPolling])

  // Start/stop polling based on enabled state
  useEffect(() => {
    if (enabled) {
      startPolling()
    } else {
      stopPolling()
    }

    return stopPolling
  }, [enabled, startPolling, stopPolling])

  // Cleanup on unmount
  useEffect(() => {
    return stopPolling
  }, [stopPolling])

  return {
    startPolling,
    stopPolling,
    restartPolling,
    isPolling: intervalRef.current !== null,
  }
}