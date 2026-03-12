//! `vc_tui` - Terminal UI for Vibe Cockpit
//!
//! This crate provides:
//! - FrankenTUI Elm-architecture terminal interface (Model trait)
//! - Legacy ratatui render path (migrating per-screen to ftui)
//! - Multiple screens (overview, machines, repos, alerts, etc.)
//! - Real-time updates via tick subscriptions
//! - Keyboard navigation

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

pub mod screens;
pub mod theme;
pub mod widgets;

pub use screens::{
    AccountsData, AlertsData, BeadsData, EventsData, GuardianData, MachinesData, MailData,
    OracleData, OverviewData, RchData, SessionsData, render_accounts, render_alerts, render_beads,
    render_events, render_guardian, render_machines, render_mail, render_oracle, render_overview,
    render_rch, render_sessions,
};
pub use theme::Theme;

/// Default dashboard refresh interval.
const TICK_INTERVAL: Duration = Duration::from_secs(5);

/// TUI errors
#[derive(Error, Debug)]
pub enum TuiError {
    #[error("Terminal error: {0}")]
    TerminalError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Query error: {0}")]
    QueryError(#[from] vc_query::QueryError),
}

/// Available screens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    Overview,
    Machines,
    Repos,
    Accounts,
    Sessions,
    Mail,
    Alerts,
    Guardian,
    Oracle,
    Events,
    Beads,
    Rch,
    Settings,
    Help,
}

impl Screen {
    /// Get screen title
    #[must_use]
    pub fn title(&self) -> &'static str {
        match self {
            Screen::Overview => "Overview",
            Screen::Machines => "Machines",
            Screen::Repos => "Repositories",
            Screen::Accounts => "Accounts",
            Screen::Sessions => "Sessions",
            Screen::Mail => "Agent Mail",
            Screen::Alerts => "Alerts",
            Screen::Guardian => "Guardian",
            Screen::Oracle => "Oracle",
            Screen::Events => "Events",
            Screen::Beads => "Beads",
            Screen::Rch => "RCH Workers",
            Screen::Settings => "Settings",
            Screen::Help => "Help",
        }
    }

    /// Get keyboard shortcut
    #[must_use]
    pub fn shortcut(&self) -> Option<char> {
        match self {
            Screen::Overview => Some('o'),
            Screen::Machines => Some('m'),
            Screen::Repos => Some('r'),
            Screen::Accounts => Some('a'),
            Screen::Sessions => Some('s'),
            Screen::Mail => Some('l'),
            Screen::Alerts => Some('!'),
            Screen::Guardian => Some('g'),
            Screen::Oracle => Some('p'),
            Screen::Events => Some('e'),
            Screen::Beads => Some('b'),
            Screen::Rch => None,
            Screen::Settings => None,
            Screen::Help => Some('?'),
        }
    }

    /// All screens in order
    #[must_use]
    pub fn all() -> &'static [Screen] {
        &[
            Screen::Overview,
            Screen::Machines,
            Screen::Repos,
            Screen::Accounts,
            Screen::Sessions,
            Screen::Mail,
            Screen::Alerts,
            Screen::Guardian,
            Screen::Oracle,
            Screen::Events,
            Screen::Beads,
            Screen::Rch,
            Screen::Settings,
            Screen::Help,
        ]
    }
}

// ==========================================================================
// Elm architecture: AppMessage + Model impl
// ==========================================================================

/// Messages that drive the Elm update loop.
#[derive(Debug)]
pub enum AppMessage {
    /// Terminal key event forwarded from the runtime.
    Key(ftui::KeyEvent),
    /// Periodic tick — triggers data refresh.
    Tick,
    /// Navigate to a specific screen.
    ScreenChanged(Screen),
    /// Fresh data arrived for a screen.
    DataRefreshed(ScreenData),
    /// An error occurred during an operation.
    Error(String),
    /// Quit the application.
    Quit,
}

/// Typed payload for screen data refreshes.
#[derive(Debug)]
pub enum ScreenData {
    Overview(Box<OverviewData>),
    Machines(Box<MachinesData>),
    Accounts(Box<AccountsData>),
    Sessions(Box<SessionsData>),
    Mail(Box<MailData>),
    Alerts(Box<AlertsData>),
    Guardian(Box<GuardianData>),
    Oracle(Box<OracleData>),
    Events(Box<EventsData>),
    Beads(Box<BeadsData>),
    Rch(Box<RchData>),
}

