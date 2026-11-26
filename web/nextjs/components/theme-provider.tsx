"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";

export type Theme = "light" | "dark" | "system";
export type Accent =
  | "zinc"
  | "slate"
  | "blue"
  | "green"
  | "rose"
  | "orange"
  | "violet";

export const ACCENTS: { value: Accent; label: string; className: string }[] = [
  { value: "zinc", label: "Zinc", className: "bg-[hsl(240_5.9%_10%)]" },
  { value: "slate", label: "Slate", className: "bg-[hsl(222.2_47.4%_11.2%)]" },
  { value: "blue", label: "Blue", className: "bg-[hsl(221.2_83.2%_53.3%)]" },
  { value: "green", label: "Green", className: "bg-[hsl(142.1_76.2%_36.3%)]" },
  { value: "rose", label: "Rose", className: "bg-[hsl(346.8_77.2%_49.8%)]" },
  { value: "orange", label: "Orange", className: "bg-[hsl(24.6_95%_53.1%)]" },
  { value: "violet", label: "Violet", className: "bg-[hsl(262.1_83.3%_57.8%)]" },
];

export const RADII: { value: string; label: string }[] = [
  { value: "0rem", label: "None" },
  { value: "0.25rem", label: "Small" },
  { value: "0.5rem", label: "Medium" },
  { value: "0.75rem", label: "Large" },
];

const STORAGE_KEY = "ferrum-theme";

type ThemeState = {
  theme: Theme;
  accent: Accent;
  radius: string;
  custom: string | null;
};

type ThemeContextValue = ThemeState & {
  resolved: "light" | "dark";
  setTheme: (theme: Theme) => void;
  setAccent: (accent: Accent) => void;
  setRadius: (radius: string) => void;
  setCustom: (custom: string | null) => void;
};

const DEFAULT_STATE: ThemeState = {
  theme: "system",
  accent: "zinc",
  radius: "0.5rem",
  custom: null,
};

const ThemeContext = createContext<ThemeContextValue | null>(null);

/** Convert a #rrggbb string to HSL components. */
function hexToHsl(hex: string): { h: number; s: number; l: number } {
  const clean = hex.replace("#", "");
  const r = parseInt(clean.slice(0, 2), 16) / 255;
  const g = parseInt(clean.slice(2, 4), 16) / 255;
  const b = parseInt(clean.slice(4, 6), 16) / 255;
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const l = (max + min) / 2;
  let h = 0;
  let s = 0;
  if (max !== min) {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    switch (max) {
      case r:
        h = (g - b) / d + (g < b ? 6 : 0);
        break;
      case g:
        h = (b - r) / d + 2;
        break;
      default:
        h = (r - g) / d + 4;
    }
    h /= 6;
  }
  return { h: Math.round(h * 360), s: Math.round(s * 100), l: Math.round(l * 100) };
}

function applyState(state: ThemeState, resolved: "light" | "dark") {
  const root = document.documentElement;
  root.classList.toggle("dark", resolved === "dark");
  root.dataset.accent = state.accent;
  root.style.setProperty("--radius", state.radius);

  if (state.custom) {
    const { h, s, l } = hexToHsl(state.custom);
    root.style.setProperty("--primary", `${h} ${s}% ${l}%`);
    root.style.setProperty("--ring", `${h} ${s}% ${Math.min(l + 10, 90)}%`);
    root.style.setProperty(
      "--primary-foreground",
      l > 62 ? "240 5.9% 10%" : "0 0% 98%",
    );
  } else {
    root.style.removeProperty("--primary");
    root.style.removeProperty("--ring");
    root.style.removeProperty("--primary-foreground");
  }
}

function systemPrefersDark() {
  return (
    typeof window !== "undefined" &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
  );
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<ThemeState>(DEFAULT_STATE);
  const [systemDark, setSystemDark] = useState(false);
  const [mounted, setMounted] = useState(false);

  // Hydrate from storage after mount so SSR markup stays stable.
  useEffect(() => {
    try {
      const raw = window.localStorage.getItem(STORAGE_KEY);
      if (raw) {
        const parsed = JSON.parse(raw) as Partial<ThemeState>;
        setState({
          theme: parsed.theme ?? DEFAULT_STATE.theme,
          accent: parsed.accent ?? DEFAULT_STATE.accent,
          radius: parsed.radius ?? DEFAULT_STATE.radius,
          custom: parsed.custom ?? null,
        });
      }
    } catch {
      /* ignore malformed storage */
    }
    setSystemDark(systemPrefersDark());
    setMounted(true);
  }, []);

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = (event: MediaQueryListEvent) => setSystemDark(event.matches);
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);

  const resolved: "light" | "dark" =
    state.theme === "system" ? (systemDark ? "dark" : "light") : state.theme;

  useEffect(() => {
    if (!mounted) return;
    applyState(state, resolved);
    try {
      window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch {
      /* storage may be unavailable */
    }
  }, [state, resolved, mounted]);

  const setTheme = useCallback((theme: Theme) => setState((s) => ({ ...s, theme })), []);
  const setAccent = useCallback(
    (accent: Accent) => setState((s) => ({ ...s, accent, custom: null })),
    [],
  );
  const setRadius = useCallback((radius: string) => setState((s) => ({ ...s, radius })), []);
  const setCustom = useCallback(
    (custom: string | null) => setState((s) => ({ ...s, custom })),
    [],
  );

  const value = useMemo<ThemeContextValue>(
    () => ({ ...state, resolved, setTheme, setAccent, setRadius, setCustom }),
    [state, resolved, setTheme, setAccent, setRadius, setCustom],
  );

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}

export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return context;
}