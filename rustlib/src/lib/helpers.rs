/// check is space / null-terminate.
pub fn cisp(c: &char) -> bool {
	return c == &' ' || c == &'\t' || c == &'\n' || c == &'\r';
}

/// check is alphabet.
fn cial(c: &char) -> bool {
	return c.is_digit(16);
}

/// check is number.
fn cinu(c: &char) -> bool {
	return c.is_digit(10);
}

fn clca(c: char) -> char {
	if c.is_alphabetic() {
		c.to_lowercase().to_string().chars().next().unwrap()
	} else {
		c
	}
}

/// check alphabet / number / space.
fn cans(c: &char) -> bool {
	return cial(c) || cinu(c) || cisp(c);
}

// fn cuca(c: &str) -> i32 {
// 	return c >= 'a' && c <= 'z' ? c - ('a' - 'A') : c;
// }

/// check length of string (until encounter null-terminator) and return total len.
///
/// TODO: handle passing empty string.
pub fn slen(s: &[char]) -> usize {
	let mut n: usize = 0;
	while s[n] != '\0' && s.get(n + 1).is_some() {
		n += 1;
	}
	return n;
}

/// count indent (depth)
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

// char*
// suca(char* s)
// {
// 	int i;
// 	for(i = 0; i < slen(s); i++)
// 		s[i] = cuca(s[i]);
// 	return s;
// }

// pub fn slca(word: &str) -> String {
// 	let mut result = String::with_capacity(word.len());

// 	if !word.is_empty() {
// 		let mut chars = word.chars();
// 		let result: String = chars
//                   .map(|c| c.to_uppercase().to_string())
//                   .collect();
// 	}

// 	result
// }

pub fn slca(s: &mut [char]) -> String {
	let mut _s = vec![];
	for i in 0..slen(s) {
		_s.push(clca(s[i]));
	}

	return _s.into_iter().collect();
}

pub fn scsw<'a>(s: &'a str, a: &str, b: &str) -> String {
	let mut result = String::with_capacity(s.len() + 1);
	// for i in 0..s.len() {
	// 	if s.as_bytes()[i] == a.as_bytes()[0] {
	// 		s.as_bytes()[i] = b.as_bytes()[0];
	// 		// result.push(s)
	// 	} else {
	// 		// s.as_bytes()[i] = s.as_bytes()[i];
	// 	}
	// }
	result = s.replace(" ", "_");

	return result;
}

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

/// return only title (indent = 0).
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
		// num = num * 10 + (s[i] - '0');
		i += 1;
	}
	return num;
}

pub fn surl(s: &str) -> bool {
	let _s = s.chars().collect::<Vec<char>>();
	// println!("surl = {:?}", &s);
	return spos(&_s, "://") >= 0 || spos(&_s, "./") >= 0;
}

// pub fn scpy(src: &[char], dest: &[char]) -> Vec<char> {
// 	let mut i = 0;
// 	let mut _dest = dest.to_vec();
// 	while _dest.len() == 0 {
// 		_dest.insert(i, src[i]);
// 		if _dest[i] != '\0' {
// 			println!("dest = {:?}", _dest);
// 			i += 1;
// 		} else {
// 			break;
// 		}
// 	}
// 	return _dest;
// }

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
