pub fn nada() {
    // No op
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_nada() {
        let result = nada();
        assert_eq!(result, ());
    }
}
