use crate::{SlideApp, ViewMode};
use enki_speech_ui::{Action, VoiceCommand};
use serde_json::json;
pub(crate) fn show(app: &mut SlideApp, ctx: &egui::Context) {
    crate::daemon::launch_when_open(app.speech.open);
    let notes = app.ui.view == ViewMode::NotesPage;
    let scope = app.session.active().map(|d| format!("{}:{}:{}:{notes}", d.uid, d.revision, d.selection.slide)).unwrap_or_default();
    let Some(action) = app.speech.show(ctx, &scope, if notes { "Append to speaker notes" } else { "Insert slide text" }) else { return };
    let result = match action {
        Action::Insert(text) if notes => {
            let before = app.session.active().and_then(|d| d.current_slide()).map(|s| s.notes_text()).unwrap_or_default();
            app.run("slide.notes", json!({"text":if before.is_empty(){text}else{format!("{before}\n{text}")}}))
        }
        Action::Insert(text) => app.run("insert.textBox", json!({"text":text})),
        Action::Command(command) => {
            let id = match command {
                VoiceCommand::OpenAssistant => "app.scribe",
                VoiceCommand::Next => "slide.next",
                VoiceCommand::Previous => "slide.previous",
                VoiceCommand::ZoomIn => "view.zoomIn",
                VoiceCommand::ZoomOut => "view.zoomOut",
                VoiceCommand::Undo => "edit.undo",
                VoiceCommand::Redo => "edit.redo",
                VoiceCommand::Stop => {
                    app.speech.open = false;
                    return;
                }
            };
            app.run(id, json!({}))
        }
    };
    if let Err(e) = result {
        app.notify(e);
    }
}
