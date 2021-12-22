use crate::core::prelude::*;
use crate::hls::bus::{SoCBusController, SoCBusResponder};
use crate::widgets::prelude::*;

// A router allows you to connect multiple bridges to a single master
// Each bridge is assigned a base address (they must be non-overlapping).
// The master then sees each port on the bridge mapped to the offset
// of it's base address.  Note that you can stack routers if needed.

#[derive(LogicBlock)]
pub struct Router<const D: usize, const A: usize, const N: usize> {
    pub upstream: SoCBusResponder<D, A>,
    pub nodes: [SoCBusController<D, A>; N],
    node_start_address: [Constant<Bits<A>>; N],
    node_end_address: [Constant<Bits<A>>; N],
    active: DFF<Bits<8>>,
    virtual_address: DFF<Bits<A>>,
    address_strobe_delay: DFF<Bit>,
}

impl<const D: usize, const A: usize, const N: usize> Router<D, A, N> {
    pub fn new(address_count: [usize; N]) -> Self {
        let zero = Constant::<Bits<A>>::new(0usize.into());
        let mut node_start_address: [Constant<Bits<A>>; N] = array_init::array_init(|_| zero);
        let mut node_end_address: [Constant<Bits<A>>; N] = array_init::array_init(|_| zero);
        let mut offset = 0;
        for (ndx, count) in address_count.iter().enumerate() {
            assert_ne!(*count, 0);
            node_start_address[ndx] = Constant::<Bits<A>>::new(offset.into());
            node_end_address[ndx] = Constant::<Bits<A>>::new((offset + count).into());
            offset = offset + count;
        }
        Self {
            upstream: Default::default(),
            nodes: array_init::array_init(|_| Default::default()),
            node_start_address,
            node_end_address,
            active: Default::default(),
            virtual_address: Default::default(),
            address_strobe_delay: Default::default(),
        }
    }
}

impl<const D: usize, const A: usize, const N: usize> Logic for Router<D, A, N> {
    #[hdl_gen]
    fn update(&mut self) {
        self.upstream.ready.next = false;
        self.upstream.to_controller.next = 0_usize.into();
        self.active.clk.next = self.upstream.clock.val();
        self.address_strobe_delay.clk.next = self.upstream.clock.val();
        self.active.d.next = self.active.q.val();
        self.virtual_address.clk.next = self.upstream.clock.val();
        self.virtual_address.d.next = self.virtual_address.q.val();
        // Delay the address strobe by 1 clock cycle to allow the virtual address
        // calculation to be pipelined.
        self.address_strobe_delay.d.next = self.upstream.address_strobe.val();
        for i in 0_usize..N {
            self.nodes[i].from_controller.next = 0_usize.into();
            self.nodes[i].address.next = 0_usize.into();
            self.nodes[i].address_strobe.next = false;
            self.nodes[i].strobe.next = false;
            self.nodes[i].clock.next = self.upstream.clock.val();
            if (self.upstream.address.val() >= self.node_start_address[i].val())
                & (self.upstream.address.val() < self.node_end_address[i].val())
                & self.upstream.address_strobe.val()
            {
                self.active.d.next = i.into();
                self.virtual_address.d.next =
                    self.upstream.address.val() - self.node_start_address[i].val();
            }
            if self.active.q.val().index() == i {
                self.nodes[i].from_controller.next = self.upstream.from_controller.val();
                self.nodes[i].address.next = self.virtual_address.q.val();
                self.nodes[i].strobe.next = self.upstream.strobe.val();
                self.upstream.to_controller.next = self.nodes[i].to_controller.val();
                self.upstream.ready.next = self.nodes[i].ready.val();
                self.nodes[i].address_strobe.next = self.address_strobe_delay.q.val();
            }
        }
        if self.upstream.address_strobe.val() {
            self.upstream.ready.next = false;
        }
    }
}

#[test]
fn test_router_is_synthesizable() {
    let mut router = Router::<16, 8, 6>::new([4, 8, 12, 4, 4, 4]);
    router.upstream.address.connect();
    router.upstream.address_strobe.connect();
    router.upstream.from_controller.connect();
    router.upstream.strobe.connect();
    router.upstream.clock.connect();
    for i in 0..6 {
        router.nodes[i].ready.connect();
        router.nodes[i].to_controller.connect();
    }
    router.connect_all();
    let vlog = generate_verilog(&router);
    println!("{}", vlog);
    yosys_validate("router", &vlog).unwrap();
}
