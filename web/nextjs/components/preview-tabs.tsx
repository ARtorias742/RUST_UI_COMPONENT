"use client";

import { useState } from "react";
import { ComponentPreview } from "@/components/component-preview";
import { CopyButton } from "@/components/copy-button";
import { MoonIcon, SunIcon } from "@/components/icons";

export function PreviewTabs({ name, code }: { name: string; code: string }) {
  const [tab, setTab] = useState<"preview" | "code">("preview");
  const [dark, setDark] = useState(false);

  return (
    <div className="overflow-hidden rounded-xl border bg-card">
      <div className="flex items-center justify-between gap-2 border-b px-3 py-2">
        <div className="flex gap-1 rounded-lg bg-muted p-1" role="tablist">
          {(["preview", "code"] as const).map((value) => (
            <button
              key={value}
              type="button"
              role="tab"
              aria-selected={tab === value}
              onClick={() => setTab(value)}
              className={`rounded-md px-3 py-1 text-xs font-medium capitalize transition ${
                tab === value
                  ? "bg-background text-foreground shadow-sm"
                  : "text-muted-foreground hover:text-foreground"
              }`}
            >
              {value}
            </button>
          ))}
        </div>
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => setDark((value) => !value)}
            aria-label={dark ? "Use light preview" : "Use dark preview"}
            className="inline-flex h-7 w-7 items-center justify-center rounded-md border bg-background text-muted-foreground transition hover:text-foreground"
          >
            {dark ? <SunIcon width={13} height={13} /> : <MoonIcon width={13} height={13} />}
          </button>
          <CopyButton value={code} label="Copy" />
        </div>
      </div>

      <div className={dark ? "dark" : undefined}>
        <div className="flex min-h-[260px] items-center justify-center bg-background p-6 sm:p-10">
          {tab === "preview" ? (
            <div className="animate-in flex w-full justify-center">
              <ComponentPreview name={name} />
            </div>
          ) : (
            <pre className="code-block w-full overflow-x-auto text-foreground">
              <code>{code}</code>
            </pre>
          )}
        </div>
      </div>
    </div>
  );
}