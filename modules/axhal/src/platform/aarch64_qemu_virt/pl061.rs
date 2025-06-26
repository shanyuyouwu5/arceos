use tock_registers::{interfaces::Writeable, register_bitfields, register_structs, registers::{ReadOnly, ReadWrite, WriteOnly}};
pub const PL061BASE:usize = 0x09030000;
pub const PL061_BASE:*mut Pl061reg = (axconfig::plat::PHYS_VIRT_OFFSET+PL061BASE) as *mut Pl061reg;  
register_bitfields![
    u32,
    pub IE [
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            clear = 0,
            set = 1
        ]
    ],
    pub IC [
        Pin0 OFFSET(0) NUMBITS(1) [
            set = 1
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            set = 1
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            set = 1
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            set = 1
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            set = 1
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            set = 1
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            set = 1
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            set = 1
        ]
    ],
    pub DIR[
        Pin0 OFFSET(0) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin1 OFFSET(1) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin2 OFFSET(2) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin3 OFFSET(3) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin4 OFFSET(4) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin5 OFFSET(5) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin6 OFFSET(6) NUMBITS(1) [
            clear = 0,
            set = 1
        ],
        Pin7 OFFSET(7) NUMBITS(1) [
            clear = 0,
            set = 1
        ]
    ]
];
register_structs!{
    pub Pl061reg{
        (0x000=>data:ReadWrite<u32>),
        (0x004=>_reserved_0),
        (0x400=>pub dir:ReadWrite<u32,DIR::Register>),
        (0x404=>is:ReadWrite<u32>),
        (0x408=>ibe:ReadWrite<u32>),
        (0x40c=>iev:ReadWrite<u32>),
        (0x410=>pub ie:ReadWrite<u32,IE::Register>),
        (0x414=>ris:ReadOnly<u32>),
        (0x418=>mis:ReadOnly<u32>),
        (0x41c=>pub ic:WriteOnly<u32,IC::Register>),
        (0x420=>pub gpio_afsel: ReadWrite<u32>),
        (0x424=>@END),
    }
}
pub fn pl061_init() {
    info!("pl0601 init");
    let pl061_reg = unsafe{&*PL061_BASE};
    // pl061_reg.dir.write(DIR::Pin3::clear);
    pl061_reg.ie.write(IE::Pin3::set)
    
}
