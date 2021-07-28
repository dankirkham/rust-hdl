use crate::bom::{Manufacturer, Supplier};
use crate::capacitors::{CapacitorKind, CapacitorTolerance};
use crate::designator::Designator;
use crate::diode::DiodeKind;
use crate::epin::EPin;
use crate::resistors::{PowerWatt, ResistorKind};
use crate::smd::SizeCode;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct PartDetails {
    pub label: String,
    pub manufacturer: Manufacturer,
    pub description: String,
    pub comment: String,
    pub hide_pin_designators: bool,
    pub pins: BTreeMap<u64, EPin>,
    pub suppliers: Vec<Supplier>,
    pub designator: Designator,
    pub size: SizeCode,
}

#[derive(Clone, Debug)]
pub struct Capacitor {
    pub details: PartDetails,
    pub value_pf: f64,
    pub kind: CapacitorKind,
    pub voltage: f64,
    pub tolerance: CapacitorTolerance,
}

#[derive(Clone, Debug)]
pub struct Resistor {
    pub details: PartDetails,
    pub value_ohms: f64,
    pub kind: ResistorKind,
    pub power_watt: PowerWatt,
    pub tolerance: f64,
    pub tempco: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Inductor {
    pub details: PartDetails,
    pub value_microhenry: f64,
    pub tolerance: f64,
    pub dc_resistance_ohms: f64,
    pub max_current_milliamps: f64,
}

#[derive(Clone, Debug)]
pub struct Diode {
    pub details: PartDetails,
    pub forward_drop_volts: f64,
    pub kind: DiodeKind,
}

#[derive(Clone, Debug)]
pub struct Regulator {
    pub details: PartDetails,
    pub input_min_voltage: f64,
    pub input_max_voltage: f64,
    pub output_nominal_voltage: f64,
    pub output_max_current_ma: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicSignalStandard {
    CMOS3V3,
    TTL,
    WideRange,
    TriState,
    TriState5v0,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicFunction {
    XOR,
    Buffer,
    Decoder,
    Multiplexer,
}

#[derive(Clone, Debug)]
pub struct Logic {
    pub details: PartDetails,
    pub drive_current_ma: f64,
    pub min_supply_voltage: f64,
    pub max_supply_voltage: f64,
    pub input_type: LogicSignalStandard,
    pub output_type: LogicSignalStandard,
    pub function: LogicFunction,
}

pub struct Net {
    source_pin: u64,
    dest_pin: u64,
    name: String,
}

pub enum CircuitNode {
    Capacitor(Capacitor),
    Resistor(Resistor),
    Diode(Diode),
    Regulator(Regulator),
    Inductor(Inductor),
    IntegratedCircuit(PartDetails),
    Circuit(Box<Circuit>),
}

pub struct Circuit {
    pins: BTreeMap<u64, EPin>,
    nodes: Vec<CircuitNode>,
    net: Vec<Net>,
}
