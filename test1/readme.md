#查看文档
rustup doc std::slice::Iter

# 调试

查看变量的地址：
1、在调试控制台上输入：p &arr 或 print &arr[0]

# 查看变量
frame variable arr

# 查看内存
memory read &arr

# 查看类型
whatis arr

# 查看更详细信息
p/x &arr        # 十六进制
p/d &arr        # 十进制  
p/t &arr        # 二进制

# 查看连续内存
x/3dw &arr      # 查看3个十进制字

