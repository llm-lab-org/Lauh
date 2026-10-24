//! Built-in sample document and templates (original text, generated in code).
//!
//! Each `pub fn` returns a ready-to-edit [`Document`] for the Start screen's template gallery.
//! Content is placeholder-only: dummy addresses, generic headings, no real people or trademarks.

use wordcraft_doc::para::InlineObject;
use wordcraft_doc::props::{Align, CharProps, NumRef, Rgb, TabAlign, TabLeader, TabStop, TextColor};
use wordcraft_doc::{Block, Document, ListKind, Paragraph, PartKind, para_block};

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

fn bold() -> CharProps {
    CharProps { bold: Some(true), ..Default::default() }
}

fn italic() -> CharProps {
    CharProps { italic: Some(true), ..Default::default() }
}

/// A right-to-left paragraph: Arabic reads naturally from the right in each tile.
fn rtl(mut p: Paragraph) -> Paragraph {
    p.props.bidi = Some(true);
    p.props.align = Some(Align::Left); // logical start = visual right in bidi
    p
}

/// Add a centred page-number footer to `d` (the standard page-N chrome).
fn add_page_footer(d: &mut Document) {
    let mut f = Paragraph::new().styled("Footer");
    f.props.align = Some(Align::Center);
    let _ = f.insert_object(
        0,
        InlineObject::Field { instr: "PAGE".into(), result: "1".into(), locked: false, dirty: false, code: false, form: String::new() },
        &CharProps::default(),
    );
    let fid = d.add_part(PartKind::Footer, vec![para_block(f)]);
    d.last_section.footers.default = Some(fid);
}

// ---------------------------------------------------------------- Qur'an / Hadith (RTL)

/// The showcase document: Surah Al-Fatiha (سورة الفاتحة), the opening chapter of the Qur'an,
/// as a right-to-left bidi document that exercises the Quran character style and the Arabic
/// font bundle (Niloufaraneh).
pub fn sample_document() -> Document {
    let mut d = Document::new();
    d.core.title = "سورة الفاتحة".into();
    d.core.creator = "Lauh".into();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(rtl(styled("سورة الفاتحة", "HadithTitleL4"))));
    b.push(Block::Para(rtl(styled("الفاتحة — فاتحة الكتاب، سبع آيات", "Subtitle"))));
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
        let mut p = rtl(Paragraph::new());
        p.props.space_after = Some(6.0);
        add(&mut p, text, CharProps { style: Some("QuranVerse".into()), ..Default::default() });
        b.push(Block::Para(p));
    }
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    add_page_footer(&mut d);
    d
}

/// A hadith-study page: title, isnad (chain), matn (text), explanation.
pub fn hadith_paper() -> Document {
    let mut d = Document::new();
    d.core.title = "ورقة حديثية".into();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(rtl(styled("حديث: إنما الأعمال بالنيات", "HadithTitleL4"))));
    b.push(Block::Para(rtl(styled("الإسناد والمتن والشرح", "Subtitle"))));
    b.push(Block::Para(rtl(styled("الإسناد", "Heading2"))));
    let mut isnad = rtl(Paragraph::new());
    add(
        &mut isnad,
        "حدثنا الحميدي عبد الله بن الزبير قال حدثنا سفيان قال حدثنا يحيى بن سعيد الأنصاري قال أخبرني محمد بن إبراهيم التيمي أنه سمع علقمة بن وقاص الليثي يقول: سمعت عمر بن الخطاب رضي الله عنه على المنبر قال: سمعت رسول الله ﷺ يقول:",
        CharProps { style: Some("RefStyle".into()), ..Default::default() },
    );
    b.push(Block::Para(isnad));
    b.push(Block::Para(rtl(styled("المتن", "Heading2"))));
    let mut matn = rtl(Paragraph::new());
    add(
        &mut matn,
        "«إنما الأعمال بالنيات، وإنما لكل امرئ ما نوى، فمن كانت هجرته إلى الله ورسوله فهجرته إلى الله ورسوله، ومن كانت هجرته لدنيا يصيبها أو امرأة ينكحها فهجرته إلى ما هاجر إليه.»",
        CharProps { style: Some("ArabicText".into()), ..Default::default() },
    );
    b.push(Block::Para(matn));
    b.push(Block::Para(rtl(styled("التخريج", "Heading2"))));
    b.push(Block::Para(rtl(para("رواه البخاري ومسلم عن عمر بن الخطاب رضي الله عنه."))));
    b.push(Block::Para(rtl(styled("الشرح", "Heading2"))));
    b.push(Block::Para(rtl(para("اكتب هنا شرح الحديث، معاني ألفاظه، والفوائد المستنبطة منه."))));
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    add_page_footer(&mut d);
    d
}

