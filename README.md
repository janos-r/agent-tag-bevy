# Bevy experiment

This is a refactor of my previous challenge [agent-tag](https://github.com/janos-r/agent-tag) into the ECS system of [Bevy](https://bevyengine.org/).

## Backstory

6 months ago, I accepted a challenge to make a independent agent system in Rust. This sounded very interesting to me and you can read more about my weekend project on it's own git [agent-tag](https://github.com/janos-r/agent-tag). I got it working, which was nice, but there was still this lingering thought that left me unsatisfied.

## Problems

I had issues with very basic principles while using pointers. I have a `Vec<Agents>`, but every single `Agent` wats to have a link to the parent `Vec` also. I made it work with weak links. Luckily I found this excelent page in the [Rust Book](https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent) that perfectly describes this issue. You get a stack overflow, a reference cycle, if a child references it's own parent with just a Rc. Thanks to the Weak link, that is blind until it's upgraded, you get the structure working. But if you than want to use it for mutation, you still can't use a already used reference, so you have to hack around it like I did with cloning the whole state and using only its links that way. This can be seen in the example from my old code bellow with my original comments that I left there for future me.

This later version was made with `Arc<Mutex<World>>` for the parent and you can just clone it to the Agents. But the proper single-thread code was with `Rc<RefCell<World>>` for the parent and through `Rc::downgrade(world)` you store it in the agents as `Weak<RefCell<World>>`. That's why in this example, you access the links with just `.lock()`. In the single-thread case you have to use `.borrow()` and `.borrow_mut()` when working from the parent (World). And prepend it with `.upgrade()` when working from inside the child (Agent). It's ironic that the single-thread syntax is a little more complicated, but it is good to know both ways.

```rust
// regret: this still feels like a hack to me
// tag agents
let agents = world.lock().unwrap().agents.clone();
agents
    .iter()
    .enumerate()
    .for_each(|(index, agent)| agent.tag(index));

// ...

pub fn tag(&self, my_index: usize) {
    // this is a cloned self, so changes on it won't influence the real world!
    // only it's links or current state are useful
    if self.status == Status::Tagged {
        if let Some(target) = self.find_neighbor() {
            if self.announce_tag {
                println!("!!!! FOUND NEIGHBOR !!!!");
            }
            self.world_link.lock().unwrap().tag_agent(my_index, target);
        }
    };
}
```

So in the end the code works. I even attempted to make this multi-threaded version with Rayon, but without managing to get a performance improvement. Multi-thread is still difficult to do right.

## A ray of light

Half a year passed and I was watching just another Rust YouTube video, not even a new one. But suddenly the lady started talking about exactly my issue above. Suddenly I felt like not a crazy person, but that this is possibly a common issue for many people. Here is the exact timestamp where she seams to talk about exactly this -> [video](https://youtu.be/aKLntZcp27M?t=1205).

She continues to talk about this being an inherent problem of OOP ways of thinking about code. How generational indexes are addressing this issue and how they are used especially in ECS systems and game development.

## Bevy

This prompted me to look up Bevy. It is only a year old, but already looks incredibly promising. I also love some of its core ideas. Obviously it's free software, but also that they don't use any fancy macros, so that using it doesn't feel like black magic. And also that it seams very easy and modular to use "plugins", making it probably easy for other developers to extend the core library.

This is my first time using Bevy, so all my negative comments are probably just lack of knowledge, but the purpose of of this article is to convey my experience and feelings with it.

- One of the drawbacks of using an ECS system is the lack of indexing your own `Vec` of entities. In my old code, I could write:

```rust
...
.agents
.iter()
.position(|agent| {...})
```

`.position()` is Rusts' own tool to give you a `Options<T>` with the index where it maybe found something. In bevy, if I didn't miss something, if I want to check the first matching entity and continue to use it for other tings, I had to do this with:

```rust
let mut origin: Option<Entity> = None;
for (status, entity) in query.iter_mut() {
    if *status == Status::Tagged {
        origin = Some(entity);
        break;
    };
}
```

It's not too bothersome, but it doesn't feel elegant.

Also connected to this is a small disappointment. [Recently](https://bevyengine.org/news/bevy-0-5/#uber-fast-for-each-query-iterators) they claimed that the new .for_each() should be preferred to .iter(). Unfortunately if you are searching for the first occurrence of something, you want to break soon to be more efficient. But you also can't use the more efficient .for_each() with break. Not horrible, but a little unfortunate.

- I know Bevy and game engines in general are made for multi-threaded work. Unfortunately my exercise had very strict turns and so I had to write something like:

```rust
.add_system(update_grid.system().label(UPDATE_GRID))
.add_system(print_grid.system().label(PRINT_GRID).after(UPDATE_GRID))
.add_system(move_agents.system().label(MOVE_AGENTS).after(PRINT_GRID))
.add_system(tag.system().label(TAG).after(MOVE_AGENTS))
.add_system(sleep2s.system().label(SLEEP).after(TAG))
.add_system(exit.system().label(EXIT).after(SLEEP))
```

Any time you want to switch around some step, think about all the editing. Again, it's not a dealbreaker, but something doesn't feel right. I tried using a single-threaded custom stage, but that also doesn't guarantee execution order. You have to be specific with the order by using labels, also opening you up to contradictions if messed up.

- The query syntax work really well. Although you have to get used to the weird feeling, that all the arguments used in the `.system()` functions are written without `&`. So you take a:

```rust
fn tag(
    mut agents: Query<(&mut Status, &Position, Entity), With<Agent>>,
    mut tag_count: ResMut<TagCount>,
    grid_size: Res<InputSize>,
    announce_tag: Res<InputAnnounceTag>,
) {...
```

But you are not really taking ownership of anything. Because you are not explicitly returning any of them at the end of the function. In a normal function this would be resource acquisition and drop at the end of the function scope. Also changing the tag_count works just like you would expect from a borrowed mut. I don't mind it, but it is something I did squint at for a little bit, doubting if I remember the ownership rules correctly. I am still not exactly sure how or why this works, but I hope I'm not writing nonsense here, lol.

## Conclusion

I didn't have any borrow-checker or syntax fighting issues with bevy. I found it very intuitive and it is amazing that you can relatively easily read its source code. It is not using strictly just a component entity-map with generational-indexes, but something called Archetypes. I got a little lost in the internal logic there, but suffice it to say that it just works.

You simply write systems with queries that have everything you need and if you don't have to be tick/frame-exact, you can just throw it into the app builder and you get a perfectly working multithreaded system that takes care of everything.

## Benchmark

Unfortunately my use-case was just a very simple single-threaded one. So when I compared the performance to my old suboptimal code without any shell printing.

10k ticks, default 40 entities on a 25x25 grid:

```sh
time cargo run --release -- -t0 -m10000 -d
```

| Bevy    | Old code |
| ---     | ---      |
| ~700 ms | < 100 ms |

The old suboptimal code is still almost 10x quicker. Even with the Arc instead of Rc, while cloning the whole state every time. So that was a little underwhelming. On the other hand, Bevy is a full-fledged ECS system with many components that make it possible to raise a real large scale project. My tiny example with just 40 entities on a small grid... maybe it should not be surprising it is 10x faster on a small code base and a small sample.

---

I tried again with a slightly bigger numbers. Still 10k ticks, but 500 entities and 50x50 grid:

```sh
time cargo run --release -- -s50 -a500 -t0 -m10000 -d
```

| Bevy    | Old code |
| ---     | ---      |
| ~880 ms | ~240 ms |

So while the suboptimal code more than doubled, Bevy increased only about +25%.

---

10x again... size 200 and 5k entities.

```sh
time cargo run --release -- -s200 -a5000 -t0 -m10000 -d
```

| Bevy   | Old code |
| ---    | ---     |
| 2.15 s | 1.62 s |

Repeating this command gives in both crates surprisingly consistent results. Even when changing the number of ticks, the suboptimal code is around 75% that of Bevy. So as the world grows, the times get closer. That is not surprising.

---

With 20k entities, Bevy is finally quicker.

```sh
time cargo run --release -- -s200 -a20000 -t0 -m10000 -d
```

| Bevy   | Old code|
| ---    | ---    |
| 5.85 s | 6.12 s |

But this is all just for fun! This is obviously not indicative of any real-game use-case. Firstly this is all single-threaded. And secondly my old code is just a pice of an experiment and not suited for any other use. Also there are probably many optimizations I could have used in Bevy that I missed. As I said, first try.

## PS

Bevy is amazing and inspiring! Especially considering it is only one year old. I might try to learn some of the graphics plugins it provides and try to learn some more in the future.
