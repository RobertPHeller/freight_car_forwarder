// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-02 15:13:52
//  Last Modified : <250911.0913>
//
//  Description	
//
//  Notes
//
//  History
//	
/////////////////////////////////////////////////////////////////////////////
//    Copyright (C) 2025  Robert Heller D/B/A Deepwoods Software
//			51 Locke Hill Road
//			Wendell, MA 01379-9728
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program; if not, write to the Free Software
//    Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// 
//
//////////////////////////////////////////////////////////////////////////////
extern crate cairo;
extern crate pango;
extern crate pango_sys;

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
    pageSize: PageSize,
    pdf_surface: cairo::PdfSurface,
    pdf_context: cairo::Context,
    layout: pango::Layout,
    Courier: pango::FontDescription,
    CourierBold: pango::FontDescription,
    CourierOblique: pango::FontDescription,
    CourierBoldOblique: pango::FontDescription,
    current_slant: TypeSlant,
    current_weight: TypeWeight,
    current_spacing: TypeSpacing,
    swidth: f64,
    sheight: f64,
    title: String,
    lines: u32,
    maxLines: u32,
    partline: bool,
    needPage: bool,
    currentColumn: u32,
    currentColumnFraction: f64,
}

        

impl Printer {
    pub fn new(filename: &str, title: &str, pageSize: PageSize) -> Self {
        let swidth: f64;
        let sheight: f64;
        match pageSize {
            PageSize::Letter => {swidth = 8.5*72.0;
                                 sheight = 11.0*72.0;},
            PageSize::A4 => {swidth = 595.0; sheight = 842.0;},
        };
        let pdf_surface = cairo::PdfSurface::new(swidth,sheight,filename)
                        .expect("Failed to open PDF output file");
        let context = cairo::Context::new(&pdf_surface).
                            expect("Failed to create a context");
        let layout = pangocairo::functions::create_layout(&context);
        let mut this = Self {
                             pageSize: pageSize,
                             pdf_surface: pdf_surface,
                             pdf_context: context,
                             layout: layout,
                             Courier: pango::FontDescription::from_string("Courier 10px"),
                             CourierBold: pango::FontDescription::from_string("Courier Bold 10px"),
                             CourierOblique: pango::FontDescription::from_string("Courier Oblique 10px"),
                             CourierBoldOblique: pango::FontDescription::from_string("Courier BoldOblique 10px"),
                             swidth: swidth, sheight: sheight, 
                             maxLines: 0,
                             partline: false,
                             needPage: false,
                             currentColumn: 0,
                             currentColumnFraction: 0.0,
                             current_slant: TypeSlant::Roman,
                             current_weight: TypeWeight::Normal,
                             current_spacing: TypeSpacing::One,
                             title: String::from(title),
                             lines: 0,
                             };
        this.maxLines = ((this.sheight-72.0) / 12.0) as u32;
        this.SetTypeSpacing(TypeSpacing::Double);
        this.PutLine(&this.title.clone());
        this.SetTypeSpacing(TypeSpacing::One);
        this
    }
    //fn ClosePrinter(&mut self) -> bool {
    //    true
    //}
    pub fn IsOpenP(&self) -> bool {true}
    pub fn PrinterPageSize(&self) -> PageSize {self.pageSize}
    pub fn SetTypeSpacing(&mut self,spacing: TypeSpacing) -> bool {
        self.current_spacing = spacing;
        true
    }
    pub fn SetTypeSlant(&mut self,slant: TypeSlant) -> bool {
        self.current_slant = slant;
        true
    }
    pub fn SetTypeWeight(&mut self,weight: TypeWeight) -> bool {
        self.current_weight = weight;
        true
    }
    pub fn NewPage(&mut self,heading: &str) -> bool {
        self.needPage = true;
        self.lines = 0;
        if heading.len() > 0 {
            let savedTS = self.current_spacing;
            self.SetTypeSpacing(TypeSpacing::Double);
            self.PutLine(heading);
            self.SetTypeSpacing(savedTS);
            true
        } else {
            true
        }
    }
    pub fn PutLine(&mut self,line: &str) -> bool {
        if line.len() > 0 {self.Put(line);}
        self.partline = false;
        self.lines += 1;
        if self.lines >= self.maxLines {self.NewPage(&self.title.clone());}
        self.currentColumn = 0;
        self.currentColumnFraction = 0.0;
        true
    }
    fn putstring(&mut self,text: &str) -> f64 {
        let desc: &pango::FontDescription = 
            match self.current_slant {
                TypeSlant::Roman => 
                    match self.current_weight {
                        TypeWeight::Normal => &self.Courier,
                        TypeWeight::Bold => &self.CourierBold,
                    },
                 TypeSlant::Italic => 
                    match self.current_weight {
                        TypeWeight::Normal => &self.CourierOblique,
                        TypeWeight::Bold => &self.CourierBoldOblique,
                    },
            };
        self.layout.set_font_description(Some(desc));
        self.layout.set_text(" ");
        let wh = self.layout.size();
        let w = wh.0;
        let h = wh.1;
        self.layout.set_font_description(Some(desc));
        self.layout.set_text(text);
        self.pdf_context.save().expect("Failed to save context");
        match self.current_spacing {
            TypeSpacing::One => self.pdf_context.scale(1.0,1.0),
            TypeSpacing::Half => self.pdf_context.scale(0.6,1.0),
            TypeSpacing::Double => self.pdf_context.scale(2.0,1.0),
        };
        pangocairo::functions::update_layout(&self.pdf_context,&self.layout);
        let y = (self.lines as f64 * pango::units_to_double(h as i32))+36.0;
        let x = (self.currentColumn as f64 * pango::units_to_double(w as i32))+36.0;
        self.pdf_context.move_to(x,y);
        pangocairo::functions::show_layout(&self.pdf_context, &self.layout);
        self.pdf_context.restore().expect("Failed to restore context");
        let pp = (w as u32 + 512) >> 10;
        (pango::units_to_double(w as i32) * text.len() as f64) / pp as f64
    }
    pub fn Tab(&mut self, column: u8) -> bool {
        while self.currentColumn < column.into() {self.Put(" ");}
        true
    }
}

