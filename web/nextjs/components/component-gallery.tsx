import type { ComponentEntry } from "@/lib/components";
import Link from "next/link";

export function ComponentPreview({ name }: { name: string }) {
  switch (name) {
    case "Button":
      return (
        <div className="flex gap-2">
          <span className="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground">
            Primary
          </span>
          <span className="rounded-md border px-3 py-1.5 text-xs">Outline</span>
        </div>
      );
    case "Badge":
      return (
        <div className="flex gap-2">
          <span className="rounded-full border px-2 py-0.5 text-xs">Default</span>
          <span className="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2 py-0.5 text-xs text-emerald-600 dark:text-emerald-400">
            Success
          </span>
        </div>
      );
    case "Alert":
      return (
        <div className="rounded-md border border-emerald-500/40 bg-emerald-500/10 p-2 text-xs">
          <strong className="block text-emerald-700 dark:text-emerald-300">Saved</strong>
          Synced just now.
        </div>
      );
    case "Avatar":
      return (
        <div className="flex items-center gap-2">
          <span className="grid h-8 w-8 place-items-center rounded-full bg-muted text-xs font-medium">
            AB
          </span>
          <span className="text-xs text-muted-foreground">Ada Lovelace</span>
        </div>
      );
    case "Card":
      return (
        <div className="w-full rounded-md border p-2">
          <p className="text-xs font-semibold">Project</p>
          <p className="text-[11px] text-muted-foreground">Ship it</p>
        </div>
      );
    case "Checkbox":
      return (
        <label className="flex items-center gap-2 text-xs">
          <span className="grid h-4 w-4 place-items-center rounded border bg-primary text-[10px] text-primary-foreground">
            ✓
          </span>
          Accept terms
        </label>
      );
    case "Input":
      return (
        <div className="w-full">
          <p className="mb-1 text-[11px] text-muted-foreground">Email</p>
          <div className="rounded-md border px-2 py-1 text-xs text-muted-foreground">
            you@example.com
          </div>
        </div>
      );
    case "Progress":
      return (
        <div className="w-full">
          <div className="h-2 overflow-hidden rounded-full bg-muted">
            <div className="h-full w-[64%] rounded-full bg-primary" />
          </div>
          <p className="mt-1 text-right text-[11px] text-muted-foreground">64%</p>
        </div>
      );
    case "Separator":
      return <div className="h-px w-full bg-border" />;
    case "Skeleton":
      return (
        <div className="w-full space-y-1">
          <div className="h-3 w-3/4 animate-pulse rounded bg-muted" />
          <div className="h-3 w-1/2 animate-pulse rounded bg-muted" />
        </div>
      );
    case "Tabs":
      return (
        <div className="flex w-full gap-1 border-b pb-1 text-xs">
          <span className="rounded-t border-b-2 border-primary px-2 py-1 font-medium">Board</span>
          <span className="px-2 py-1 text-muted-foreground">Activity</span>
        </div>
      );
    case "Accordion":
      return (
        <div className="w-full rounded-md border text-xs">
          <div className="flex items-center justify-between px-2 py-1.5">
            Is it accessible? <span>⌄</span>
          </div>
          <div className="border-t px-2 py-1.5 text-muted-foreground">Yes.</div>
        </div>
      );
    case "Dialog":
      return (
        <div className="w-full rounded-md border p-2 text-xs shadow-sm">
          <p className="font-semibold">New task</p>
          <p className="text-[11px] text-muted-foreground">Create a task.</p>
        </div>
      );
    case "Menu":
      return (
        <div className="w-full rounded-md border p-1 text-xs shadow-sm">
          <div className="rounded px-2 py-1 hover:bg-accent">New task</div>
          <div className="rounded px-2 py-1 text-muted-foreground">Open</div>
        </div>
      );
    case "Toast":
      return (
        <div className="w-full rounded-md border bg-background p-2 text-xs shadow-sm">
          <strong className="block">Changes saved</strong>
          <span className="text-[11px] text-muted-foreground">Synced just now</span>
        </div>
      );
    case "Breadcrumb":
      return (
        <div className="flex w-full items-center gap-1 text-xs text-muted-foreground">
          <span>Home</span><span>/</span><span className="text-foreground">Docs</span>
        </div>
      );
    case "EmptyState":
      return (
        <div className="w-full py-3 text-center text-xs">
          <p className="font-medium">No results</p>
          <p className="text-[11px] text-muted-foreground">Try another filter</p>
        </div>
      );
    case "Kbd":
      return (
        <div className="flex w-full gap-2 text-xs">
          <kbd className="rounded border px-1.5 py-0.5 font-mono">⌘</kbd>
          <kbd className="rounded border px-1.5 py-0.5 font-mono">K</kbd>
        </div>
      );
    case "Label":
      return (
        <div className="w-full text-xs">
          <p className="font-medium">Email</p>
          <div className="mt-1 rounded border px-2 py-1 text-muted-foreground">you@example.com</div>
        </div>
      );
    case "RadioGroup":
      return (
        <div className="w-full space-y-1 text-xs">
          <label className="flex items-center gap-2"><span className="h-3 w-3 rounded-full border-2 border-primary" />Small</label>
          <label className="flex items-center gap-2 text-muted-foreground"><span className="h-3 w-3 rounded-full border" />Medium</label>
        </div>
      );
    case "Slider":
      return (
        <div className="w-full text-xs">
          <div className="h-1.5 rounded-full bg-muted">
            <div className="h-full w-[64%] rounded-full bg-primary" />
          </div>
          <p className="mt-1 text-right text-[11px] text-muted-foreground">64</p>
        </div>
      );
    case "Switch":
      return (
        <div className="flex w-full items-center gap-2 text-xs">
          <span className="flex h-4 w-7 items-center rounded-full bg-primary p-0.5">
            <span className="ml-auto h-3 w-3 rounded-full bg-primary-foreground" />
          </span>
          Dark mode
        </div>
      );
    case "Textarea":
      return (
        <div className="w-full text-xs">
          <p className="font-medium">Bio</p>
          <div className="mt-1 h-12 rounded border px-2 py-1 text-muted-foreground">Tell us about yourself…</div>
        </div>
      );
    case "Tooltip":
      return (
        <div className="w-full text-xs">
          <span className="rounded border-b border-dotted px-1 py-0.5">More info</span>
          <div className="mt-1 inline-block rounded bg-primary px-2 py-1 text-[10px] text-primary-foreground">
            Tooltip text
          </div>
        </div>
      );
    case "Spinner":
      return (
        <div className="flex w-full items-center gap-2 text-xs text-muted-foreground">
          <span className="h-4 w-4 animate-spin rounded-full border-2 border-muted border-t-primary" />
          Loading
        </div>
      );
    default:
      return null;
  }
}

