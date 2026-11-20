pub struct PresetWarrior {
    pub id: &'static str,
    pub name: &'static str,
    pub author: &'static str,
    pub category: &'static str,
    pub code: &'static str,
}

pub const PRESETS: &[PresetWarrior] = &[
    PresetWarrior {
        id: "dwarf",
        name: "Dwarf",
        author: "A. K. Dewdney",
        category: "Stone / Bomber",
        code: ";redcode-94\n;name Dwarf\n;author A. K. Dewdney\n;strategy Classic bomber: drops DAT bombs every 4 cells\n        ADD.AB  #4,     bomb\n        MOV.I   bomb,   @bomb\n        JMP     -2\nbomb    DAT     #0,     #0\n        END",
    },
    PresetWarrior {
        id: "imp",
        name: "Imp",
        author: "A. K. Dewdney",
        category: "Imp",
        code: ";redcode-94\n;name Imp\n;author A. K. Dewdney\n;strategy Simplest mobile warrior: copies itself ahead\n        MOV.I   $0,     $1\n        END",
    },
    PresetWarrior {
        id: "mice",
        name: "Mice",
        author: "Chip Wendell",
        category: "Paper / Replicator",
        code: ";redcode-94\n;name Mice\n;author Chip Wendell\n;strategy 1986 ICWS champion replicator\nptr     DAT     #0,     #0\nstart   MOV.AB  #12,    ptr\nloop    MOV.I   @ptr,   <copy\n        DJN     loop,   ptr\n        SPL     @copy,  #0\n        ADD.AB  #653,   copy\n        JMZ     start,  ptr\ncopy    DAT     #0,     #833\n        END     start",
    },
    PresetWarrior {
        id: "silk",
        name: "Silk Warrior",
        author: "Juha Pohjalainen",
        category: "Paper / Silk",
        code: ";redcode-94\n;name Silk Warrior\n;author Juha Pohjalainen\n;strategy Rapid parallel replicator using post-increment\n        SPL     1,      #0\n        SPL     1,      #0\n        MOV.I   -1,     #0\nsilk    SPL     @0,     >1600\n        MOV.I   }silk,  >silk\n        MOV.I   bomber, >3400\n        JMP     silk,   <silk\nbomber  DAT     <2667,  <5334\n        END",
    },
    PresetWarrior {
        id: "scanner",
        name: "Myrmidon Scanner",
        author: "Core War Classic",
        category: "Scissors / Scanner",
        code: ";redcode-94\n;name Myrmidon Scanner\n;author Core War Classic\n;strategy Scans core for non-zero cells, then SPL/DAT carpet bombs\nstep    EQU     10\nscan    ADD.AB  #step,  ptr\n        JMZ.B   scan,   @ptr\n        MOV.I   splbmb, @ptr\n        MOV.I   datbmb, <ptr\n        DJN.B   -2,     #4\n        JMP     scan\nptr     DAT     #0,     #100\nsplbmb  SPL     #0,     #0\ndatbmb  DAT     #0,     #0\n        END     scan",
    },
    PresetWarrior {
        id: "vampire",
        name: "Pit Vampire",
        author: "Core War Classic",
        category: "Vampire",
        code: ";redcode-94\n;name Pit Vampire\n;author Core War Classic\n;strategy Scatters JMP fangs that trap enemy processes in a SPL pit\n        ORG     loop\nfang    JMP     pit-40, 40\nloop    ADD.F   inc,    fang\n        MOV.I   fang,   @fang\n        JMP     loop\ninc     DAT     #-16,   #16\npit     SPL     #0,     #0\n        JMP     pit,    <-10\n        END",
    },
];
