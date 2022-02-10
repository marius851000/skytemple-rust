/*
 * Copyright 2021-2022 Capypara and the SkyTemple Contributors
 *
 * This file is part of SkyTemple.
 *
 * SkyTemple is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * SkyTemple is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with SkyTemple.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::bytes::StBytes;
use crate::python::*;

mod implem {
    pub use crate::dse::st_swdl::model;
    pub use crate::dse::st_swdl::wavi;
    pub use crate::dse::st_swdl::pcmd;
    pub use crate::dse::st_swdl::prgi;
    pub use crate::dse::st_swdl::kgrp;
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlPcmd {
    #[pyo3(get, set)]
    chunk_data: StBytes
}

impl From<implem::pcmd::SwdlPcmd> for SwdlPcmd {
    fn from(source: implem::pcmd::SwdlPcmd) -> Self {
        SwdlPcmd {
            chunk_data: source.chunk_data,
        }
    }
}

impl From<SwdlPcmd> for implem::pcmd::SwdlPcmd {
    fn from(source: SwdlPcmd) -> Self {
        implem::pcmd::SwdlPcmd {
            chunk_data: source.chunk_data,
        }
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlPcmdReference {
    #[pyo3(get, set)]
    pcmd: Py<SwdlPcmd>,
    #[pyo3(get, set)]
    offset: u32,
    #[pyo3(get, set)]
    length: u32
}

impl From<implem::wavi::SwdlPcmdReference> for SwdlPcmdReference {
    fn from(source: implem::wavi::SwdlPcmdReference) -> Self {
        Python::with_gil(|py| {
            SwdlPcmdReference {
                pcmd: Py::new(py, SwdlPcmd::from(source.pcmd)).unwrap(),
                offset: source.offset,
                length: source.length,
            }
        })
    }
}

impl From<SwdlPcmdReference> for implem::wavi::SwdlPcmdReference {
    fn from(source: SwdlPcmdReference) -> Self {
        Python::with_gil(|py| {
            implem::wavi::SwdlPcmdReference {
                pcmd: source.pcmd.extract::<SwdlPcmd>(py).unwrap().into(),
                offset: source.offset,
                length: source.length,
            }
        })
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlSampleInfoTblEntry {
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    ftune: u32,
    #[pyo3(get, set)]
    ctune: u32,
    #[pyo3(get, set)]
    rootkey: u32,
    #[pyo3(get, set)]
    ktps: u32,
    #[pyo3(get, set)]
    volume: u32,
    #[pyo3(get, set)]
    pan: u32,
    #[pyo3(get, set)]
    unk5: u32,
    #[pyo3(get, set)]
    unk58: u32,
    #[pyo3(get, set)]
    sample_format: u32,
    #[pyo3(get, set)]
    unk9: u32,
    #[pyo3(get, set)]
    unk10: u32,
    #[pyo3(get, set)]
    unk11: u32,
    #[pyo3(get, set)]
    unk12: u32,
    #[pyo3(get, set)]
    unk13: u32,
    #[pyo3(get, set)]
    sample_rate: u32,
    #[pyo3(get, set)]
    sample: Option<Py<SwdlPcmdReference>>,
    #[pyo3(get, set)]
    loop_begin_pos: u32,
    #[pyo3(get, set)]
    loop_length: u32,
    #[pyo3(get, set)]
    envelope: u32,
    #[pyo3(get, set)]
    envelope_multiplier: u32,
    #[pyo3(get, set)]
    unk19: u32,
    #[pyo3(get, set)]
    unk20: u32,
    #[pyo3(get, set)]
    unk21: u32,
    #[pyo3(get, set)]
    unk22: u32,
    #[pyo3(get, set)]
    attack_volume: u32,
    #[pyo3(get, set)]
    attack: u32,
    #[pyo3(get, set)]
    decay: u32,
    #[pyo3(get, set)]
    sustain: u32,
    #[pyo3(get, set)]
    hold: u32,
    #[pyo3(get, set)]
    decay2: u32,
    #[pyo3(get, set)]
    release: u32,
    #[pyo3(get, set)]
    unk57: u32
}

impl From<implem::wavi::SwdlSampleInfoTblEntry> for SwdlSampleInfoTblEntry {
    fn from(source: implem::wavi::SwdlSampleInfoTblEntry) -> Self {
        Python::with_gil(|py| {
            SwdlSampleInfoTblEntry {
                id: source.id,
                unk11: source.unk11,
                unk12: source.unk12,
                unk13: source.unk13,
                sample_rate: source.sample_rate,
                sample: source.sample.map(|v| Py::new(py, SwdlPcmdReference::from(v)).unwrap()),
                loop_begin_pos: source.loop_begin_pos,
                ftune: source.ftune,
                ctune: source.ctune,
                rootkey: source.rootkey,
                ktps: source.ktps,
                volume: source.volume,
                pan: source.pan,
                unk5: source.unk5,
                unk58: source.unk58,
                sample_format: source.sample_format,
                unk9: source.unk9,
                unk22: source.unk22,
                envelope: source.envelope,
                envelope_multiplier: source.envelope_multiplier,
                unk19: source.unk19,
                unk20: source.unk20,
                attack_volume: source.attack_volume,
                attack: source.attack,
                decay: source.decay,
                sustain: source.sustain,
                hold: source.hold,
                decay2: source.decay2,
                release: source.release,
                unk10: source.unk10,
                loop_length: source.loop_length,
                unk21: source.unk21,
                unk57: source.unk57
            }
        })
    }
}

impl From<SwdlSampleInfoTblEntry> for implem::wavi::SwdlSampleInfoTblEntry {
    fn from(source: SwdlSampleInfoTblEntry) -> Self {
        Python::with_gil(|py| {
            implem::wavi::SwdlSampleInfoTblEntry {
                id: source.id,
                unk11: source.unk11,
                unk12: source.unk12,
                unk13: source.unk13,
                sample_rate: source.sample_rate,
                sample: source.sample.map(|v| v.extract::<SwdlPcmdReference>(py).unwrap().into()),
                loop_begin_pos: source.loop_begin_pos,
                ftune: source.ftune,
                ctune: source.ctune,
                rootkey: source.rootkey,
                ktps: source.ktps,
                volume: source.volume,
                pan: source.pan,
                unk5: source.unk5,
                unk58: source.unk58,
                sample_format: source.sample_format,
                unk9: source.unk9,
                unk22: source.unk22,
                envelope: source.envelope,
                envelope_multiplier: source.envelope_multiplier,
                unk19: source.unk19,
                unk20: source.unk20,
                attack_volume: source.attack_volume,
                attack: source.attack,
                decay: source.decay,
                sustain: source.sustain,
                hold: source.hold,
                decay2: source.decay2,
                release: source.release,
                unk10: source.unk10,
                loop_length: source.loop_length,
                unk21: source.unk21,
                unk57: source.unk57
            }
        })
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlWavi {
    #[pyo3(get, set)]
    sample_info_table: Vec<Option<Py<SwdlSampleInfoTblEntry>>>
}

impl From<implem::wavi::SwdlWavi> for SwdlWavi {
    fn from(source: implem::wavi::SwdlWavi) -> Self {
        Python::with_gil(|py| {
            SwdlWavi {
                sample_info_table: source.sample_info_table.into_iter().map(|v| v.map(|v| Py::new(py, SwdlSampleInfoTblEntry::from(v)).unwrap())).collect(),
            }
        })
    }
}

impl From<SwdlWavi> for implem::wavi::SwdlWavi {
    fn from(source: SwdlWavi) -> Self {
        Python::with_gil(|py| {
            implem::wavi::SwdlWavi {
                sample_info_table: source.sample_info_table.into_iter().map(|v| v.map(|v| v.extract::<SwdlSampleInfoTblEntry>(py).unwrap().into())).collect(),
            }
        })
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlLfoEntry {
    #[pyo3(get, set)]
    unk34: u32,
    #[pyo3(get, set)]
    unk52: u32,
    #[pyo3(get, set)]
    dest: u32,
    #[pyo3(get, set)]
    wshape: u32,
    #[pyo3(get, set)]
    rate: u32,
    #[pyo3(get, set)]
    unk29: u32,
    #[pyo3(get, set)]
    depth: u32,
    #[pyo3(get, set)]
    delay: u32,
    #[pyo3(get, set)]
    unk32: u32,
    #[pyo3(get, set)]
    unk33: u32,
}

impl From<implem::prgi::SwdlLfoEntry> for SwdlLfoEntry {
    fn from(source: implem::prgi::SwdlLfoEntry) -> Self {
        SwdlLfoEntry {
            unk34: source.unk34,
            unk52: source.unk52,
            dest: source.dest,
            wshape: source.wshape,
            rate: source.rate,
            unk29: source.unk29,
            depth: source.depth,
            delay: source.delay,
            unk32: source.unk32,
            unk33: source.unk33,
        }
    }
}

impl From<SwdlLfoEntry> for implem::prgi::SwdlLfoEntry {
    fn from(source: SwdlLfoEntry) -> Self {
        implem::prgi::SwdlLfoEntry {
            unk34: source.unk34,
            unk52: source.unk52,
            dest: source.dest,
            wshape: source.wshape,
            rate: source.rate,
            unk29: source.unk29,
            depth: source.depth,
            delay: source.delay,
            unk32: source.unk32,
            unk33: source.unk33,
        }
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlSplitEntry {
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    unk11: u32,
    #[pyo3(get, set)]
    unk25: u32,
    #[pyo3(get, set)]
    lowkey: u32,
    #[pyo3(get, set)]
    hikey: u32,
    #[pyo3(get, set)]
    lolevel: u32,
    #[pyo3(get, set)]
    hilevel: u32,
    #[pyo3(get, set)]
    unk16: u32,
    #[pyo3(get, set)]
    unk17: u32,
    #[pyo3(get, set)]
    sample_id: u32,
    #[pyo3(get, set)]
    ftune: u32,
    #[pyo3(get, set)]
    ctune: u32,
    #[pyo3(get, set)]
    rootkey: u32,
    #[pyo3(get, set)]
    ktps: u32,
    #[pyo3(get, set)]
    sample_volume: u32,
    #[pyo3(get, set)]
    sample_pan: u32,
    #[pyo3(get, set)]
    keygroup_id: u32,
    #[pyo3(get, set)]
    unk22: u32,
    #[pyo3(get, set)]
    unk23: u32,
    #[pyo3(get, set)]
    unk24: u32,
    #[pyo3(get, set)]
    envelope: u32,
    #[pyo3(get, set)]
    envelope_multiplier: u32,
    #[pyo3(get, set)]
    unk37: u32,
    #[pyo3(get, set)]
    unk38: u32,
    #[pyo3(get, set)]
    unk39: u32,
    #[pyo3(get, set)]
    unk40: u32,
    #[pyo3(get, set)]
    attack_volume: u32,
    #[pyo3(get, set)]
    attack: u32,
    #[pyo3(get, set)]
    decay: u32,
    #[pyo3(get, set)]
    sustain: u32,
    #[pyo3(get, set)]
    hold: u32,
    #[pyo3(get, set)]
    decay2: u32,
    #[pyo3(get, set)]
    release: u32,
    #[pyo3(get, set)]
    unk53: u32,
}

impl From<implem::prgi::SwdlSplitEntry> for SwdlSplitEntry {
    fn from(source: implem::prgi::SwdlSplitEntry) -> Self {
        SwdlSplitEntry {
            id: source.id,
            unk11: source.unk11,
            unk25: source.unk25,
            lowkey: source.lowkey,
            hikey: source.hikey,
            lolevel: source.lolevel,
            hilevel: source.hilevel,
            unk16: source.unk16,
            unk17: source.unk17,
            sample_id: source.sample_id,
            ftune: source.ftune,
            ctune: source.ctune,
            rootkey: source.rootkey,
            ktps: source.ktps,
            sample_volume: source.sample_volume,
            sample_pan: source.sample_pan,
            keygroup_id: source.keygroup_id,
            unk22: source.unk22,
            unk23: source.unk23,
            unk24: source.unk24,
            envelope: source.envelope,
            envelope_multiplier: source.envelope_multiplier,
            unk37: source.unk37,
            unk38: source.unk38,
            unk39: source.unk39,
            unk40: source.unk40,
            attack_volume: source.attack_volume,
            attack: source.attack,
            decay: source.decay,
            sustain: source.sustain,
            hold: source.hold,
            decay2: source.decay2,
            release: source.release,
            unk53: source.unk53,
        }
    }
}

impl From<SwdlSplitEntry> for implem::prgi::SwdlSplitEntry {
    fn from(source: SwdlSplitEntry) -> Self {
        implem::prgi::SwdlSplitEntry {
            id: source.id,
            unk11: source.unk11,
            unk25: source.unk25,
            lowkey: source.lowkey,
            hikey: source.hikey,
            lolevel: source.lolevel,
            hilevel: source.hilevel,
            unk16: source.unk16,
            unk17: source.unk17,
            sample_id: source.sample_id,
            ftune: source.ftune,
            ctune: source.ctune,
            rootkey: source.rootkey,
            ktps: source.ktps,
            sample_volume: source.sample_volume,
            sample_pan: source.sample_pan,
            keygroup_id: source.keygroup_id,
            unk22: source.unk22,
            unk23: source.unk23,
            unk24: source.unk24,
            envelope: source.envelope,
            envelope_multiplier: source.envelope_multiplier,
            unk37: source.unk37,
            unk38: source.unk38,
            unk39: source.unk39,
            unk40: source.unk40,
            attack_volume: source.attack_volume,
            attack: source.attack,
            decay: source.decay,
            sustain: source.sustain,
            hold: source.hold,
            decay2: source.decay2,
            release: source.release,
            unk53: source.unk53,
        }
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlProgramTable {
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    prg_volume: u32,
    #[pyo3(get, set)]
    prg_pan: u32,
    #[pyo3(get, set)]
    unk3: u32,
    #[pyo3(get, set)]
    that_f_byte: u32,
    #[pyo3(get, set)]
    unk4: u32,
    #[pyo3(get, set)]
    unk5: u32,
    #[pyo3(get, set)]
    unk7: u32,
    #[pyo3(get, set)]
    unk8: u32,
    #[pyo3(get, set)]
    unk9: u32,
    #[pyo3(get, set)]
    lfos: Vec<Py<SwdlLfoEntry>>,
    #[pyo3(get, set)]
    splits: Vec<Py<SwdlSplitEntry>>,
}

impl From<implem::prgi::SwdlProgramTable> for SwdlProgramTable {
    fn from(source: implem::prgi::SwdlProgramTable) -> Self {
        Python::with_gil(|py| {
            SwdlProgramTable {
                id: source.id,
                prg_volume: source.prg_volume,
                prg_pan: source.prg_pan,
                unk3: source.unk3,
                that_f_byte: source.that_f_byte,
                unk4: source.unk4,
                unk5: source.unk5,
                unk7: source.unk7,
                unk8: source.unk8,
                unk9: source.unk9,
                lfos: source.lfos.into_iter().map(|v| Py::new(py, SwdlLfoEntry::from(v)).unwrap()).collect(),
                splits: source.splits.into_iter().map(|v| Py::new(py, SwdlSplitEntry::from(v)).unwrap()).collect(),
            }
        })
    }
}

impl From<SwdlProgramTable> for implem::prgi::SwdlProgramTable {
    fn from(source: SwdlProgramTable) -> Self {
        Python::with_gil(|py| {
            implem::prgi::SwdlProgramTable {
                id: source.id,
                prg_volume: source.prg_volume,
                prg_pan: source.prg_pan,
                unk3: source.unk3,
                that_f_byte: source.that_f_byte,
                unk4: source.unk4,
                unk5: source.unk5,
                unk7: source.unk7,
                unk8: source.unk8,
                unk9: source.unk9,
                lfos: source.lfos.into_iter().map(|v| v.extract::<SwdlLfoEntry>(py).unwrap().into()).collect(),
                splits: source.splits.into_iter().map(|v| v.extract::<SwdlSplitEntry>(py).unwrap().into()).collect(),
            }
        })
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlPrgi {
    #[pyo3(get, set)]
    program_table: Vec<Option<Py<SwdlProgramTable>>>
}

impl From<implem::prgi::SwdlPrgi> for SwdlPrgi {
    fn from(source: implem::prgi::SwdlPrgi) -> Self {
        Python::with_gil(|py| {
            SwdlPrgi {
                program_table: source.program_table.into_iter().map(|v| v.map(|v| Py::new(py, SwdlProgramTable::from(v)).unwrap())).collect(),
            }
        })
    }
}

impl From<SwdlPrgi> for implem::prgi::SwdlPrgi {
    fn from(source: SwdlPrgi) -> Self {
        Python::with_gil(|py| {
            implem::prgi::SwdlPrgi {
                program_table: source.program_table.into_iter().map(|v| v.map(|v| v.extract::<SwdlProgramTable>(py).unwrap().into())).collect(),
            }
        })
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlKeygroup {
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    poly: u32,
    #[pyo3(get, set)]
    priority: u32,
    #[pyo3(get, set)]
    vclow: u32,
    #[pyo3(get, set)]
    vchigh: u32,
    #[pyo3(get, set)]
    unk50: u32,
    #[pyo3(get, set)]
    unk51: u32,
}

impl From<implem::kgrp::SwdlKeygroup> for SwdlKeygroup {
    fn from(source: implem::kgrp::SwdlKeygroup) -> Self {
        SwdlKeygroup {
            id: source.id,
            poly: source.poly,
            priority: source.priority,
            vclow: source.vclow,
            vchigh: source.vchigh,
            unk50: source.unk50,
            unk51: source.unk51,
        }
    }
}

impl From<SwdlKeygroup> for implem::kgrp::SwdlKeygroup {
    fn from(source: SwdlKeygroup) -> Self {
        implem::kgrp::SwdlKeygroup {
            id: source.id,
            poly: source.poly,
            priority: source.priority,
            vclow: source.vclow,
            vchigh: source.vchigh,
            unk50: source.unk50,
            unk51: source.unk51,
        }
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlKgrp {
    #[pyo3(get, set)]
    keygroups: Vec<Py<SwdlKeygroup>>
}

impl From<implem::kgrp::SwdlKgrp> for SwdlKgrp {
    fn from(source: implem::kgrp::SwdlKgrp) -> Self {
        Python::with_gil(|py| {
            SwdlKgrp {
                keygroups: source.keygroups.into_iter().map(|v| Py::new(py, SwdlKeygroup::from(v)).unwrap()).collect(),
            }
        })
    }
}

impl From<SwdlKgrp> for implem::kgrp::SwdlKgrp {
    fn from(source: SwdlKgrp) -> Self {
        Python::with_gil(|py| {
            implem::kgrp::SwdlKgrp {
                keygroups: source.keygroups.into_iter().map(|v| v.extract::<SwdlKeygroup>(py).unwrap().into()).collect(),
            }
        })
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlPcmdLen {
    #[pyo3(get, set)]
    reference: Option<u32>,
    #[pyo3(get, set)]
    external: bool,
}

impl From<implem::model::SwdlPcmdLen> for SwdlPcmdLen {
    fn from(source: implem::model::SwdlPcmdLen) -> Self {
        SwdlPcmdLen {
            reference: source.reference,
            external: source.external,
        }
    }
}

impl From<SwdlPcmdLen> for implem::model::SwdlPcmdLen {
    fn from(source: SwdlPcmdLen) -> Self {
        implem::model::SwdlPcmdLen {
            reference: source.reference,
            external: source.external,
        }
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct SwdlHeader {
    #[pyo3(get, set)]
    version: u32,
    #[pyo3(get, set)]
    unk1: u32,
    #[pyo3(get, set)]
    unk2: u32,
    #[pyo3(get, set)]
    modified_date: StBytes,
    #[pyo3(get, set)]
    file_name: StBytes,
    #[pyo3(get, set)]
    unk13: u32,
    #[pyo3(get, set)]
    pcmdlen: u32,
    #[pyo3(get, set)]
    unk17: u32,
}

impl From<implem::model::SwdlHeader> for SwdlHeader {
    fn from(source: implem::model::SwdlHeader) -> Self {
        SwdlHeader {
            version: source.version,
            unk1: source.unk1,
            unk2: source.unk2,
            modified_date: source.modified_date.into(),
            file_name: source.file_name.into(),
            unk13: source.unk13,
            pcmdlen: source.pcmdlen,
            unk17: source.unk17,
        }
    }
}

impl From<SwdlHeader> for implem::model::SwdlHeader {
    fn from(mut source: SwdlHeader) -> Self {
        implem::model::SwdlHeader {
            version: source.version,
            unk1: source.unk1,
            unk2: source.unk2,
            modified_date: (&mut source.modified_date).into(),
            file_name: (&mut source.file_name).into(),
            unk13: source.unk13,
            pcmdlen: source.pcmdlen,
            unk17: source.unk17,
        }
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone)]
pub(crate) struct Swdl {
    #[pyo3(get, set)]
    header: Py<SwdlHeader>,
    #[pyo3(get, set)]
    wavi: Py<SwdlWavi>,
    #[pyo3(get, set)]
    pcmd: Option<Py<SwdlPcmd>>,
    #[pyo3(get, set)]
    prgi: Option<Py<SwdlPrgi>>,
    #[pyo3(get, set)]
    kgrp: Option<Py<SwdlKgrp>>,
}

#[pymethods]
impl Swdl {
    #[new]
    pub fn new(data: StBytes) -> Self {
        implem::model::Swdl::from(data).into()
    }
}

#[pyclass(module = "skytemple_rust.st_swdl")]
#[derive(Clone, Default)]
pub(crate) struct SwdlWriter;

#[pymethods]
impl SwdlWriter {
    #[new]
    pub fn new() -> Self {
        Self
    }
    pub fn write(&self, model: Swdl, py: Python) -> StBytes {
        implem::model::Swdl::from(model).into()
    }
}

impl From<implem::model::Swdl> for Swdl {
    fn from(source: implem::model::Swdl) -> Self {
        Python::with_gil(|py| {
            Swdl {
                header: Py::new(py, SwdlHeader::from(source.header)).unwrap(),
                wavi: Py::new(py, SwdlWavi::from(source.wavi)).unwrap(),
                pcmd: source.pcmd.map(|v| Py::new(py, SwdlPcmd::from(v)).unwrap()),
                prgi: source.prgi.map(|v| Py::new(py, SwdlPrgi::from(v)).unwrap()),
                kgrp: source.kgrp.map(|v| Py::new(py, SwdlKgrp::from(v)).unwrap()),
            }
        })
    }
}

impl From<Swdl> for implem::model::Swdl {
    fn from(source: Swdl) -> Self {
        Python::with_gil(|py| {
            implem::model::Swdl {
                header: source.header.extract::<SwdlHeader>(py).unwrap().into(),
                wavi: source.wavi.extract::<SwdlWavi>(py).unwrap().into(),
                pcmd: source.pcmd.map(|v| v.extract::<SwdlPcmd>(py).unwrap().into()),
                prgi: source.prgi.map(|v| v.extract::<SwdlPrgi>(py).unwrap().into()),
                kgrp: source.kgrp.map(|v| v.extract::<SwdlKgrp>(py).unwrap().into()),
            }
        })
    }
}

pub(crate) fn create_st_swdl_module(py: Python) -> PyResult<(&str, &PyModule)> {
    let name: &'static str = "skytemple_rust.st_swdl";
    let m = PyModule::new(py, name)?;
    m.add_class::<Swdl>()?;
    m.add_class::<SwdlWriter>()?;

    Ok((name, m))
}
