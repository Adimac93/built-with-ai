//! Microphone recording via a JS eval bridge. Replaces the Angular
//! MediaRecorder plumbing — the JS side owns the recorder lifecycle and
//! resolves once with the base64 payload, avoiding wasm closure management.

use dioxus::document;

pub struct Recording {
    pub audio_base64: String,
    pub mime_type: String,
}

pub enum RecordingOutcome {
    Ok(Recording),
    Empty,
    Failed,
}

/// Mirrors Angular's `speechInputAvailable()`.
pub async fn speech_input_available() -> bool {
    document::eval(
        "return Boolean(navigator.mediaDevices && navigator.mediaDevices.getUserMedia) \
         && typeof MediaRecorder !== 'undefined';",
    )
    .await
    .ok()
    .and_then(|v| v.as_bool())
    .unwrap_or(false)
}

/// Starts recording and resolves when the recorder stops (stop button,
/// 55s cap, or browser error). The recorder is parked on `window` so
/// [`stop_recording`] can reach it.
pub async fn record() -> RecordingOutcome {
    const JS: &str = r#"
        try {
            const stream = await navigator.mediaDevices.getUserMedia({
                audio: { echoCancellation: true, noiseSuppression: true },
            });
            const types = ['audio/webm;codecs=opus', 'audio/webm', 'audio/ogg;codecs=opus'];
            const mime = types.find((t) => MediaRecorder.isTypeSupported(t)) || '';
            const recorder = mime ? new MediaRecorder(stream, { mimeType: mime }) : new MediaRecorder(stream);
            const chunks = [];
            recorder.ondataavailable = (e) => { if (e.data.size > 0) chunks.push(e.data); };
            const done = new Promise((resolve) => {
                recorder.onstop = () => resolve(true);
                recorder.onerror = () => resolve(false);
            });
            window.__heurekaRecorder = recorder;
            recorder.start();
            setTimeout(() => { if (recorder.state === 'recording') recorder.stop(); }, 55000);
            const clean = await done;
            window.__heurekaRecorder = null;
            stream.getTracks().forEach((t) => t.stop());
            if (!clean) return { status: 'failed' };
            const blob = new Blob(chunks, { type: recorder.mimeType || mime || 'audio/webm' });
            if (blob.size === 0) return { status: 'empty' };
            const base64 = await new Promise((resolve) => {
                const reader = new FileReader();
                reader.onloadend = () => resolve(String(reader.result).split(',')[1] || '');
                reader.readAsDataURL(blob);
            });
            return { status: 'ok', audio: base64, mime: blob.type };
        } catch (err) {
            window.__heurekaRecorder = null;
            return { status: 'failed' };
        }
    "#;

    let Ok(value) = document::eval(JS).await else {
        return RecordingOutcome::Failed;
    };
    match value.get("status").and_then(|s| s.as_str()) {
        Some("ok") => RecordingOutcome::Ok(Recording {
            audio_base64: value
                .get("audio")
                .and_then(|a| a.as_str())
                .unwrap_or_default()
                .to_string(),
            mime_type: value
                .get("mime")
                .and_then(|m| m.as_str())
                .unwrap_or("audio/webm")
                .to_string(),
        }),
        Some("empty") => RecordingOutcome::Empty,
        _ => RecordingOutcome::Failed,
    }
}

/// Second press of the mic button: stop the in-flight recorder.
pub fn stop_recording() {
    document::eval(
        "if (window.__heurekaRecorder && window.__heurekaRecorder.state === 'recording') \
         { window.__heurekaRecorder.stop(); }",
    );
}

/// Moves keyboard focus to an element by id (Angular used ViewChild refs).
pub fn focus_element(id: &str) {
    document::eval(&format!(
        "const el = document.getElementById('{id}'); if (el) el.focus();"
    ));
}

/// Scrolls a diagram node into view inside the modal (replaces ng-diagram's
/// `centerOnNode` viewport follow).
pub fn scroll_node_into_view(id: &str) {
    document::eval(&format!(
        "const el = document.getElementById('{id}'); \
         if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'center', inline: 'center' }});"
    ));
}
