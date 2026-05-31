#![allow(unused_imports, non_camel_case_types, non_snake_case, dead_code)]
use std::mem::ManuallyDrop;
use std::ops::DerefMut;

// ── ELF primitive type aliases 

pub type Elf32_Half = u16;
pub type Elf64_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf32_Addr = u32;
pub type Elf64_Addr = u64;
pub type Elf32_Off = u32;
pub type Elf64_Off = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

// ── ELF standard header structs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Shdr {
    pub sh_name: Elf32_Word,
    pub sh_type: Elf32_Word,
    pub sh_flags: Elf32_Word,
    pub sh_addr: Elf32_Addr,
    pub sh_offset: Elf32_Off,
    pub sh_size: Elf32_Word,
    pub sh_link: Elf32_Word,
    pub sh_info: Elf32_Word,
    pub sh_addralign: Elf32_Word,
    pub sh_entsize: Elf32_Word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf32_Phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut libc::c_void,
    pub d_type: libc::c_uint,
    pub d_version: libc::c_uint,
    pub d_size: usize,
    pub d_off: i64,
    pub d_align: usize,
}

#[repr(C)]
pub struct Elf_Arsym {
    pub as_name: *mut libc::c_char,
    pub as_off: usize,
    pub as_hash: libc::c_ulong,
}

#[repr(C)]
pub struct Elf_Arhdr {
    pub ar_name: *mut libc::c_char,
    pub ar_date: libc::time_t,
    pub ar_uid: libc::uid_t,
    pub ar_gid: libc::gid_t,
    pub ar_mode: libc::mode_t,
    pub ar_size: i64,
    pub ar_rawname: *mut libc::c_char,
}

// ── libelf internal structs 

#[repr(C)]
pub struct ElfDataScn {
    pub d: Elf_Data,
    pub s: *mut ElfScn,
}

#[repr(C)]
pub struct ElfDataList {
    pub data: ElfDataScn,
    pub next: *mut ElfDataList,
    pub flags: libc::c_int,
}

#[repr(C)]
pub union ElfScnShdr {
    pub e32: *mut Elf32_Shdr,
    pub e64: *mut Elf64_Shdr,
}

#[repr(C)]
pub struct ElfScn {
    pub data_list: ElfDataList,
    pub data_list_rear: *mut ElfDataList,
    pub rawdata: ElfDataScn,
    pub data_read: libc::c_int,
    pub shndx_index: libc::c_int,
    pub index: usize,
    pub elf: *mut ElfInternal,
    pub shdr: ElfScnShdr,
    pub shdr_flags: libc::c_uint,
    pub flags: libc::c_uint,
    pub rawdata_base: *mut libc::c_char,
    pub data_base: *mut libc::c_char,
    pub zdata_base: *mut libc::c_char,
    pub zdata_size: usize,
    pub zdata_align: usize,
    pub list: *mut ElfScnList,
}

#[repr(C)]
pub struct ElfScnList {
    pub cnt: libc::c_uint,
    pub max: libc::c_uint,
    pub next: *mut ElfScnList,
    pub data: [ElfScn; 0],
}

#[repr(C)]
pub union ElfDataChunkUnion {
    pub dummy_scn: ManuallyDrop<ElfScn>,
    pub next: *mut ElfDataChunk,
}

#[repr(C)]
pub struct ElfDataChunk {
    pub data: ElfDataScn,
    pub __bindgen_anon_1: ElfDataChunkUnion,
}

#[repr(C)]
pub struct ElfStateCommon {
    pub ehdr: *mut libc::c_void,
    pub shdr: *mut libc::c_void,
    pub phdr: *mut libc::c_void,
    pub scns_last: *mut ElfScnList,
    pub rawchunks: *mut ElfDataChunk,
    pub scnincr: libc::c_uint,
    pub ehdr_flags: libc::c_int,
    pub phdr_flags: libc::c_int,
    pub shdr_malloced: libc::c_int,
    pub sizestr_offset: i64,
}

