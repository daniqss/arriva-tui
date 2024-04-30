use crossterm::event::{self, KeyCode};
use ratatui::widgets::ListState;
use crate::prelude::*;

use crate::App;

#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl Default for StatefulList<Stop> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

impl StatefulList<Stop> {
    pub fn draw_stop_list() {

    }

    pub fn handle_events(&mut self, code: &KeyCode) -> Option<Stop> {
        match code {
            KeyCode::Up => self.previous(),
            KeyCode::Down => self.next(),
            KeyCode::Enter => return Some(self.items.get(self.state.selected().unwrap()).unwrap().clone()),
            _ => {}
        };

        None
    }
}

impl StatefulList<Expedition> {
    pub fn draw_stop_list() {

    }
    
    pub fn handle_events(&mut self, code: &KeyCode) -> Option<Expedition>  {
        // match code {
        //     KeyCode::Char('q') => app.exit = true,
        //     KeyCode::Up => app.stops.previous(),
        //     KeyCode::Down => app.stops.next(),
        //     KeyCode::Enter => {
        //         let selected_stop = app.stops.items.get(app.stops.state.selected().unwrap()).unwrap();
        //         match app.desired_stops {
        //             (None, None) => app.desired_stops.0 = Some(selected_stop.clone()),
        //             (Some(_), None) => app.desired_stops.1 = Some(selected_stop.clone()),
        //             (_) => {}
        //         }
        //     }
        //     _ => {}
        // }
        println!("{:?}", code);
        None
    }
    
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}