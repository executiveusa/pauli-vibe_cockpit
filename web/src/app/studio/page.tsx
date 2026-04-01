"use client";

const sections = [
  {
    title: "Conversation Shell",
    body: "Primary voice/chat surface for directing the system in natural language.",
  },
  {
    title: "Live Activity",
    body: "Shows the current run, active tool, active agent, and execution progress.",
  },
  {
    title: "Agent Theater",
    body: "Reserved for live computer-use frames, browser context, and visible work playback.",
  },
  {
    title: "Approvals",
    body: "Approval inbox for writes, deploys, public actions, and other gated mutations.",
  },
  {
    title: "Artifacts",
    body: "Project reviews, trajectory plans, diff summaries, images, and commit/PR outputs.",
  },
  {
    title: "Workspace Control",
    body: "Workspace model/provider selection, white-label branding, and backend access state.",
  },
];

export default function StudioPage() {
  return (
    <div style={{ display: "grid", gap: 20 }}>
      <section
        style={{
          border: "1px solid #2b3345",
          borderRadius: 18,
          padding: 24,
          background:
            "radial-gradient(circle at top right, rgba(96,165,250,0.16), rgba(17,24,39,0.98) 55%)",
        }}
      >
        <div style={{ color: "#93c5fd", fontSize: 12, letterSpacing: "0.18em", textTransform: "uppercase" }}>
          Studio Shell
        </div>
        <h1 style={{ margin: "10px 0 8px", fontSize: 36 }}>Jarvis-style operator cockpit</h1>
        <p style={{ margin: 0, color: "#cbd5e1", maxWidth: 880, lineHeight: 1.6 }}>
          This route is the frontend anchor for the universal operator experience: one conversation surface,
          one live execution view, and one approval/artifact layer powered by ArchonX as the backend system of record.
        </p>
      </section>

      <section style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(240px, 1fr))", gap: 16 }}>
        {sections.map((section) => (
          <div
            key={section.title}
            style={{
              border: "1px solid #243041",
              borderRadius: 16,
              padding: 18,
              backgroundColor: "#0f172a",
              boxShadow: "0 12px 32px rgba(0,0,0,0.24)",
            }}
          >
            <h2 style={{ marginTop: 0, fontSize: 18 }}>{section.title}</h2>
            <p style={{ marginBottom: 0, color: "#94a3b8", lineHeight: 1.55 }}>{section.body}</p>
          </div>
        ))}
      </section>

      <section
        style={{
          border: "1px dashed #334155",
          borderRadius: 16,
          padding: 20,
          backgroundColor: "#111827",
        }}
      >
        <h2 style={{ marginTop: 0 }}>First integration targets</h2>
        <ul style={{ color: "#cbd5e1", lineHeight: 1.8, marginBottom: 0 }}>
          <li>Bind this shell to ArchonX run status, workspace policy, and provider routing.</li>
          <li>Add MCP App surfaces for run monitor, project review, repo actions, and asset mapping.</li>
          <li>Add live theater streaming for computer-use visibility.</li>
          <li>Add white-label theme packs so the same shell can be deployed per client.</li>
        </ul>
      </section>
    </div>
  );
}
