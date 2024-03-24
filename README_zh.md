# 项目介绍
## 初步计划
基于risc-v芯片移植trezoe固件，后期对CPU内核也用开源CPU替代，实现全开源硬件钱包


初步移植选型的芯片为BL702：https://www.bouffalolab.com/product/?type=detail&id=4

# 项目排期
1. BootLoader移植（引导到Firmware_hello，带验签流程）
2. Firmware移植
    1. 启动代码 + PMP
    2. Crypto engine
3. 外设移植
    1. Flash driver
    2. Oled driver + Oled画面数据适配（屏幕尺寸有差异，需要适配）
    3. Buttom
