//! USB TPM protocol implementation.
//!
//! This module implements the `usb-test-protocol` for communicating with a TPM.

#![no_std]

use aligned::{Aligned, A4};
use hal_usb::driver::{UsbDriver, UsbEvent, UsbPacket};
use hal_usb::{
    Direction, EndpointDescriptor, InterfaceDescriptor, Recipient, Request, RequestType,
    SetupPacket, StringHandle, TransferType,
};
use usb_driver::{EpIn, EpOut};
use usb_stack::{Transfer, UsbAction, UsbClass, EMPTY};
use zerocopy::{FromBytes, Immutable, IntoBytes};

/// USB TPM specific constants.
pub const USB_CLASS_VENDOR: u8 = 0xFF;
pub const USB_SUBCLASS_TPM: u8 = 0xCF;
pub const USB_PROTOCOL_TPM: u8 = 0x66;

/// FourCC constants.
pub const FOURCC_TPM: u32 = 0x5F4D5054; // "TPM_"
pub const FOURCC_REQ: u32 = 0x5145525F; // "_REQ"
pub const FOURCC_RSP: u32 = 0x5053525F; // "_RSP"

/// Command codes.
pub const MS_SIM_TPM_SEND_COMMAND: u32 = 8;
pub const TPM_SESSION_END: u32 = 20;

/// Status codes.
pub const STATUS_SUCCESS: u32 = 0;
pub const STATUS_ERR_UNKNOWN_CMD: u32 = 12;
#[allow(dead_code)]
pub const STATUS_ERR_INTERNAL: u32 = 13;
#[allow(dead_code)]
pub const STATUS_ERR_BUSY: u32 = 14;

/// Vendor requests.
pub const REQ_SET_LOCALITY: Request = Request::new(
    Direction::HostToDevice,
    RequestType::Vendor,
    Recipient::Interface,
    0x01,
);
pub const REQ_CANCEL: Request = Request::new(
    Direction::HostToDevice,
    RequestType::Vendor,
    Recipient::Interface,
    0x02,
);

/// TPM command header.
#[repr(C, packed)]
#[derive(IntoBytes, FromBytes, Immutable, Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct UsbTpmHeader {
    pub header: u32,
    pub identifier: u32,
    pub command_status: u32,
    pub length: u32,
}

/// A trait for implementing TPM actions.
pub trait TpmDevice {
    /// Executes a raw TPM command.
    fn execute_command(&mut self, locality: u8, command: &[u8], response: &mut [u8]) -> usize;
    /// Signals the device to abort the current TPM command.
    fn cancel(&mut self);
}

/// A builder for UsbTpm class configuration.
#[derive(Copy, Clone)]
pub struct UsbTpmBuilder {
    pub interface_num: u8,
    pub bulk_in_ep: u8,
    pub bulk_out_ep: u8,
}

impl UsbTpmBuilder {
    /// Creates a new UsbTpm configuration.
    pub const fn new(interface_num: u8, bulk_in_ep: u8, bulk_out_ep: u8) -> Self {
        Self {
            interface_num,
            bulk_in_ep,
            bulk_out_ep,
        }
    }

    /// Returns the endpoints for the interface.
    pub const fn endpoints(&self) -> [EndpointDescriptor; 2] {
        [
            EndpointDescriptor {
                direction: Direction::HostToDevice,
                endpoint_num: self.bulk_out_ep,
                interval: 0,
                max_packet_size: 64,
                transfer_type: TransferType::Bulk,
            },
            EndpointDescriptor {
                direction: Direction::DeviceToHost,
                endpoint_num: self.bulk_in_ep,
                interval: 0,
                max_packet_size: 64,
                transfer_type: TransferType::Bulk,
            },
        ]
    }

    /// Constructs the UsbTpm interface descriptor.
    pub const fn interface(
        &self,
        name: StringHandle,
        endpoints: &'static [EndpointDescriptor],
    ) -> InterfaceDescriptor {
        InterfaceDescriptor {
            name,
            interface_number: self.interface_num,
            alternate_setting: 0,
            interface_class: USB_CLASS_VENDOR,
            interface_sub_class: USB_SUBCLASS_TPM,
            interface_protocol: USB_PROTOCOL_TPM,
            func_descs: &[],
            endpoints,
        }
    }

