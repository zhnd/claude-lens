import type { Metadata } from 'next'
import './globals.css'
import { ThemeProvider } from '@/components/theme/theme-provider'
import { Navigation } from '@/components/dashboard/navigation'
import { ErrorBoundary } from '@/components/dashboard/error-boundary'

export const metadata: Metadata = {
  title: 'Claude Lens - Monitoring Dashboard',
  description: 'Claude Code monitoring tool with OpenTelemetry data collection',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="h-full" suppressHydrationWarning>
      <body className="h-full bg-background font-sans antialiased">
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <div className="min-h-full">
            <Navigation />
            <main className="min-h-screen bg-background">
              <ErrorBoundary>
                {children}
              </ErrorBoundary>
            </main>
          </div>
        </ThemeProvider>
      </body>
    </html>
  )
}