export function ComponentGallery({ items }: { items: ComponentEntry[] }) {
  return (
    <section id="components" className="mx-auto max-w-6xl px-6 py-20">
      <div className="max-w-2xl">
        <p className="text-sm font-medium text-muted-foreground">Component library</p>
        <h2 className="mt-2 text-3xl font-semibold tracking-tight">
          Twenty-five primitives, one model
        </h2>
        <p className="mt-3 text-muted-foreground">
          Every component ships with props, ARIA, keyboard behavior, a registry template and a
          visual preview.
        </p>
      </div>
      <div className="mt-10 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {items.map((item) => (
          <article
            key={item.name}
            className="flex flex-col rounded-xl border bg-card p-4 transition hover:border-ring/60"
          >
            <div className="flex items-center justify-between">
              <h3 className="font-medium">{item.name}</h3>
              <span className="rounded-full bg-muted px-2 py-0.5 text-[10px] uppercase tracking-wide text-muted-foreground">
                {item.category}
              </span>
            </div>
            <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
            <div className="mt-4 flex min-h-16 items-center rounded-lg border bg-muted/20 p-3">
              <ComponentPreview name={item.name} />
            </div>
            <code className="mt-3 block truncate rounded bg-muted/50 px-2 py-1 font-mono text-[10px] text-muted-foreground">
              {item.code}
            </code>
            <Link
              href={`/components/${item.slug}`}
              className="mt-3 text-xs font-medium text-foreground underline-offset-4 hover:underline"
            >
              Read docs →
            </Link>
          </article>
        ))}
      </div>
    </section>
  );
}