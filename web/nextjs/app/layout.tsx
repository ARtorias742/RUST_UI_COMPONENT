import type { Metadata } from "next";
import "./globals.css";
import { ThemeProvider } from "@/components/theme-provider";

export const metadata: Metadata = {
  title: {
    default: "Ferrum UI — accessible components for Rust",
    template: "%s — Ferrum UI",
  },
  description:
    "Renderer-neutral, source-distributed UI components for Rust frameworks. 25 accessible primitives with a registry CLI, light/dark themes and full docs.",
};

// Applies the saved theme before first paint to avoid a flash of the wrong theme.
const themeScript = `
(function(){
  try {
    var raw = localStorage.getItem('ferrum-theme');
    var s = raw ? JSON.parse(raw) : {};
    var t = s.theme || 'system';
    var dark = t === 'dark' || (t === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
    var r = document.documentElement;
    if (dark) r.classList.add('dark');
    r.dataset.accent = s.accent || 'zinc';
    if (s.radius) r.style.setProperty('--radius', s.radius);
    if (s.custom) {
      var h = s.custom.replace('#','');
      var rr = parseInt(h.slice(0,2),16)/255, gg = parseInt(h.slice(2,4),16)/255, bb = parseInt(h.slice(4,6),16)/255;
      var max = Math.max(rr,gg,bb), min = Math.min(rr,gg,bb), l = (max+min)/2, hh = 0, ss = 0;
      if (max !== min) {
        var d = max-min;
        ss = l > 0.5 ? d/(2-max-min) : d/(max+min);
        if (max === rr) hh = (gg-bb)/d + (gg < bb ? 6 : 0);
        else if (max === gg) hh = (bb-rr)/d + 2;
        else hh = (rr-gg)/d + 4;
        hh /= 6;
      }
      hh = Math.round(hh*360); ss = Math.round(ss*100); l = Math.round(l*100);
      r.style.setProperty('--primary', hh+' '+ss+'% '+l+'%');
      r.style.setProperty('--ring', hh+' '+ss+'% '+Math.min(l+10,90)+'%');
      r.style.setProperty('--primary-foreground', l > 62 ? '240 5.9% 10%' : '0 0% 98%');
    }
  } catch (e) {}
})();
`;

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <script dangerouslySetInnerHTML={{ __html: themeScript }} />
      </head>
      <body className="min-h-screen antialiased">
        <ThemeProvider>{children}</ThemeProvider>
      </body>
    </html>
  );
}