#[repr(C)]
pub struct ElfState32 {
    pub ehdr: *mut Elf32_Ehdr,
    pub shdr: *mut Elf32_Shdr,
    pub phdr: *mut Elf32_Phdr,
    pub scns_last: *mut ElfScnList,
    pub rawchunks: *mut ElfDataChunk,
    pub scnincr: libc::c_uint,
    pub ehdr_flags: libc::c_int,
    pub phdr_flags: libc::c_int,
    pub shdr_malloced: libc::c_int,
    pub sizestr_offset: i64,
    pub ehdr_mem: Elf32_Ehdr,
    pub __e32scnspad: [libc::c_char; 12],
    pub scns: ElfScnList,
}

#[repr(C)]
pub struct ElfState64 {
    pub ehdr: *mut Elf64_Ehdr,
    pub shdr: *mut Elf64_Shdr,
    pub phdr: *mut Elf64_Phdr,
    pub scns_last: *mut ElfScnList,
    pub rawchunks: *mut ElfDataChunk,
    pub scnincr: libc::c_uint,
    pub ehdr_flags: libc::c_int,
    pub phdr_flags: libc::c_int,
    pub shdr_malloced: libc::c_int,
    pub sizestr_offset: i64,
    pub ehdr_mem: Elf64_Ehdr,
    pub scns: ElfScnList,
}

#[repr(C)]
pub struct ElfStateAr {
    pub children: *mut ElfInternal,
    pub ar_sym: *mut Elf_Arsym,
    pub ar_sym_num: usize,
    pub long_names: *mut libc::c_char,
    pub long_names_len: usize,
    pub offset: i64,
    pub elf_ar_hdr: Elf_Arhdr,
    pub ar_hdr: [libc::c_char; 60],
    pub ar_name: [libc::c_char; 16],
    pub raw_name: [libc::c_char; 17],
}

#[repr(C)]
pub union ElfState {
    pub elf: ManuallyDrop<ElfStateCommon>,
    pub elf32: ManuallyDrop<ElfState32>,
    pub elf64: ManuallyDrop<ElfState64>,
    pub ar: ManuallyDrop<ElfStateAr>,
}

#[repr(C)]
pub struct ElfInternal {
    pub map_address: *mut libc::c_void,
    pub parent: *mut ElfInternal,
    pub next: *mut ElfInternal,
    pub kind: libc::c_uint,
    pub cmd: libc::c_uint,
    pub class: libc::c_uint,
    pub fildes: libc::c_int,
    pub start_offset: i64,
    pub maximum_size: usize,
    pub flags: libc::c_int,
    pub ref_count: libc::c_int,
    pub lock: [u8; 56],
    pub state: ElfState,
}

// ── Constants used by elf_end 

pub const ELFCLASS32: u32 = 1;
pub const ELF_K_AR: u32 = 1;
pub const ELF_K_ELF: u32 = 3;
pub const ELF_F_MALLOCED: u32 = 128;
pub const ELF_F_MMAPPED: u32 = 64;

// ── elf_end 

