use super::*;
use std::{fs, iter};

impl DataFile {
    fn push_frame(
        &mut self,
        truck_id: &str,
        frame_data: Vec<Vec<String>>,
    ) -> Result<(), TruckError> {
        let mut truck_ref: Option<&mut Truck> = None;
        for truck in &mut self.trucks {
            if truck.name == truck_id {
                truck_ref = Some(truck);
                break;
            }
        }
        if let Some(truck) = truck_ref {
            truck.push_raw_frame(frame_data)?;
        } else {
            let mut truck = Truck::create_truck(truck_id);
            truck.push_raw_frame(frame_data)?;
            self.trucks.push(truck);
        }
        Ok(())
    }

    pub fn create_from_files(paths: &Vec<String>) -> Result<Self, DataParseError> {
        let mut current_frame = String::new();
        let mut frame_buffer = vec![];
        let mut data = Self { trucks: vec![] };

        for file_path in paths {
            for data_point in fs::read_to_string(&file_path)?
                .trim()
                .split("\n")
                .filter_map(|a| (a.len() > 0).then_some(a.trim().trim_matches('\u{feff}'))) // ensure BOM marker is stripped
                .skip_while(|a| !a.starts_with("T") || a.starts_with("Truck")) // start timmer
                .take_while(|a| a.starts_with("T"))
                .chain(iter::once(EOF_MARKER))
            // eof_marker ensures data at end is flushed as well
            {
                let mut entry = data_point.split(",");
                let Some(id) = entry.next() else {
                    return Err(DataParseError::MalformedEntry(data_point.to_string()));
                };

                if current_frame.len() == 0 {
                    // NOT len - this ensures that after file ends we push frame
                    current_frame = id.to_string();
                }

                if id != current_frame {
                    data.push_frame(&current_frame, Vec::from_iter(frame_buffer.drain(..)))
                        .map_err(|e| DataParseError::TruckError(current_frame.to_string(), e))?;

                    current_frame = id.to_string();
                }
                frame_buffer.push(Vec::from_iter(entry.map(|a| a.trim().to_string())));
            }
            current_frame.drain(..);
            frame_buffer.drain(..);
        }

        Ok(data)
    }
}