    /// Returns the hardware endpoint configuration.
    pub const fn eps(&self) -> ([EpIn; 1], [EpOut; 1]) {
        (
            [EpIn {
                num: self.bulk_in_ep,
                buf_pool_size: 1,
            }],
            [EpOut {
                num: self.bulk_out_ep,
                set_nak: false,
            }],
        )
    }
}

/// UsbTpm state.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum State {
    Idle,
    ReceivingPayload,
    SendingHeader,
    SendingPayload,
}

/// UsbTpm class handler.
pub struct UsbTpm<'a, T: TpmDevice, const PAYLOAD_WORDS: usize> {
    config: UsbTpmBuilder,
    device: &'a mut T,
    state: State,
    locality: u8,
    req_header: UsbTpmHeader,
    payload_transfer: Transfer<PAYLOAD_WORDS>,
    response_buf: Aligned<A4, [u32; PAYLOAD_WORDS]>,
    response_len: usize,
    tx_offset: usize,
    status: u32,
}

impl<'a, T: TpmDevice, const PAYLOAD_WORDS: usize> UsbTpm<'a, T, PAYLOAD_WORDS> {
    /// Creates a new UsbTpm class handler.
    pub fn new(config: UsbTpmBuilder, device: &'a mut T) -> Self {
        Self {
            config,
            device,
            state: State::Idle,
            locality: 0,
            req_header: UsbTpmHeader::default(),
            payload_transfer: Transfer::new(),
            response_buf: Aligned([0u32; PAYLOAD_WORDS]),
            response_len: 0,
            tx_offset: 0,
            status: STATUS_SUCCESS,
        }
    }

    fn handle_setup(&mut self, pkt: SetupPacket) -> (UsbAction<'static>, bool) {
        if !(pkt.request().recipient() == Recipient::Interface
            && (pkt.index() as u8) == self.config.interface_num)
        {
            return (UsbAction::None, false);
        }

        match pkt.request() {
            REQ_SET_LOCALITY => {
                self.locality = pkt.value() as u8;
                (
                    UsbAction::TransferIn {
                        endpoint: 0,
                        data: EMPTY,
                        zlp: true,
                    },
                    true,
                )
            }
            REQ_CANCEL => {
                self.device.cancel();
                (
                    UsbAction::TransferIn {
                        endpoint: 0,
                        data: EMPTY,
                        zlp: true,
                    },
                    true,
                )
            }
            _ => (UsbAction::None, false),
        }
    }

    fn process_payload_inner(
        device: &mut T,
        locality: u8,
        command_status: u32,
        payload: &[u8],
        response_buf: &mut [u8],
    ) -> (u32, usize) {
        //pw_log::info!("Processing {} in locality {}", command_status as u32, locality as u8);
        //hexdump(payload);
        match command_status {
            MS_SIM_TPM_SEND_COMMAND => {
                let len = device.execute_command(locality, payload, response_buf);
                (STATUS_SUCCESS, len)
            }
            TPM_SESSION_END => (STATUS_SUCCESS, 0),
            _ => (STATUS_ERR_UNKNOWN_CMD, 0),
        }
    }

    fn process_payload(&mut self, payload: &[u8]) {
        let (status, resp_len) = Self::process_payload_inner(
            self.device,
            self.locality,
            self.req_header.command_status,
            payload,
            self.response_buf.as_mut_bytes(),
        );
        self.status = status;
        self.response_len = resp_len;
        self.state = if self.req_header.command_status == TPM_SESSION_END {
            // We don't send any reply or even response header for session end.
            State::Idle
        } else { State::SendingHeader} ;
        self.tx_offset = 0;
        //pw_log::info!("TPM Response: status={} len={}", self.status as u32, self.response_len as u32);
        //hexdump(&self.response_buf.as_bytes()[..self.response_len]);
    }

    /// Polls the state and performs necessary actions.
    pub fn poll_transmit<D: UsbDriver>(&mut self, driver: &mut D) {
        match self.state {
            State::SendingHeader => {
                let header = UsbTpmHeader {
                    header: FOURCC_TPM,
                    identifier: FOURCC_RSP,
                    command_status: self.status,
                    length: self.response_len as u32,
                };
                let bytes = header.as_bytes();
                let n = driver.transfer_in_unaligned(
                    self.config.bulk_in_ep,
                    &bytes[self.tx_offset..],
                    true,
                );
                self.tx_offset += n;
                if self.tx_offset == 16 {
                    self.tx_offset = 0;
                    if self.response_len > 0 {
                        self.state = State::SendingPayload;
                    } else {
                        self.state = State::Idle;
                    }
                }
            }
            State::SendingPayload => {
                let data = &self.response_buf.as_bytes()[self.tx_offset..self.response_len];
                let n = driver.transfer_in_unaligned(self.config.bulk_in_ep, data, true);
                self.tx_offset += n;
                if self.tx_offset == self.response_len {
                    self.state = State::Idle;
                    self.tx_offset = 0;
                }
            }
            _ => {}
        }
    }
}