pub unsafe fn elf_end(elf: *mut ElfInternal) -> libc::c_int {
    if elf.is_null() {
        return 0;
    }

    (*elf).ref_count -= 1;
    if (*elf).ref_count != 0 {
        return (*elf).ref_count;
    }

    if (*elf).kind == ELF_K_AR {
        let ar_sym = (*elf).state.ar.deref_mut().ar_sym;
        if ar_sym != (0usize as *mut _) && ar_sym != (-1isize as *mut _) {
            libc::free(ar_sym as *mut libc::c_void);
        }
        (*elf).state.ar.deref_mut().ar_sym = std::ptr::null_mut();

        if !(*elf).state.ar.deref_mut().children.is_null() {
            return 0;
        }
    }

    let parent = (*elf).parent;
    if !parent.is_null() {
        if (*parent).state.ar.deref_mut().children == elf {
            (*parent).state.ar.deref_mut().children = (*elf).next;
        } else {
            let mut child = (*parent).state.ar.deref_mut().children;
            while (*child).next != elf {
                child = (*child).next;
            }
            (*child).next = (*elf).next;
        }
    }

    match (*elf).kind {
        ELF_K_AR => {
            let long_names = (*elf).state.ar.deref_mut().long_names;
            if !long_names.is_null() {
                libc::free(long_names as *mut libc::c_void);
            }
        }
        ELF_K_ELF => {
            let rawchunks = if (*elf).class == ELFCLASS32 {
                (*elf).state.elf32.deref_mut().rawchunks
            } else {
                (*elf).state.elf64.deref_mut().rawchunks
            };
            let mut rc = rawchunks;
            while !rc.is_null() {
                let next = (*rc).__bindgen_anon_1.next;
                if (&(*rc).__bindgen_anon_1.dummy_scn).flags & (ELF_F_MALLOCED as u32) != 0 {
                    libc::free((*rc).data.d.d_buf);
                }
                libc::free(rc as *mut libc::c_void);
                rc = next;
            }

            let mut list: *mut ElfScnList = if (*elf).class == ELFCLASS32 {
                &mut (*elf).state.elf32.deref_mut().scns
            } else {
                &mut (*elf).state.elf64.deref_mut().scns
            };

            let initial_scns = list;
            loop {
                let mut cnt = (*list).max as usize;
                while cnt > 0 {
                    cnt -= 1;
                    let scn: *mut ElfScn = &mut (*list).data[cnt];

                    if (*scn).shdr_flags & (ELF_F_MALLOCED as u32) != 0 {
                        libc::free((*scn).shdr.e32 as *mut libc::c_void);
                    }

                    if (*scn).zdata_base != (*scn).rawdata_base {
                        libc::free((*scn).zdata_base as *mut libc::c_void);
                    }

                    if (*scn).data_base != (*scn).rawdata_base {
                        libc::free((*scn).data_base as *mut libc::c_void);
                    }

                    if (*elf).map_address.is_null()
                        || (*scn).rawdata_base == (*scn).zdata_base
                        || (*scn).flags & (ELF_F_MALLOCED as u32) != 0
                    {
                        libc::free((*scn).rawdata_base as *mut libc::c_void);
                    }

                    let mut runp = (*scn).data_list.next;
                    while !runp.is_null() {
                        let oldp = runp;
                        runp = (*runp).next;
                        if (*oldp).flags & (ELF_F_MALLOCED as i32) != 0 {
                            libc::free(oldp as *mut libc::c_void);
                        }
                    }
                }

                let oldp = list;
                list = (*list).next;
                debug_assert!(list.is_null() || (*oldp).cnt == (*oldp).max);
                if oldp != initial_scns {
                    libc::free(oldp as *mut libc::c_void);
                }

                if list.is_null() {
                    break;
                }
            }

            if (*elf).state.elf.deref_mut().shdr_malloced != 0 {
                let shdr = if (*elf).class == ELFCLASS32 {
                    (*elf).state.elf32.deref_mut().shdr as *mut libc::c_void
                } else {
                    (*elf).state.elf64.deref_mut().shdr as *mut libc::c_void
                };
                libc::free(shdr);
            }

            if (*elf).state.elf.deref_mut().phdr_flags & (ELF_F_MALLOCED as i32) != 0 {
                let phdr = if (*elf).class == ELFCLASS32 {
                    (*elf).state.elf32.deref_mut().phdr as *mut libc::c_void
                } else {
                    (*elf).state.elf64.deref_mut().phdr as *mut libc::c_void
                };
                libc::free(phdr);
            }
        }
        _ => {}
    }

    if !(*elf).map_address.is_null() && parent.is_null() {
        if (*elf).flags & (ELF_F_MALLOCED as i32) != 0 {
            libc::free((*elf).map_address);
        } else if (*elf).flags & (ELF_F_MMAPPED as i32) != 0 {
            libc::munmap((*elf).map_address, (*elf).maximum_size);
        }
    }

    libc::free(elf as *mut libc::c_void);

    if !parent.is_null() && (*parent).ref_count == 0 {
        elf_end(parent)
    } else {
        0
    }
}
