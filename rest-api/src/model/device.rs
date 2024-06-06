use paho_mqtt as mqtt;

struct DeviceBMC {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceStatusResponse {
    
}

impl DeviceBMC {

    pub async fn add_device(model: &ModelManager, req: AddDeviceRequest) -> Result<()> {

        todo!()
    }
}

