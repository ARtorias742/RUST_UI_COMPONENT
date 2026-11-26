import { ComponentGallery } from "@/components/component-preview";
import { Comparison } from "@/components/comparison";
import { Footer } from "@/components/footer";
import { Hero } from "@/components/hero";
import { Registry } from "@/components/registry";
import { SiteHeader } from "@/components/site-header";
import { components } from "@/lib/components";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col">
      <SiteHeader />
      <main className="flex-1">
        <Hero />
        <section id="components" className="mx-auto max-w-content px-4 py-20 sm:px-6">
          <div className="max-w-2xl">
            <p className="text-sm font-medium text-muted-foreground">Component library</p>
            <h2 className="mt-2 text-3xl font-semibold tracking-tight">
              Twenty-five primitives, one model
            </h2>
            <p className="mt-3 text-muted-foreground">
              Every component ships with typed props, ARIA, keyboard behavior, a registry
              template and a live preview. Pick one to read the full docs.
            </p>
          </div>
          <div className="mt-10">
            <ComponentGallery items={components} />
          </div>
        </section>
        <Comparison />
        <Registry />
      </main>
      <Footer />
    </div>
  );
}