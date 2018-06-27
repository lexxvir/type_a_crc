pub fn crc16(data: &[u8]) -> u16 {
	let mut crc: u32 = 0x6363;

	for byte in data {
		let mut bt = *byte;

		bt = bt ^ (crc as u8);
		bt = bt ^ (bt << 4);

		crc = (crc >> 8) ^ (((bt as u32) << 8) ^ ((bt as u32) << 3) ^ ((bt as u32) >> 4));
	}

	(((crc & 0xFF) as u16) << 8) | (((crc >> 8) & 0xFF) as u16)
}

