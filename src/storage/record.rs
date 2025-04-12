use crate::configs::config::Config::PAGE_SIZE;

//Create an individual Record for data to be stored.
//Have Serialize and Deserialize functions
pub struct Record {
    pub total_size_of_fixed_fields: u16,
    pub no_of_variable_fields: u16,
    pub sizes_of_variable_fields: Vec<u16>,
    pub fixed_fields_data: Vec<u8>,
    pub variable_fields_data: Vec<u8>,
}
impl Record {
    pub fn serialize(&self, buffer: &mut Vec<u8>, starting_offset: usize) {
        let mut offset = starting_offset;

        buffer[offset..offset + 2].copy_from_slice(&self.total_size_of_fixed_fields.to_le_bytes());
        offset += 2;

        buffer[offset..offset + 2].copy_from_slice(&self.no_of_variable_fields.to_le_bytes());
        offset += 2;

        for size in &self.sizes_of_variable_fields {
            buffer[offset..offset + 2].copy_from_slice(&size.to_le_bytes());
            offset += 2;
        }

        buffer[offset..offset + self.fixed_fields_data.len()]
            .copy_from_slice(&self.fixed_fields_data);
        offset += self.fixed_fields_data.len();

        buffer[offset..offset + self.variable_fields_data.len()]
            .copy_from_slice(&self.variable_fields_data);
    }

    pub fn deserialize(buffer: &Vec<u8>, starting_offset: usize) -> Self {
        let mut offset = starting_offset;
    
        let total_size_of_fixed_fields =
            u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
        offset += 2;
    
        let no_of_variable_fields =
            u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
        offset += 2;
    
        let mut sizes_of_variable_fields: Vec<u16> =
            Vec::with_capacity(no_of_variable_fields as usize);
        for _ in 0..no_of_variable_fields {
            let size = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
            sizes_of_variable_fields.push(size);
            offset += 2;
        }
    
        let fixed_fields_data =
            buffer[offset..offset + total_size_of_fixed_fields as usize].to_vec();
        offset += total_size_of_fixed_fields as usize;
    
        let total_variable_size: usize = sizes_of_variable_fields.iter().map(|&s| s as usize).sum();
        let variable_fields_data = buffer[offset..offset + total_variable_size].to_vec();
    
        Self {
            total_size_of_fixed_fields,
            no_of_variable_fields,
            sizes_of_variable_fields,
            fixed_fields_data,
            variable_fields_data,
        }
    }
    
    pub fn new(
        total_size_of_fixed_fields: u16,
        no_of_variable_fields: u16,
        sizes_of_variable_fields: Vec<u16>,
        fixed_fields_data: Vec<u8>,
        variable_fields_data: Vec<u8>,
    ) -> Self {
        Self {
            total_size_of_fixed_fields,
            no_of_variable_fields,
            sizes_of_variable_fields,
            fixed_fields_data,
            variable_fields_data,
        }
    }
}
