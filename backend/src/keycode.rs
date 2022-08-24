// has_keycode logic; convert to/from int
// serialize (Display?)/format
// serde: serialize/deserialize as string

type Mod = String;

enum Keycode {
    // KC_NONE?
    Basic(Vec<Mod>, Option<String>),
    MT(Vec<Mod>, String),
    LT(usize, String),
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

    let mut mods = Vec::new();
    loop {
        mods.push(tokens.next()?.to_string());
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
