"use client";

import { useState } from "react";

export function CopyCommand({ command }: { command: string }) {
  const [copied, setCopied] = useState(false);

  async function copy() {
    await navigator.clipboard.writeText(command);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1500);
  }

  return (
    <button
      type="button"
      onClick={copy}
      className="group flex w-full items-center justify-between gap-4 rounded-lg border bg-muted/40 px-4 py-3 text-left font-mono text-sm transition hover:border-ring/60 hover:bg-muted"
    >
      <span className="truncate">
        <span className="mr-2 select-none text-muted-foreground">$</span>
        {command}
      </span>
      <span className="shrink-0 text-xs text-muted-foreground group-hover:text-foreground">
        {copied ? "copied" : "copy"}
      </span>
    </button>
  );
}