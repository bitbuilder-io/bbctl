use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Table, Row, Cell, Tabs},
    Frame,
};

use crate::app::{App, AppMode};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.area());

    // Render the title bar
    let titles = vec!["Home", "Instances", "Volumes", "Networks", "Settings", "Help"];
    let tabs = Tabs::new(
        titles
            .iter()
            .map(|t| Line::from(vec![Span::styled(*t, Style::default())]))
            .collect::<Vec<_>>(),
    )
    .block(
        Block::bordered()
            .title("BitBuilder Cloud CLI")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded),
    )
    .select(app.mode as usize)
    .style(Style::default())
    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    frame.render_widget(tabs, chunks[0]);

    // Render main content based on the current app mode
    match app.mode {
        AppMode::Home => render_home(app, frame, chunks[1]),
        AppMode::Instances => render_instances(app, frame, chunks[1]),
        AppMode::Volumes => render_volumes(app, frame, chunks[1]),
        AppMode::Networks => render_networks(app, frame, chunks[1]),
        AppMode::Settings => render_settings(app, frame, chunks[1]),
        AppMode::Help => render_help(app, frame, chunks[1]),
    }

    // Render footer with keybindings
    render_footer(app, frame, chunks[2]);
}

fn render_home(app: &mut App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("Welcome to ", Style::default()),
            Span::styled("BitBuilder Cloud CLI", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from("Your infrastructure at a glance:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Instances: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{}", app.instances.len()), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Volumes: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{}", app.volumes.len()), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Networks: ", Style::default().fg(Color::Yellow)),
            Span::styled(format!("{}", app.networks.len()), Style::default()),
        ]),
        Line::from(""),
        Line::from("Use the numbered keys or Tab to navigate between views."),
        Line::from("Press ? for help."),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::bordered().title("Dashboard").border_type(BorderType::Rounded))
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn render_instances(app: &mut App, frame: &mut Frame, area: Rect) {
    let instances = Block::bordered()
        .title("Instances")
        .border_type(BorderType::Rounded);
    
    if app.instances.is_empty() {
        let text = Text::from("No instances found. Press 'a' to add a new instance.");
        let paragraph = Paragraph::new(text)
            .block(instances)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .instances
        .iter()
        .map(|instance| {
            let status_style = match instance.status.as_str() {
                "running" => Style::default().fg(Color::Green),
                "stopped" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Yellow),
            };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{}: ", instance.name), Style::default().fg(Color::Cyan)),
                    Span::styled(instance.status.clone(), status_style),
                ]),
                Line::from(vec![
                    Span::styled(format!("Provider: {}", instance.provider), Style::default()),
                    Span::styled(format!(" | Region: {}", instance.region), Style::default()),
                ]),
                Line::from(vec![
                    Span::styled(format!("IP: {}", instance.ip), Style::default()),
                    Span::styled(
                        format!(" | CPU: {} | Memory: {} GB | Disk: {} GB", 
                                instance.cpu, instance.memory_gb, instance.disk_gb),
                        Style::default()
                    ),
                ]),
                Line::from(""),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(instances)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    // Use a stateful widget
    let mut state = ListState::default();
    state.select(Some(app.selected_index));
    
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_volumes(app: &mut App, frame: &mut Frame, area: Rect) {
    let volumes = Block::bordered()
        .title("Volumes")
        .border_type(BorderType::Rounded);

    if app.volumes.is_empty() {
        let text = Text::from("No volumes found. Press 'a' to add a new volume.");
        let paragraph = Paragraph::new(text)
            .block(volumes)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .volumes
        .iter()
        .map(|volume| {
            let attached_info = match &volume.attached_to {
                Some(instance_id) => {
                    let instance_name = app.instances
                        .iter()
                        .find(|i| &i.id == instance_id)
                        .map_or(instance_id.as_str(), |i| i.name.as_str());
                    format!("Attached to: {}", instance_name)
                },
                None => "Not attached".to_string(),
            };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{}: ", volume.name), Style::default().fg(Color::Cyan)),
                    Span::styled(format!("{} GB", volume.size_gb), Style::default()),
                ]),
                Line::from(vec![
                    Span::styled(format!("Region: {}", volume.region), Style::default()),
                    Span::styled(format!(" | {}", attached_info), Style::default()),
                ]),
                Line::from(""),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(volumes)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    // Use a stateful widget
    let mut state = ListState::default();
    state.select(Some(app.selected_index));
    
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_networks(app: &mut App, frame: &mut Frame, area: Rect) {
    let networks = Block::bordered()
        .title("Networks")
        .border_type(BorderType::Rounded);

    if app.networks.is_empty() {
        let text = Text::from("No networks found. Press 'a' to add a new network.");
        let paragraph = Paragraph::new(text)
            .block(networks)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .networks
        .iter()
        .map(|network| {
            let instance_count = network.instances.len();
            
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{}: ", network.name), Style::default().fg(Color::Cyan)),
                    Span::styled(network.cidr.clone(), Style::default()),
                ]),
                Line::from(vec![
                    Span::styled(
                        format!("{} instance{} connected", 
                                instance_count, 
                                if instance_count == 1 { "" } else { "s" }),
                        Style::default()
                    ),
                ]),
                Line::from(""),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(networks)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    // Use a stateful widget
    let mut state = ListState::default();
    state.select(Some(app.selected_index));
    
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_settings(_app: &mut App, frame: &mut Frame, area: Rect) {
    let settings = vec![
        ("API Endpoint", "https://api.bitbuilder.io"),
        ("Default Region", "nyc"),
        ("Default Provider", "vyos"),
        ("Auto Update", "Enabled"),
        ("Telemetry", "Disabled"),
    ];

    let rows = settings.iter().map(|(key, value)| {
        Row::new(vec![
            Cell::from(Span::styled(*key, Style::default().fg(Color::Yellow))),
            Cell::from(Span::styled(*value, Style::default())),
        ])
    });

    let table = Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
        .block(Block::bordered().title("Settings").border_type(BorderType::Rounded))
        .header(
            Row::new(vec![
                Cell::from(Span::styled("Setting", Style::default().add_modifier(Modifier::BOLD))),
                Cell::from(Span::styled("Value", Style::default().add_modifier(Modifier::BOLD))),
            ])
        )
        .column_spacing(2);

    frame.render_widget(table, area);
}

fn render_help(_app: &mut App, frame: &mut Frame, area: Rect) {
    let keys = vec![
        ("1-5", "Switch tabs"),
        ("Tab/Shift+Tab", "Next/Previous tab"),
        ("j/k or ↑/↓", "Navigate items"),
        ("a", "Add new item"),
        ("d", "Delete selected item"),
        ("e", "Edit selected item"),
        ("r", "Refresh data"),
        ("q or ESC", "Quit"),
        ("?", "Show help"),
    ];

    let rows = keys.iter().map(|(key, desc)| {
        Row::new(vec![
            Cell::from(Span::styled(*key, Style::default().fg(Color::Yellow))),
            Cell::from(Span::styled(*desc, Style::default())),
        ])
    });

    let table = Table::new(rows, [Constraint::Percentage(20), Constraint::Percentage(80)])
        .block(Block::bordered().title("Keyboard Shortcuts").border_type(BorderType::Rounded))
        .column_spacing(2);

    frame.render_widget(table, area);
}

fn render_footer(app: &mut App, frame: &mut Frame, area: Rect) {
    let text = match app.mode {
        AppMode::Home => "1-5: Navigate | q: Quit",
        AppMode::Instances => "a: Add | d: Delete | e: Edit | r: Restart | s: Stop | ↑/↓: Navigate",
        AppMode::Volumes => "a: Add | d: Delete | e: Edit | a: Attach | d: Detach | ↑/↓: Navigate",
        AppMode::Networks => "a: Add | d: Delete | e: Edit | c: Connect VM | ↑/↓: Navigate",
        AppMode::Settings => "e: Edit Setting | r: Reset to Default",
        AppMode::Help => "Press any key to return",
    };

    let footer = Paragraph::new(text)
        .block(Block::bordered().border_type(BorderType::Rounded))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
