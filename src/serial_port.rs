use serialport::{available_ports, DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::time::Duration;

/// Keeps serial port configuration in one place and provides helper methods
/// for listing and opening device ports used by Rotrics hardware.
pub struct SerialPortManager {
    port_name: String,
    baud_rate: u32,
    data_bits: DataBits,
    flow_control: FlowControl,
    parity: Parity,
    stop_bits: StopBits,
    timeout: Duration,
}

impl SerialPortManager {
    /// Creates a manager with sensible defaults for CNC/robot control traffic.
    pub fn new(port_name: &str) -> Self {
        Self {
            port_name: port_name.to_string(),
            baud_rate: 115_200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(500),
        }
    }

    /// Returns serial device names that are visible to the current process.
    ///
    /// The UI can call this through the Tauri command and present the result to users.
    pub fn list_available_ports() -> Vec<String> {
        available_ports()
            .map(|ports| ports.into_iter().map(|port| port.port_name).collect())
            .unwrap_or_default()
    }

    /// Opens the configured serial port and returns a boxed serial port handle.
    pub fn open(&self) -> Result<Box<dyn SerialPort>, String> {
        serialport::new(&self.port_name, self.baud_rate)
            .data_bits(self.data_bits)
            .flow_control(self.flow_control)
            .parity(self.parity)
            .stop_bits(self.stop_bits)
            .timeout(self.timeout)
            .open()
            .map_err(|err| format!("failed to open {}: {err}", self.port_name))
    }

    /// Sends bytes to the provided serial connection.
    pub fn send(&self, port: &mut dyn SerialPort, data: &[u8]) -> Result<(), String> {
        use std::io::Write;
        port.write_all(data)
            .and_then(|_| port.flush())
            .map_err(|err| format!("failed to send data on {}: {err}", self.port_name))
    }

    /// Receives data from the provided serial connection.
    pub fn receive(&self, port: &mut dyn SerialPort) -> Result<Vec<u8>, String> {
        use std::io::Read;
        let mut buffer = vec![0_u8; 1024];
        let bytes_read = port
            .read(&mut buffer)
            .map_err(|err| format!("failed to read data on {}: {err}", self.port_name))?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }
}
