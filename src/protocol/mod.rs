pub struct ProtocolHandler {
    can_driver: CanDriver,
    xcp_handler: XcpHandler,
    uds_handler: UdsHandler,
}

impl ProtocolHandler {
    pub fn new() -> Self {
        Self {
            can_driver: CanDriver::new(),
            xcp_handler: XcpHandler::new(),
            uds_handler: UdsHandler::new(),
        }
    }

    pub fn handle_xcp_message(&mut self, msg: XcpMessage) -> Result<(), ProtocolError> {
        match msg.command {
            XcpCommand::Download => self.handle_download(msg),
            XcpCommand::Program => self.handle_program(msg),
            XcpCommand::Verify => self.handle_verify(msg),
            // Add other XCP commands
        }
    }

    pub fn handle_uds_message(&mut self, msg: UdsMessage) -> Result<(), ProtocolError> {
        match msg.service {
            UdsService::DiagnosticSessionControl => self.handle_diagnostic_session(msg),
            UdsService::SecurityAccess => self.handle_security_access(msg),
            // Add other UDS services
        }
    }
} 
