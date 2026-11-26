import Link from "next/link";
import { DocsSidebar } from "@/components/docs-sidebar";
import { Footer } from "@/components/footer";
import { SiteHeader } from "@/components/site-header";
import { components } from "@/lib/components";

export default function ComponentsPage() {
  return (
    <div className="flex min-h-screen flex-col">
      <SiteHeader />
      <div className="mx-auto flex w-full max-w-content flex-1 gap-8 px-4 sm:px-6">
        <DocsSidebar />
        <main className="min-w-0 flex-1 py-10">
          <p className="text-sm font-medium text-muted-foreground">Component library</p>
          <h1 className="mt-2 text-3xl font-semibold tracking-tight sm:text-4xl">
            All components
          </h1>
          <p className="mt-3 max-w-2xl text-muted-foreground">
            Twenty-five renderer-neutral primitives. Every page includes a live preview, usage
            code, a full props table, accessibility notes and keyboard behavior.
          </p>

          <div className="mt-8 grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
            {components.map((item) => (
              <Link
                key={item.slug}
                href={`/components/${item.slug}`}
                className="group rounded-xl border bg-card p-4 transition hover:border-ring/60 hover:shadow-sm"
              >
                <div className="flex items-center justify-between">
                  <span className="font-medium">{item.name}</span>
                  <span className="rounded-full bg-muted px-2 py-0.5 text-[10px] uppercase tracking-wide text-muted-foreground">
                    {item.category}
                  </span>
                </div>
                <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
                <code className="mt-3 block truncate rounded bg-muted/50 px-2 py-1 font-mono text-[10px] text-muted-foreground">
                  {item.code}
                </code>
              </Link>
            ))}
          </div>
        </main>
      </div>
      <Footer />
    </div>
  );
}