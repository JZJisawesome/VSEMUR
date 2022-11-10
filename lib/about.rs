pub mod version {
    pub static STRING: &str = env!("CARGO_PKG_VERSION");
    pub static MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    pub static MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    pub static PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

    pub fn major_u128() -> u128 {
        return MAJOR.parse::<u128>().unwrap();
    }

    pub fn minor_u128() -> u128 {
        return MINOR.parse::<u128>().unwrap();
    }

    pub fn patch_u128() -> u128 {
        return PATCH.parse::<u128>().unwrap();
    }

    pub fn pretty_string() -> String {
        return format!("libvsemur v{} ({} Build)", env!("CARGO_PKG_VERSION"), if cfg!(debug_assertions) {"Debug"} else {"Release"});
    }
}

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

pub static DEBUG: bool = cfg!(debug_assertions);
