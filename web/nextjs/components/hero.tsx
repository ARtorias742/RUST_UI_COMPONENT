import { CopyCommand } from "@/components/copy-command";

export function Hero() {
  return (
    <section className="relative overflow-hidden border-b">
      <div className="pointer-events-none absolute inset-x-0 top-0 h-[420px] grid-bg" />
      <div className="relative mx-auto max-w-6xl px-6 py-24 text-center">
        <span className="inline-flex items-center gap-2 rounded-full border bg-muted/50 px-3 py-1 text-xs text-muted-foreground">
          <span className="h-1.5 w-1.5 rounded-full bg-emerald-500" />
          Phase 4 complete · 15 components
        </span>
        <h1 className="mx-auto mt-6 max-w-3xl text-balance text-5xl font-semibold tracking-tight sm:text-6xl">
          Accessible UI components for Rust frameworks
        </h1>
        <p className="mx-auto mt-6 max-w-2xl text-pretty text-lg text-muted-foreground">
          Renderer-neutral models, source distribution, semantic tokens and a registry CLI.
          One component model for web, SSR and native.
        </p>
        <div className="mx-auto mt-10 grid max-w-2xl gap-3">
          <CopyCommand command="cargo add ferrum-ui" />
          <CopyCommand command="ferrum add button dialog menu tabs toast" />
        </div>
      </div>
    </section>
  );
}