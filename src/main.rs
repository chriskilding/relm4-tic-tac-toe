use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::rc::Rc;
use relm4::prelude::*;
use gtk::prelude::*;
use relm4::{SharedState};
use relm4_components::open_dialog::{OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings};
use relm4_components::save_dialog::{SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings};
use serde::{Deserialize, Serialize};

///
/// Game of tic-tac-toe
///

const DEFAULT_SPACING: i32 = 5;

///
/// The view
///

struct GameView {
    // document state which is external to the Component, intended to be saved/loaded
    document: Rc<SharedState<Document>>,

    // local state within the Component
    foo: String,
}

#[derive(Debug)]
enum GameViewInput {
    Play(usize, usize),

    None,
}

#[relm4::component(pub)]
impl SimpleComponent for GameView {
    type Input = GameViewInput;
    type Output = ();
    type Init = Rc<SharedState<Document>>;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: DEFAULT_SPACING,

            gtk::Box {
                set_spacing: DEFAULT_SPACING,

                gtk::Label {
                    set_text: "Foo:"
                },

                gtk::Label {
                    set_text: &model.foo,
                },
            },

            gtk::Box {
                set_spacing: DEFAULT_SPACING,

                gtk::Label {
                    set_text: "Current Player:"
                },

                gtk::Label {
                    #[watch]
                    set_text: &model.document.read().game.current_player.to_string(),
                }
            },

            gtk::Grid {
                set_row_spacing: 5,
                set_column_spacing: 5,

                attach[0, 0, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[0][0].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[0][0].is_sensitive(),

                    connect_clicked => GameViewInput::Play(0, 0),
                },

                attach[1, 0, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[1][0].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[1][0].is_sensitive(),

                    connect_clicked => GameViewInput::Play(1, 0),
                },

                attach[2, 0, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[2][0].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[2][0].is_sensitive(),

                    connect_clicked => GameViewInput::Play(2, 0),
                },

                attach[0, 1, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[0][1].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[0][1].is_sensitive(),

                    connect_clicked => GameViewInput::Play(0, 1),
                },

                attach[1, 1, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[1][1].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[1][1].is_sensitive(),

                    connect_clicked => GameViewInput::Play(1, 1),
                },

                attach[2, 1, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[2][1].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[2][1].is_sensitive(),

                    connect_clicked => GameViewInput::Play(2, 1),
                },

                attach[0, 2, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[0][2].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[0][2].is_sensitive(),

                    connect_clicked => GameViewInput::Play(0, 2),
                },

                attach[1, 2, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[1][2].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[1][2].is_sensitive(),

                    connect_clicked => GameViewInput::Play(1, 2),
                },

                attach[2, 2, 1, 1] = &gtk::Button {
                    #[watch]
                    set_label: &model.document.read().game.board[2][2].to_string(),

                    #[watch]
                    set_sensitive: model.document.read().game.board[2][2].is_sensitive(),

                    connect_clicked => GameViewInput::Play(2, 2),
                },
            }
        }
    }

    fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = Self {
            document: init,
            foo: "Hello".into(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            GameViewInput::Play(x, y) => {
                println!("played at ({}, {})", x, y);

                // bounds checking not included
                let mut doc = self.document.write();
                doc.game.board[x][y] = Piece::from(doc.game.current_player);
                doc.game.current_player.toggle();
            }
            GameViewInput::None => {}
        }
    }
}

impl From<Player> for Piece {
    fn from(value: Player) -> Self {
        match value {
            Player::X => Piece::X,
            Player::O => Piece::O,
        }
    }
}

///
/// The 'document'
///

#[derive(Debug, Default)]
struct Document {
    game: Game
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Game {
    board: Board,
    current_player: Player,
}

type Board = Vec<Vec<Piece>>;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::X
    }
}

impl Player {
    fn toggle(&mut self) {
        match self {
            Player::X => {
                *self = Player::O;
            },
            Player::O => {
                *self = Player::X;
            },
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Deserialize, Serialize)]
enum Piece {
    X,
    O,
    Empty,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Piece::X => "X",
            Piece::O => "O",
            Piece::Empty => "",
        };

        write!(f, "{}", str)
    }
}

impl Piece {
    fn is_sensitive(&self) -> bool {
        *self == Piece::Empty
    }
}

fn standard_document() -> SharedState<Document> {
    let document: SharedState<Document> = SharedState::new();
    document.write().game = standard_game();

    document
}

