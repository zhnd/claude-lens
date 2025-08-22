import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Claude Scope - Monitoring Dashboard',
  description: 'Claude Code monitoring tool with OpenTelemetry data collection',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="h-full">
      <body className="h-full bg-gray-50 font-sans">
        <div className="min-h-full">
          {children}
        </div>
      </body>
    </html>
  )
}