/// A tafsir page: a Qur'anic verse followed by a short commentary paragraph.
pub fn quran_commentary() -> Document {
    let mut d = Document::new();
    d.core.title = "تفسير آية".into();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(rtl(styled("تفسير سورة الإخلاص", "HadithTitleL4"))));
    b.push(Block::Para(rtl(styled("الآية والمعنى والفوائد", "Subtitle"))));
    b.push(Block::Para(rtl(styled("الآية", "Heading2"))));
    let mut v = rtl(Paragraph::new());
    add(
        &mut v,
        "﴿قُلْ هُوَ اللَّهُ أَحَدٌ * اللَّهُ الصَّمَدُ * لَمْ يَلِدْ وَلَمْ يُولَدْ * وَلَمْ يَكُن لَّهُ كُفُوًا أَحَدٌ﴾",
        CharProps { style: Some("QuranVerse".into()), ..Default::default() },
    );
    b.push(Block::Para(v));
    b.push(Block::Para(rtl(styled("المعنى الإجمالي", "Heading2"))));
    b.push(Block::Para(rtl(para("اكتب هنا المعنى الإجمالي للآيات."))));
    b.push(Block::Para(rtl(styled("الفوائد", "Heading2"))));
    for s in ["إثبات الوحدانية لله تعالى.", "نفي الصاحبة والولد.", "نفي الكفؤ والشبيه."] {
        let mut p = rtl(para(s).styled("ListParagraph"));
        p.props.numbering = None;
        b.push(Block::Para(p));
    }
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    add_page_footer(&mut d);
    d
}

/// A sermon (khutbah): title, opening, three sections, closing.
pub fn khutbah() -> Document {
    let mut d = Document::new();
    d.core.title = "خطبة الجمعة".into();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(rtl(styled("خطبة الجمعة", "HadithTitleL4"))));
    b.push(Block::Para(rtl(styled("عنوان الخطبة", "Subtitle"))));
    b.push(Block::Para(rtl(styled("الحمد والثناء", "Heading1"))));
    b.push(Block::Para(rtl(para(
        "إن الحمد لله نحمده ونستعينه ونستغفره، ونعوذ بالله من شرور أنفسنا وسيئات أعمالنا. اكتب هنا مقدمة الخطبة.",
    ))));
    b.push(Block::Para(rtl(styled("المقدمة", "Heading1"))));
    b.push(Block::Para(rtl(para("اكتب هنا المقدمة والموضوع العام للخطبة."))));
    b.push(Block::Para(rtl(styled("الموضوع", "Heading1"))));
    b.push(Block::Para(rtl(para("اكتب هنا تفاصيل الموضوع والأدلة والوصايا."))));
    b.push(Block::Para(rtl(styled("الخاتمة والدعاء", "Heading1"))));
    b.push(Block::Para(rtl(para("اكتب هنا الخاتمة والدعاء."))));
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    add_page_footer(&mut d);
    d
}

/// An Arabic academic paper skeleton: title, abstract, sections, references.
pub fn arabic_paper() -> Document {
    let mut d = Document::new();
    d.core.title = "ورقة أكاديمية".into();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(rtl(styled("عنوان الورقة", "Title"))));
    b.push(Block::Para(rtl(styled("اسم المؤلف — الجهة التابعة", "Subtitle"))));
    b.push(Block::Para(rtl(styled("الملخص", "Heading1"))));
    b.push(Block::Para(rtl(para("اكتب هنا ملخصاً مختصراً للبحث: الأهداف، المنهجية، والنتائج الرئيسية."))));
    b.push(Block::Para(rtl(styled("المقدمة", "Heading1"))));
    b.push(Block::Para(rtl(para("اكتب هنا مقدمة الورقة وأهميتها."))));
    b.push(Block::Para(rtl(styled("المنهجية", "Heading1"))));
    b.push(Block::Para(rtl(para("صف هنا المنهج والمنهجية المتبعة."))));
    b.push(Block::Para(rtl(styled("النتائج والمناقشة", "Heading1"))));
    b.push(Block::Para(rtl(para("اعرض هنا النتائج ومناقشتها."))));
    b.push(Block::Para(rtl(styled("الخاتمة", "Heading1"))));
    b.push(Block::Para(rtl(para("اكتب هنا خاتمة البحث والتوصيات."))));
    b.push(Block::Para(rtl(styled("المراجع", "Heading1"))));
    b.push(Block::Para(rtl(para("أدرج هنا قائمة المراجع."))));
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    add_page_footer(&mut d);
    d
}