pub trait __Put<T> {
    fn Put(&mut self,object: T) -> bool;
}

impl __Put<u8> for Printer {
    fn Put(&mut self,object: u8) -> bool {
        self.Put(format!("{}",object));
        true
    }
}

impl __Put<usize> for Printer {
    fn Put(&mut self,object: usize) -> bool {
        self.Put(format!("{}",object));
        true
    }
}

impl __Put<isize> for Printer {
    fn Put(&mut self,object: isize) -> bool {
        self.Put(format!("{}",object));
        true
    }
}

impl __Put<u32> for Printer {
    fn Put(&mut self,object: u32) -> bool {
        self.Put(format!("{}",object));
        true
    }
}

impl __Put<i32> for Printer {
    fn Put(&mut self,object: i32) -> bool {
        self.Put(format!("{}",object));
        true
    }
}

impl __Put<f64> for Printer {
    fn Put(&mut self,object: f64) -> bool {
        self.Put(format!("{}",object));
        true
    }
}

impl __Put<String> for Printer {
    fn Put(&mut self,object: String) -> bool {
        self.Put(object.as_str());
        true
    }
}

impl __Put<&str> for Printer {
    fn Put(&mut self,text: &str) -> bool {
        let mut lineIter = text.lines();
        let mut line = lineIter.next();
        loop {
            if line.is_none() {break;}
            if self.needPage {
                self.pdf_context.show_page().expect("Error showing page");
                self.needPage = false;
            }
            self.currentColumnFraction += self.putstring(line.unwrap());
            self.currentColumn = self.currentColumnFraction as u32;
            self.partline = true;
            line = lineIter.next();
            if line.is_some() {
                self.currentColumn = 0;
                self.currentColumnFraction = 0.0;
                self.partline = false;
                self.lines += 1;
                if self.lines >= self.maxLines {self.NewPage(&self.title.clone());}
            }
        }
        true
    }
}

//impl Drop for Printer {
//    fn drop(&mut self) {
//        self.ClosePrinter();
//    }
//}
