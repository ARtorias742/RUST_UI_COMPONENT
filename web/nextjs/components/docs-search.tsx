"use client";

import Link from "next/link";
import { useMemo, useState } from "react";
import type { ComponentEntry } from "@/lib/components";

export function DocsSearch({ items }: { items: ComponentEntry[] }) {
  const [query, setQuery] = useState("");
  const normalized = query.trim().toLowerCase();

  const results = useMemo(() => {
    if (!normalized) return items;
    return items.filter((item) =>
      [item.name, item.category, item.description, item.code, ...item.props]
        .join(" ")
        .toLowerCase()
        .includes(normalized),
    );
  }, [items, normalized]);

  return (
    <div role="search" className="mt-8">
      <label htmlFor="docs-search" className="sr-only">
        Search components
      </label>
      <input
        id="docs-search"
        type="search"
        value={query}
        onChange={(event) => setQuery(event.target.value)}
        placeholder="Search components, props, categories…"
        className="w-full rounded-lg border bg-background px-4 py-3 text-sm outline-none ring-ring/40 placeholder:text-muted-foreground focus-visible:ring-2"
      />
      <p className="mt-2 text-xs text-muted-foreground" aria-live="polite">
        {normalized ? `${results.length} result${results.length === 1 ? "" : "s"}` : `${items.length} components`}
      </p>

      {results.length === 0 ? (
        <div className="mt-6 rounded-xl border p-8 text-center">
          <p className="font-medium">No components found</p>
          <p className="mt-1 text-sm text-muted-foreground">Try a different name or prop.</p>
        </div>
      ) : (
        <div className="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {results.map((item) => (
            <Link
              key={item.slug}
              href={`/components/${item.slug}`}
              className="rounded-xl border bg-card p-4 transition hover:border-ring/60 hover:bg-accent/30"
            >
              <div className="flex items-center justify-between">
                <span className="font-medium">{item.name}</span>
                <span className="text-xs text-muted-foreground">Docs →</span>
              </div>
              <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}