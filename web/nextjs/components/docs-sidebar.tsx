"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { categories, components, type ComponentCategory } from "@/lib/components";

const GROUPS: { label: string; items: { name: string; slug: string }[] }[] = [
  { label: "Overview", items: [{ name: "Introduction", slug: "" }] },
  {
    label: "Components",
    items: components.map((item) => ({ name: item.name, slug: item.slug })),
  },
];

export function DocsSidebar() {
  const pathname = usePathname();

  return (
    <aside className="hidden w-56 shrink-0 lg:block">
      <div className="scroll-thin sticky top-16 max-h-[calc(100vh-4rem)] overflow-y-auto py-8 pr-4">
        <nav className="space-y-6">
          {GROUPS.map((group) => (
            <div key={group.label}>
              <p className="px-2 text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                {group.label}
              </p>
              <ul className="mt-2 space-y-0.5">
                {group.items.map((item) => {
                  const href = item.slug ? `/components/${item.slug}` : "/";
                  const active =
                    item.slug === ""
                      ? pathname === "/"
                      : pathname === `/components/${item.slug}`;
                  return (
                    <li key={item.slug || "intro"}>
                      <Link
                        href={href}
                        className={`block rounded-md px-2 py-1.5 text-sm transition ${
                          active
                            ? "bg-accent font-medium text-accent-foreground"
                            : "text-muted-foreground hover:bg-accent/60 hover:text-foreground"
                        }`}
                      >
                        {item.name}
                      </Link>
                    </li>
                  );
                })}
              </ul>
            </div>
          ))}
        </nav>
      </div>
    </aside>
  );
}

export function CategoryPill({ category }: { category: ComponentCategory }) {
  return (
    <span className="rounded-full border bg-muted px-2 py-0.5 text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
      {category}
    </span>
  );
}

export { categories };