impl From<ftui::Event> for AppMessage {
    fn from(event: ftui::Event) -> Self {
        match event {
            ftui::Event::Key(k) => AppMessage::Key(k),
            ftui::Event::Tick => AppMessage::Tick,
            _ => {
                // Resize, Mouse, Paste, Focus, Clipboard, Ime → no-op key
                AppMessage::Key(ftui::KeyEvent::new(ftui::KeyCode::Null))
            }
        }
    }
}

/// Application state
pub struct App {
    pub current_screen: Screen,
    pub should_quit: bool,
    pub last_error: Option<String>,
    pub theme: Theme,
    // Screen data — all screens represented
    pub overview_data: OverviewData,
    pub machines_data: MachinesData,
    pub accounts_data: AccountsData,
    pub sessions_data: SessionsData,
    pub mail_data: MailData,
    pub alerts_data: AlertsData,
    pub guardian_data: GuardianData,
    pub oracle_data: OracleData,
    pub events_data: EventsData,
    pub beads_data: BeadsData,
    pub rch_data: RchData,
}

impl App {
    /// Create a new app instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Overview,
            should_quit: false,
            last_error: None,
            theme: Theme::default(),
            overview_data: OverviewData::default(),
            machines_data: MachinesData::default(),
            accounts_data: AccountsData::default(),
            sessions_data: SessionsData::default(),
            mail_data: MailData::default(),
            alerts_data: AlertsData::default(),
            guardian_data: GuardianData::default(),
            oracle_data: OracleData::default(),
            events_data: EventsData::default(),
            beads_data: BeadsData::default(),
            rch_data: RchData::default(),
        }
    }

    /// Render the current screen (legacy ratatui path).
    ///
    /// Kept during migration — screen port beads (bd-0di, bd-1l8, bd-rq9, bd-1l1)
    /// will replace each arm with ftui equivalents.
    pub fn render(&self, f: &mut Frame) {
        match self.current_screen {
            Screen::Overview => {
                render_overview(f, &self.overview_data, &self.theme);
            }
            Screen::Machines => {
                render_machines(f, &self.machines_data, &self.theme);
            }
            Screen::Accounts => {
                render_accounts(f, &self.accounts_data, &self.theme);
            }
            Screen::Sessions => {
                render_sessions(f, &self.sessions_data, &self.theme);
            }
            Screen::Mail => {
                render_mail(f, &self.mail_data, &self.theme);
            }
            Screen::Alerts => {
                render_alerts(f, &self.alerts_data, &self.theme);
            }
            Screen::Guardian => {
                render_guardian(f, &self.guardian_data, &self.theme);
            }
            Screen::Oracle => {
                render_oracle(f, &self.oracle_data, &self.theme);
            }
            Screen::Events => {
                render_events(f, &self.events_data, &self.theme);
            }
            Screen::Beads => {
                render_beads(f, &self.beads_data, &self.theme);
            }
            Screen::Rch => {
                render_rch(f, &self.rch_data, &self.theme);
            }
            _ => {
                // Repos, Settings, Help — render placeholder
                use ratatui::widgets::{Block, Borders, Paragraph};
                let text = Paragraph::new(format!(
                    "Screen: {} - Press 'o' for Overview",
                    self.current_screen.title()
                ))
                .block(Block::default().title("Vibe Cockpit").borders(Borders::ALL));
                f.render_widget(text, f.area());
            }
        }
    }

    /// Handle keyboard input (legacy crossterm path).
    ///
    /// Kept during migration — bd-mhu will port to ftui event system.
    pub fn handle_key(&mut self, key: KeyEvent) {
        // Global shortcuts
        if key.modifiers.contains(KeyModifiers::CONTROL)
            && let KeyCode::Char('c' | 'q') = key.code
        {
            self.should_quit = true;
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') => self.current_screen = Screen::Help,
            KeyCode::Char(c) => {
                // Check screen shortcuts
                for screen in Screen::all() {
                    if screen.shortcut() == Some(c) {
                        self.current_screen = *screen;
                        break;
                    }
                }
            }
            KeyCode::Tab => {
                // Cycle to next screen
                let screens = Screen::all();
                let current_idx = screens
                    .iter()
                    .position(|s| *s == self.current_screen)
                    .unwrap_or(0);
                let next_idx = (current_idx + 1) % screens.len();
                self.current_screen = screens[next_idx];
            }
            _ => {}
        }
    }

    /// Write a string into an ftui buffer at the given row.
    fn write_line(buf: &mut ftui::Buffer, y: u16, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            let x = i as u16;
            if x < buf.width() && y < buf.height() {
                buf.set(x, y, ftui::Cell::from_char(ch));
            }
        }
    }
}

