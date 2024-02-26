use pyo3::prelude::*;
use splendor_tourney::*;

/// A Python wrapper for the `Color` enum
#[pyclass]
#[derive(Debug, Clone)]
pub enum PyColor {
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
}

#[pymethods]
impl PyTokens {
    #[new]
    pub fn new(onyx: Option<i8>, 
               sapphire: Option<i8>,
               emerald: Option<i8>,
               ruby: Option<i8>,
               diamond: Option<i8>,
               gold: Option<i8>) -> Self {

        PyTokens {
            onyx : onyx.unwrap_or(0),
            sapphire : sapphire.unwrap_or(0),
            emerald : emerald.unwrap_or(0),
            ruby : ruby.unwrap_or(0),
            diamond : diamond.unwrap_or(0),
            gold : gold.unwrap_or(0),
        }
    }

    pub fn __str__ (&self) -> String{
        //TODO : perhaps we ignore the 0 values?
        format!("onyx: {}, sapphire: {}, emerald: {}, ruby: {}, diamond: {}, gold: {}", 
                self.onyx, self.sapphire, self.emerald, self.ruby, self.diamond, self.gold)
    }

    pub fn __repr__ (&self) -> String{
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
    pub fn from(action : Action) ->  Self {
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
            Action::Purchase((card_id , _)) => Some(*card_id),
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
}


/// Implement duck typing functions for the PyAction enum
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
            },
            PyActionType::Reserve => {
                let card_id = self.card_id();
                format!("Reserve(card_id : {})", card_id)
            },
            PyActionType::ReserveHidden => {
                let tier = self.tier();
                format!("ReserveHidden(tier : {})", tier)
            },
            PyActionType::Discard => {
                let tokens = self.tokens();
                format!("Discard({})", tokens.__str__())
            },
            PyActionType::Purchase => {
                let card_id = self.card_id();
                let tokens = self.tokens();
                format!("Purchase({}, {})", card_id, tokens.__str__())
            },
            PyActionType::AttractNoble => {
                let noble_id = self.noble_id();
                format!("AttractNoble({})", noble_id)
            },
            PyActionType::Pass => "Pass".to_string(),
            PyActionType::Continue => "Continue".to_string(),
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.__str__())
    }

    pub fn card_id(&self) -> CardId {
        let error_message = format!("This action ({:?}) does not have a card_id", self.action_type);
        self.card_id.expect(&error_message)
    }

    pub fn noble_id(&self) -> NobleId {

        let error_message = format!("This action ({:?}) does not have a noble_id", self.action_type);
        self.noble_id.expect(&error_message)
    }

    pub fn tokens(&self) -> PyTokens {
        match self.tokens.clone() {
            None => panic!("This action ({:?}) does not have tokens", self.action_type),
            Some(tokens) => tokens
        }
    }

    pub fn tier(&self) -> usize {
        match self.tier{
            None => panic!("This action ({:?}) does not have tokens", self.action_type),
            Some(tier) => tier
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
        let py_player_public_info = client_info.players.iter().map(PyPlayerPublicInfo::from).collect();
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
        let py_history = history.into_iter().map(|(player_num, action)| (player_num, PyAction::from(action))).collect();
        PyGameHistory {
            history: py_history,
        }
    }
}

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyfunction]
pub fn test_from_json(json : String) -> PyClientInfo {
    println!("before {}", json);
    let client_info = ClientInfo::from_json(&json);
    let py_client_info = PyClientInfo::from_client_info(client_info);
    py_client_info
}

#[pymodule]
fn ffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(test_from_json, m)?)?;
    m.add_class::<PyClientInfo>()?;
    m.add_class::<PyPlayer>()?;
    m.add_class::<PyActionType>()?;
    m.add_class::<PyTokens>()?;
    m.add_class::<PyAction>()?;
    Ok(())
}