impl<'a, T: TpmDevice, const PAYLOAD_WORDS: usize> UsbClass for UsbTpm<'a, T, PAYLOAD_WORDS> {
    fn handle_event<'b, P: UsbPacket>(
        &'b mut self,
        event: UsbEvent<P>,
    ) -> Result<UsbAction<'b>, UsbEvent<P>> {
        match event {
            UsbEvent::SetupPacket { pkt, endpoint } if endpoint == 0 => {
                let (action, claimed) = self.handle_setup(pkt);
                if action != UsbAction::None || claimed {
                    Ok(action)
                } else {
                    Err(UsbEvent::SetupPacket { pkt, endpoint })
                }
            }
            UsbEvent::DataOutPacket(pkt) => {
                if pkt.endpoint_index() == self.config.bulk_out_ep as usize {
                    match self.state {
                        State::Idle => {
                            let mut tmp = [0u8; 16];
                            let buf = pkt.copy_to_unaligned(&mut tmp);
                            if buf.len() == 16 {
                                if let Ok(header) = UsbTpmHeader::read_from_bytes(buf) {
                                    if header.header == FOURCC_TPM
                                        && header.identifier == FOURCC_REQ
                                    {
                                        self.req_header = header;
                                        if header.length > 0 {
                                            self.state = State::ReceivingPayload;
                                        } else {
                                            self.process_payload(&[]);
                                        }
                                        return Ok(UsbAction::None);
                                    }
                                }
                            }
                            // Invalid header. Spec says we should scan for FOURCC_TPM.
                            // For now we just stall.
                            Ok(UsbAction::StallInAndOut {
                                endpoint: self.config.bulk_out_ep,
                            })
                        }
                        State::ReceivingPayload => {
                            let payload_len = self.req_header.length as usize;
                            let mut payload_complete = false;
                            match self.payload_transfer.splice(pkt) {
                                Ok(Some(data)) => {
                                    if data.len() >= payload_len {
                                        payload_complete = true;
                                    }
                                }
                                Ok(None) => {}
                                Err(_) => {
                                    self.state = State::Idle;
                                }
                            }
                            if payload_complete {
                                let payload_len = self.req_header.length as usize;
                                let command_status = self.req_header.command_status;
                                let locality = self.locality;
                                let payload =
                                    &self.payload_transfer.buffer.as_bytes()[..payload_len];

                                let (status, resp_len) = Self::process_payload_inner(
                                    self.device,
                                    locality,
                                    command_status,
                                    payload,
                                    self.response_buf.as_mut_bytes(),
                                );
                                self.status = status;
                                self.response_len = resp_len;
                                self.state = State::SendingHeader;
                                self.tx_offset = 0;
                                //pw_log::info!("TPM Response: status={} len={}", self.status as u32, self.response_len as u32);
                                //hexdump(&self.response_buf.as_bytes()[..self.response_len]);
                            }
                            Ok(UsbAction::None)
                        }
                        _ => Ok(UsbAction::None),
                    }
                } else {
                    Err(UsbEvent::DataOutPacket(pkt))
                }
            }
            _ => Err(event),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hal_usb::SetupPacket;
    use usb_stack::testing::FakeUsbPacket;

    struct MockTpm {
        last_locality: u8,
        cancelled: bool,
    }

    impl TpmDevice for MockTpm {
        fn execute_command(
            &mut self,
            locality: u8,
            _command: &[u8],
            _response: &mut [u8],
        ) -> usize {
            self.last_locality = locality;
            0
        }
        fn cancel(&mut self) {
            self.cancelled = true;
        }
    }

    #[test]
    fn test_usb_tpm_header_zerocopy() {
        let header = UsbTpmHeader {
            header: FOURCC_TPM,
            identifier: FOURCC_REQ,
            command_status: MS_SIM_TPM_SEND_COMMAND,
            length: 123,
        };
        let bytes = header.as_bytes();
        assert_eq!(bytes.len(), 16);
        let header2 = UsbTpmHeader::read_from_bytes(bytes).unwrap();
        assert_eq!(header, header2);
    }

    #[test]
    fn test_state_transitions() {
        let mut mock_tpm = MockTpm {
            last_locality: 0,
            cancelled: false,
        };
        let builder = UsbTpmBuilder::new(1, 0x81, 1);
        let mut tpm = UsbTpm::<_, 32>::new(builder, &mut mock_tpm);
        assert_eq!(tpm.state, State::Idle);

        // 1. Receive Header
        let header = UsbTpmHeader {
            header: FOURCC_TPM,
            identifier: FOURCC_REQ,
            command_status: MS_SIM_TPM_SEND_COMMAND,
            length: 4,
        };
        let pkt = FakeUsbPacket {
            data: header.as_bytes(),
            ep: 1,
        };
        if let Err(_) = tpm.handle_event::<FakeUsbPacket>(UsbEvent::DataOutPacket(pkt)) {
            panic!("handle_event failed");
        }
        assert_eq!(tpm.state, State::ReceivingPayload);

        // 2. Receive Payload
        let payload = [0x11, 0x22, 0x33, 0x44];
        let pkt = FakeUsbPacket {
            data: &payload,
            ep: 1,
        };
        if let Err(_) = tpm.handle_event::<FakeUsbPacket>(UsbEvent::DataOutPacket(pkt)) {
            panic!("handle_event failed");
        }
        assert_eq!(tpm.state, State::SendingHeader);
        assert_eq!(tpm.status, STATUS_SUCCESS);
        assert_eq!(tpm.response_len, 0);
    }

    #[test]
    fn test_set_locality() {
        let mut mock_tpm = MockTpm {
            last_locality: 0,
            cancelled: false,
        };
        let builder = UsbTpmBuilder::new(1, 0x81, 1);
        let mut tpm = UsbTpm::<_, 32>::new(builder, &mut mock_tpm);

        // Word 0: 0x03 (locality) << 16 | 0x01 (bRequest) << 8 | 0x41 (bmRequestType)
        // Word 1: 0x00 (wLength) << 16 | 0x01 (interface_num)
        let pkt = SetupPacket::new([0x03_01_41, 0x00_01]);
        let action =
            match tpm.handle_event::<FakeUsbPacket>(UsbEvent::SetupPacket { pkt, endpoint: 0 }) {
                Ok(action) => action,
                Err(_) => panic!("handle_event failed"),
            };

        assert!(matches!(action, UsbAction::TransferIn { .. }));
        assert_eq!(tpm.locality, 3);
    }

    #[test]
    fn test_cancel() {
        let mut mock_tpm = MockTpm {
            last_locality: 0,
            cancelled: false,
        };
        let builder = UsbTpmBuilder::new(1, 0x81, 1);
        let mut tpm = UsbTpm::<_, 32>::new(builder, &mut mock_tpm);

        // Word 0: 0x00 (value) << 16 | 0x02 (bRequest) << 8 | 0x41 (bmRequestType)
        // Word 1: 0x00 (wLength) << 16 | 0x01 (interface_num)
        let pkt = SetupPacket::new([0x00_02_41, 0x00_01]);
        let action =
            match tpm.handle_event::<FakeUsbPacket>(UsbEvent::SetupPacket { pkt, endpoint: 0 }) {
                Ok(action) => action,
                Err(_) => panic!("handle_event failed"),
            };

        assert!(matches!(action, UsbAction::TransferIn { .. }));
        assert!(mock_tpm.cancelled);
    }

    #[test]
    fn test_unknown_command() {
        let mut mock_tpm = MockTpm {
            last_locality: 0,
            cancelled: false,
        };
        let builder = UsbTpmBuilder::new(1, 0x81, 1);
        let mut tpm = UsbTpm::<_, 32>::new(builder, &mut mock_tpm);

        let header = UsbTpmHeader {
            header: FOURCC_TPM,
            identifier: FOURCC_REQ,
            command_status: 99, // Unknown
            length: 0,
        };
        let pkt = FakeUsbPacket {
            data: header.as_bytes(),
            ep: 1,
        };
        if let Err(_) = tpm.handle_event::<FakeUsbPacket>(UsbEvent::DataOutPacket(pkt)) {
            panic!("handle_event failed");
        }
        assert_eq!(tpm.state, State::SendingHeader);
        assert_eq!(tpm.status, STATUS_ERR_UNKNOWN_CMD);
    }
}