impl ftui::Model for App {
    type Message = AppMessage;

    fn init(&mut self) -> ftui::Cmd<Self::Message> {
        // Start on Overview, schedule first data load
        self.current_screen = Screen::Overview;
        ftui::Cmd::msg(AppMessage::Tick)
    }

    fn update(&mut self, msg: Self::Message) -> ftui::Cmd<Self::Message> {
        match msg {
            AppMessage::Key(k) => self.handle_ftui_key(k),
            AppMessage::Tick => {
                // In the future, this returns a Cmd::Task that fetches data.
                // For now, just schedule the next tick.
                ftui::Cmd::tick(TICK_INTERVAL)
            }
            AppMessage::ScreenChanged(screen) => {
                self.current_screen = screen;
                ftui::Cmd::none()
            }
            AppMessage::DataRefreshed(data) => {
                match data {
                    ScreenData::Overview(d) => self.overview_data = *d,
                    ScreenData::Machines(d) => self.machines_data = *d,
                    ScreenData::Accounts(d) => self.accounts_data = *d,
                    ScreenData::Sessions(d) => self.sessions_data = *d,
                    ScreenData::Mail(d) => self.mail_data = *d,
                    ScreenData::Alerts(d) => self.alerts_data = *d,
                    ScreenData::Guardian(d) => self.guardian_data = *d,
                    ScreenData::Oracle(d) => self.oracle_data = *d,
                    ScreenData::Events(d) => self.events_data = *d,
                    ScreenData::Beads(d) => self.beads_data = *d,
                    ScreenData::Rch(d) => self.rch_data = *d,
                }
                ftui::Cmd::none()
            }
            AppMessage::Error(e) => {
                self.last_error = Some(e);
                ftui::Cmd::none()
            }
            AppMessage::Quit => {
                self.should_quit = true;
                ftui::Cmd::quit()
            }
        }
    }

    fn view(&self, frame: &mut ftui::Frame) {
        // Stub dispatch — each screen renders a placeholder.
        // Screen port beads (bd-0di, bd-1l8, bd-rq9, bd-1l1) will replace
        // each arm with the full ftui widget implementation.
        let title = format!("Vibe Cockpit | {}", self.current_screen.title());
        Self::write_line(&mut frame.buffer, 0, &title);

        let hint = match self.current_screen {
            Screen::Overview => "Fleet health, active agents, recent events",
            Screen::Machines => "Machine status, SSH connectivity, resource usage",
            Screen::Repos => "Repository status and sync state",
            Screen::Accounts => "AI coding account status and rate limits",
            Screen::Sessions => "Active agent sessions and history",
            Screen::Mail => "Agent-to-agent mail threads",
            Screen::Alerts => "Active alerts, rules, and acknowledgements",
            Screen::Guardian => "Self-healing protocols and approval queue",
            Screen::Oracle => "Predictions, cost forecasts, failure risks",
            Screen::Events => "DCG events, PT findings, Rano timeline",
            Screen::Beads => "Task graph, blockers, recommendations",
            Screen::Rch => "Remote compilation workers and build queue",
            Screen::Settings => "Configuration and preferences",
            Screen::Help => "Keyboard shortcuts: o m r a s l ! g p e b ? | Tab cycles | q quits",
        };
        Self::write_line(&mut frame.buffer, 2, hint);

        if let Some(ref err) = self.last_error {
            let err_line = format!("Error: {err}");
            Self::write_line(&mut frame.buffer, 4, &err_line);
        }

        // Navigation bar at bottom
        let nav = "o:Overview m:Machines r:Repos a:Accounts s:Sessions l:Mail !:Alerts g:Guardian p:Oracle e:Events b:Beads ?:Help q:Quit";
        let bottom_y = frame.height().saturating_sub(1);
        Self::write_line(&mut frame.buffer, bottom_y, nav);
    }

