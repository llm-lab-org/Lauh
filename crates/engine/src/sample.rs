//! Built-in sample document (original text, generated in code).

use wordcraft_doc::para::InlineObject;
use wordcraft_doc::props::{Align, CharProps};
use wordcraft_doc::{Block, Document, Paragraph, PartKind, para_block};

fn para(text: &str) -> Paragraph {
    Paragraph::with_text(text, CharProps::default())
}

fn styled(text: &str, style: &str) -> Paragraph {
    para(text).styled(style)
}

/// Append text with formatting to a paragraph.
fn add(p: &mut Paragraph, text: &str, props: CharProps) {
    let at = p.len();
    let _ = p.insert_text(at, text, &props);
}

/// The showcase document: Surah Al-Fatiha (سورة الفاتحة), the opening chapter of the Qur'an,
/// as a right-to-left bidi document that exercises the Quran character style and the Arabic
/// font bundle (Niloufaraneh).
pub fn sample_document() -> Document {
    let mut d = Document::new();
    d.core.title = "سورة الفاتحة".into();
    d.core.creator = "Lauh".into();
    let mut b: Vec<Block> = Vec::new();
    // Title + subtitle, both right-to-left.
    let mut title = styled("سورة الفاتحة", "HadithTitleL4");
    title.props.bidi = Some(true);
    title.props.align = Some(Align::Right);
    b.push(Block::Para(title));
    let mut subtitle = styled("The Opening — the first chapter of the Qur'an, seven verses", "Subtitle");
    subtitle.props.align = Some(Align::Right);
    b.push(Block::Para(subtitle));
    let verses = [
        "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
        "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ",
        "الرَّحْمَٰنِ الرَّحِيمِ",
        "مَالِكِ يَوْمِ الدِّينِ",
        "إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ",
        "اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ",
        "صِرَاطَ الَّذِينَ أَنْعَمْتَ عَلَيْهِمْ غَيْرِ الْمَغْضُوبِ عَلَيْهِمْ وَلَا الضَّالِّينَ",
    ];
    for text in verses {
        let mut p = Paragraph::new();
        p.props.bidi = Some(true);
        p.props.align = Some(Align::Right);
        p.props.space_after = Some(6.0);
        add(&mut p, text, CharProps { style: Some("QuranVerse".into()), ..Default::default() });
        b.push(Block::Para(p));
    }
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    // Footer: centred page number.
    let mut f = Paragraph::new().styled("Footer");
    f.props.align = Some(Align::Center);
    let _ = f.insert_object(
        0,
        InlineObject::Field { instr: "PAGE".into(), result: "1".into(), locked: false, dirty: false, code: false, form: String::new() },
        &CharProps::default(),
    );
    let fid = d.add_part(PartKind::Footer, vec![para_block(f)]);
    d.last_section.footers.default = Some(fid);
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_is_valid() {
        let d = sample_document();
        assert!(d.word_count() > 5);
        let l = wordcraft_layout::layout(&d, &mut wordcraft_layout::LayoutCache::new(), &Default::default());
        assert!(!l.pages.is_empty());
    }
}
