import Link from "next/link";
import { DocsSearch } from "@/components/docs-search";
import { Footer } from "@/components/footer";
import { SiteHeader } from "@/components/site-header";
import { categories, components } from "@/lib/components";

export default function ComponentsPage() {
  return (
    <div className="relative overflow-hidden">
      <SiteHeader />
      <main className="mx-auto max-w-6xl px-6 py-16">
        <p className="text-sm font-medium text-muted-foreground">Component library</p>
        <h1 className="mt-2 text-4xl font-semibold tracking-tight">All components</h1>
        <p className="mt-3 max-w-2xl text-muted-foreground">
          Twenty-five renderer-neutral primitives. Every page includes a preview, usage code,
          props, accessibility notes and keyboard behavior.
        </p>

        <DocsSearch items={components} />

        <div className="mt-12 space-y-12">
          {categories.map((category) => {
            const items = components.filter((item) => item.category === category);
            if (items.length === 0) return null;
            return (
              <section key={category}>
                <h2 className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                  {category}
                </h2>
                <div className="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                  {items.map((item) => (
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
                      <code className="mt-3 block truncate rounded bg-muted/50 px-2 py-1 font-mono text-[10px] text-muted-foreground">
                        {item.code}
                      </code>
                    </Link>
                  ))}
                </div>
              </section>
            );
          })}
        </div>
      </main>
      <Footer />
    </div>
  );
}