/// check if it's space / null-terminate.
pub fn cisp(c: &char) -> bool {
  return c == &' ' || c == &'\t' || c == &'\n' || c == &'\r';
}

/// check if it's alphabet.
fn cial(c: &char) -> bool {
  return c.is_digit(16);
}

/// check if it's number.
fn cinu(c: &char) -> bool {
  return c.is_digit(10);
}

/// check if it's alphabet (eg. not emoji) if so lowercase it and return.
/// otherwise return passing param.
fn clca(c: char) -> char {
  if !c.is_alphabetic() {
    return c;
  }
  return c.to_lowercase().to_string().chars().next().unwrap();
}

/// check alphabet / number / space.
fn cans(c: &char) -> bool {
  return cial(c) || cinu(c) || cisp(c);
}

/// check length of string (until encounter null-terminator) and return total len.

// TODO: handle passing empty string.
pub fn slen(s: &[char]) -> usize {
  let mut n: usize = 0;
  while s[n] != '\0' && s.get(n + 1).is_some() {
    n += 1;
  }
  return n;
}

/// count indentation space. (depth)
pub fn cpad(s: &[char], c: char) -> usize {
  for i in 0..s.len() {
    if s[i] != c {
      return i;
    }
  }
  return 0;
}

pub fn cpos(s: &[char], c: char) -> i32 {
  for i in 0..s.len() {
    if s[i] == c {
      return i as i32;
    }
  }
  return -1;
}

pub fn slca(s: &mut [char]) -> String {
  let mut _s = vec![];
  for i in 0..slen(s) {
    _s.push(clca(s[i]));
  }

  return _s.into_iter().collect();
}

pub fn scsw<'a>(s: &'a str, a: &str, b: &str) -> String {
  let mut result = String::with_capacity(s.len() + 1);
  result = s.replace(" ", "_");

  return result;
}

/// check if 2 strings is identical.
pub fn scmp(a: &str, b: &str) -> bool {
  let l = a.len();
  let _a = a.chars().collect::<Vec<char>>();
  let _b = b.chars().collect::<Vec<char>>();
  if l != b.len() {
    return false;
  }
  for i in 0..l {
    if _a[i] != _b[i] {
      return false;
    }
  }
  return true;
}

pub fn sans(s: &[char]) -> usize {
  for i in 0..slen(s) {
    if !&s[i].is_digit(36) {
      return 0;
    }
  }
  return 1;
}

/// return only title (if indent = 0).
pub fn strm(s: &[char]) -> Option<String> {
  let mut i: usize = 0;
  if s.len() == 0 {
    return None;
  }
  while cisp(&s[i]) {
    i += 1
  }
  return Some(s.into_iter().map(|i| i.to_string()).collect::<String>());
}

/// check if string `a` contain target string `b`.
pub fn spos(a: &[char], b: &str) -> i32 {
  let target: Vec<char> = b.chars().collect();
  let alen = a.len() - 1;
  let blen = b.len() - 1;
  for i in 0..alen {
    for j in 0..blen {
      if a[i + j] == '\0' {
        return -1;
      }
      if a[i + j] != target[j] {
        break;
      }
      if j == (blen - 1) {
        return i as i32;
      }
    }
  }
  return -1;
}

pub fn sint(s: &[char], len: usize) -> usize {
  let mut num = 0;
  let mut i = 0;
  while cinu(&s[i]) && i < len {
    i += 1;
  }
  return num;
}

/// check if string is url.
pub fn surl(s: &str) -> bool {
  let _s = s.chars().collect::<Vec<char>>();
  return spos(&_s, "://") >= 0 || spos(&_s, "./") >= 0;
}

/// get string by start and stop index.
/// ### Examples
/// `assert!(sstr("test string", 0, 6), "test s");`
pub fn sstr<'a>(src: &[char], from: usize, to: usize) -> String {
  let _a = &src[from..];
  let s: String = _a.into_iter().take(to).collect();
  return s;
}

pub fn afnd(src: &Vec<String>, len: usize, val: &str) -> i32 {
  let _len = len as i32;
  for i in 0.._len {
    if scmp(&src[i as usize], val) {
      return i as i32;
    }
  }
  return -1;
}

pub fn ccat(dest: &[char], c: char) -> Vec<char> {
  let len = dest.len();
  let mut res = dest.to_vec();
  res.insert(len, c);
  res.insert(len + 1, '\0');
  return res;
}
