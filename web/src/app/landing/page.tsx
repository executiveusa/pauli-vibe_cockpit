export default function LandingPage() {
  return (
    <main
      style={{
        minHeight: "100vh",
        padding: "72px 24px",
        background:
          "radial-gradient(circle at 20% 20%, rgba(59,130,246,0.16), transparent 35%), radial-gradient(circle at 80% 10%, rgba(168,85,247,0.14), transparent 28%), linear-gradient(180deg, #030712 0%, #0b1020 100%)",
        color: "#f8fafc",
      }}
    >
      <section style={{ maxWidth: 1200, margin: "0 auto", display: "grid", gap: 28 }}>
        <div style={{ color: "#93c5fd", fontSize: 12, letterSpacing: "0.22em", textTransform: "uppercase" }}>
          ArchonX Operating System
        </div>
        <h1 style={{ margin: 0, fontSize: "clamp(44px, 7vw, 88px)", lineHeight: 0.95, maxWidth: 900 }}>
          A cinematic backend for voice-first agents, branded worlds, and white-label operator products.
        </h1>
        <p style={{ margin: 0, maxWidth: 820, color: "#cbd5e1", fontSize: 20, lineHeight: 1.6 }}>
          Build the public shell anywhere. Point it at ArchonX. Let users bring their own provider,
          talk naturally to the system, and watch agents work in real time.
        </p>

        <div
          style={{
            display: "grid",
            gridTemplateColumns: "1.1fr 0.9fr",
            gap: 24,
            alignItems: "stretch",
          }}
        >
          <div
            style={{
              borderRadius: 28,
              padding: 32,
              border: "1px solid rgba(148,163,184,0.16)",
              background: "rgba(15,23,42,0.76)",
              backdropFilter: "blur(18px)",
            }}
          >
            <h2 style={{ marginTop: 0, fontSize: 28 }}>Brand-first narrative</h2>
            <p style={{ color: "#cbd5e1", lineHeight: 1.7 }}>
              The final hero treatment should depict the seated operator in a rattan chair, drifting through space,
              with the vessel and starfield moving subtly behind the scene. This route is the frontend anchor for that
              polished animated landing page experience.
            </p>
            <ul style={{ color: "#cbd5e1", lineHeight: 1.9, marginBottom: 0 }}>
              <li>Voice-first agent operating system</li>
              <li>Visible execution and live agent theater</li>
              <li>White-label frontend with backend subscription control</li>
              <li>BYO model and BYO key routing through ArchonX</li>
            </ul>
          </div>

          <div
            style={{
              borderRadius: 28,
              padding: 32,
              border: "1px solid rgba(96,165,250,0.18)",
              background:
                "radial-gradient(circle at 50% 20%, rgba(96,165,250,0.18), rgba(15,23,42,0.94) 60%)",
              minHeight: 420,
              position: "relative",
              overflow: "hidden",
            }}
          >
            <div
              style={{
                position: "absolute",
                inset: 0,
                background:
                  "radial-gradient(circle at 60% 30%, rgba(255,255,255,0.12), transparent 18%), radial-gradient(circle at 40% 60%, rgba(255,255,255,0.08), transparent 12%), radial-gradient(circle at 70% 75%, rgba(255,255,255,0.08), transparent 10%)",
                opacity: 0.9,
              }}
            />
            <div style={{ position: "relative", zIndex: 1 }}>
              <div style={{ fontSize: 12, letterSpacing: "0.18em", textTransform: "uppercase", color: "#bfdbfe" }}>
                Hero treatment placeholder
              </div>
              <div style={{ marginTop: 18, fontSize: 18, color: "#e2e8f0", lineHeight: 1.7 }}>
                Animated ship / drift scene placeholder for the ArchonX brand story.
              </div>
            </div>
          </div>
        </div>
      </section>
    </main>
  );
}
