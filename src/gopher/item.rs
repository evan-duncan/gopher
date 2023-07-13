/**
 *   Gopher client and server software.
 *   Copyright (C) 2023  Evan Duncan
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use super::server::Server;
use std::fmt;

struct ItemType {
    code: char,
}

const FILE: ItemType = ItemType { code: '0' };
const DIRECTORY: ItemType = ItemType { code: '1' };
const CSO_PHONE_BOOK_SERVER: ItemType = ItemType { code: '2' };
const ERROR: ItemType = ItemType { code: '3' };
const BIN_HEXED_MACHINTOH_FILE: ItemType = ItemType { code: '4' };
const DOS_BINARY_ARCHIVE: ItemType = ItemType { code: '5' };
const UNIX_UUENCODED_FILE: ItemType = ItemType { code: '6' };
const INDEXED_SEARCH_SERVER: ItemType = ItemType { code: '7' };
const TELNET_SESSION: ItemType = ItemType { code: '8' };
const BINARY_FILE: ItemType = ItemType { code: '9' };
const REDUNDANT_SERVER: ItemType = ItemType { code: '+' };
const TN_3270_SESSION: ItemType = ItemType { code: 'T' };
const GIF: ItemType = ItemType { code: 'g' };
const IMAGE: ItemType = ItemType { code: 'I' };

pub struct Item {
    item_type: ItemType,
    user_display: String,
    selector: String,
    server: Server,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}\t{}\t{}\t{}",
            self.item_type.code,
            self.user_display,
            self.selector,
            self.server.hostname,
            self.server.port
        )
    }
}
