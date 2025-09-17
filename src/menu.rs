// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-09-15 13:11:06
//  Last Modified : <250916.2222>
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

use std::io;
use crossterm::event::KeyEventKind;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

/// Read one Character.  Used by the menu and wait_any_key functions.
/// Lifted from the Crossterm interactive-demo example.
/// ## Parameters:
/// None.
///
/// __Returns__ one char, wrapped in an io::Result.
pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

/// Display a menu screen and read one character.  Loops until the character
/// read is one of the chars in the supplied slice.
/// 
/// The menu screen is displayed and one character is read. If the character
/// is one of the allowed characters, the function returns that character.
/// if it is any other character, the character is flagged as an undefined
/// command character and the menu is redisplayed and another character is
/// read.  This is repeated until a valid character is read, in which case the
/// character is returned.
///
/// Adapted from the Crossterm interactive-demo example.
/// ## Parameters:
/// - w The io::Write object for the terminal (typically from stdout).
/// - thescreen A &str that is the text of the menu screen.
/// - chars A slice contains the allowed characters.
///
/// __Returns__ a character wrapped in an io::Result.
pub fn menu<W>(w: &mut W, thscreen: &str, chars: &[char]) -> io::Result<char>
where
    W: io::Write,
{
    //execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;
    let mut theresult;
    let mut message: String = String::from("");
    loop {

        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 1)
        )?;
        if message.len() > 0 {
            queue!(w, style::Print(message), cursor::MoveToNextLine(1))?;
        }
        for line in thscreen.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        theresult = read_char()?;
        if chars.contains(&theresult) {
            break;
        } else {
            message = format!("Undefined command letter: {}",theresult);
        }
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        //terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;
    Ok(theresult)
}

/// Display a one line message and wait for any keypress.
/// ## Parameters:
/// - w The io::Write to write to, usually stdout.
/// - message The one line message.
///
/// __Returns__ nothing wrapped in an io::Result.
pub fn wait_any_key<W>(w: &mut W, message: &str) -> io::Result<()>
where
    W: io::Write,
{
    //execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    queue!(
        w,
        style::ResetColor,
        cursor::Hide,
        style::Print(message))?;

    w.flush()?;

    read_char()?;
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        //terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;
    Ok(())
}
