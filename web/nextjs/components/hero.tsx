import { CopyCommand } from "@/components/copy-command";

const stats = [
  { value: "25", label: "components" },
  { value: "5", label: "categories" },
  { value: "0", label: "runtime deps" },
  { value: "100%", label: "typed props" },
];

export function Hero() {
  return (
    <section className="relative overflow-hidden border-b">
      <div className="pointer-events-none absolute inset-x-0 top-0 h-[420px] grid-bg" />
      <div className="relative mx-auto max-w-content px-4 py-24 text-center sm:px-6">
        <span className="inline-flex items-center gap-2 rounded-full border bg-background/70 px-3 py-1 text-xs text-muted-foreground backdrop-blur">
          <span className="h-1.5 w-1.5 rounded-full bg-emerald-500" />
          v0.1.0 · 25 accessible primitives
        </span>
        <h1 className="mx-auto mt-6 max-w-3xl text-balance text-4xl font-semibold tracking-tight sm:text-6xl">
          Accessible UI components for Rust frameworks
        </h1>
        <p className="mx-auto mt-6 max-w-2xl text-pretty text-lg text-muted-foreground">
          Renderer-neutral models, source distribution, semantic tokens and a registry CLI. One
          component model for web, SSR and native.
        </p>
        <div className="mx-auto mt-10 grid max-w-xl gap-3">
          <CopyCommand command="cargo add ferrum-ui" />
          <CopyCommand command="ferrum add button dialog menu tabs toast" />
        </div>

        <dl className="mx-auto mt-14 grid max-w-2xl grid-cols-2 gap-px overflow-hidden rounded-xl border bg-border sm:grid-cols-4">
          {stats.map((stat) => (
            <div key={stat.label} className="bg-background px-4 py-5">
              <dt className="text-xs uppercase tracking-wide text-muted-foreground">
                {stat.label}
              </dt>
              <dd className="mt-1 text-2xl font-semibold tracking-tight">{stat.value}</dd>
            </div>
          ))}
        </dl>
      </div>
    </section>
  );
}