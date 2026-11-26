import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Ferrum UI — accessible components for Rust",
  description:
    "Renderer-neutral, source-distributed UI components for Rust frameworks. 15 accessible primitives with a registry CLI.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen antialiased">{children}</body>
    </html>
  );
}