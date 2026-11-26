export function Footer() {
  return (
    <footer className="border-t">
      <div className="mx-auto flex max-w-6xl flex-col items-start justify-between gap-4 px-6 py-10 text-sm text-muted-foreground sm:flex-row sm:items-center">
        <p>Ferrum UI · renderer-neutral components for Rust frameworks.</p>
        <div className="flex gap-4">
          <a href="#components" className="hover:text-foreground">
            Components
          </a>
          <a href="#registry" className="hover:text-foreground">
            Registry
          </a>
          <a
            href="https://github.com/ARtorias742/RUST_Frontend_Framework"
            target="_blank"
            rel="noreferrer"
            className="hover:text-foreground"
          >
            Framework
          </a>
        </div>
      </div>
    </footer>
  );
}