# Eg

`Default`, but for example data.

## Why Eg?

More instructive documentation:

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

For const-compatible use cases, this is probably an acceptable approach!
For everything else, there's `Eg` (◕‿◕)
