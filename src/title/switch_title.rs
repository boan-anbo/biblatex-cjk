
use biblatex::{Chunk, ChunksExt, Entry};
use crate::models::titles::Titles;

pub fn read_entry_titles(entry: &Entry) -> Option<Titles> {
    let title = entry.get("title").clone();
    let titleaddon = entry.get("titleaddon").clone();

    if title.is_none() || titleaddon.is_none() {
        return None;
    }

    print!("Yes, there are both title and title addon");
    Some(Titles {
        title: title.unwrap().format_verbatim(),
        titleaddon: titleaddon.unwrap().format_verbatim(),
    })
}

pub fn write_entry_titles(entry: &mut Entry, titles: Titles) -> &mut Entry {
    let new_title = Chunk::Verbatim((titles.titleaddon.clone().to_string()));
    let new_title_addon = Chunk::Verbatim((titles.title.clone().to_string()));
    entry.set("title", vec![new_title]);
    entry.set("titleaddon", vec![new_title_addon]);
    entry
}

pub fn switch_title_titleaddon(entry: &mut Entry) -> &mut Entry {
    let titles = read_entry_titles(entry);

    if titles.is_some() {
        write_entry_titles(entry, titles.unwrap())
    } else {
        entry
    }

}