    fn subscriptions(&self) -> Vec<Box<dyn ftui::runtime::Subscription<Self::Message>>> {
        vec![Box::new(ftui::runtime::Every::new(
            TICK_INTERVAL,
            || AppMessage::Tick,
        ))]
    }
}

impl App {
    /// Handle an ftui key event (Elm path).
    fn handle_ftui_key(&mut self, key: ftui::KeyEvent) -> ftui::Cmd<AppMessage> {
        // Quit shortcuts
        if key.ctrl() && matches!(key.code, ftui::KeyCode::Char('c' | 'q')) {
            return ftui::Cmd::msg(AppMessage::Quit);
        }

        match key.code {
            ftui::KeyCode::Char('q') => ftui::Cmd::msg(AppMessage::Quit),
            ftui::KeyCode::Tab => {
                let screens = Screen::all();
                let current_idx = screens
                    .iter()
                    .position(|s| *s == self.current_screen)
                    .unwrap_or(0);
                let next_idx = (current_idx + 1) % screens.len();
                ftui::Cmd::msg(AppMessage::ScreenChanged(screens[next_idx]))
            }
            ftui::KeyCode::Char(c) => {
                for screen in Screen::all() {
                    if screen.shortcut() == Some(c) {
                        return ftui::Cmd::msg(AppMessage::ScreenChanged(*screen));
                    }
                }
                ftui::Cmd::none()
            }
            _ => ftui::Cmd::none(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================================================
    // Screen Tests
    // ==========================================================================

    #[test]
    fn test_screen_shortcuts() {
        assert_eq!(Screen::Overview.shortcut(), Some('o'));
        assert_eq!(Screen::Machines.shortcut(), Some('m'));
        assert_eq!(Screen::Repos.shortcut(), Some('r'));
        assert_eq!(Screen::Accounts.shortcut(), Some('a'));
        assert_eq!(Screen::Sessions.shortcut(), Some('s'));
        assert_eq!(Screen::Mail.shortcut(), Some('l'));
        assert_eq!(Screen::Alerts.shortcut(), Some('!'));
        assert_eq!(Screen::Guardian.shortcut(), Some('g'));
        assert_eq!(Screen::Oracle.shortcut(), Some('p'));
        assert_eq!(Screen::Events.shortcut(), Some('e'));
        assert_eq!(Screen::Beads.shortcut(), Some('b'));
        assert_eq!(Screen::Rch.shortcut(), None);
        assert_eq!(Screen::Settings.shortcut(), None);
        assert_eq!(Screen::Help.shortcut(), Some('?'));
    }

    #[test]
    fn test_screen_titles() {
        assert_eq!(Screen::Overview.title(), "Overview");
        assert_eq!(Screen::Machines.title(), "Machines");
        assert_eq!(Screen::Repos.title(), "Repositories");
        assert_eq!(Screen::Accounts.title(), "Accounts");
        assert_eq!(Screen::Sessions.title(), "Sessions");
        assert_eq!(Screen::Mail.title(), "Agent Mail");
        assert_eq!(Screen::Alerts.title(), "Alerts");
        assert_eq!(Screen::Guardian.title(), "Guardian");
        assert_eq!(Screen::Oracle.title(), "Oracle");
        assert_eq!(Screen::Events.title(), "Events");
        assert_eq!(Screen::Beads.title(), "Beads");
        assert_eq!(Screen::Rch.title(), "RCH Workers");
        assert_eq!(Screen::Settings.title(), "Settings");
        assert_eq!(Screen::Help.title(), "Help");
    }

    #[test]
    fn test_screen_all() {
        let screens = Screen::all();
        assert_eq!(screens.len(), 14);
        assert_eq!(screens[0], Screen::Overview);
        assert_eq!(screens[screens.len() - 1], Screen::Help);
    }

    #[test]
    fn test_screen_serialization() {
        let screen = Screen::Overview;
        let json = serde_json::to_string(&screen).unwrap();
        assert_eq!(json, "\"Overview\"");

        let parsed: Screen = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Screen::Overview);
    }

    #[test]
    fn test_screen_rch_serialization() {
        let screen = Screen::Rch;
        let json = serde_json::to_string(&screen).unwrap();
        assert_eq!(json, "\"Rch\"");

        let parsed: Screen = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Screen::Rch);
    }

    #[test]
    fn test_all_screens_serialize_roundtrip() {
        for screen in Screen::all() {
            let json = serde_json::to_string(screen).unwrap();
            let parsed: Screen = serde_json::from_str(&json).unwrap();
            assert_eq!(*screen, parsed);
        }
    }

    // ==========================================================================
    // App Tests (legacy crossterm path)
    // ==========================================================================

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert_eq!(app.current_screen, Screen::Overview);
        assert!(!app.should_quit);
        assert!(app.last_error.is_none());
    }

