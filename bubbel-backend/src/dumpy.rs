use std::fs;

pub fn put_default_stuff() {
    let pfp = include_bytes!("./defaults/pfp.jpeg");
    let banner = include_bytes!("./defaults/banner.jpeg");

    let _ =
        fs::create_dir_all("/bubbel/dumpster/profile_picture/7avTMWf4xDcGekB9N59uuXQU5lk6Oepu/");
    let _ = fs::create_dir_all("/bubbel/dumpster/banner_picture/4pFqco0xtk6NaRGNLF0AU2ZP3CvP2483/");

    let _ = fs::write(
        "/bubbel/dumpster/profile_picture/7avTMWf4xDcGekB9N59uuXQU5lk6Oepu/pfp150x150.jpeg",
        pfp,
    );
    let _ = fs::write(
        "/bubbel/dumpster/banner_picture/4pFqco0xtk6NaRGNLF0AU2ZP3CvP2483/banner1200x200.jpeg",
        banner,
    );
}
