use std::vec;

use crate::prelude::*;
use crate::app::tui::*;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::borrow::Cow;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::app::{stop_list, expedition_list};

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


#[derive(Debug, Default)]
pub struct App {
    pub stops: StatefulList<Stop>,
    pub desired_stops: (Option<Stop>, Option<Stop>),
    pub expeditions: Option<Vec<Expedition>>,
    pub ready_for_expeditions: bool,
    pub exit: bool,
}

impl App {
    pub fn new(stops: Vec<Stop>) -> Self {
        App {
            stops: StatefulList::with_items(stops),
            desired_stops: (None, None),
            expeditions: None,
            ready_for_expeditions: false,
            exit: false,
        }
    }

    // runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {

        let constraints = vec![Constraint::Percentage(50)];
        let chunks = Layout::horizontal(constraints).split(frame.size());
        let title = Title::from(" Arriva Terminal User Interface ".fg(PRIMARY_COLOR_RTT).bold());
        let buttons = vec![            
            " Decrement ".into(),
            "<Up>".fg(PRIMARY_COLOR_RTT).bold(),
            " Increment ".into(),
            "<Down>".fg(PRIMARY_COLOR_RTT).bold(),
            " Quit ".into(),
            "<Q> ".fg(PRIMARY_COLOR_RTT).bold(),
            " Select ".into(),
            "<Enter> ".fg(PRIMARY_COLOR_RTT).bold(),
        ];
        let instructions = Title::from(Line::from(buttons.clone()));
        let stops_list: Vec<ListItem> = self.stops.items
            .iter()
            .map(|i| {
                ListItem::new(vec![text::Line::from(Span::raw(i.get_nombre()))])
            }).collect();

        // let block = Block::default()
        //     .title(title.alignment(Alignment::Center))
        //     .title(
        //         instructions
        //             .alignment(Alignment::Center)
        //             .position(Position::Bottom),
        //     )
        //     .borders(Borders::ALL)
        //     .border_set(border::THICK);

            // ...

        let stops_list = List::new(stops_list)
            .block(Block::default().borders(Borders::ALL).border_set(border::THICK)
                .title(title.alignment(Alignment::Center))
                .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            )
            
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).style().fg(PRIMARY_COLOR_RTT))
            .highlight_symbol("   => ");
            frame.render_stateful_widget(stops_list, chunks[0], &mut self.stops.state.clone());

    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                // match key_event.code {
                //     KeyCode::Char('q') => self.exit = true,
                //     KeyCode::Up => self.stops.previous(),
                //     KeyCode::Down => self.stops.next(),
                //     KeyCode::Enter => {
                //         let selected_stop = self.stops.items.get(self.stops.state.selected().unwrap()).unwrap();
                //         match self.desired_stops {
                //             (None, None) => self.desired_stops.0 = Some(selected_stop.clone()),
                //             (Some(_), None) => self.desired_stops.1 = Some(selected_stop.clone()),
                //             (_) => {}
                //         }
                //     }
                //     _ => {}
                // }

                if key_event.code == KeyCode::Char('q') {
                    self.exit = true;
                }
                else if !self.ready_for_expeditions {
                    stop_list::handle_events(self, &key_event.code)
                }
                else {
                    expedition_list::handle_events(self, &key_event.code)
                }
            }
            _ => {}
        };
        Ok(())
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn render() {
    //     let app = App::default();
    //     let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

    //     app.render(buf.area, &mut buf);

    //     let mut expected = Buffer::with_lines(vec![
    //         "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
    //         "┃                    Value: 0                    ┃",
    //         "┃                                                ┃",
    //         "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
    //     ]);
    //     let title_style = Style::new().bold();
    //     let counter_style = Style::new().yellow();
    //     let key_style = Style::new().blue().bold();
    //     expected.set_style(Rect::new(14, 0, 22, 1), title_style);
    //     expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
    //     expected.set_style(Rect::new(13, 3, 6, 1), key_style);
    //     expected.set_style(Rect::new(30, 3, 7, 1), key_style);
    //     expected.set_style(Rect::new(43, 3, 4, 1), key_style);

    //     // note ratatui also has an assert_buffer_eq! macro that can be used to
    //     // compare buffers and display the differences in a more readable way
    //     assert_eq!(buf, expected);
    // }

    // #[test]
    // fn handle_key_event() -> Result<()> {
    //     let mut app = App::default();
    //     app.handle_key_event(KeyCode::Right.into());
    //     assert_eq!(app.counter, 1);

    //     app.handle_key_event(KeyCode::Left.into());
    //     assert_eq!(app.counter, 0);

    //     let mut app = App::default();
    //     app.handle_key_event(KeyCode::Char('q').into());
    //     assert_eq!(app.exit, true);

    //     Ok(())
    // }
}