/// Create a default game on a 3x3 board and Player 'X' starting
fn standard_game() -> Game {
    let board = standard_board();
    let current_player = Player::X;

    Game {
        board, current_player
    }
}

fn standard_board() -> Vec<Vec<Piece>> {
    vec![
        vec![Piece::Empty, Piece::Empty, Piece::Empty],
        vec![Piece::Empty, Piece::Empty, Piece::Empty],
        vec![Piece::Empty, Piece::Empty, Piece::Empty],
    ]
}

///
/// The app wrapper
///

struct App {
    document: Rc<SharedState<Document>>,
    game_view: Controller<GameView>,
    save_dialog: Controller<SaveDialog>,
    load_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
enum AppInput {
    NewGame,

    SaveGameResponse(PathBuf),
    SaveGame,

    LoadGame,
    LoadGameResponse(PathBuf),

    None // FIXME dummy event
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Input = AppInput;
    type Output = ();
    type Init = ();

    view! {
        gtk::ApplicationWindow {
            set_default_width: 640,
            set_default_height: 480,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::HeaderBar {
                    set_show_title_buttons: false,

                    pack_start = &gtk::Button {
                        set_label: "Reset",
                        connect_clicked => AppInput::NewGame,
                    },

                    pack_end = &gtk::Button {
                        set_label: "Save",
                        connect_clicked => AppInput::SaveGame,
                    },

                    pack_end = &gtk::Button {
                        set_label: "Load",
                        connect_clicked => AppInput::LoadGame,
                    }
                },

                model.game_view.widget(),
            }
        }
    }

    fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        // this is the game document that will be displayed in the window
        let document = Rc::new(standard_document());

        let game_view = GameView::builder()
            .launch(Rc::clone(&document))
            .detach();

        let save_dialog = SaveDialog::builder()
            .transient_for_native(root)
            .launch(SaveDialogSettings {
                create_folders: true,
                accept_label: "Save".into(),
                cancel_label: "Cancel".into(),
                is_modal: true,
                filters: game_filename_filters(),
            })
            .forward(sender.input_sender(), |response| match response {
                SaveDialogResponse::Accept(path) => AppInput::SaveGameResponse(path),
                SaveDialogResponse::Cancel => AppInput::None,
            });

        let load_dialog = OpenDialog::builder()
            .transient_for_native(root)
            .launch(OpenDialogSettings {
                create_folders: false,
                folder_mode: false,
                cancel_label: "Cancel".into(),
                accept_label: "Load".into(),
                is_modal: true,
                filters: game_filename_filters(),
            })
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => AppInput::LoadGameResponse(path),
                OpenDialogResponse::Cancel => AppInput::None,
            });

        let model = Self {
            document: Rc::clone(&document),
            game_view,
            save_dialog,
            load_dialog,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::NewGame => {
                let mut g = self.document.write();
                g.game.board = standard_board();
                g.game.current_player = Player::X;

                // FIXME changes applied to model, but the GameView does not refresh. We must raise a dummy event on GameView to trigger the refresh.
                self.game_view.emit(GameViewInput::None);
            }
            AppInput::SaveGame => {
                let name = "game.json".into();
                self.save_dialog.emit(SaveDialogMsg::SaveAs(name));
            }
            AppInput::SaveGameResponse(path) => {
                let doc = self.document.read();
                let json = serde_json::to_string(&doc.game).unwrap();
                std::fs::write(path, json).unwrap();
            }
            AppInput::LoadGame => {
                self.load_dialog.emit(OpenDialogMsg::Open);
            }
            AppInput::LoadGameResponse(path) => {
                let json = std::fs::read_to_string(path).unwrap();
                let game: Game = serde_json::from_str(&*json).unwrap();
                let mut doc = self.document.write();
                doc.game = game;

                // FIXME changes applied to model, but the GameView does not refresh. We must raise a dummy event on GameView to trigger the refresh.
                self.game_view.emit(GameViewInput::None);
            }
            AppInput::None => {}
        }
    }
}

fn game_filename_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::default();
    filename_filter.set_name(Some("JSON (.json)"));
    filename_filter.add_suffix("json");

    vec![filename_filter]
}

///
/// The application entry point
///

fn main() {
    let app = RelmApp::new("org.relm4.examples.TicTacToe");

    app.run::<App>(());
}
