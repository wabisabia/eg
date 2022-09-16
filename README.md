Eg: `Default` for dummy data.
================================

[<img alt="github" src="https://img.shields.io/badge/github-wabisabia/eg-495cbf?style=for-the-badge&labelColor=55555&logo=github">](https://github.com/wabisabia/eg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/eg?style=for-the-badge&logo=rust">](https://crates.io/crates/eg)
[<img alt="license" src="https://img.shields.io/crates/l/eg?style=for-the-badge&logo=apache">](https://crates.io/crates/eg)

## Why Eg?

Use `Eg` to provide example values:

```rust
use eg::Eg;

#[derive(Debug, PartialEq, Eg)]
struct Wizard {
	#[eg = "Harry Potter"]
	name: String,
	#[eg = "11"]
	age: usize,
	#[eg = "totally_random_spell"]
	fav_spell: Spell,
}

#[derive(Debug, PartialEq)]
enum Spell {
	Expelliarmus,
	WinguardiumLeviosa,
}

fn totally_random_spell() -> Spell {
	Spell::Expelliarmus
}

assert_eq!(
	Wizard {
		name: "Harry Potter".to_string(),
		age: 11,
		fav_spell: Spell::Expelliarmus
	},
	Wizard::eg()
);
```

When `Default` doesn't make sense, isn't available, or isn't expressive enough:

```rust
#[test]
fn my_database_test() {
	// database stuff ...
	db.insert(Wizard::eg());
	// more database stuff ...
}
```

## Why not a global `const`?

In `const`-compatible contexts, this is probably an acceptable approach!
For everything else, there's `Eg` 😉

## Contribution

This crate is still learning to walk; feedback, issues and PRs are all welcome! (◕‿◕)