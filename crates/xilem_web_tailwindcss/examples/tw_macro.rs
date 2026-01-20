use xilem_web_tailwindcss::tw;

fn main() {
    let classes = tw!(
        "px-4 py-2 text-sm",
        if true => "bg-blue-600 text-white",
        if false => "bg-gray-200 text-gray-900",
    );

    println!("{classes:?}");
}
