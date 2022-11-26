/* about.rs
 * Copyright (C) 2022 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * Holds information about the libvsemur library for compatibilty, versioning, and licensing
 *
*/

//!Holds information about the libvsemur library for compatibilty, versioning, and licensing

//Note: we use static so that the values that aren't functions can be accessed through their memory addresses (allows for symbol lookup/future compatibilty)

///libvsemur version information
pub mod version {
    ///str of libvsemur's semantic version
    pub static STRING: &str = env!("CARGO_PKG_VERSION");
    ///str of libvsemur's semantic version, major number
    pub static MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    ///str of libvsemur's semantic version, minor number
    pub static MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    ///str of libvsemur's semantic version, patch number
    pub static PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

    ///Returns libvsemur's semantic version, major number
    pub fn major_u128() -> u128 {
        return MAJOR.parse::<u128>().unwrap();
    }

    ///Returns libvsemur's semantic version, minor number
    pub fn minor_u128() -> u128 {
        return MINOR.parse::<u128>().unwrap();
    }

    ///Returns libvsemur's semantic version, patch number
    pub fn patch_u128() -> u128 {
        return PATCH.parse::<u128>().unwrap();
    }

    ///Returns 1-line "pretty string" containing libvsemur's version, build information, and other neat info
    pub fn pretty_string() -> String {
        return format!("libvsemur v{} ({} Build)", env!("CARGO_PKG_VERSION"), if cfg!(debug_assertions) {"Debug"} else {"Release"});
    }
}

///License text for libvsemur
pub static LICENSE: &'static str = "
VSEMUR
Copyright (C) 2022 John Jekel

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.




My old, now unmaintained VSEMU project was used as a nice starting base for this project:

VSEMU
Copyright (C) 2022 John Jekel

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.



Partly developed with code from my rv32esim project from earlier this year.
I didn't open source that project, but for the purposes of this one, all code
from rv32esim in this project is released under the same license as above.



Uses reverse-engineering documentation (not code) from: https://github.com/MooglyGuy/unsp/tree/master/vsmile

MIT License

Copyright (c) 2019 MooglyGuy

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.



Also developed with reference to parts of the MAME source code:

MAME
Copyright (C) 1997-2022  MAMEDev and contributors

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
";

///true if this is a debug build of libvsemur
pub static DEBUG: bool = cfg!(debug_assertions);