/// A right-to-left Arabic business letter.
pub fn arabic_letter() -> Document {
    let mut d = Document::new();
    d.core.title = "رسالة".into();
    let date_text = crate::cmd::insert::format_date("yyyy/MM/dd");
    let lines: Vec<Paragraph> = vec![
        rtl(styled("اسمك الكريم", "Title")),
        rtl(para("العنوان · المدينة، الرمز البريدي · البريد الإلكتروني")),
        rtl(para("")),
        rtl(para(&date_text)),
        rtl(para("")),
        rtl(para("إلى: اسم المستلم")),
        rtl(para("المسمى الوظيفي، الجهة")),
        rtl(para("")),
        rtl(para("السلام عليكم ورحمة الله وبركاته،")),
        rtl(para(
            "اكتب هنا موضوع الرسالة باختصار في جملة واحدة. ثم فصّل الموضوع في فقرات قصيرة واختم بما تودّ أن يحدث بعد ذلك.",
        )),
        rtl(para("وتفضلوا بقبول فائق الاحترام.")),
        rtl(para("")),
        rtl(para("اسمك")),
    ];
    d.body = lines.into_iter().map(para_block).collect();
    d
}

// ---------------------------------------------------------------- English / LTR classics

/// A block-style letter.
pub fn letter() -> Document {
    let mut d = Document::new();
    let lines: Vec<Paragraph> = vec![
        styled("Your Name", "Title"),
        para("123 Your Street · City, ST 00000 · you@example.com"),
        para(""),
        para(&crate::cmd::insert::format_date("MMMM d, yyyy")),
        para(""),
        para("Recipient Name"),
        para("Title, Company"),
        para("Street Address"),
        para(""),
        para("Dear Recipient,"),
        para(
            "Start with a sentence that says why you are writing. Keep paragraphs short and specific, and close with what you would like to happen next.",
        ),
        para("Thank you for your time."),
        para("Sincerely,"),
        para(""),
        para("Your Name"),
    ];
    d.body = lines.into_iter().map(para_block).collect();
    d
}

/// A one-page résumé.
pub fn resume() -> Document {
    let mut d = Document::new();
    let mut b: Vec<Block> =
        vec![Block::Para(styled("Alex Rivera", "Title")), Block::Para(para("Illustrator · alex@example.com · portfolio.example.com"))];
    for (head, items) in [
        ("Experience", vec![("Lead Illustrator, Harbor Books", "2021 – present"), ("Freelance Illustrator", "2016 – 2021")]),
        ("Education", vec![("BFA Illustration, Coastal College of Art", "2016")]),
        ("Skills", vec![("Ink, gouache, digital painting, book layout, lettering", "")]),
    ] {
        b.push(Block::Para(styled(head, "Heading1")));
        for (what, when) in items {
            let mut p = para(what);
            p.props.tabs = Some(vec![TabStop { pos: 468.0, align: TabAlign::Right, leader: TabLeader::None }]);
            if !when.is_empty() {
                add(&mut p, &format!("\t{when}"), italic());
            }
            b.push(Block::Para(p));
        }
    }
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    d
}

/// A report with a cover heading and sections.
pub fn report() -> Document {
    let mut d = Document::new();
    let b: Vec<Block> = vec![
        Block::Para(styled("Report Title", "Title")),
        Block::Para(styled("Subtitle or date", "Subtitle")),
        Block::Para(styled("Summary", "Heading1")),
        Block::Para(para("Summarise the findings in two or three sentences.")),
        Block::Para(styled("Background", "Heading1")),
        Block::Para(para("Describe the context and the question the report answers.")),
        Block::Para(styled("Findings", "Heading1")),
        Block::Para(styled("First finding", "Heading2")),
        Block::Para(para("Explain the evidence.")),
        Block::Para(styled("Recommendations", "Heading1")),
        Block::Para(para("List what should happen next.")),
    ];
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    add_page_footer(&mut d);
    d
}

