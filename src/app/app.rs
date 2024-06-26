use std::vec;

use crate::app::tui::*;
use crate::{fetch_initial_data, get_expeditions, prelude::*};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::borrow::Cow;

use super::stateful_list::StatefulList;

#[derive(Debug, Default)]
pub struct App {
    pub from_stops: StatefulList<Stop>,
    pub to_stops: StatefulList<Stop>,
    pub desired_stops: (Option<Stop>, Option<Stop>),
    pub expeditions: Option<(StatefulList<Expedition>, StatefulList<Expedition>)>,
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
    pub async fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            if self.ready_for_expeditions {
                let from = self
                    .desired_stops
                    .0
                    .as_ref()
                    .expect("Missing from stop reference");
                let to = self
                    .desired_stops
                    .1
                    .as_ref()
                    .expect("Missing to stop reference");
                let expeditions_value: Value = match get_expeditions((&from, &to), None).await {
                    Ok(expeditions) => expeditions,
                    Err(err) => panic!("Error fetching expeditions: \n {:?}", err),
                };

                let expeditions_vecs: (Vec<Expedition>, Vec<Expedition>) =
                    match deserialize_expeditions(expeditions_value) {
                        Ok(expeditions) => expeditions,
                        Err(err) => panic!("Error deserializing expeditions: \n {:?}", err),
                    };

                self.expeditions = Some((
                    StatefulList::with_items(expeditions_vecs.0),
                    StatefulList::with_items(expeditions_vecs.1),
                ));
            }
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let main_constraints = vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ];
        let main_chunks = Layout::vertical(main_constraints).split(frame.size());
        let title = Title::from(
            " Arriva Terminal User Interface "
                .fg(PRIMARY_COLOR_RTT)
                .bold(),
        );
        let constraints = vec![
            Constraint::Percentage(3),
            Constraint::Percentage(47),
            Constraint::Percentage(0),
            Constraint::Percentage(47),
            Constraint::Percentage(3),
        ];
        let chunks = Layout::horizontal(constraints).split(main_chunks[1]);

        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Up>".fg(PRIMARY_COLOR_RTT).bold(),
            " Increment ".into(),
            "<Down>".fg(PRIMARY_COLOR_RTT).bold(),
            " Quit ".into(),
            "<Q> ".fg(PRIMARY_COLOR_RTT).bold(),
            " Select ".into(),
            "<Enter> ".fg(PRIMARY_COLOR_RTT).bold(),
        ]));

        let title_block = Block::default()
            .borders(Borders::NONE)
            .border_set(border::THICK)
            .title(title.alignment(Alignment::Center).position(Position::Top));

        let instructions_block = Block::default()
            .borders(Borders::NONE)
            .border_set(border::THICK)
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            );

        frame.render_widget(title_block, main_chunks[0]);
        if (!self.ready_for_expeditions) {
            let from_list: Vec<ListItem> = self
                .from_stops
                .items
                .iter()
                .map(|i| {
                    ListItem::new(text::Line::from(vec![
                        Span::raw(i.get_parada().to_string()).fg(SECUNDARY_COLOR_RTT),
                        Span::raw(" - "),
                        Span::raw(i.get_nombre()).fg(PRIMARY_COLOR_RTT),
                    ]))
                })
                .collect();
            let to_list: Vec<ListItem> = self
                .to_stops
                .items
                .iter()
                .map(|i| {
                    ListItem::new(text::Line::from(vec![
                        Span::raw(i.get_parada().to_string()).fg(SECUNDARY_COLOR_RTT),
                        Span::raw(" - "),
                        Span::raw(i.get_nombre()).fg(PRIMARY_COLOR_RTT),
                    ]))
                })
                .collect();
            let from_block = List::new(from_list)
                .block(
                    Block::default()
                        .borders(match self.desired_stops {
                            (None, None) => Borders::ALL,
                            (Some(_), None) => Borders::NONE,
                            _ => Borders::NONE,
                        })
                        .title(Title::from("From: ")),
                )
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                )
                .highlight_symbol("->  ");

            let to_block = List::new(to_list)
                .block(
                    Block::default()
                        .borders(match self.desired_stops {
                            (None, None) => Borders::NONE,
                            (Some(_), None) => Borders::ALL,
                            _ => Borders::NONE,
                        })
                        .title(Title::from("To: ")),
                )
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                )
                .highlight_symbol("->  ");

            frame.render_stateful_widget(from_block, chunks[1], &mut self.from_stops.state.clone());
            frame.render_stateful_widget(to_block, chunks[3], &mut self.to_stops.state.clone());
        } else {
            if let Some((outward_list, return_list)) = &self.expeditions {
                let outward_rows = outward_list.items.iter().map(|i| {
                    Row::new(vec![
                        Cell::from(i.get_name()).style(
                            Style::default()
                                .fg(PRIMARY_COLOR_RTT)
                                .add_modifier(style::Modifier::BOLD),
                        ),
                        Cell::from(i.get_departure()).style(
                            Style::default()
                                .fg(SECUNDARY_COLOR_RTT)
                                .add_modifier(style::Modifier::ITALIC),
                        ),
                        Cell::from(i.get_arrival()).style(
                            Style::default()
                                .fg(SECUNDARY_COLOR_RTT)
                                .add_modifier(style::Modifier::ITALIC),
                        ),
                        Cell::from(i.get_cost()).style(
                            Style::default()
                                .fg(PRIMARY_COLOR_RTT)
                                .add_modifier(style::Modifier::ITALIC),
                        ),
                    ])
                });

                let return_rows = return_list.items.iter().map(|i| {
                    Row::new(vec![
                        Cell::from(i.get_name()).style(
                            Style::default()
                                .fg(PRIMARY_COLOR_RTT)
                                .add_modifier(style::Modifier::BOLD),
                        ),
                        Cell::from(i.get_departure()).style(
                            Style::default()
                                .fg(SECUNDARY_COLOR_RTT)
                                .add_modifier(style::Modifier::ITALIC),
                        ),
                        Cell::from(i.get_arrival()).style(
                            Style::default()
                                .fg(SECUNDARY_COLOR_RTT)
                                .add_modifier(style::Modifier::ITALIC),
                        ),
                        Cell::from(i.get_cost()).style(
                            Style::default()
                                .fg(PRIMARY_COLOR_RTT)
                                .add_modifier(style::Modifier::ITALIC),
                        ),
                    ])
                });

                let widths = [
                    Constraint::Percentage(55),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ];

                // Creamos las tablas con las filas correspondientes
                let outward_table = Table::new(outward_rows, widths)
                    .block(
                        Block::default().borders(Borders::ALL).title(
                            Span::raw("IDA").style(
                                Style::default()
                                    .fg(PRIMARY_COLOR_RTT)
                                    .add_modifier(style::Modifier::BOLD),
                            ),
                        ),
                    )
                    .header(
                        Row::new(vec!["LINEA, SALIDA, CHEGADA, COSTE(€)"]).style(
                            Style::default()
                                .fg(Color::Black)
                                .add_modifier(style::Modifier::BOLD),
                        ),
                    )
                    .widths(&[
                        Constraint::Percentage(55),
                        Constraint::Percentage(15),
                        Constraint::Percentage(15),
                        Constraint::Percentage(15),
                    ]);

                let return_table = Table::new(return_rows, widths)
                    .block(
                        Block::default().borders(Borders::ALL).title(
                            Span::raw("VOLTA").style(
                                Style::default()
                                    .fg(PRIMARY_COLOR_RTT)
                                    .add_modifier(style::Modifier::BOLD),
                            ),
                        ),
                    )
                    .header(
                        Row::new(vec!["LINEA, SALIDA, CHEGADA, COSTE(€)"]).style(
                            Style::default()
                                .fg(Color::Black)
                                .add_modifier(style::Modifier::BOLD),
                        ),
                    )
                    .widths(&[
                        Constraint::Percentage(55),
                        Constraint::Percentage(15),
                        Constraint::Percentage(15),
                        Constraint::Percentage(15),
                    ]);

                // Renderizamos las tablas en el frame
                frame.render_widget(outward_table, chunks[1]);
                frame.render_widget(return_table, chunks[3]);
            }
        }
        frame.render_widget(instructions_block, main_chunks[2]);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.code == KeyCode::Char('q') {
                    self.exit = true;
                } else if !self.ready_for_expeditions {
                    match self.desired_stops {
                        (None, None) => {
                            self.desired_stops.0 =
                                match self.from_stops.handle_events(&key_event.code) {
                                    Some(stop) => Some(stop),
                                    None => None,
                                };
                        }
                        (Some(_), None) => {
                            self.desired_stops.1 =
                                match self.to_stops.handle_events(&key_event.code) {
                                    Some(stop) => Some(stop),
                                    None => None,
                                };
                            if self.desired_stops.1 != None {
                                self.ready_for_expeditions = true;
                            }
                        }
                        _ => {}
                    }
                } else {
                    match self.expeditions {
                        Some(ref mut expeditions) => {}
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
