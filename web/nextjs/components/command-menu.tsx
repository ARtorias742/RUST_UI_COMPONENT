"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import { useRouter } from "next/navigation";
import type { ComponentEntry } from "@/lib/components";
import { SearchIcon, XIcon } from "@/components/icons";

export function CommandMenu({ items }: { items: ComponentEntry[] }) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        setOpen((value) => !value);
      }
      if (event.key === "Escape") setOpen(false);
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, []);

  useEffect(() => {
    if (open) {
      setQuery("");
      window.setTimeout(() => inputRef.current?.focus(), 20);
      document.body.style.overflow = "hidden";
    } else {
      document.body.style.overflow = "";
    }
    return () => {
      document.body.style.overflow = "";
    };
  }, [open]);

  const results = useMemo(() => {
    const q = query.trim().toLowerCase();
    const pages = [
      { name: "Overview", slug: "", category: "Page", description: "Welcome to Ferrum UI" },
      {
        name: "All components",
        slug: "components",
        category: "Page",
        description: "Browse every primitive",
      },
    ];
    const all = [...pages, ...items];
    if (!q) return all;
    return all.filter((item) =>
      [item.name, item.category, item.description]
        .join(" ")
        .toLowerCase()
        .includes(q),
    );
  }, [items, query]);

  function go(slug: string) {
    setOpen(false);
    router.push(slug ? `/components/${slug}` : "/");
  }

  if (!open) {
    return (
      <button
        type="button"
        onClick={() => setOpen(true)}
        className="hidden h-9 items-center gap-2 rounded-md border bg-background px-3 text-sm text-muted-foreground transition hover:bg-accent hover:text-foreground sm:inline-flex"
      >
        <SearchIcon width={15} height={15} />
        <span>Search docs…</span>
        <kbd className="ml-2 rounded border bg-muted px-1.5 py-0.5 font-mono text-[10px]">
          ⌘K
        </kbd>
      </button>
    );
  }

  return (
    <div
      className="fixed inset-0 z-[60] flex items-start justify-center bg-black/40 p-4 pt-[12vh] backdrop-blur-sm"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) setOpen(false);
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Search documentation"
        className="animate-in w-full max-w-xl overflow-hidden rounded-xl border bg-popover shadow-2xl"
      >
        <div className="flex items-center gap-2 border-b px-4">
          <SearchIcon className="text-muted-foreground" />
          <input
            ref={inputRef}
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search components, categories, pages…"
            className="h-12 flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
          />
          <button
            type="button"
            onClick={() => setOpen(false)}
            aria-label="Close search"
            className="rounded p-1 text-muted-foreground hover:text-foreground"
          >
            <XIcon width={15} height={15} />
          </button>
        </div>
        <div className="scroll-thin max-h-80 overflow-y-auto p-2">
          {results.length === 0 ? (
            <p className="px-3 py-8 text-center text-sm text-muted-foreground">
              No results for “{query}”
            </p>
          ) : (
            results.map((item) => (
              <button
                key={item.slug || "home"}
                type="button"
                onClick={() => go(item.slug)}
                className="flex w-full items-center justify-between gap-3 rounded-lg px-3 py-2.5 text-left text-sm transition hover:bg-accent"
              >
                <span>
                  <span className="font-medium">{item.name}</span>
                  <span className="ml-2 text-xs text-muted-foreground">
                    {item.description}
                  </span>
                </span>
                <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-[10px] uppercase tracking-wide text-muted-foreground">
                  {item.category}
                </span>
              </button>
            ))
          )}
        </div>
        <div className="flex items-center justify-between border-t px-4 py-2 text-[11px] text-muted-foreground">
          <span>Navigate with Tab · Select with Enter</span>
          <span>Esc to close</span>
        </div>
      </div>
    </div>
  );
}