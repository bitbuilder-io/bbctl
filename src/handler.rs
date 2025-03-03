use crate::app::{App, AppMode, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            if app.mode == AppMode::Home {
                app.quit();
            } else {
                app.change_mode(AppMode::Home);
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            app.next_item();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous_item();
        }
        
        // Mode switching 
        KeyCode::Char('1') => {
            app.change_mode(AppMode::Home);
        }
        KeyCode::Char('2') => {
            app.change_mode(AppMode::Instances);
        }
        KeyCode::Char('3') => {
            app.change_mode(AppMode::Volumes);
        }
        KeyCode::Char('4') => {
            app.change_mode(AppMode::Networks);
        }
        KeyCode::Char('5') => {
            app.change_mode(AppMode::Settings);
        }
        KeyCode::Char('?') => {
            app.change_mode(AppMode::Help);
        }
        
        // Tab navigation
        KeyCode::Tab => {
            match app.mode {
                AppMode::Home => app.change_mode(AppMode::Instances),
                AppMode::Instances => app.change_mode(AppMode::Volumes),
                AppMode::Volumes => app.change_mode(AppMode::Networks),
                AppMode::Networks => app.change_mode(AppMode::Settings),
                AppMode::Settings => app.change_mode(AppMode::Help),
                AppMode::Help => app.change_mode(AppMode::Home),
            }
        }
        
        // Shift+Tab for reverse navigation
        KeyCode::BackTab => {
            match app.mode {
                AppMode::Home => app.change_mode(AppMode::Help),
                AppMode::Instances => app.change_mode(AppMode::Home),
                AppMode::Volumes => app.change_mode(AppMode::Instances),
                AppMode::Networks => app.change_mode(AppMode::Volumes),
                AppMode::Settings => app.change_mode(AppMode::Networks),
                AppMode::Help => app.change_mode(AppMode::Settings),
            }
        }
        
        // Other handlers
        _ => {}
    }
    Ok(())
}
