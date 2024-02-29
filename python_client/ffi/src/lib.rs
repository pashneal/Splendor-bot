use pyo3::prelude::*;
use splendor_tourney::*;
use url::Url;
use tungstenite::{connect, Message};

/// A Python wrapper for the `GemType` enum
#[pyclass]
#[derive(Debug, Clone)]
pub enum PyGemType {
    Onyx,
    Sapphire,
    Emerald,
    Ruby,
    Diamond,
    Gold,
}

/// A python wrapper for the `Tokens` struct
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyTokens {
    #[pyo3(get)]
    pub onyx: i8,
    #[pyo3(get)]
    pub sapphire: i8,
    #[pyo3(get)]
    pub emerald: i8,
    #[pyo3(get)]
    pub ruby: i8,
    #[pyo3(get)]
    pub diamond: i8,
    #[pyo3(get)]
    pub gold: i8,
}

impl PyTokens {
    pub fn from_cost(cost: Cost) -> Self {
        let tokens = cost.to_tokens();
        PyTokens {
            onyx: tokens.onyx,
            sapphire: tokens.sapphire,
            emerald: tokens.emerald,
            ruby: tokens.ruby,
            diamond: tokens.diamond,
            gold: tokens.gold,
        }
    }
    pub fn from(tokens: Tokens) -> Self {
        PyTokens {
            onyx: tokens.onyx,
            sapphire: tokens.sapphire,
            emerald: tokens.emerald,
            ruby: tokens.ruby,
            diamond: tokens.diamond,
            gold: tokens.gold,
        }
    }

    pub fn into_tokens(self) -> Tokens {
        Tokens {
            onyx: self.onyx,
            sapphire: self.sapphire,
            emerald: self.emerald,
            ruby: self.ruby,
            diamond: self.diamond,
            gold: self.gold,
        }
    }
}

#[pymethods]
impl PyTokens {
    #[new]
    pub fn new(
        onyx: Option<i8>,
        sapphire: Option<i8>,
        emerald: Option<i8>,
        ruby: Option<i8>,
        diamond: Option<i8>,
        gold: Option<i8>,
    ) -> Self {
        PyTokens {
            onyx: onyx.unwrap_or(0),
            sapphire: sapphire.unwrap_or(0),
            emerald: emerald.unwrap_or(0),
            ruby: ruby.unwrap_or(0),
            diamond: diamond.unwrap_or(0),
            gold: gold.unwrap_or(0),
        }
    }

