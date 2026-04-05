use tokio::sync::broadcast;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};
use serde_json::json;

/// A custom Tracing Layer that broadcasts log events to the frontend via SSE
/// This provides the real-time cognitive load (Coal) tracking in the React Web UI.
pub struct BeastTelemetryLayer {
    sender: broadcast::Sender<String>,
}

impl BeastTelemetryLayer {
    pub fn new(sender: broadcast::Sender<String>) -> Self {
        Self { sender }
    }
}

struct BeastVisitor {
    message: String,
}

impl tracing::field::Visit for BeastVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}

impl<S> Layer<S> for BeastTelemetryLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        if self.sender.receiver_count() == 0 {
            return;
        }

        // We only care about INFO and above to avoid spamming the HUD
        let level = event.metadata().level();
        if level > &tracing::Level::INFO {
            return;
        }

        let mut visitor = BeastVisitor { message: String::new() };
        event.record(&mut visitor);

        let clean_message = visitor.message.trim_matches('"').to_string();
        
        if clean_message.is_empty() {
            return;
        }

        let payload = json!({
            "type": "telemetry",
            "level": level.as_str(),
            "target": event.metadata().target(),
            "message": clean_message,
        });

        let _ = self.sender.send(payload.to_string());
    }
}
