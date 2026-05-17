use crate::core::note::NoteService;
use crate::core::tag::TagService;
use crate::storage::Database;

const NOTES: &[(&str, &str, &[&str])] = &[
    (
        "Project Alpha — Architecture Overview",
        "<h1>Project Alpha</h1>
<p>This document outlines the architecture for <b>Project Alpha</b>, our next-gen platform.</p>
<h2>System Design</h2>
<p>The system follows a <b>microservices architecture</b> with three primary services:</p>
<ul>
<li><b>API Gateway</b> — handles auth, rate limiting, and routing</li>
<li><b>Core Service</b> — business logic and data processing</li>
<li><b>Event Bus</b> — async message passing between services</li>
</ul>
<h2>Tech Stack</h2>
<ul>
<li><b>Backend:</b> Rust + Actix-web</li>
<li><b>Database:</b> PostgreSQL + Redis cache</li>
<li><b>Frontend:</b> React + TypeScript</li>
<li><b>Infrastructure:</b> Kubernetes + Docker</li>
</ul>
<blockquote><i>Key insight:</i> The event bus ensures loose coupling between services, enabling independent deployment.</blockquote>",
        &["architecture", "project-alpha"],
    ),
    (
        "Sprint Planning — Week 20",
        "<h1>Sprint 20 — Planning</h1>
<h2>Goals</h2>
<ol>
<li>Complete API integration tests</li>
<li>Deploy beta to staging environment</li>
<li>Begin performance benchmarking</li>
</ol>
<h2>Team Assignments</h2>
<ul>
<li><b>@alice</b> — API test suite &amp; CI pipeline</li>
<li><b>@bob</b> — Staging deployment &amp; monitoring</li>
<li><b>@carol</b> — Load testing with k6</li>
</ul>
<h2>Risks</h2>
<p>The database migration for v2 schema is <b>critical path</b>. Must be completed before Thursday.</p>",
        &["sprint", "project-alpha"],
    ),
    (
        "Meeting Notes — Architecture Review",
        "<h1>Architecture Review — 14 May</h1>
<h2>Attendees</h2>
<p>Alice, Bob, Carol, Dave</p>
<h2>Agenda</h2>
<ol>
<li>Review current API response times</li>
<li>Discuss caching strategy for user profiles</li>
<li>Plan migration to WebSockets for real-time features</li>
</ol>
<h2>Decisions</h2>
<ul>
<li>✅ Adopt <b>Redis</b> for session caching (target: &lt;5ms latency)</li>
<li>✅ Migrate to <b>WebSockets</b> by end of Q2</li>
<li>❌ Postpone GraphQL adoption to Q3</li>
</ul>
<h2>Action Items</h2>
<ul>
<li>[ ] Alice: Benchmark current API endpoints by Friday</li>
<li>[ ] Bob: Draft WebSocket migration plan</li>
<li>[ ] Carol: Research Redis cluster configuration</li>
</ul>",
        &["meetings", "project-alpha"],
    ),
    (
        "Personal Knowledge Base — Rust Patterns",
        "<h1>Rust Patterns &amp; Best Practices</h1>
<h2>Error Handling</h2>
<p>Use <code>thiserror</code> for library crates and <code>anyhow</code> for applications.</p>
<pre><code>#[derive(thiserror::Error)]
pub enum ServiceError {
    #[error(\"not found: {0}\")]
    NotFound(String),
    #[error(\"validation failed: {0}\")]
    Validation(String),
}</code></pre>
<h2>Async Patterns</h2>
<p>Prefer <code>tokio</code> for async runtime. Use <code>spawn</code> for fire-and-forget tasks.</p>
<h2>Testing</h2>
<ul>
<li>Unit tests with <code>#[cfg(test)]</code></li>
<li>Integration tests in <code>tests/</code> directory</li>
<li>Property-based testing with <code>proptest</code></li>
</ul>",
        &["rust", "learning"],
    ),
    (
        "Travel Plans — Summer 2026",
        "<h1>Summer Travel Plans</h1>
<h2>Destinations</h2>
<ul>
<li><b>Tokyo</b> — June 15-22</li>
<li><b>Kyoto</b> — June 22-28</li>
<li><b>Osaka</b> — June 28-July 2</li>
</ul>
<h2>Packing List</h2>
<ul>
<li>[ ] Passport &amp; visa documents</li>
<li>[ ] Travel insurance</li>
<li>[ ] Portable charger</li>
<li>[ ] Universal adapter</li>
<li>[ ] Lightweight rain jacket</li>
</ul>
<h2>Budget</h2>
<table>
<thead><tr><th>Category</th><th>Budget</th></tr></thead>
<tbody>
<tr><td>Flights</td><td>$1,200</td></tr>
<tr><td>Accommodation</td><td>$2,400</td></tr>
<tr><td>Food &amp; Activities</td><td>$1,800</td></tr>
</tbody>
</table>",
        &["personal", "travel"],
    ),
    (
        "Weekly Review — 2026-W19",
        "<h1>Weekly Review — Week 19</h1>
<h2>Accomplished</h2>
<ul>
<li>✅ Shipped v2.1.0 with performance improvements</li>
<li>✅ Resolved 12 out of 15 open bugs</li>
<li>✅ Completed team performance reviews</li>
</ul>
<h2>In Progress</h2>
<ul>
<li>🔄 Documentation overhaul (60% complete)</li>
<li>🔄 CI/CD pipeline optimization</li>
</ul>
<h2>Blockers</h2>
<ul>
<li>⏳ Awaiting security audit results for deployment</li>
</ul>
<h2>Next Week Priorities</h2>
<ol>
<li>Finalize documentation overhaul</li>
<li>Begin Q3 roadmap planning</li>
<li>Schedule 1:1s with direct reports</li>
</ol>",
        &["work", "reviews"],
    ),
];

/// Seed the database with demo data if it's empty.
pub fn seed_database(db: &Database) {
    let note_svc = NoteService::new(db);

    let count: i64 = db
        .conn()
        .query_row("SELECT count(*) FROM notes", [], |row| row.get(0))
        .unwrap_or(0);
    if count > 0 {
        return;
    }

    let tag_svc = TagService::new(db);

    for (title, content, tags) in NOTES {
        if let Ok(note) = note_svc.create(title, content) {
            for tag_name in *tags {
                let tag = tag_svc
                    .get_by_name(tag_name)
                    .ok()
                    .flatten()
                    .or_else(|| tag_svc.create(tag_name, None, None).ok());
                if let Some(t) = tag {
                    let _ = tag_svc.assign_to_note(&t.id, &note.id);
                }
            }
        }
    }
}
