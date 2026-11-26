import { CopyCommand } from "@/components/copy-command";

const commands = [
  ["list", "ferrum list"],
  ["add", "ferrum add progress"],
  ["diff", "ferrum diff button src/ui/button.rs"],
  ["update", "ferrum update button 0.2.0"],
] as const;

export function Registry() {
  return (
    <section id="registry" className="border-y bg-muted/20">
      <div className="mx-auto grid max-w-content gap-10 px-6 py-20 lg:grid-cols-2">
        <div>
          <p className="text-sm font-medium text-muted-foreground">Distribution</p>
          <h2 className="mt-2 text-3xl font-semibold tracking-tight">
            Source you own, versioned by the registry
          </h2>
          <p className="mt-3 text-muted-foreground">
            <code className="font-mono text-foreground">ferrum add</code> writes the component
            source, CSS and test into your crate. No runtime dependency, no lock-in.
          </p>
          <ul className="mt-6 space-y-2 text-sm text-muted-foreground">
            <li>• Path-traversal guard and idempotent writes</li>
            <li>• Semver updates with deprecation warnings</li>
            <li>• JSON registry with built-in manifests</li>
          </ul>
        </div>
        <div className="grid gap-3">
          {commands.map(([label, command]) => (
            <div key={label}>
              <p className="mb-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
                {label}
              </p>
              <CopyCommand command={command} />
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}