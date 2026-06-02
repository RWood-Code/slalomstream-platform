/**
 * SlalomStream Cloud Hub (Phase 6 stub)
 * Registry snapshot hosting + future VOD — not required for venue-day offline operation.
 */
import express from "express";

const app = express();
app.use(express.json());

const PORT = Number(process.env.HUB_PORT ?? 3021);

app.get("/healthz", (_req, res) => {
  res.json({ status: "ok", service: "slalomstream-hub", version: "3.0.0-alpha.1" });
});

/** Placeholder: federations publish signed registry bundles here */
app.get("/registry/:federation/latest", (req, res) => {
  res.json({
    federation: req.params.federation,
    version: "0",
    message: "Upload registry bundles via POST /registry (not implemented in alpha)",
    officials: [],
  });
});

app.listen(PORT, () => {
  console.log(`SlalomStream Hub API http://127.0.0.1:${PORT}`);
});
