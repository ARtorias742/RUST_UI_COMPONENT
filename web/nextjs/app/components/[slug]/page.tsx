import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { ComponentPreview } from "@/components/component-gallery";
import { CopyCommand } from "@/components/copy-command";
import { Footer } from "@/components/footer";
import { SiteHeader } from "@/components/site-header";
import { components } from "@/lib/components";

type PageProps = { params: { slug: string } };

export function generateStaticParams() {
  return components.map((item) => ({ slug: item.slug }));
}

export function generateMetadata({ params }: PageProps): Metadata {
  const item = components.find((entry) => entry.slug === params.slug);
  if (!item) return { title: "Component not found" };
  return {
    title: `${item.name} — Ferrum UI`,
    description: item.description,
  };
}

export default function ComponentPage({ params }: PageProps) {
  const item = components.find((entry) => entry.slug === params.slug);
  if (!item) notFound();

  return (
    <div className="relative overflow-hidden">
      <SiteHeader />
      <main className="mx-auto max-w-4xl px-6 py-16">
        <Link href="/components" className="text-sm text-muted-foreground hover:text-foreground">
          ← All components
        </Link>

        <div className="mt-6 flex flex-wrap items-center gap-3">
          <h1 className="text-4xl font-semibold tracking-tight">{item.name}</h1>
          <span className="rounded-full bg-muted px-2.5 py-1 text-xs uppercase tracking-wide text-muted-foreground">
            {item.category}
          </span>
        </div>
        <p className="mt-3 text-lg text-muted-foreground">{item.description}</p>

        <section className="mt-10">
          <h2 className="text-sm font-semibold uppercase tracking-widest text-muted-foreground">
            Preview
          </h2>
          <div className="mt-3 flex min-h-40 items-center rounded-xl border bg-muted/20 p-8">
            <ComponentPreview name={item.name} />
          </div>
        </section>

        <section className="mt-10">
          <h2 className="text-sm font-semibold uppercase tracking-widest text-muted-foreground">
            Usage
          </h2>
          <pre className="mt-3 overflow-x-auto rounded-xl border bg-muted/30 p-4 font-mono text-sm">
            <code>{item.code}</code>
          </pre>
          <div className="mt-3">
            <CopyCommand command={`ferrum add ${item.slug}`} />
          </div>
        </section>

        <section className="mt-10">
          <h2 className="text-sm font-semibold uppercase tracking-widest text-muted-foreground">
            Props
          </h2>
          <div className="mt-3 overflow-hidden rounded-xl border">
            <table className="w-full text-left text-sm">
              <tbody>
                {item.props.map((prop) => (
                  <tr key={prop} className="border-t first:border-t-0">
                    <td className="px-4 py-2 font-mono text-xs">{prop}</td>
                    <td className="px-4 py-2 text-muted-foreground">See the Rust type definition</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>

        <div className="mt-10 grid gap-4 sm:grid-cols-2">
          <section className="rounded-xl border p-5">
            <h2 className="text-sm font-semibold uppercase tracking-widest text-muted-foreground">
              Accessibility
            </h2>
            <p className="mt-2 text-sm text-muted-foreground">{item.a11y}</p>
          </section>
          <section className="rounded-xl border p-5">
            <h2 className="text-sm font-semibold uppercase tracking-widest text-muted-foreground">
              Keyboard
            </h2>
            <p className="mt-2 text-sm text-muted-foreground">{item.keyboard}</p>
          </section>
        </div>
      </main>
      <Footer />
    </div>
  );
}