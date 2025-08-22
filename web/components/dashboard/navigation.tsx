'use client';

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { ThemeToggle } from '@/components/theme/theme-toggle';
import { cn } from '@/lib/utils';
import { 
  BarChart3, 
  Home, 
  Menu, 
  X,
  Github,
  Activity
} from 'lucide-react';

const navigation = [
  { name: 'Dashboard', href: '/', icon: Home },
  { name: 'Analytics', href: '/analytics', icon: BarChart3 },
];

export function Navigation() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const pathname = usePathname();

  return (
    <nav className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex h-16 justify-between">
          <div className="flex">
            {/* Logo */}
            <div className="flex flex-shrink-0 items-center">
              <Link href="/" className="flex items-center space-x-2">
                <Activity className="h-8 w-8 text-primary" />
                <span className="hidden font-bold text-xl sm:block">Claude Scope</span>
              </Link>
            </div>
            
            {/* Desktop navigation */}
            <div className="hidden sm:ml-6 sm:flex sm:space-x-8">
              {navigation.map((item) => {
                const isActive = pathname === item.href;
                return (
                  <Link
                    key={item.name}
                    href={item.href}
                    className={cn(
                      'inline-flex items-center border-b-2 px-1 pt-1 text-sm font-medium transition-colors',
                      isActive
                        ? 'border-primary text-foreground'
                        : 'border-transparent text-muted-foreground hover:border-muted-foreground/50 hover:text-foreground'
                    )}
                  >
                    <item.icon className="mr-2 h-4 w-4" />
                    {item.name}
                  </Link>
                );
              })}
            </div>
          </div>

          {/* Right side */}
          <div className="hidden sm:ml-6 sm:flex sm:items-center sm:space-x-4">
            {/* Status badge */}
            <Badge variant="outline" className="text-xs">
              <div className="mr-1 h-2 w-2 rounded-full bg-green-500"></div>
              Live
            </Badge>
            
            {/* GitHub link */}
            <Button variant="ghost" size="sm" asChild>
              <a 
                href="https://github.com/zhnd/claude-scope" 
                target="_blank" 
                rel="noopener noreferrer"
              >
                <Github className="h-4 w-4" />
                <span className="sr-only">GitHub</span>
              </a>
            </Button>
            
            {/* Theme toggle */}
            <ThemeToggle />
          </div>

          {/* Mobile menu button */}
          <div className="flex items-center sm:hidden">
            <div className="flex items-center space-x-2 mr-2">
              <Badge variant="outline" className="text-xs">
                <div className="mr-1 h-1.5 w-1.5 rounded-full bg-green-500"></div>
                Live
              </Badge>
              <ThemeToggle />
            </div>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            >
              {mobileMenuOpen ? (
                <X className="h-5 w-5" />
              ) : (
                <Menu className="h-5 w-5" />
              )}
              <span className="sr-only">Toggle menu</span>
            </Button>
          </div>
        </div>
      </div>

      {/* Mobile menu */}
      {mobileMenuOpen && (
        <div className="sm:hidden">
          <div className="space-y-1 pb-3 pt-2 border-t">
            {navigation.map((item) => {
              const isActive = pathname === item.href;
              return (
                <Link
                  key={item.name}
                  href={item.href}
                  className={cn(
                    'flex items-center border-l-4 py-2 pl-3 pr-4 text-base font-medium transition-colors',
                    isActive
                      ? 'border-primary bg-primary/10 text-primary'
                      : 'border-transparent text-muted-foreground hover:border-muted-foreground/50 hover:bg-muted/50 hover:text-foreground'
                  )}
                  onClick={() => setMobileMenuOpen(false)}
                >
                  <item.icon className="mr-3 h-5 w-5" />
                  {item.name}
                </Link>
              );
            })}
            
            <div className="border-t pt-3 mt-3">
              <a
                href="https://github.com/zhnd/claude-scope"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium text-muted-foreground hover:border-muted-foreground/50 hover:bg-muted/50 hover:text-foreground transition-colors"
                onClick={() => setMobileMenuOpen(false)}
              >
                <Github className="mr-3 h-5 w-5" />
                GitHub Repository
              </a>
            </div>
          </div>
        </div>
      )}
    </nav>
  );
}