/* NAME//TODO
 * By: John Jekel
 *
 * TODO description
 *
*/

/* Imports */

//TODO (include "use" and "mod" here)

/* Constants */

pub(super) const MEM_SIZE_WORDS: usize = 1 << 22;//TODO set this to 0xFFFF since everything above this should not be writable

/* Macros */

//TODO (also pub(crate) use the_macro statements here too)

/* Static Variables */

//TODO

/* Types */

pub(super) trait Memory {
    fn read_addr(self: &Self, addr: u32) -> u16;
    fn write_addr(self: &mut Self, addr: u32, data: u16);

    fn read_page_addr(self: &Self, page: u8, addr: u16) -> u16 {
        return self.read_addr(((page as u32) << 16) | (addr as u32));
    }

    fn write_page_addr(self: &mut Self, page: u8, addr: u16, data: u16) {
        self.write_addr(((page as u32) << 16) | (addr as u32), data);
    }
}

/* Associated Functions and Methods */

//TODO

/* Functions */

//TODO