    #[test]
    fn test_app_default() {
        let app1 = App::new();
        let app2 = App::default();
        assert_eq!(app1.current_screen, app2.current_screen);
        assert_eq!(app1.should_quit, app2.should_quit);
        assert_eq!(
            app1.overview_data.fleet_health,
            app2.overview_data.fleet_health
        );
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::new();
        assert!(!app.should_quit);
        app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(app.should_quit);
    }

    #[test]
    fn test_app_quit_ctrl_c() {
        let mut app = App::new();
        assert!(!app.should_quit);
        app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(app.should_quit);
    }

    #[test]
    fn test_app_quit_ctrl_q() {
        let mut app = App::new();
        assert!(!app.should_quit);
        app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL));
        assert!(app.should_quit);
    }

    #[test]
    fn test_app_screen_navigation_shortcuts() {
        let mut app = App::new();

        // Navigate to Machines with 'm'
        app.handle_key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Machines);

        // Navigate to Repos with 'r'
        app.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Repos);

        // Navigate to Help with '?'
        app.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Help);

        // Navigate back to Overview with 'o'
        app.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Overview);
    }

    #[test]
    fn test_app_tab_navigation() {
        let mut app = App::new();
        assert_eq!(app.current_screen, Screen::Overview);

        // Tab should move to next screen
        app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Machines);

        // Tab again
        app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Repos);
    }

    #[test]
    fn test_app_tab_wraps_around() {
        let mut app = App::new();
        let screens = Screen::all();

        // Navigate to last screen
        app.current_screen = screens[screens.len() - 1];
        assert_eq!(app.current_screen, Screen::Help);

        // Tab should wrap to first screen
        app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(app.current_screen, Screen::Overview);
    }

    #[test]
    fn test_app_unknown_key_no_effect() {
        let mut app = App::new();
        let initial_screen = app.current_screen;

        // Random key should not change state
        app.handle_key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
        assert_eq!(app.current_screen, initial_screen);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_app_all_screen_shortcuts_work() {
        for screen in Screen::all() {
            if let Some(shortcut) = screen.shortcut() {
                let mut app = App::new();
                app.handle_key(KeyEvent::new(KeyCode::Char(shortcut), KeyModifiers::NONE));
                assert_eq!(
                    app.current_screen, *screen,
                    "Shortcut '{shortcut}' should navigate to {screen:?}"
                );
            }
        }
    }

    // ==========================================================================
    // Elm Model Tests
    // ==========================================================================

    #[test]
    fn test_model_init_returns_overview() {
        use ftui::Model;
        let mut app = App::new();
        let _cmd = app.init();
        assert_eq!(app.current_screen, Screen::Overview);
    }

    #[test]
    fn test_model_init_returns_tick_cmd() {
        use ftui::Model;
        let mut app = App::new();
        let cmd = app.init();
        // init() returns Cmd::Msg(Tick) to trigger first data load
        assert!(matches!(cmd, ftui::Cmd::Msg(AppMessage::Tick)));
    }

    #[test]
    fn test_model_update_screen_changed() {
        use ftui::Model;
        let mut app = App::new();
        assert_eq!(app.current_screen, Screen::Overview);

        let _cmd = app.update(AppMessage::ScreenChanged(Screen::Machines));
        assert_eq!(app.current_screen, Screen::Machines);
    }

    #[test]
    fn test_model_update_key_tab_cycles() {
        use ftui::Model;
        let mut app = App::new();
        assert_eq!(app.current_screen, Screen::Overview);

        let cmd = app.update(AppMessage::Key(ftui::KeyEvent::new(ftui::KeyCode::Tab)));
        // Should produce a ScreenChanged message for Machines
        assert!(matches!(cmd, ftui::Cmd::Msg(AppMessage::ScreenChanged(Screen::Machines))));
    }

    #[test]
    fn test_model_update_key_shortcut_navigates() {
        use ftui::Model;
        let mut app = App::new();

        let cmd = app.update(AppMessage::Key(ftui::KeyEvent::new(ftui::KeyCode::Char('e'))));
        assert!(matches!(cmd, ftui::Cmd::Msg(AppMessage::ScreenChanged(Screen::Events))));
    }

    #[test]
    fn test_model_update_tick_returns_tick_cmd() {
        use ftui::Model;
        let mut app = App::new();

        let cmd = app.update(AppMessage::Tick);
        assert!(matches!(cmd, ftui::Cmd::Tick(_)));
    }

    #[test]
    fn test_model_update_data_refreshed() {
        use ftui::Model;
        let mut app = App::new();

        let new_overview = OverviewData {
            fleet_health: 95.0,
            ..OverviewData::default()
        };
        let _cmd = app.update(AppMessage::DataRefreshed(ScreenData::Overview(Box::new(
            new_overview,
        ))));
        assert!((app.overview_data.fleet_health - 95.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_model_update_quit() {
        use ftui::Model;
        let mut app = App::new();

        let cmd = app.update(AppMessage::Quit);
        assert!(app.should_quit);
        assert!(matches!(cmd, ftui::Cmd::Quit));
    }

    #[test]
    fn test_model_update_error() {
        use ftui::Model;
        let mut app = App::new();
        assert!(app.last_error.is_none());

        let _cmd = app.update(AppMessage::Error("test error".to_string()));
        assert_eq!(app.last_error.as_deref(), Some("test error"));
    }

    #[test]
    fn test_model_update_key_q_quits() {
        use ftui::Model;
        let mut app = App::new();

        let cmd = app.update(AppMessage::Key(ftui::KeyEvent::new(ftui::KeyCode::Char('q'))));
        assert!(matches!(cmd, ftui::Cmd::Msg(AppMessage::Quit)));
    }

    #[test]
    fn test_model_update_key_ctrl_c_quits() {
        use ftui::Model;
        let mut app = App::new();

        let key = ftui::KeyEvent::new(ftui::KeyCode::Char('c'))
            .with_modifiers(ftui::Modifiers::CTRL);
        let cmd = app.update(AppMessage::Key(key));
        assert!(matches!(cmd, ftui::Cmd::Msg(AppMessage::Quit)));
    }

    #[test]
    fn test_model_view_dispatches_all_screens() {
        use ftui::Model;
        let mut pool = ftui::GraphemePool::default();

        for screen in Screen::all() {
            let mut app = App::new();
            app.current_screen = *screen;
            let mut frame = ftui::Frame::new(80, 24, &mut pool);
            // Should not panic for any screen
            app.view(&mut frame);
        }
    }

    #[test]
    fn test_model_subscriptions_returns_tick() {
        use ftui::Model;
        let app = App::new();
        let subs = app.subscriptions();
        assert_eq!(subs.len(), 1);
    }

    #[test]
    fn test_from_event_key() {
        let event = ftui::Event::Key(ftui::KeyEvent::new(ftui::KeyCode::Char('x')));
        let msg: AppMessage = event.into();
        assert!(matches!(msg, AppMessage::Key(_)));
    }

    #[test]
    fn test_from_event_tick() {
        let event = ftui::Event::Tick;
        let msg: AppMessage = event.into();
        assert!(matches!(msg, AppMessage::Tick));
    }

    #[test]
    fn test_from_event_resize_becomes_key_null() {
        let event = ftui::Event::Resize {
            width: 80,
            height: 24,
        };
        let msg: AppMessage = event.into();
        assert!(matches!(msg, AppMessage::Key(_)));
    }

    // ==========================================================================
    // TuiError Tests
    // ==========================================================================

    #[test]
    fn test_tui_error_display() {
        let err = TuiError::TerminalError("resize failed".to_string());
        assert_eq!(format!("{err}"), "Terminal error: resize failed");
    }

    #[test]
    fn test_tui_error_from_io() {
        let io_err = std::io::Error::other("test");
        let tui_err: TuiError = io_err.into();
        assert!(matches!(tui_err, TuiError::IoError(_)));
    }
}
