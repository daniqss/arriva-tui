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

use super::stateful_list::StatefulList;

#[derive(Debug, Default)]
pub struct App {
    pub from_stops: StatefulList<Stop>,
    pub to_stops: StatefulList<Stop>,
    pub desired_stops: (Option<Stop>, Option<Stop>),
    pub expeditions: Option<StatefulList<Expedition>>,
    pub ready_for_expeditions: bool,
    pub exit: bool,
}

impl App {
    pub fn new(stops: Vec<Stop>) -> Self {
        App {
            from_stops: StatefulList::with_items(stops.clone()),
            to_stops: StatefulList::with_items(stops),
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
        let from_list: Vec<ListItem> = self.from_stops.items
            .iter()
            .map(|i| {
                ListItem::new(vec![text::Line::from(Span::raw(i.get_nombre()))])
            }).collect();

        let stops_list: Vec<ListItem> = self.from_stops.items
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

        let block = List::new(stops_list)
            .block(Block::default().borders(Borders::ALL).border_set(border::THICK)
                .title(title.alignment(Alignment::Center))
                .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).style().fg(PRIMARY_COLOR_RTT))
            .highlight_symbol("   => ");

        frame.render_stateful_widget(block, chunks[0], &mut self.from_stops.state.clone());
        // frame.render_widget(block, chunks[1]);

    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('q') {
                    self.exit = true;
                }
                else if !self.ready_for_expeditions {
                    match self.desired_stops {
                        (None, None) => {
                            self.desired_stops.0 = match self.from_stops.handle_events(&key_event.code) {
                                Some(stop) => Some(stop),
                                None => None,
                            };
                        },
                        (Some(_), None) => {
                            self.desired_stops.0 = match self.from_stops.handle_events(&key_event.code) {
                                Some(stop) => Some(stop),
                                None => None,
                            };
                            if self.desired_stops.1 != None {
                                self.ready_for_expeditions = true;
                            }
                        },
                        _ => {}
                    }
                }
                else {
                    match self.expeditions {
                        Some(ref mut expeditions) => {
                            match expeditions.handle_events(&key_event.code) {
                                Some(expedition) => {
                                    // do something with the selected expedition
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
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