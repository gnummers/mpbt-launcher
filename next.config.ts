import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Static export — Tauri bundles the output/ dir in production.
  // The Next.js dev server is still used during `tauri dev`.
  output: "export",
  images: { unoptimized: true },
};

export default nextConfig;
