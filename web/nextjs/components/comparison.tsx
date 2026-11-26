const rows = [
  ["Distribution", "Copy-source + JSON registry", "Copy-source + JSON registry + ferrum CLI"],
  ["Primitives", "Radix UI + Tailwind", "Renderer-neutral ferrum-ui-core + tokens"],
  ["Variants", "class-variance-authority", "Typed enums + builder methods"],
  ["Theming", "CSS variables", "Semantic tokens + light/dark/RTL"],
  ["A11y", "Radix behavior", "Focus scope, roving tabindex, ARIA models"],
  ["Targets", "Web only", "Web, SSR, native style parity"],
  ["Docs", "Full shadcn/ui site", "Next.js site + SSR previews + axe matrix"],
  ["Maturity", "Production", "Alpha foundation"],
] as const;

export function Comparison() {
  return (
    <section id="comparison" className="border-y bg-muted/20">
      <div className="mx-auto max-w-content px-6 py-20">
        <div className="max-w-2xl">
          <p className="text-sm font-medium text-muted-foreground">Comparison</p>
          <h2 className="mt-2 text-3xl font-semibold tracking-tight">
            shadcn/ui → Rust, is it possible?
          </h2>
          <p className="mt-3 text-muted-foreground">
            Yes. The architecture maps cleanly: copy-source distribution, headless behavior,
            variant data and CSS-variable tokens all have direct Rust equivalents. The gap is
            ecosystem maturity, not feasibility.
          </p>
        </div>

        <div className="mt-10 overflow-hidden rounded-xl border">
          <table className="w-full text-left text-sm">
            <thead className="bg-muted/50 text-xs uppercase tracking-wide text-muted-foreground">
              <tr>
                <th className="px-4 py-3">Concern</th>
                <th className="px-4 py-3">shadcn/ui</th>
                <th className="px-4 py-3">Ferrum UI</th>
              </tr>
            </thead>
            <tbody>
              {rows.map(([concern, shadcn, ferrum]) => (
                <tr key={concern} className="border-t">
                  <td className="px-4 py-3 font-medium">{concern}</td>
                  <td className="px-4 py-3 text-muted-foreground">{shadcn}</td>
                  <td className="px-4 py-3 text-muted-foreground">{ferrum}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="mt-6 rounded-xl border bg-background p-5">
          <p className="font-medium">What is not a 100% clone</p>
          <p className="mt-1 text-sm text-muted-foreground">
            Full docs search, the complete ~50-component inventory, Figma parity, visual
            regression baselines, published releases and an audited supply chain are still
            missing. The component model and design language are production-grade; the
            surrounding ecosystem is not yet at shadcn/ui maturity.
          </p>
        </div>
      </div>
    </section>
  );
}