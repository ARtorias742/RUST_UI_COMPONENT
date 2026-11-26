"use client";

import Link from "next/link";
import { useState } from "react";
import { CommandMenu } from "@/components/command-menu";
import { ThemeToggle } from "@/components/theme-toggle";
import { GithubIcon, MenuIcon, XIcon } from "@/components/icons";
import { components } from "@/lib/components";

const NAV = [
  { href: "/components", label: "Components" },
  { href: "/#comparison", label: "Comparison" },
  { href: "/#registry", label: "Registry" },
];

export function SiteHeader() {
  const [mobileOpen, setMobileOpen] = useState(false);

  return (
    <header className="sticky top-0 z-40 border-b bg-background/80 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="mx-auto flex h-16 max-w-content items-center gap-3 px-4 sm:px-6">
        <Link href="/" className="flex shrink-0 items-center gap-2 font-semibold tracking-tight">
          <span className="grid h-8 w-8 place-items-center rounded-md bg-primary text-sm text-primary-foreground">
            F
          </span>
          <span className="hidden sm:inline">Ferrum UI</span>
        </Link>

        <nav className="ml-4 hidden items-center gap-1 text-sm md:flex">
          {NAV.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className="rounded-md px-3 py-1.5 text-muted-foreground transition hover:bg-accent hover:text-foreground"
            >
              {item.label}
            </Link>
          ))}
        </nav>

        <div className="ml-auto flex items-center gap-2">
          <CommandMenu items={components} />
          <ThemeToggle />
          <a
            href="https://github.com/ARtorias742/RUST_Frontend_Framework"
            target="_blank"
            rel="noreferrer"
            aria-label="GitHub"
            className="hidden h-9 w-9 items-center justify-center rounded-md border bg-background text-muted-foreground transition hover:bg-accent hover:text-foreground sm:inline-flex"
          >
            <GithubIcon width={16} height={16} />
          </a>
          <button
            type="button"
            onClick={() => setMobileOpen((value) => !value)}
            aria-label="Toggle navigation"
            aria-expanded={mobileOpen}
            className="inline-flex h-9 w-9 items-center justify-center rounded-md border bg-background text-muted-foreground transition hover:bg-accent hover:text-foreground md:hidden"
          >
            {mobileOpen ? <XIcon /> : <MenuIcon />}
          </button>
        </div>
      </div>

      {mobileOpen ? (
        <div className="animate-in border-t bg-background px-4 py-3 md:hidden">
          <nav className="grid gap-1 text-sm">
            {NAV.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                onClick={() => setMobileOpen(false)}
                className="rounded-md px-3 py-2 text-muted-foreground transition hover:bg-accent hover:text-foreground"
              >
                {item.label}
              </Link>
            ))}
            <a
              href="https://github.com/ARtorias742/RUST_Frontend_Framework"
              target="_blank"
              rel="noreferrer"
              className="rounded-md px-3 py-2 text-muted-foreground transition hover:bg-accent hover:text-foreground"
            >
              GitHub
            </a>
          </nav>
        </div>
      ) : null}
    </header>
  );
}