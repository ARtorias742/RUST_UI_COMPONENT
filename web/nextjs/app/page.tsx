import { ComponentGallery } from "@/components/component-gallery";
import { Comparison } from "@/components/comparison";
import { Footer } from "@/components/footer";
import { Hero } from "@/components/hero";
import { Registry } from "@/components/registry";
import { SiteHeader } from "@/components/site-header";
import { components } from "@/lib/components";

export default function Home() {
  return (
    <div className="relative overflow-hidden">
      <SiteHeader />
      <main>
        <Hero />
        <ComponentGallery items={components} />
        <Comparison />
        <Registry />
      </main>
      <Footer />
    </div>
  );
}