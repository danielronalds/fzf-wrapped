# fzf-wrapper

A library for integrating the `fzf` cli tool into your rust program!

This library provides bindings for integrating the `fzf` cli program into your rust
projects.

**NOTE** this does mean that the end user must have `fzf` installed on their system.

#### fzf version

This crate was developed with `fzf` v0.40.0 in mind, however there should be no reason why it
shouldn't work above it, and most of the features should work below it. If your program relies
on v0.40.0 features, it might be a good idea to check the version of `fzf` your program has
access to

## Example

Say we're wanting to get the user to select their favourite colour using the power of fuzzy
finding. First we'd create a list of colours, which will be a collection we'll pass to `fzf`

```rust
let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
```

The next step is to construct an instance of [`Fzf`] and start it:

```rust
let mut fzf = Fzf::default();

fzf.run().expect("Failed to start fzf");
```

The code above fetches the default [`Fzf`] configuration, which runs `fzf` with no arguments,
and then calls the `run()` method. This displays `fzf` to the user.

At the moment all that will be displayed is a blank screen, as we haven't actually told `fzf`
to display anything. There are two ways to do this, the `add_item()` method, and the
`add_items()` method. They are both nearly identical, with the only difference being that
`add_items()` takes a [`Vec`] of [`String`]'s as items, and passes them one by one to
`add_item()`.

```rust
let mut fzf = Fzf::default();
fzf.run().expect("Failed to start fzf");
fzf.add_items(colours).expect("Failed to add items");
```

The only thing left to do is to get what the user selected! This will be returned as an
[`Option`] containing `None` if the user exited `fzf`, or `Some(String)` with the string being
the item they selected. To get the output we simply call the `output()` method, which will
blocks execution until the user selects an item with `fzf`

```rust
let mut fzf = Fzf::default();
fzf.run().expect("Failed to start fzf");
fzf.add_items(colours).expect("Failed to add items");
let users_selection = fzf.output().expect("Failed to get the user's output");
```

The code in it's entirety looks like the following. 

```rust
use fzf_wrapped::Fzf;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
    
    let mut fzf = Fzf::default();
    fzf.run().expect("Failed to start fzf");

    fzf.add_items(colours).expect("Failed to add items");

    let users_selection = fzf.output().expect("Failed to get the user's output");
}
```

This operation of using `fzf` to select from a predetermined [`Vec`] of items is so common that
a helper function exists to streamline the work involved. Using it, the code looks like the 
following:

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
    
    let users_selection = run_with_output(Fzf::default(), colours).expect("Something went wrong!");
}
```

## Running `fzf` with arguments

Now, our favourite color picker program is pretty cool, but our `fzf` interface is a bit
confusing, how are our user's supposed to know what they're picking?

Thankfully `fzf` provides many different ways to customize it, many of which have been
implemented in this library. So far we have been calling the [`Fzf`] structs default
implementation, however we can build more complex instances using [`FzfBuilder`]. 

We can use two different ways to get an [`FzfBuilder`], either through it's own `new()` method,
or the `builder()` method on [`Fzf`]. Let's switch out the default call for a builder call.

```rust
let fzf = Fzf::builder().build().unwrap();

let users_selection = run_with_output(fzf, colours).expect("Something went wrong!");
```

**NOTE**: It is safe to unwrap the `build()` call, as every field has a default implementation.

If we run the program now, we'll notice... nothing has changed. This is because the `default()`
method calls the exact line of code we just replaced it with.

Let's make things interesting! First we should probably give the finder a label. Without a 
border, the label won't show so lets give it a border too! 

### Adding a border and border label

If you were using `fzf` straight from the command line, you would pass it the `--border` flag
with the name of one of the supported types of borders, however this can lead to errors if the
border doesn't exist. For this reason, `fzf-wrapped` makes use of enums to ensure that these
choices are always valid! For example, to choose a border we'd call the `border()` method on
our [`FzfBuilder`], contain our chosen variant of the [`Border`] enum.

Adding a rounded border makes our code look like this:

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::Border;
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .border(Border::Rounded)
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);
}
```

And adding a label is even more simple

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::Border;
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .border(Border::Rounded)
        .border_label("Favourite Colour")
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);
}
```

Running this should display an `fzf` finder with a rounded border, and a centered label
containing "Favourite Colour".

### Changing the layout of `fzf`

Well, now that we've got a border, we may as well change up the layout. This is almost
identical to changing the border, as all possible layout's are mapped to an enum.

All we need to add to our builder is the `layout()` method with our chosen [`Layout`] variant

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::{Border, Layout};
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("Favourite Colour")
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);
}
```

### Choosing colours

`fzf` lets us pick from a few colour themes, but for this one we're going to keep it simple and
use the black and white theme. Similar to borders and the layout, this is selected using an
enum. Adding it to our builder results in the following code:

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::{Border, Color, Layout};
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("Favourite Colour")
        .color(Color::Bw)
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);
}
```

If you run the program now, you'll notice that the ui is in black and white!

### Adding a header

Our user might still be confused about what they're picking, so to add some more context, `fzf`
lets us set a header. To do this all we do is call the `header()` method on our [`FzfBuilder`] 
struct, and pass it anything with the `Into<String>` trait. We also want the header to appear
above our search field, so we'll call the `header_first()` method with `true`.

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::{Border, Color, Layout};
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("Favourite Colour")
        .color(Color::Bw)
        .header("Pick your favourite colour")
        .header_first(true)
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);
}
```

### Using an argument not supported by `fzf_wrapped`

If by chance the argument you want to run with `fzf` is not a method included on the [`Fzf`]
struct, do not worry! The `custom_args()` command will let you pass any argument you want! For
example, say we wanted to make our colour picker program not take up the full screen, say only
10% of it. `fzf` has the `--height=` flag, however the [`Fzf`] struct doesn't support it! To
add it all we'll need to do is to call the `custom_args()` command on our builder, and the
arguments we pass into it will be run with the `run()` method.

Implementing it would look like:

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::{Border, Color, Layout};
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("Favourite Colour")
        .color(Color::Bw)
        .header("Pick your favourite colour")
        .header_first(true)
        .custom_args(vec!["--height=10".to_string()])
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);

    if let Some(colour) = users_selection {
        println!("{} is an awesome colour!", colour);
    }
}
```

And with that, our program is almost done!

All we need to do is print some kind of nice message, and while we're at it, we may as well use
some proper error handling.

```rust
use fzf_wrapped::Fzf;
use fzf_wrapped::{Border, Color, Layout};
use fzf_wrapped::run_with_output;

fn main() {
    let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];

    let fzf = Fzf::builder()
        .layout(Layout::Reverse)
        .border(Border::Rounded)
        .border_label("Favourite Colour")
        .color(Color::Bw)
        .header("Pick your favourite colour")
        .header_first(true)
        .custom_args(vec!["--height=10".to_string()])
        .build()
        .unwrap();
    
    let users_selection = run_with_output(fzf, colours);

    if let Some(colour) = users_selection {
        println!("{} is an awesome colour!", colour);
    }
}
```

## Adding Items at runtime of `fzf`

With the power of this library, you can use `fzf` to select from a list of items, even if those
items have not been fetched yet. Using the `add_item` and `add_items` method adds items to 
`fzf`'s list, even while fzf is running. This means that if you're calling information from a 
REST api, you can display result's as they come in straight to `fzf`, or even hide the slight 
delay by starting up `fzf`.

For an example of this, look at my [workflows](https://github.com/danielronalds/workflows) project
