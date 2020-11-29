/// check is space / null-terminate.
pub fn cisp(c: &char) -> bool {
	return c == &' ' || c == &'\t' || c == &'\n' || c == &'\r';
}

/// check is alphabet.
fn cial(c: &char) -> bool {
	// return (c >= &"a" && c <= &"z") || (c >= &"A" && c <= &"Z");
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
	while s[n] != '\0' && s.get(n+1).is_some() { 
		n += 1;
	};
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
	let i: usize;

	for i in 0..s.len() {
		if s[i] == c { return i as i32; }
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
	for i in 0..slen(s){
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




// int
// scmp(char* a, char* b)
// {
// 	int i, l = slen(a);
// 	if(l != slen(b))
// 		return 0;
// 	for(i = 0; i < l; ++i)
// 		if(a[i] != b[i])
// 			return 0;
// 	return 1;
// }

pub fn sans(s: &[char]) -> usize {
	for i in 0..slen(s) {
		// if !cans(&s[i]) {
		if !&s[i].is_digit(36) {
			return 0;
		}
	}
	return 1;
}


/// return only title (indent = 0).
pub fn strm(s: &[char]) -> Option<String> {
	let mut i: usize = 0;
	if s.len() == 0 { return None }
	while cisp(&s[i]) { i += 1 };
	// if s[i] == '\0' { 
	// 	return Some(s.into_iter()
	// 					.map(|i| i.to_string())
	// 					.collect::<String>()); 
	// };
	return Some(s.into_iter()
					.map(|i| i.to_string())
					.collect::<String>());
}

pub fn spos(a: &[char], b: &str) -> i32 {
	let alen = a.len();
	let blen = b.len();

	for i in 0..alen {
		for j in 0..blen {
			if a[i + j] == '\0' { return -1; }
			if a[i + j] != b.chars().nth(j).unwrap() { break; }
			if j == blen - 1 { return i as i32; }
		}
	}
	return -1;
}

// int
// sint(char* s, int len)
// {
// 	int num = 0, i = 0;
// 	while(s[i] && cinu(s[i]) && i < len) {
// 		num = num * 10 + (s[i] - '0');
// 		i++;
// 	}
// 	return num;
// }

// int
// surl(char* s)
// {
// 	return spos(s, "://") >= 0 || spos(s, "./") >= 0;
// }

// char*
// scpy(char* src, char* dest)
// {
// 	int i = 0;
// 	while((dest[i] = src[i]) != '\0')
// 		i++;
// 	return dest;
// }

pub fn sstr<'a>(src: &[char], from: usize, to: usize) -> String {
	let _a = &src[from..];
	let s: String  = _a.into_iter().take(to).collect();
	return s;
}

// int
// afnd(char* src[], int len, char* val)
// {
// 	int i;
// 	for(i = 0; i < len; i++)
// 		if(scmp(src[i], val))
// 			return i;
// 	return -1;
// }

// char*
// ccat(char* dest, char c)
// {
// 	int len = slen(dest);
// 	dest[len] = c;
// 	dest[len + 1] = '\0';
// 	return dest;
// }

// char*
// scat(char* dest, const char* src)
// {
// 	char* ptr = dest + slen(dest);
// 	while(*src != '\0')
// 		*ptr++ = *src++;
// 	*ptr = '\0';
// 	return dest;
// }

// /* old */

// void
// swapstr(char* src, char* dest, char* a, char* b)
// {
// 	char head[1024], tail[1024];
// 	int index = spos(src, a);
// 	if(index < 0)
// 		return;
// 	sstr(src, head, 0, index);
// 	sstr(src, tail, index + slen(a), slen(src) - index - slen(a));
// 	dest[0] = '\0';
// 	scat(dest, head);
// 	scat(dest, b);
// 	scat(dest, tail);
// }

// void
// firstword(char* src, char* dest)
// {
// 	int until = cpos(src, ' ');
// 	if(until > -1)
// 		sstr(src, dest, 0, until);
// 	else
// 		sstr(src, dest, 0, slen(src));
// }

// float
// clock_since(clock_t start)
// {
// 	double cpu_time_used = ((double)(clock() - start)) / CLOCKS_PER_SEC;
// 	return cpu_time_used * 1000;
// }

// char*
// nowstr(void)
// {
// 	time_t now;
// 	time(&now);
// 	return ctime(&now);
// }

// void
// fputs_rfc2822(FILE* f, time_t t)
// {
// 	char rfc_2822[40];
// 	strftime(rfc_2822, sizeof(rfc_2822), "%a, %d %b %Y 00:00:00 +0900", localtime(&t));
// 	fprintf(f, "%s", rfc_2822);
// }

// void
// fputs_rfc3339(FILE* f, time_t t)
// {
// 	struct tm* tm;
// 	if((tm = localtime(&t)) == NULL)
// 		return;
// 	fprintf(f, "%04d-%02d-%02dT%02d:%02d:%02d%c%02d:%02d",
// 	        tm->tm_year + 1900, tm->tm_mon + 1, tm->tm_mday,
// 	        tm->tm_hour, tm->tm_min, tm->tm_sec,
// 	        '-', 7, 0); /* Vancouver GMT-7*/
// }