/// A memo: masthead with To / From / Date / Subject, then body sections.
pub fn memo() -> Document {
    let mut d = Document::new();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(styled("MEMORANDUM", "Title")));
    for (k, v) in [("To:", "Recipient"), ("From:", "Your Name"), ("Date:", "")] {
        let mut p = para("");
        add(&mut p, k, bold());
        add(&mut p, &format!("\t{v}"), CharProps::default());
        p.props.tabs = Some(vec![TabStop { pos: 72.0, align: TabAlign::Left, leader: TabLeader::None }]);
        b.push(Block::Para(p));
    }
    let mut subject = para("");
    add(&mut subject, "Subject:", bold());
    add(&mut subject, "\tOne-line description", CharProps::default());
    subject.props.tabs = Some(vec![TabStop { pos: 72.0, align: TabAlign::Left, leader: TabLeader::None }]);
    b.push(Block::Para(subject));
    b.push(Block::Para(styled("Overview", "Heading2")));
    b.push(Block::Para(para("One or two sentences summarising why you are writing this memo.")));
    b.push(Block::Para(styled("Details", "Heading2")));
    b.push(Block::Para(para("Explain the context, data, or history.")));
    b.push(Block::Para(styled("Action required", "Heading2")));
    b.push(Block::Para(para("State clearly what the recipient should do and by when.")));
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    d
}

/// A newsletter: masthead and short lead articles.
pub fn newsletter() -> Document {
    let mut d = Document::new();
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(styled("The Monthly Bulletin", "Title")));
    b.push(Block::Para(styled("Issue 1 · Your organisation", "Subtitle")));
    b.push(Block::Para(styled("From the editor", "Heading1")));
    b.push(Block::Para(para(
        "Welcome to the first issue. Keep each article short — one or two paragraphs — and let the headings do the signposting.",
    )));
    b.push(Block::Para(styled("Feature story", "Heading1")));
    b.push(Block::Para(para("Lead with the news. One sentence that captures the point, then two or three that back it up.")));
    b.push(Block::Para(styled("Upcoming events", "Heading1")));
    let mut num = d.numbering.clone();
    let list = num.add_list(ListKind::Bullet);
    d.numbering = num;
    for s in ["Event name — date, location.", "Event name — date, location.", "Event name — date, location."] {
        let mut p = para(s).styled("ListParagraph");
        p.props.numbering = Some(NumRef { num: list, level: 0 });
        b.push(Block::Para(p));
    }
    b.push(Block::Para(styled("Contact", "Heading1")));
    b.push(Block::Para(para("editor@example.com · phone · website")));
    d.body = b.into_iter().map(std::sync::Arc::new).collect();
    d
}

/// A single-page flyer: big headline, hero line, call-to-action.
pub fn flyer() -> Document {
    let mut d = Document::new();
    let mut hero = styled("YOUR BIG HEADLINE", "Title");
    hero.props.align = Some(Align::Center);
    let mut sub = styled("A short, memorable subhead on one line", "Subtitle");
    sub.props.align = Some(Align::Center);
    let mut body = para("Explain in a sentence or two what this is, when it happens, where to find it, and who it is for.");
    body.props.align = Some(Align::Center);
    let mut cta = para("");
    add(
        &mut cta,
        "Learn more",
        CharProps { bold: Some(true), color: Some(TextColor::Rgb(Rgb(0x15, 0x60, 0x82))), ..Default::default() },
    );
    add(&mut cta, " at example.com — contact@example.com", CharProps::default());
    cta.props.align = Some(Align::Center);
    d.body = vec![Block::Para(hero), Block::Para(sub), Block::Para(body), Block::Para(cta)].into_iter().map(std::sync::Arc::new).collect();
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_are_valid() {
        for d in
            [sample_document(), hadith_paper(), quran_commentary(), khutbah(), arabic_paper(), arabic_letter(), letter(), resume(), report(), memo(), newsletter(), flyer()]
        {
            assert!(d.word_count() > 3);
            let l = wordcraft_layout::layout(&d, &mut wordcraft_layout::LayoutCache::new(), &Default::default());
            assert!(!l.pages.is_empty());
        }
    }
}
