import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { CopyButton } from "@/components/copy-button";
import { DocsSidebar } from "@/components/docs-sidebar";
import { Footer } from "@/components/footer";
import { PageToc, type TocItem } from "@/components/page-toc";
import { PreviewTabs } from "@/components/preview-tabs";
import { SiteHeader } from "@/components/site-header";
import { ArrowLeftIcon, ArrowRightIcon } from "@/components/icons";
import { components, getAdjacent, getComponent } from "@/lib/components";

type PageProps = { params: { slug: string } };

export function generateStaticParams() {
  return components.map((item) => ({ slug: item.slug }));
}

export function generateMetadata({ params }: PageProps): Metadata {
  const item = getComponent(params.slug);
  if (!item) return { title: "Component not found" };
  return {
    title: item.name,
    description: item.description,
  };
}

const TOC: TocItem[] = [
  { id: "preview", label: "Preview" },
  { id: "usage", label: "Usage" },
  { id: "examples", label: "Examples" },
  { id: "api", label: "API reference" },
  { id: "accessibility", label: "Accessibility" },
];

export default function ComponentPage({ params }: PageProps) {
  const item = getComponent(params.slug);
  if (!item) notFound();

  const { previous, next } = getAdjacent(item.slug);
  const pageMarkdown = [
    `# ${item.name}`,
    "",
    item.description,
    "",
    "## Usage",
    "```rust",
    item.code,
    "```",
    "",
    "## API",
    ...item.props.map((prop) => `- \`${prop.name}\` (${prop.type}, default ${prop.default}): ${prop.description}`),
    "",
    "## Accessibility",
    ...item.a11y.map((line) => `- ${line}`),
    "",
    "## Keyboard",
    ...item.keyboard.map((line) => `- ${line}`),
  ].join("\n");

  return (
    <div className="flex min-h-screen flex-col">
      <SiteHeader />

      <div className="border-b bg-muted/20">
        <div className="mx-auto max-w-content px-4 py-8 sm:px-6 sm:py-10">
          <nav className="flex items-center gap-1.5 text-xs text-muted-foreground">
            <Link href="/components" className="hover:text-foreground">
              Components
            </Link>
            <span>/</span>
            <span className="text-foreground">{item.name}</span>
          </nav>

          <div className="mt-4 flex flex-wrap items-start justify-between gap-4">
            <div>
              <div className="flex flex-wrap items-center gap-3">
                <h1 className="text-3xl font-semibold tracking-tight sm:text-4xl">
                  {item.name}
                </h1>
                <span className="rounded-full border bg-background px-2.5 py-0.5 text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
                  {item.category}
                </span>
              </div>
              <p className="mt-2 max-w-2xl text-muted-foreground">{item.description}</p>
            </div>
            <div className="flex items-center gap-2">
              <CopyButton value={pageMarkdown} label="Copy page" />
              <CopyButton value={`ferrum add ${item.slug}`} label="Install" />
            </div>
          </div>
        </div>
      </div>

      <div className="mx-auto flex w-full max-w-content flex-1 gap-8 px-4 sm:px-6">
        <DocsSidebar />

        <main className="min-w-0 flex-1 py-10">
          <section id="preview" className="scroll-mt-24">
            <PreviewTabs name={item.name} code={item.code} />
          </section>

          <section id="usage" className="mt-12 scroll-mt-24">
            <h2 className="text-xl font-semibold tracking-tight">Usage</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              Add the component to your crate, then construct it with the typed props below.
            </p>
            <div className="mt-4 overflow-hidden rounded-xl border">
              <div className="flex items-center justify-between border-b bg-muted/40 px-4 py-2">
                <span className="font-mono text-xs text-muted-foreground">terminal</span>
                <CopyButton value={`ferrum add ${item.slug}`} label="Copy" />
              </div>
              <pre className="code-block overflow-x-auto p-4">
                <code>
                  <span className="select-none text-muted-foreground">$ </span>
                  ferrum add {item.slug}
                </code>
              </pre>
            </div>
            <div className="mt-3 overflow-hidden rounded-xl border">
              <div className="flex items-center justify-between border-b bg-muted/40 px-4 py-2">
                <span className="font-mono text-xs text-muted-foreground">rust</span>
                <CopyButton value={item.code} label="Copy" />
              </div>
              <pre className="code-block overflow-x-auto p-4">
                <code>{item.code}</code>
              </pre>
            </div>
          </section>

          <section id="examples" className="mt-12 scroll-mt-24">
            <h2 className="text-xl font-semibold tracking-tight">Examples</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              Common configurations and states.
            </p>
            <div className="mt-4 space-y-4">
              {item.examples.map((example) => (
                <div key={example.title} className="rounded-xl border bg-card">
                  <div className="border-b px-4 py-3">
                    <p className="text-sm font-medium">{example.title}</p>
                    <p className="text-xs text-muted-foreground">{example.description}</p>
                  </div>
                  <div className="flex items-center justify-between gap-3 px-4 py-3">
                    <pre className="code-block min-w-0 flex-1 overflow-x-auto">
                      <code>{example.code}</code>
                    </pre>
                    <CopyButton value={example.code} label="Copy" />
                  </div>
                </div>
              ))}
            </div>
          </section>

          <section id="api" className="mt-12 scroll-mt-24">
            <h2 className="text-xl font-semibold tracking-tight">API reference</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              Every prop is typed; invalid combinations are rejected at compile time.
            </p>
            <div className="mt-4 overflow-x-auto rounded-xl border">
              <table className="w-full min-w-[640px] text-left text-sm">
                <thead className="bg-muted/50 text-xs uppercase tracking-wide text-muted-foreground">
                  <tr>
                    <th className="px-4 py-3 font-medium">Prop</th>
                    <th className="px-4 py-3 font-medium">Type</th>
                    <th className="px-4 py-3 font-medium">Default</th>
                    <th className="px-4 py-3 font-medium">Description</th>
                  </tr>
                </thead>
                <tbody>
                  {item.props.map((prop) => (
                    <tr key={prop.name} className="border-t">
                      <td className="px-4 py-3 font-mono text-xs">{prop.name}</td>
                      <td className="px-4 py-3 font-mono text-xs text-muted-foreground">
                        {prop.type}
                      </td>
                      <td className="px-4 py-3 font-mono text-xs text-muted-foreground">
                        {prop.default}
                      </td>
                      <td className="px-4 py-3 text-muted-foreground">{prop.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>

          <section id="accessibility" className="mt-12 scroll-mt-24">
            <h2 className="text-xl font-semibold tracking-tight">Accessibility</h2>
            <div className="mt-4 grid gap-4 md:grid-cols-2">
              <div className="rounded-xl border bg-card p-5">
                <h3 className="text-sm font-semibold">Screen readers</h3>
                <ul className="mt-2 space-y-1.5 text-sm text-muted-foreground">
                  {item.a11y.map((line) => (
                    <li key={line} className="flex gap-2">
                      <span aria-hidden className="text-emerald-500">
                        ✓
                      </span>
                      <span>{line}</span>
                    </li>
                  ))}
                </ul>
              </div>
              <div className="rounded-xl border bg-card p-5">
                <h3 className="text-sm font-semibold">Keyboard</h3>
                <ul className="mt-2 space-y-1.5 text-sm text-muted-foreground">
                  {item.keyboard.map((line) => (
                    <li key={line} className="flex gap-2">
                      <span aria-hidden className="text-sky-500">
                        ⌨
                      </span>
                      <span>{line}</span>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          </section>

          <nav className="mt-14 grid gap-3 border-t pt-6 sm:grid-cols-2">
            {previous ? (
              <Link
                href={`/components/${previous.slug}`}
                className="group rounded-xl border p-4 transition hover:border-ring/60 hover:bg-accent/30"
              >
                <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
                  <ArrowLeftIcon width={13} height={13} />
                  Previous
                </span>
                <span className="mt-1 block font-medium">{previous.name}</span>
              </Link>
            ) : (
              <span />
            )}
            {next ? (
              <Link
                href={`/components/${next.slug}`}
                className="group rounded-xl border p-4 text-right transition hover:border-ring/60 hover:bg-accent/30"
              >
                <span className="flex items-center justify-end gap-1.5 text-xs text-muted-foreground">
                  Next
                  <ArrowRightIcon width={13} height={13} />
                </span>
                <span className="mt-1 block font-medium">{next.name}</span>
              </Link>
            ) : null}
          </nav>
        </main>

        <PageToc items={TOC} />
      </div>

      <Footer />
    </div>
  );
}