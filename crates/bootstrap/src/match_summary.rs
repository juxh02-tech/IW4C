use bevy::prelude::*;

pub(crate) fn register(app: &mut App) {
    app.add_systems(Last, log_match_summary);
}

#[derive(Default)]
struct Window {
    match_secs: f64,
    secs: f64,
    frames: u32,
    movement_report_secs: f64,
}

fn log_match_summary(
    time: Res<Time<Real>>,
    screen: Option<Res<frame::AppScreen>>,
    roster: Option<Res<bots::BotRoster>>,
    prediction: Option<Res<net::ClientPredictionState>>,
    mut window: Local<Window>,
) {
    if screen.as_deref() != Some(&frame::AppScreen::InGame) {
        *window = Window::default();
        return;
    }
    let dt = time.delta_secs_f64();
    window.match_secs += dt;
    window.secs += dt;
    window.frames += 1;
    if window.match_secs - window.movement_report_secs >= 5.0 {
        window.movement_report_secs = window.match_secs;
        if let Some(prediction) = prediction {
            let error = prediction.0.predicted_error();
            diag::info!(
                Launch,
                "movement summary: {} correction_last={:.3} correction_offset={:?}",
                prediction.0.metrics().report_line(),
                error.last_miss_len(),
                error.offset()
            );
        }
    }
    if window.secs < diag::events::MATCH_SUMMARY_EVERY_SECS {
        return;
    }
    let fps = (window.secs > 0.0).then(|| (f64::from(window.frames) / window.secs) as f32);
    let bots = roster.map_or(0, |roster| roster.bots.len() as u32);
    diag::events::match_summary(window.match_secs, bots, fps);
    window.secs = 0.0;
    window.frames = 0;
}
