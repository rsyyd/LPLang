// LPLang Standard Library — Result and Option types (Phase 12)
// These are regular enums; no compiler magic required.

enum Option {
    Some(int),
    None,
}

enum Result {
    Ok(int),
    Err(string),
}

fn is_some(opt: Option) -> bool {
    match opt {
        Some(_) => { return true; },
        None => { return false; },
    }
    return false;
}

fn is_ok(res: Result) -> bool {
    match res {
        Ok(_) => { return true; },
        Err(_) => { return false; },
    }
    return false;
}

fn unwrap_or(opt: Option, fallback: int) -> int {
    match opt {
        Some(v) => { return v; },
        None => { return fallback; },
    }
    return fallback;
}

fn unwrap_result_or(res: Result, fallback: int) -> int {
    match res {
        Ok(v) => { return v; },
        Err(_) => { return fallback; },
    }
    return fallback;
}
