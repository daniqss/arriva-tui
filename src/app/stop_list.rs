use crossterm::event::{self, KeyCode};
use crate::App;

pub fn draw_stop_list() {

}

pub fn handle_events(app: &mut App, code: & KeyCode) {
    match code {
        KeyCode::Up => app.stops.previous(),
        KeyCode::Down => app.stops.next(),
        KeyCode::Enter => {
            let selected_stop = app.stops.items.get(app.stops.state.selected().unwrap()).unwrap();
            
            match app.desired_stops {
                (None, None) => app.desired_stops.0 = Some(selected_stop.clone()),
                (Some(_), None) => app.desired_stops.1 = Some(selected_stop.clone()),
                (_) => {}
            }
        }
        _ => {}
    }
}
