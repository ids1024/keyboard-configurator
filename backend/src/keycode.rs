// has_keycode logic; convert to/from int
// serialize (Display?)/format
// serde: serialize/deserialize as string

use bitflags::bitflags;

bitflags! {
    pub struct Mods: u16 {
        const CTRL = 0x1;
        const SHIFT = 0x2;
        const ALT = 0x4;
        const SUPER = 0x8;
        const RIGHT = 0x10;
    }
}

impl Mods {
    // Convert single modifier from name
    fn from_mod_str(s: &str) -> Option<Self> {
        match s {
            "LEFT_CTRL" => Some(Self::CTRL),
            "LEFT_SHIFT" => Some(Self::SHIFT),
            "LEFT_ALT" => Some(Self::ALT),
            "LEFT_SUPER" => Some(Self::SUPER),
            "RIGHT_CTRL" => Some(Self::RIGHT | Self::CTRL),
            "RIGHT_SHIFT" => Some(Self::RIGHT | Self::SHIFT),
            "RIGHT_ALT" => Some(Self::RIGHT | Self::ALT),
            "RIGHT_SUPER" => Some(Self::RIGHT | Self::SUPER),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Keycode {
    Basic(Mods, String),
    MT(Mods, String),
    LT(usize, String),
}

impl Keycode {
    pub fn is_none(&self) -> bool {
        if let Keycode::Basic(mode, keycode) = self {
            mode.is_empty() && keycode.as_str() == "NONE"
        } else {
            false
        }
    }

    pub fn is_roll_over(&self) -> bool {
        if let Keycode::Basic(mode, keycode) = self {
            mode.is_empty() && keycode.as_str() == "ROLL_OVER"
        } else {
            false
        }
    }
}

impl std::fmt::Display for Keycode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // XXX
        write!(f, "{:?}", self)
    }
}

const SEPARATORS: &[char] = &[',', '|', '(', ')'];

// Tokenize into iterator of &str, splitting on whitespace and putting
// separators in their own tokens.
fn tokenize(mut s: &str) -> impl Iterator<Item = &str> {
    std::iter::from_fn(move || {
        s = s.trim_start_matches(' ');
        let idx = if SEPARATORS.contains(&s.chars().next()?) {
            1
        } else {
            s.find(|c| c == ' ' || SEPARATORS.contains(&c))
                .unwrap_or(s.len())
        };
        let tok = &s[..idx];
        s = &s[idx..];
        Some(tok)
    })
}

// Use Vec or bitflags; need to represent left/right. Could use struct of bools. Or vec of enum.
// Bitflags may be easier to deal with.
fn parse_mods() {}

fn parse_mt<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Option<Keycode> {
    if tokens.next() != Some("(") {
        return None;
    }

    let mut mods = Mods::empty();
    loop {
        mods |= Mods::from_mod_str(tokens.next()?)?;
        match tokens.next()? {
            "," => {}
            ")" => {
                break;
            }
            _ => {
                return None;
            }
        }
    }

    let keycode = tokens.next()?.to_string();

    if (tokens.next(), tokens.next()) != (Some(")"), None) {
        return None;
    }

    Some(Keycode::MT(mods, keycode))
}

fn parse_lt<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Option<Keycode> {
    if tokens.next() != Some("(") {
        return None;
    }

    let layer = tokens.next()?.parse().ok()?;

    if tokens.next() != Some(",") {
        return None;
    }

    let keycode = tokens.next()?.to_string();

    if (tokens.next(), tokens.next()) != (Some(")"), None) {
        return None;
    }

    Some(Keycode::LT(layer, keycode))
}

// XXX handle mods
fn parse_basic<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Option<Keycode> {
    let keycode = tokens.next()?;
    let first_c = keycode.chars().next()?;
    if !first_c.is_alphanumeric() || tokens.next().is_some() {
        return None;
    }
    Some(Keycode::Basic(Mods::empty(), keycode.to_string()))
}

// XXX result
// Need to recurse handing `|`?
fn parse(s: &str) -> Option<Keycode> {
    let mut tokens = tokenize(s);
    match tokens.next()? {
        "MT" => parse_mt(tokens),
        "LT" => parse_lt(tokens),
        _ => None,
    }
}
