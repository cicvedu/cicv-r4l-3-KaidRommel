// SPDX-License-Identifier: GPL-2.0

//! Rust for commpletion.
use core::result::Result::Err;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x1000;

module! {
    type: RustChrdev,
    name: "completion",
    author: "KaidRommel",
    description: "Rust character device",
    license: "GPL",
}

static GLOBALMEM_BUF: Mutex<[u8;GLOBALMEM_SIZE]> = unsafe {
    // 创建一个长度为GLOBALMEM_SIZE（4096字节）的字节数据类型为u8数组，每个元素都被初始化为0
    Mutex::new([0u8;GLOBALMEM_SIZE])
};

struct RustFile {
    #[allow(dead_code)]
    inner: &'static Mutex<[u8;GLOBALMEM_SIZE]>,
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        Ok(
            Box::try_new(RustFile {
                inner: &GLOBALMEM_BUF
            })?
        )
    }

    fn write(this: &Self,_file: &file::File,reader: &mut impl kernel::io_buffer::IoBufferReader,offset:u64,) -> Result<usize> {
        let offset = offset.try_into()?;
        let mut vec = this.inner.lock();
        // 计算要写入的数据长度，取 reader 中数据的长度和 vec 中从 offset 开始的剩余空间的最小值
        let len = core::cmp::min(reader.len(), vec.len().saturating_sub(offset));

        // 从 reader 中读取数据，并写入到 vec 中从 offset 开始的位置，长度为 len。如果读取失败，函数会返回错误
        reader.read_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }

    fn read(this: &Self,_file: &file::File,writer: &mut impl kernel::io_buffer::IoBufferWriter,offset:u64,) -> Result<usize> {
        let offset = offset.try_into()?;
        let mut vec = this.inner.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