    pub fn __str__(&self) -> String {
        //TODO : perhaps we ignore the 0 values?
        format!(
            "onyx: {}, sapphire: {}, emerald: {}, ruby: {}, diamond: {}, gold: {}",
            self.onyx, self.sapphire, self.emerald, self.ruby, self.diamond, self.gold
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum PyActionType {
    TakeGems,
    Reserve,
    ReserveHidden,
    Discard,
    Purchase,
    AttractNoble,
    Pass,
    Continue,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyAction {
    action_type: PyActionType,
    card_id: Option<CardId>,
    noble_id: Option<NobleId>,
    tokens: Option<PyTokens>,
    tier: Option<usize>,
}

/// Rust side only functions for the PyAction enum
impl PyAction {
    pub fn from(action: Action) -> Self {
        let action_type = match &action {
            Action::TakeDouble(_) => PyActionType::TakeGems,
            Action::TakeDistinct(_) => PyActionType::TakeGems,
            Action::Reserve(_) => PyActionType::Reserve,
            Action::ReserveHidden(_) => PyActionType::ReserveHidden,
            Action::Discard(_) => PyActionType::Discard,
            Action::Purchase(_) => PyActionType::Purchase,
            Action::AttractNoble(_) => PyActionType::AttractNoble,
            Action::Pass => PyActionType::Pass,
            Action::Continue => PyActionType::Continue,
        };
        let card_id = match &action {
            Action::Reserve(card_id) => Some(*card_id),
            Action::Purchase((card_id, _)) => Some(*card_id),
            _ => None,
        };

        let noble_id = match &action {
            Action::AttractNoble(noble_id) => Some(*noble_id),
            _ => None,
        };

        let tokens = match &action {
            Action::TakeDouble(color) => {
                let color = *color;
                let mut tokens = Tokens::empty();
                tokens += Tokens::one(color.clone());
                tokens += Tokens::one(color.clone());
                Some(PyTokens::from(tokens))
            }
            Action::TakeDistinct(color_set) => Some(PyTokens::from(Tokens::from_set(color_set))),
            Action::Discard(tokens) => Some(PyTokens::from(*tokens)),
            Action::Purchase((_, tokens)) => Some(PyTokens::from(*tokens)),
            _ => None,
        };

        let tier = match &action {
            Action::ReserveHidden(tier) => Some(*tier),
            _ => None,
        };

        PyAction {
            action_type,
            card_id,
            noble_id,
            tokens,
            tier,
        }
    }

    pub fn into_action(self) -> Action  {
        match self.action_type {
             PyActionType::TakeGems => {
                 let py_tokens = self.tokens();
                 let tokens = py_tokens.into_tokens();
                 let is_double = tokens.total() == 2 && tokens.to_set().len() == 1;

                 match is_double {
                     true => {
                         let mut color = GemType::Gold;
                         for c in tokens.to_set() {
                             color = c
                         }
                         Action::TakeDouble(color)
                     }
                     false => {
                         Action::TakeDistinct(tokens.to_set())
                     }
                 }
             }
             PyActionType::Reserve =>Action::Reserve(self.card_id()),
             PyActionType::ReserveHidden =>Action::ReserveHidden(self.tier()),
             PyActionType::Discard =>Action::Discard(self.tokens().into_tokens()),
             PyActionType::Purchase =>Action::Purchase((self.card_id(), self.tokens().into_tokens())),
             PyActionType::AttractNoble =>Action::AttractNoble(self.noble_id()),
             PyActionType::Pass =>Action::Pass,
             PyActionType::Continue =>Action::Continue,
        }

    }
}

/// Separate the Rust-only struct enum Action to Python-like objects with PyAction
/// TODO: (if i'm feeling nice) make error messages more helpful?
#[pymethods]
impl PyAction {
    pub fn action_type(&self) -> PyActionType {
        self.action_type.clone()
    }

    pub fn __str__(&self) -> String {
        match self.action_type.clone() {
            PyActionType::TakeGems => {
                let tokens = self.tokens();
                format!("TakeGems({})", tokens.__str__())
            }
            PyActionType::Reserve => {
                let card_id = self.card_id();
                format!("Reserve(card_id : {})", card_id)
            }
            PyActionType::ReserveHidden => {
                let tier = self.tier();
                format!("ReserveHidden(tier : {})", tier)
            }
            PyActionType::Discard => {
                let tokens = self.tokens();
                format!("Discard({})", tokens.__str__())
            }
            PyActionType::Purchase => {
                let card_id = self.card_id();
                let tokens = self.tokens();
                format!("Purchase({}, {})", card_id, tokens.__str__())
            }
            PyActionType::AttractNoble => {
                let noble_id = self.noble_id();
                format!("AttractNoble({})", noble_id)
            }
            PyActionType::Pass => "Pass".to_string(),
            PyActionType::Continue => "Continue".to_string(),
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.__str__())
    }

    pub fn card_id(&self) -> CardId {
        let error_message = format!(
            "This action ({:?}) does not have a card_id",
            self.action_type
        );
        self.card_id.expect(&error_message)
    }

    pub fn noble_id(&self) -> NobleId {
        let error_message = format!(
            "This action ({:?}) does not have a noble_id",
            self.action_type
        );
        self.noble_id.expect(&error_message)
    }

    pub fn tokens(&self) -> PyTokens {
        match self.tokens.clone() {
            None => panic!("This action ({:?}) does not have tokens", self.action_type),
            Some(tokens) => tokens,
        }
    }

    pub fn tier(&self) -> usize {
        match self.tier {
            None => panic!("This action ({:?}) does not have tokens", self.action_type),
            Some(tier) => tier,
        }
    }
}

/// A Python wrapper for the `ClientInfo` struct
#[pyclass]
pub struct PyClientInfo {
    #[pyo3(get)]
    pub board: PyBoard,
    #[pyo3(get)]
    pub game_history: PyGameHistory,
    #[pyo3(get)]
    pub players: Vec<PyPlayerPublicInfo>,
    #[pyo3(get)]
    pub current_player: PyPlayer,
    #[pyo3(get)]
    pub current_player_num: usize,
    #[pyo3(get)]
    pub legal_actions: Vec<PyAction>,
}

impl PyClientInfo {
    pub fn from_client_info(client_info: ClientInfo) -> Self {
        let legal_actions = client_info.legal_actions;
        let py_legal_actions = legal_actions.into_iter().map(PyAction::from).collect();
        let py_current_player = PyPlayer::from(&client_info.current_player);
        let py_player_public_info = client_info
            .players
            .iter()
            .map(PyPlayerPublicInfo::from)
            .collect();
        let py_board = PyBoard::from(&client_info.board);
        let py_game_history = PyGameHistory::from(client_info.history);

        PyClientInfo {
            board: py_board,
            game_history: py_game_history,
            players: py_player_public_info,
            current_player: py_current_player,
            current_player_num: client_info.current_player_num,
            legal_actions: py_legal_actions,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyPlayer {
    #[pyo3(get)]
    points: u8,
    #[pyo3(get)]
    reserved: Vec<CardId>,
    #[pyo3(get)]
    gems: PyTokens,
    #[pyo3(get)]
    developments: PyTokens,
    #[pyo3(get)]
    blind_reserved: Vec<CardId>,
}

impl PyPlayer {
    pub fn from(player: &Player) -> Self {
        PyPlayer {
            points: player.points(),
            reserved: player.all_reserved(),
            gems: PyTokens::from(*player.gems()),
            developments: PyTokens::from(*player.developments()),
            blind_reserved: player.blind_reserved(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyPlayerPublicInfo {
    #[pyo3(get)]
    points: u8,
    #[pyo3(get)]
    num_reserved: usize,
    #[pyo3(get)]
    developments: PyTokens,
    #[pyo3(get)]
    gems: PyTokens,
}

impl PyPlayerPublicInfo {
    pub fn from(player: &PlayerPublicInfo) -> Self {
        PyPlayerPublicInfo {
            points: player.points,
            num_reserved: player.num_reserved,
            developments: PyTokens::from(player.developments.to_tokens()),
            gems: PyTokens::from(player.gems),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBoard {
    #[pyo3(get)]
    pub deck_counts: [usize; 3],
    #[pyo3(get)]
    pub available_cards: Vec<Vec<CardId>>,
    #[pyo3(get)]
    pub nobles: Vec<NobleId>,
    #[pyo3(get)]
    pub tokens: PyTokens,
}

impl PyBoard {
    pub fn from(board: &Board) -> Self {
        PyBoard {
            deck_counts: board.deck_counts,
            available_cards: board.available_cards.clone(),
            nobles: board.nobles.clone(),
            tokens: PyTokens::from(board.tokens),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyGameHistory {
    // TODO: encapsulate history information in clean intuitive interface
    // rather than just exposing the raw history
    #[pyo3(get)]
    pub history: Vec<(usize, PyAction)>,
}

impl PyGameHistory {
    pub fn from(history: GameHistory) -> Self {
        let py_history = history
            .into_iter()
            .map(|(player_num, action)| (player_num, PyAction::from(action)))
            .collect();
        PyGameHistory {
            history: py_history,
        }
    }
}

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}


#[pymodule]
fn ffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(run_python_bot, m)?)?;

    m.add_class::<PyClientInfo>()?;
    m.add_class::<PyPlayer>()?;
    m.add_class::<PyActionType>()?;
    m.add_class::<PyTokens>()?;
    m.add_class::<PyAction>()?;
    Ok(())
}

/// A struct for making sure that the bot on the Python side
/// has proper access to the log stream protocol of the library
#[pyclass]
pub struct PyLog  {
    log : Log,
}

impl PyLog {
    pub fn new(port : u16) -> Self {
        PyLog {
            log : Log::new(port),
        }
    }
}

/// Expose a method that allows for python-side logging
#[pymethods]
impl PyLog {
    pub fn send(&mut self, message: &str) {
        self.log.send(message);
    }
}

#[pyfunction]
pub fn run_python_bot(py: Python, bot_class: &PyAny) {
    let port = 3030;

    let url = format!("ws://127.0.0.1:{}/game", port);
    let url = Url::parse(&url).unwrap();
    let (mut game_socket, _) = connect(url).expect("Can't connect to the game server");

    // Give the server a chance to start up
    std::thread::sleep(std::time::Duration::from_millis(100));

    let py_log = PyCell::new(py , PyLog::new(port)).unwrap();

    let bot_instance = bot_class.call1((py_log.try_borrow_mut().unwrap(),)).expect("Unable to launch bot, could not call __init__");

    loop {
        let msg = game_socket.read().expect("Error reading message");
        let msg = msg.to_text().expect("Error converting message to text");
        let info: ClientInfo = serde_json::from_str(msg).expect("Error parsing message");
        let py_info = PyClientInfo::from_client_info(info);
        let result = bot_instance.call_method1("take_action", (py_info, py_log.try_borrow_mut().unwrap()));
        let py_action : PyAction = result.expect("Error when calling method take_action()")
            .extract()
            .expect("Incorrect type returned by method take_action()");

        let action = py_action.into_action();

        let msg = ClientMessage::Action(action);
        let msg_str = serde_json::to_string(&msg).expect("Error converting action to string");
        game_socket.send(Message::Text(msg_str)).expect("Error sending message");
    }

}
