- better error handling.
- less C-ish style.
- use `Weak<T>` instead of `Rc<T>` on `parent` field, to avoid reference cycles.
