//! Shared test helper: a small cron-validity check.
//!
//! The suite asserts exact strings and that each string is a valid 5-field
//! cron expression. The two checks together catch both wrong output and
//! malformed shapes. The grammar here is the standard one: no seconds, no name
//! aliases.

/// Standard per-field value ranges, in field order.
const RANGES: [(i64, i64); 5] = [
    (0, 59), // minute
    (0, 23), // hour
    (1, 31), // day of month
    (1, 12), // month
    (0, 7),  // day of week (0 and 7 both Sunday)
];

/// Return true when `expr` is a valid 5-field cron string.
///
/// A field is `*`, or a comma list of terms. A term is `n`, `n-m`, `*/s`,
/// `n-m/s`, or `*`. Every integer must sit inside the field range and every
/// step must be positive.
pub fn is_valid_cron(expr: &str) -> bool {
    let fields: Vec<&str> = expr.split(' ').collect();
    if fields.len() != 5 {
        return false;
    }
    fields
        .iter()
        .zip(RANGES.iter())
        .all(|(field, &(lo, hi))| field_is_valid(field, lo, hi))
}

/// Check one cron field against its range.
fn field_is_valid(field: &str, lo: i64, hi: i64) -> bool {
    if field.is_empty() {
        return false;
    }
    field.split(',').all(|term| term_is_valid(term, lo, hi))
}

/// Check one comma term: value, range, or stepped form.
fn term_is_valid(term: &str, lo: i64, hi: i64) -> bool {
    if term.is_empty() {
        return false;
    }

    let (base, step) = match term.split_once('/') {
        Some((b, s)) => (b, Some(s)),
        None => (term, None),
    };

    if let Some(step) = step {
        match step.parse::<i64>() {
            Ok(s) if s > 0 => {}
            _ => return false,
        }
    }

    base_is_valid(base, lo, hi)
}

/// Check the part before a step: `*`, a single value, or a range.
fn base_is_valid(base: &str, lo: i64, hi: i64) -> bool {
    if base == "*" {
        return true;
    }
    if let Some((a, b)) = base.split_once('-') {
        return in_range(a, lo, hi) && in_range(b, lo, hi);
    }
    in_range(base, lo, hi)
}

/// Parse an integer and confirm it sits within `[lo, hi]`.
fn in_range(token: &str, lo: i64, hi: i64) -> bool {
    match token.parse::<i64>() {
        Ok(n) => n >= lo && n <= hi,
        Err(_) => false,
    }
}
