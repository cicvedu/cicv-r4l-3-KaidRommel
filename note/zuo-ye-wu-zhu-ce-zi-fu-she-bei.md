# 作业五：注册字符设备   
# 编写 write/read 函数   
```
fn write(this: &Self,_file: &file::File,reader: &mut impl kernel::io_buffer::IoBufferReader,offset:u64,) -> Result<usize> {
        let offset = offset.try_into()?;
        let mut vec = this.inner.lock();
        // 计算要写入的数据长度，取 reader 中数据的长度和 vec 中从 offset 开始的剩余空间的最小值
        let len = core::cmp::min(reader.len(), vec.len().saturating_sub(offset));

        // 从 reader 中读取数据，并写入到 vec 中从 offset 开始的位置，长度为 len。如果读取失败，函数会返回错误
        reader.read_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }

    fn read(this: &Self,_file: &file::File,writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
        let offset = offset.try_into()?;
        let vec = this.inner.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }
```
# 编译   
```
make menuconfig
```
选中 `Character device` ，按 `Y` 编译进内核：   
```
Kernel hacking
  ---> Sample Kernel code
      ---> Rust samples
              ---> <*>Character device (NEW)

```
编译内核：   
```
make LLVM=1 -j$(nproc)
```
# 测试   
进入虚拟环境：   
```
./build_image.sh
```
![image.png](files\image_e.png)    
# Question:   
- **作业5中的字符设备 `/dev/cicv` 是怎么创建的？它的设备号是多少**？**它是如何与我们写的字符设备驱动关联上的？**   
    设备文件 `/dev/cicv` 通过设备号与字符设备驱动关联。当应用程序对 `/dev/cicv` 进行读写操作时，内核会通过设备号将这些操作路由到对应的字符设备驱动。在驱动代码中，使用 `chrdev::Registration` 注册字符设备，并指定了设备名称和设备号。   
    ```
let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;
```
    这段代码注册了一个名为 `name` 的字符设备，并指定了次设备号为 0。内核会为该设备分配一个主设备号。当加载驱动模块后，通过 `mknod` 命令创建的设备文件 `/dev/cicv` 就会与该驱动关联。当应用程序对 `/dev/cicv` 进行读写操作时，内核会调用驱动中的 `read` 和 `write` 函数处理这些操作。   
