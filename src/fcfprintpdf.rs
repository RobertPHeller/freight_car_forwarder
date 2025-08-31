extern crate genpdf;
use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::{elements, fonts, style};

//const ROMAN_FONT_NAME: &'static str = "Courier";
//const BOLD_FONT_NAME: &'static str = "Courier-Bold";
//const ITALIC_FONT_NAME: &'static str = "Courier-Oblique";
//const BOLDITALIC_FONT_NAME: &'static str = "Courier-BoldOblique";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PageSize {
    Letter,
    A4,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeSpacing {
    One,
    Half,
    Double,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeWeight {
    Normal,
    Bold,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeSlant {
    Roman,
    Italic,
}


pub struct Printer {
    document: genpdf::Document,
    pageSize: PageSize,
    filename: String,
}

impl Printer {
    pub fn new(filename: String, title: String, pageSize: PageSize) -> Self {
        let font = fonts::from_files("","",Some(fonts::Builtin::Courier))
                .expect("Failed to load the monospace font family");
        let mut this = Self {document: genpdf::Document::new(font),
                             pageSize: pageSize,
                             filename: filename};
        this.document.set_title(title.as_str());
        this.document.set_minimal_conformance();
        this.document.set_line_spacing(1.25);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        decorator.set_header(move |page| {
            let mut layout = elements::LinearLayout::vertical();
            layout.push(
                elements::Paragraph::new(format!("{:65} Page {:5}",title,page))
                                .aligned(Alignment::Right), 
            );
            layout.push(elements::Break::new(1));
            layout.styled(style::Style::new().with_font_size(10))
        });
        this.document.set_page_decorator(decorator);
        this
    }
    
}

impl Drop for Printer {
    fn drop(&mut self) {
        let doc : genpdf::Document = self.document;
        doc.render_to_file(self.filename.clone())
            .expect("Failed to write output file");
    }
}
