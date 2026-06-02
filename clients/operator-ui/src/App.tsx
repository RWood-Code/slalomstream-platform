import { useEffect, useState } from "react";
import { Link, Route, Switch, useLocation } from "wouter";
import { api, connectVenueWs } from "./api";

function Nav() {
  const [loc] = useLocation();
  const link = (href: string, label: string) => (
    <Link href={href} className={loc === href ? "active" : ""}>
      {label}
    </Link>
  );
  return (
    <nav>
      {link("/", "Home")}
      {link("/console", "Console")}
      {link("/media", "Media")}
      {link("/registry", "Registry")}
    </nav>
  );
}

function Home() {
  const [health, setHealth] = useState<string>("…");
  useEffect(() => {
    api<{ status: string; version: string }>("/healthz")
      .then((h) => setHealth(`${h.status} (v${h.version})`))
      .catch((e) => setHealth(String(e)));
  }, []);
  return (
    <div className="card">
      <h1>SlalomStream Venue</h1>
      <p>
        Operator UI for the Rust venue node. Build plan: Phase 5 — connected to{" "}
        <code>venue-node</code> on port 3010.
      </p>
      <p>
        Health: <span className="status-ok">{health}</span>
      </p>
    </div>
  );
}

function Console() {
  const [events, setEvents] = useState<string[]>([]);
  const [tournaments, setTournaments] = useState<unknown[]>([]);

  useEffect(() => {
    api<unknown[]>("/api/tournaments").then(setTournaments).catch(console.error);
    const ws = connectVenueWs((msg) =>
      setEvents((prev) => [msg, ...prev].slice(0, 50)),
    );
    return () => ws.close();
  }, []);

  async function createTournament() {
    const name = prompt("Tournament name?");
    if (!name) return;
    await api("/api/tournaments", {
      method: "POST",
      body: JSON.stringify({ name, judge_count: 3 }),
    });
    setTournaments(await api("/api/tournaments"));
  }

  return (
    <>
      <div className="card">
        <h2>Operator console</h2>
        <button type="button" onClick={createTournament}>
          New tournament
        </button>
        <pre>{JSON.stringify(tournaments, null, 2)}</pre>
      </div>
      <div className="card">
        <h3>Live events (WebSocket)</h3>
        <div className="event-log">
          {events.map((e, i) => (
            <div key={i}>{e}</div>
          ))}
        </div>
      </div>
    </>
  );
}

function Media() {
  const [status, setStatus] = useState<unknown>(null);
  const [preflight, setPreflight] = useState<unknown>(null);

  async function refresh() {
    setStatus(await api("/api/media/status"));
    setPreflight(await api("/api/media/preflight"));
  }

  useEffect(() => {
    refresh().catch(console.error);
  }, []);

  return (
    <div className="card">
      <h2>Media engine</h2>
      <button type="button" onClick={() => api("/api/media/preview/start", { method: "POST" }).then(refresh)}>
        Start preview
      </button>
      <button type="button" onClick={() => api("/api/media/arm", { method: "POST" }).then(refresh)}>
        Arm
      </button>
      <button type="button" className="secondary" onClick={() => api("/api/media/disarm", { method: "POST" }).then(refresh)}>
        Disarm
      </button>
      <h3>Status</h3>
      <pre>{JSON.stringify(status, null, 2)}</pre>
      <h3>Preflight</h3>
      <pre>{JSON.stringify(preflight, null, 2)}</pre>
    </div>
  );
}

function Registry() {
  const [meta, setMeta] = useState<unknown>(null);
  const [officials, setOfficials] = useState<unknown[]>([]);

  useEffect(() => {
    api("/api/registry/meta").then(setMeta).catch(console.error);
    api<unknown[]>("/api/officials").then(setOfficials).catch(console.error);
  }, []);

  async function importSample() {
    const csv = `NZTWSA,Jane,Doe,Canterbury,J2*,2027-12-31
NZTWSA,John,Smith,Auckland,J3,,`;
    await api("/api/registry/import", {
      method: "POST",
      body: JSON.stringify({
        version: "2026-06-03-sample",
        source: "manual-sample",
        csv,
      }),
    });
    setMeta(await api("/api/registry/meta"));
    setOfficials(await api("/api/officials"));
  }

  return (
    <div className="card">
      <h2>Officials registry</h2>
      <p>Versioned import — no hardcoded NZ seed in the binary.</p>
      <button type="button" onClick={importSample}>
        Import sample CSV
      </button>
      <pre>{JSON.stringify(meta, null, 2)}</pre>
      <p>{officials.length} active officials</p>
    </div>
  );
}

export default function App() {
  return (
    <div className="app">
      <Nav />
      <Switch>
        <Route path="/" component={Home} />
        <Route path="/console" component={Console} />
        <Route path="/media" component={Media} />
        <Route path="/registry" component={Registry} />
      </Switch>
    </div>
  );
}
