# How to use the skill manager?

It may be like this:
* received msg: The character is going to cast a skill called "hit"

```rust
skill_tocast = String::from("hit");
char.skills.cast(skill_tocast);

fn cast(s: String ) {
    //balabala
}
```

No. When the player passes the instruction, he must passes all the params once.
So the code can be **explicit**!
Like:

```rust
trait OnlyCaster {
    fn cast (char: &mut Character){}
}
struct SelfHeal;
impl OnlyCaster for SelfHeal {
    fn cast (char: &mut Character){
        Character.hp.regen_percent(20);
    }
}
```