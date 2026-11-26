"use client";

import { useEffect, useRef, useState } from "react";
import {
  ACCENTS,
  RADII,
  useTheme,
  type Accent,
  type Theme,
} from "@/components/theme-provider";
import {
  CheckIcon,
  ChevronDownIcon,
  MonitorIcon,
  MoonIcon,
  PaletteIcon,
  SunIcon,
} from "@/components/icons";

const THEMES: { value: Theme; label: string; Icon: typeof SunIcon }[] = [
  { value: "light", label: "Light", Icon: SunIcon },
  { value: "dark", label: "Dark", Icon: MoonIcon },
  { value: "system", label: "System", Icon: MonitorIcon },
];

export function ThemeToggle() {
  const { theme, accent, radius, custom, setTheme, setAccent, setRadius, setCustom } =
    useTheme();
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const onPointer = (event: MouseEvent) => {
      if (ref.current && !ref.current.contains(event.target as Node)) setOpen(false);
    };
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") setOpen(false);
    };
    document.addEventListener("mousedown", onPointer);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onPointer);
      document.removeEventListener("keydown", onKey);
    };
  }, [open]);

  return (
    <div className="relative" ref={ref}>
      <button
        type="button"
        onClick={() => setOpen((value) => !value)}
        aria-haspopup="dialog"
        aria-expanded={open}
        aria-label="Change theme"
        className="inline-flex h-9 w-9 items-center justify-center rounded-md border bg-background text-muted-foreground transition hover:bg-accent hover:text-foreground"
      >
        <PaletteIcon width={17} height={17} />
      </button>

      {open ? (
        <div
          role="dialog"
          aria-label="Theme settings"
          className="animate-in absolute right-0 z-50 mt-2 w-72 origin-top-right rounded-xl border bg-popover p-4 text-popover-foreground shadow-lg"
        >
          <div>
            <p className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
              Theme
            </p>
            <div className="mt-2 grid grid-cols-3 gap-1 rounded-lg bg-muted p-1">
              {THEMES.map(({ value, label, Icon }) => (
                <button
                  key={value}
                  type="button"
                  onClick={() => setTheme(value)}
                  aria-pressed={theme === value}
                  className={`inline-flex items-center justify-center gap-1.5 rounded-md px-2 py-1.5 text-xs font-medium transition ${
                    theme === value
                      ? "bg-background text-foreground shadow-sm"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                >
                  <Icon width={14} height={14} />
                  {label}
                </button>
              ))}
            </div>
          </div>

          <div className="mt-4">
            <p className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
              Accent
            </p>
            <div className="mt-2 flex flex-wrap gap-2">
              {ACCENTS.map((entry) => (
                <button
                  key={entry.value}
                  type="button"
                  onClick={() => setAccent(entry.value as Accent)}
                  aria-label={entry.label}
                  aria-pressed={!custom && accent === entry.value}
                  title={entry.label}
                  className={`h-7 w-7 rounded-full ${entry.className} ring-offset-2 ring-offset-popover transition ${
                    !custom && accent === entry.value
                      ? "ring-2 ring-foreground"
                      : "hover:scale-110"
                  }`}
                />
              ))}
              <label
                className="relative inline-flex h-7 w-7 cursor-pointer items-center justify-center rounded-full border border-dashed text-muted-foreground transition hover:text-foreground"
                title="Custom color"
              >
                <PaletteIcon width={13} height={13} />
                <input
                  type="color"
                  value={custom ?? "#18181b"}
                  onChange={(event) => setCustom(event.target.value)}
                  className="absolute inset-0 cursor-pointer opacity-0"
                  aria-label="Custom accent color"
                />
              </label>
            </div>
            {custom ? (
              <p className="mt-2 font-mono text-[11px] text-muted-foreground">
                custom {custom}
              </p>
            ) : null}
          </div>

          <div className="mt-4">
            <p className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
              Radius
            </p>
            <div className="mt-2 grid grid-cols-4 gap-1 rounded-lg bg-muted p-1">
              {RADII.map((entry) => (
                <button
                  key={entry.value}
                  type="button"
                  onClick={() => setRadius(entry.value)}
                  aria-pressed={radius === entry.value}
                  className={`rounded-md px-1 py-1.5 text-[11px] font-medium transition ${
                    radius === entry.value
                      ? "bg-background text-foreground shadow-sm"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                >
                  {entry.label}
                </button>
              ))}
            </div>
          </div>

          <p className="mt-4 flex items-center gap-1.5 border-t pt-3 text-[11px] text-muted-foreground">
            <CheckIcon width={12} height={12} />
            Saved to this browser
          </p>
        </div>
      ) : null}
    </div>
  );
}

export function ThemeToggleCompact() {
  const { resolved, setTheme } = useTheme();
  const next = resolved === "dark" ? "light" : "dark";
  return (
    <button
      type="button"
      onClick={() => setTheme(next)}
      aria-label={`Switch to ${next} theme`}
      className="inline-flex h-9 w-9 items-center justify-center rounded-md border bg-background text-muted-foreground transition hover:bg-accent hover:text-foreground"
    >
      {resolved === "dark" ? <SunIcon /> : <MoonIcon />}
    </button>
  );
}

export { ChevronDownIcon };