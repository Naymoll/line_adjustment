#[derive(Default)]
struct Line<'a> {
    words: Vec<&'a str>,
    symbols: usize,
}

impl<'a> Line<'a> {
    fn new(word: &'a str) -> Self {
        Self {
            words: vec![word],
            symbols: word.len(),
        }
    }

    fn push(&mut self, word: &'a str) {
        self.words.push(word);
        self.symbols += word.len();
    }
}

pub fn transform(input: &str, line_width: u32) -> String {
    let line_width = line_width as usize;

    let is_pad_okay = |symbols, spaces| line_width.saturating_sub(symbols) / spaces != 0;
    let lines_result: Result<_, String> =
        input
            .split_whitespace()
            .try_fold(Vec::<Line>::new(), |mut lines, word| {
                match lines.last_mut() {
                    Some(last) if is_pad_okay(last.symbols + word.len(), last.words.len()) => {
                        last.push(word)
                    }
                    _ if word.len() <= line_width => lines.push(Line::new(word)),
                    _ => return Err(word.to_owned()), // Если длина слова больше line_width
                }
                Ok(lines)
            });

    let lines = match lines_result {
        Ok(l) => l,
        Err(_) => return "".to_owned(),
    };

    let mut result = String::with_capacity(lines.len() * (line_width + 1));
    let mut iter = lines.into_iter().peekable();
    while let Some(line) = iter.next() {
        let Line { words, symbols } = line;

        let (pad, mut extra) = {
            let diff = line_width - symbols;
            let len = usize::max(words.len() - 1, 1);
            (diff / len, diff % len)
        };

        match words.split_last() {
            Some((last, [])) => {
                let offset = std::iter::repeat(' ').take(pad + extra);
                result.extend(last.chars().chain(offset));
            }
            Some((last, rest)) => {
                for word in rest {
                    let offset = std::iter::repeat(' ').take(pad + usize::from(extra > 0));
                    result.extend(word.chars().chain(offset));
                    extra = extra.saturating_sub(1);
                }
                result.push_str(last);
            }
            None => unreachable!(), // Должно быть как минимум одно слово
        }

        if iter.peek().is_some() {
            result.push('\n')
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
            ("Lorem     ipsum    dolor", 3, ""),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}
