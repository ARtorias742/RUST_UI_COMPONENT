export function SiteHeader() {
  return (
    <header className="sticky top-0 z-40 border-b bg-background/80 backdrop-blur">
      <div className="mx-auto flex h-16 max-w-6xl items-center justify-between px-6">
        <a href="/" className="flex items-center gap-2 font-semibold tracking-tight">
          <span className="grid h-8 w-8 place-items-center rounded-md bg-primary text-primary-foreground">
            F
          </span>
          Ferrum UI
        </a>
        <nav className="flex items-center gap-6 text-sm text-muted-foreground">
          <a href="/components" className="transition hover:text-foreground">
            Components
          </a>
          <a href="#comparison" className="transition hover:text-foreground">
            Comparison
          </a>
          <a href="#registry" className="transition hover:text-foreground">
            Registry
          </a>
          <a
            href="https://github.com/ARtorias742/RUST_Frontend_Framework"
            target="_blank"
            rel="noreferrer"
            className="rounded-md border px-3 py-1.5 text-foreground transition hover:bg-accent"
          >
            GitHub
          </a>
        </nav>
      </div>
    </header>
  );
}