use fzf_wrapped::run_with_output;
use fzf_wrapped::Fzf;
use fzf_wrapped::{Border, Color, Layout};

fn main() {
    let colours = vec![
        "red", "orange", "yellow", "green", "blue", "indigo", "violet",
    ];

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
