use std::ops::Deref;
use biblatex::Person;
use biblatex::{Chunk, ChunksExt, Entry};
use crate::jieba::jieba::{transliterate_full, transliterate_tokenized};
use crate::models::titles::Titles;

// pub fn read_entry_names(entry: &Entry) -> Option<Titles> {
//     let title = entry.get("title").clone();
//     let titleaddon = entry.get("titleaddon").clone();
//
//     if title.is_none() || titleaddon.is_none() {
//         return None;
//     }
//
//     print!("Yes, there are both title and title addon");
//     Some(Titles {
//         title: title.unwrap().format_verbatim(),
//         titleaddon: titleaddon.unwrap().format_verbatim(),
//     })
// }
//
// pub fn write_entry_titles(entry: &mut Entry, titles: Titles) -> &mut Entry {
//     let new_title = Chunk::Verbatim((titles.titleaddon.clone().to_string()));
//     let new_title_addon = Chunk::Verbatim((titles.title.clone().to_string()));
//     entry.set("title", vec![new_title]);
//     entry.set("titleaddon", vec![new_title_addon]);
//     entry
// }
//
// pub fn switch_first_name_last_name(entry: &mut Entry) -> &mut Entry {
//     let titles = read_entry_names(entry);
//
//     if titles.is_some() {
//         write_entry_titles(entry, titles.unwrap())
//     } else {
//         entry
//     }
//
// }

pub fn transliterate_all_names(entry: &mut Entry) -> &mut Entry {
    let authors: Option<Vec<Person>> = entry.author();
    let mut changed_authors = authors.unwrap().iter_mut().map(|author| {
        author.name = transliterate_full(author.name.as_str());
        author.given_name = transliterate_full(author.given_name.as_str());
        author.deref().clone()
    })
        .collect::<Vec<Person>>();

    entry.set_author(changed_authors);
    entry
}

#[cfg(test)]
mod test_transliterate_names {
    use biblatex::ChunksExt;
    use crate::fsutil::read_write::read_bibliography_from_file;
    use crate::name::transliterate_names::transliterate_all_names;

    #[test]
    fn test_transliterate_names() {
        let mut biblio = read_bibliography_from_file(r"test-latex/test.bib");
        let first_entry  = biblio.iter_mut().into_slice().first_mut().unwrap();
        let result = transliterate_all_names(first_entry);

        println!("finished");
        assert_eq!(result.author().unwrap().iter_mut().into_slice()[1].given_name, "test")

    }
}