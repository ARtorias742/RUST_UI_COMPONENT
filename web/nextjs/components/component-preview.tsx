import type { ComponentEntry } from "@/lib/components";

/**
 * Static, token-driven previews. Every preview uses the same CSS variables as
 * the docs shell, so it re-themes with the global (or per-preview) theme.
 */
export function ComponentPreview({ name }: { name: string }) {
  switch (name) {
    case "Button":
      return (
        <div className="flex flex-wrap items-center gap-2">
          <span className="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground">
            Primary
          </span>
          <span className="rounded-md border bg-background px-3 py-1.5 text-xs font-medium">
            Outline
          </span>
          <span className="rounded-md bg-secondary px-3 py-1.5 text-xs font-medium text-secondary-foreground">
            Secondary
          </span>
          <span className="rounded-md px-3 py-1.5 text-xs font-medium text-muted-foreground underline-offset-4 hover:underline">
            Ghost
          </span>
          <span className="rounded-md bg-destructive px-3 py-1.5 text-xs font-medium text-destructive-foreground">
            Destructive
          </span>
          <span className="inline-flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground opacity-60">
            <span className="h-3 w-3 animate-spin rounded-full border-2 border-primary-foreground/40 border-t-primary-foreground" />
            Loading
          </span>
        </div>
      );
    case "Badge":
      return (
        <div className="flex flex-wrap items-center gap-2">
          <span className="rounded-full border px-2 py-0.5 text-xs">Default</span>
          <span className="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2 py-0.5 text-xs text-emerald-700 dark:text-emerald-300">
            Success
          </span>
          <span className="rounded-full border border-amber-500/40 bg-amber-500/10 px-2 py-0.5 text-xs text-amber-700 dark:text-amber-300">
            Warning
          </span>
          <span className="rounded-full border border-destructive/40 bg-destructive/10 px-2 py-0.5 text-xs text-destructive">
            Error
          </span>
          <span className="rounded-full border border-sky-500/40 bg-sky-500/10 px-2 py-0.5 text-xs text-sky-700 dark:text-sky-300">
            Info
          </span>
        </div>
      );
    case "Alert":
      return (
        <div className="w-full max-w-sm space-y-2">
          <div className="rounded-md border border-emerald-500/40 bg-emerald-500/10 p-3 text-xs">
            <strong className="block text-emerald-700 dark:text-emerald-300">Saved</strong>
            <span className="text-muted-foreground">Your changes were synced just now.</span>
          </div>
          <div className="rounded-md border bg-card p-3 text-xs">
            <strong className="block">Heads up</strong>
            <span className="text-muted-foreground">This action cannot be undone.</span>
          </div>
        </div>
      );
    case "Avatar":
      return (
        <div className="flex items-center gap-3">
          <span className="grid h-9 w-9 place-items-center rounded-full bg-primary text-xs font-semibold text-primary-foreground">
            AB
          </span>
          <span className="grid h-9 w-9 place-items-center rounded-full bg-muted text-xs font-semibold">
            CD
          </span>
          <div className="text-xs">
            <p className="font-medium">Ada Lovelace</p>
            <p className="text-muted-foreground">ada@example.com</p>
          </div>
        </div>
      );
    case "Card":
      return (
        <div className="w-full max-w-xs rounded-lg border bg-card p-4 shadow-sm">
          <p className="text-sm font-semibold">Project</p>
          <p className="mt-1 text-xs text-muted-foreground">
            A titled container with an optional footer.
          </p>
          <div className="mt-3 flex justify-end">
            <span className="rounded-md bg-primary px-2.5 py-1 text-[11px] font-medium text-primary-foreground">
              Done
            </span>
          </div>
        </div>
      );
    case "Checkbox":
      return (
        <div className="space-y-2 text-xs">
          <label className="flex items-center gap-2">
            <span className="grid h-4 w-4 place-items-center rounded border bg-primary text-[10px] text-primary-foreground">
              ✓
            </span>
            Accept terms
          </label>
          <label className="flex items-center gap-2 text-muted-foreground">
            <span className="h-4 w-4 rounded border" />
            Disabled option
          </label>
        </div>
      );
    case "Input":
      return (
        <div className="w-full max-w-xs space-y-3">
          <div>
            <p className="mb-1 text-[11px] font-medium">Email</p>
            <div className="rounded-md border bg-background px-2.5 py-1.5 text-xs text-muted-foreground">
              you@example.com
            </div>
          </div>
          <div>
            <p className="mb-1 text-[11px] font-medium">Password</p>
            <div className="rounded-md border border-destructive/60 bg-background px-2.5 py-1.5 text-xs text-destructive">
              Too short
            </div>
          </div>
        </div>
      );
    case "Progress":
      return (
        <div className="w-full max-w-xs">
          <div className="h-2 overflow-hidden rounded-full bg-muted">
            <div className="h-full w-[64%] rounded-full bg-primary" />
          </div>
          <p className="mt-1.5 text-right text-[11px] text-muted-foreground">64%</p>
        </div>
      );
    case "Separator":
      return (
        <div className="w-full max-w-xs space-y-3">
          <div className="h-px w-full bg-border" />
          <div className="flex items-center gap-3 text-[11px] text-muted-foreground">
            <div className="h-px flex-1 bg-border" />
            or
            <div className="h-px flex-1 bg-border" />
          </div>
          <div className="h-px w-full bg-border" />
        </div>
      );
    case "Skeleton":
      return (
        <div className="w-full max-w-xs space-y-2">
          <div className="h-3 w-3/4 animate-pulse rounded bg-muted" />
          <div className="h-3 w-1/2 animate-pulse rounded bg-muted" />
          <div className="h-3 w-2/3 animate-pulse rounded bg-muted" />
        </div>
      );
    case "Tabs":
      return (
        <div className="w-full max-w-xs text-xs">
          <div className="flex gap-1 border-b">
            <span className="rounded-t border-b-2 border-primary px-2.5 py-1.5 font-medium">
              Board
            </span>
            <span className="px-2.5 py-1.5 text-muted-foreground">Activity</span>
            <span className="px-2.5 py-1.5 text-muted-foreground">Settings</span>
          </div>
          <p className="pt-3 text-muted-foreground">Panel content for the active tab.</p>
        </div>
      );
    case "Accordion":
      return (
        <div className="w-full max-w-xs rounded-md border text-xs">
          <div className="flex items-center justify-between px-3 py-2">
            <span className="font-medium">Is it accessible?</span>
            <span className="text-muted-foreground">⌄</span>
          </div>
          <div className="border-t px-3 py-2 text-muted-foreground">
            Yes. It follows the WAI-ARIA accordion pattern.
          </div>
          <div className="flex items-center justify-between border-t px-3 py-2">
            <span className="font-medium">Is it styled?</span>
            <span className="text-muted-foreground">›</span>
          </div>
        </div>
      );
    case "Dialog":
      return (
        <div className="w-full max-w-xs rounded-lg border bg-card p-4 text-xs shadow-lg">
          <p className="text-sm font-semibold">New task</p>
          <p className="mt-1 text-muted-foreground">Create a task and assign a project.</p>
          <div className="mt-3 flex justify-end gap-2">
            <span className="rounded-md border px-2.5 py-1">Cancel</span>
            <span className="rounded-md bg-primary px-2.5 py-1 font-medium text-primary-foreground">
              Create
            </span>
          </div>
        </div>
      );
    case "Menu":
      return (
        <div className="w-full max-w-[200px] rounded-md border bg-popover p-1 text-xs shadow-md">
          <div className="rounded bg-accent px-2 py-1.5 font-medium">New task</div>
          <div className="rounded px-2 py-1.5 text-muted-foreground">Open</div>
          <div className="my-1 h-px bg-border" />
          <div className="rounded px-2 py-1.5 text-muted-foreground">Settings</div>
        </div>
      );
    case "Toast":
      return (
        <div className="w-full max-w-xs rounded-md border bg-popover p-3 text-xs shadow-md">
          <div className="flex items-start justify-between gap-3">
            <div>
              <strong className="block">Changes saved</strong>
              <span className="text-muted-foreground">Synced just now</span>
            </div>
            <span className="text-muted-foreground">✕</span>
          </div>
        </div>
      );
    case "Breadcrumb":
      return (
        <div className="flex w-full items-center gap-1.5 text-xs text-muted-foreground">
          <span className="hover:text-foreground">Home</span>
          <span>/</span>
          <span className="hover:text-foreground">Components</span>
          <span>/</span>
          <span className="font-medium text-foreground">Breadcrumb</span>
        </div>
      );
    case "EmptyState":
      return (
        <div className="w-full max-w-xs py-4 text-center text-xs">
          <div className="mx-auto mb-2 grid h-9 w-9 place-items-center rounded-full bg-muted text-muted-foreground">
            ⌕
          </div>
          <p className="font-medium">No results</p>
          <p className="mt-0.5 text-muted-foreground">Try another filter.</p>
        </div>
      );
    case "Kbd":
      return (
        <div className="flex w-full items-center gap-2 text-xs">
          <span className="text-muted-foreground">Quick search</span>
          <kbd className="rounded border bg-muted px-1.5 py-0.5 font-mono">⌘</kbd>
          <kbd className="rounded border bg-muted px-1.5 py-0.5 font-mono">K</kbd>
        </div>
      );
    case "Label":
      return (
        <div className="w-full max-w-xs text-xs">
          <p className="font-medium">Email</p>
          <div className="mt-1 rounded border bg-background px-2.5 py-1.5 text-muted-foreground">
            you@example.com
          </div>
        </div>
      );
    case "RadioGroup":
      return (
        <div className="w-full space-y-1.5 text-xs">
          <label className="flex items-center gap-2">
            <span className="h-3.5 w-3.5 rounded-full border-2 border-primary bg-primary/20" />
            Small
          </label>
          <label className="flex items-center gap-2 text-muted-foreground">
            <span className="h-3.5 w-3.5 rounded-full border" />
            Medium
          </label>
          <label className="flex items-center gap-2 text-muted-foreground">
            <span className="h-3.5 w-3.5 rounded-full border" />
            Large
          </label>
        </div>
      );
    case "Slider":
      return (
        <div className="w-full max-w-xs text-xs">
          <div className="relative h-1.5 rounded-full bg-muted">
            <div className="h-full w-[64%] rounded-full bg-primary" />
            <div className="absolute -top-1 left-[64%] h-3.5 w-3.5 -translate-x-1/2 rounded-full border-2 border-primary bg-background" />
          </div>
          <p className="mt-2 text-right text-[11px] text-muted-foreground">64</p>
        </div>
      );
    case "Switch":
      return (
        <div className="flex w-full items-center gap-2.5 text-xs">
          <span className="flex h-5 w-9 items-center rounded-full bg-primary p-0.5">
            <span className="ml-auto h-4 w-4 rounded-full bg-primary-foreground" />
          </span>
          <span className="font-medium">Dark mode</span>
        </div>
      );
    case "Textarea":
      return (
        <div className="w-full max-w-xs text-xs">
          <p className="mb-1 font-medium">Bio</p>
          <div className="h-16 rounded border bg-background px-2.5 py-1.5 text-muted-foreground">
            Tell us about yourself…
          </div>
        </div>
      );
    case "Tooltip":
      return (
        <div className="flex w-full flex-col items-center gap-2 text-xs">
          <span className="rounded bg-primary px-2 py-1 text-[10px] text-primary-foreground">
            More info
          </span>
          <span className="rounded border-b border-dotted px-1 py-0.5">Info</span>
        </div>
      );
    case "Spinner":
      return (
        <div className="flex w-full items-center gap-2.5 text-xs text-muted-foreground">
          <span className="h-5 w-5 animate-spin rounded-full border-2 border-muted border-t-primary" />
          Loading…
        </div>
      );
    default:
      return null;
  }
}

export function ComponentGallery({ items }: { items: ComponentEntry[] }) {
  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {items.map((item) => (
        <a
          key={item.name}
          href={`/components/${item.slug}`}
          className="group flex flex-col rounded-xl border bg-card p-4 transition hover:border-ring/60 hover:shadow-sm"
        >
          <div className="flex items-center justify-between">
            <h3 className="font-medium">{item.name}</h3>
            <span className="rounded-full bg-muted px-2 py-0.5 text-[10px] uppercase tracking-wide text-muted-foreground">
              {item.category}
            </span>
          </div>
          <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
          <div className="mt-4 flex min-h-20 items-center rounded-lg border bg-muted/20 p-3">
            <ComponentPreview name={item.name} />
          </div>
          <code className="mt-3 block truncate rounded bg-muted/50 px-2 py-1 font-mono text-[10px] text-muted-foreground">
            {item.code}
          </code>
          <span className="mt-3 text-xs font-medium text-foreground underline-offset-4 group-hover:underline">
            Read docs →
          </span>
        </a>
      ))}
    </div>
  );
}