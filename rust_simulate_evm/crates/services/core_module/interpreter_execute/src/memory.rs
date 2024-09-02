use lib_utils::error::RunnerError;

#[derive(Debug)]
pub struct Memory {
    pub heap: Vec<u8>,
}

impl Memory {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        Self {
            heap: data.unwrap_or_default(),
        }
    }
    //扩展向量到指定的大小 填充0
    pub fn extend(&mut self, size: usize) {
        self.heap.resize(self.heap.len() + size, 0);
    }

    // 读取32个字节从指定位置
    pub fn mload(&mut self, offset: usize) -> Result<[u8; 32], RunnerError> {
        //计算偏移量
        if offset + 32 > self.heap.len() {
            let nearest_multiple = if offset % 32 == 0 {
                offset + 32
            } else {
                (offset + 32) + (32 - (offset + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.heap.len());
        }

        let mut result = [0u8; 32];
        /* 使用切片避开原始指针unsafe */
        result.copy_from_slice(&self.heap[offset..offset + 32]);

        Ok(result)
    }

    //返回当前内存堆（heap）的大小
    pub fn msize(&self) -> usize {
        self.heap.len()
    }

    //在heap指定offset位置写入data
    pub fn mstore(&mut self, offset: usize, data: [u8; 32]) -> Result<(), RunnerError> {
        //check heap size be satisfying
        if offset + 32 > self.heap.len() {
            self.extend(offset + 32 - self.heap.len());
        }
        self.heap[offset..offset + 32].copy_from_slice(&data);

        Ok(())
    }

    //from offset copy memory size to dest_offset
    pub fn mcopy(&mut self, dest_offset: usize, offset: usize, size: usize) -> Result<(), RunnerError> {
        if size == 0 {
            return Ok(());
        }

        let required_size = std::cmp::max(dest_offset + size, offset + size);
        if required_size > self.heap.len() {
            self.extend(required_size - self.heap.len());
        }

        // 使用临时缓冲区来防止源和目标区域重叠时的数据损坏
        let mut buffer = vec![0u8; size];
        buffer.copy_from_slice(&self.heap[offset..offset + size]);
        self.heap[dest_offset..dest_offset + size].copy_from_slice(&buffer);

        Ok(())
    }

    // 读取指定数量的字节从指定位置
    pub fn read(&mut self, offset: usize, size: usize) -> Result<Vec<u8>, RunnerError> {
        // 如果偏移量加上读取的大小超过了当前内存的长度，计算最近的32字节倍数并扩展内存
        if offset + size > self.heap.len() {
            let nearest_multiple = if offset % 32 == 0 {
                offset + size // 若偏移量正好是32的倍数，直接加上大小
            } else {
                (offset + size) + (32 - (offset + size) % 32) // 否则，计算下一个32字节的倍数
            };

            // 扩展内存堆到最近的32字节倍数
            self.extend(nearest_multiple - self.heap.len());
        }

        // 创建一个动态大小的Vec用于存储读取的数据
        let mut result = vec![0u8; size];
        // 使用切片从内存中复制数据到结果中
        result.copy_from_slice(&self.heap[offset..offset + size]);

        Ok(result)
    }

    // 指定数量的字节写入到指定位置
    pub fn write(&mut self, offset: usize, data: Vec<u8>) -> Result<(), RunnerError> {
        // 如果偏移量加上写入的大小超过了当前内存的长度，计算最近的32字节倍数并扩展内存
        if offset + data.len() > self.heap.len() {
            let nearest_multiple = if offset % 32 == 0 {
                offset + data.len() + 32
            } else {
                (offset + data.len() + 32 ) + (32 - (offset + data.len() + 32 ) % 32) // 否则，计算下一个32字节的倍数
            };

            // 扩展内存堆到最近的32字节倍数
            self.extend(nearest_multiple - self.heap.len());
        }

        // 使用切片从内存中复制数据到结果中
        self.heap[offset..offset + data.len()].copy_from_slice(&data);

        Ok(())
    }
}

impl Clone for Memory {
    fn clone(&self) -> Self {
        Memory {
            heap: self.heap.clone(),
        }
    }
}