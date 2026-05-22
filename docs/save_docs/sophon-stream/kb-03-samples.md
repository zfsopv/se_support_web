# Sophon-Stream 示例应用 (Samples) 参考文档

> 本文档由 samples/ 目录下所有中文 README.md 合并而成。
> 生成时间: 2026-05-22 15:04:00

## samples 总览

# sophon-stream samples说明

[English](README_EN.md) | 简体中文

## 1. 简介

本目录下包含sophon-stream提供的参考例程。

本目录结构如下所示：

```bash
./samples/
├── bytetrack                                       # 检测+跟踪例程
├── CMakeLists.txt                                  # cmake文件
├── include                                         # 绘图函数等头文件
├── license_plate_recognition                       # 车辆检测+车牌识别例程
├── openpose                                        # 姿态识别例程
├── README.md                                       # 用户手册
├── README_EN.md                                    # 用户手册_英文版
├── resnet                                          # 分类例程
├── retinaface                                      # 人脸检测例程
├── retinaface_distributor_resnet_faiss_converger   # 人脸检测+人脸识别例程
├── src                                             # 唯一的入口函数
├── yolov5                                          # yolov5检测例程
├── yolov5_bytetrack_distributor_resnet_converger   # 检测+跟踪+识别例程
├── yolox                                           # yolox检测例程
└── yolox_bytetrack_osd_encode                      # 检测+跟踪+画图+推流例程
```

本目录包含了多个例程，如人脸检测、车牌识别等。每个例程的目录下都包含配置文件和下载模型、视频等数据的脚本。对于各个例程而言，它们共用同一个入口函数，即`sophon-stream/samples/src/main.cc`。该入口函数主要作用是根据预设的路径，获取配置文件，解析例程的输入数据，并调用统一的底层接口配置所有的element，启动整个pipeline。

例如，为了运行`yolov5_bytetrack_distributor_resnet_converger`demo:

```bash
./main --demo_config_path=../yolov5_bytetrack_distributor_resnet_converger/config/yolov5_bytetrack_distributor_resnet_converger_demo.json
```

此时，main函数会前往`yolov5_bytetrack_distributor_resnet_converger`例程目录下寻找配置文件，并根据其配置文件来搭建pipeline。此时，运行起来的pipeline就具有`检测+跟踪+识别`的功能。

或者，为了运行`license_plate_recognition`demo：

```bash
./main --demo_config_path=../license_plate_recognition/config/license_plate_recognition_demo.json
```

此种情况下，main函数则会前往`license_plate_recognition`例程目录下寻找配置文件。按照该配置文件搭建起来的pipeline，则具有`车辆检测+车牌识别`的功能。

这样设计的好处是隔离了用户与底层代码，使用户可以只关注于json文件的配置，而不需要考虑stream框架的运作逻辑。


> 注意
* 关于每个例程的详细信息，请参考每个例程目录下的README.md文件。

## 2. 配置参数
sophon-stream demo的输入具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：


```json
  "channels": [
    {
      "channel_id": 2,
      "url": "../data/videos/mot17_01_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "skip_element": [5005, 5006],
      "fps": 1,
      "sample_interval": 5,
      "sample_strategy": "KEEP",
      "roi":{
        "left": 0,
        "top": 0,
        "width": 800,
        "height": 600
      }
    },
    {
      "channel_id": 3,
      "url": "../data/videos/mot17_03_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 20,
      "url": "../data/videos/mot17_06_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 30,
      "url": "../data/videos/mot17_08_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    }
  ]
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
| channel_id | 整数   | 无 | 输入数据通道编号 |
|   url      | 字符串 | 无 | 输入数据路径，包括本地视频、图片、视频流和base64对应url后缀 |
|source_type | 字符串  | 无  | 输入数据类型，"RTSP"代表RTSP视频流，“RTMP”代表RTMP视频流，“GB28181”代表GB28181视频流，“VIDEO”代表本地视频，“IMG_DIR”代表图片文件夹， “BASE64”代表base64数据 |
|sample_interval | 整数  | 1  |抽帧数，如设置为5，表示每5帧有1帧会被后续处理，即为ObjectMata mFilter字段为false|
|loop_num | 整数  | 1  | 循环次数，仅适用于source_type为"VIDEO"和“IMG_DIR”，值为0时无限循环|
|fps | 浮点数  | 30 | 用于控制视频流的fps，fps=-1表示不控制fps；其它情况下，source_type为"IMG_DIR"或"BASE64"时由设置的值决定，其他source_type从视频流读取fps，设置的值不生效|
|base64_port | 整数  | 12348 | base64对应http端口 |
|skip_element| list | 无 | 设置该路数据是否跳过某些element，目前只对osd和encode生效。不设置时，认为不跳过任何element|
|sample_strategy|字符串|"DROP"|在有抽帧的情况下，设置被抽掉的帧是保留还是直接丢弃。"DROP"表示丢弃，"KEEP"表示保留|
|decode_id|整数|-1|单个decode element的情况不需要填写；多个decode element情况下，标识了某一路由对应id的decode element进行解码|
|roi|字典|无|设置ROI时，将把解码结果进行裁剪并向下传递；否则默认传递原图|
|graph_id| 整数 | 0 | 当前码流归属的Graph Id |


其中，channel_id为输入视频的通道编号，与[编码器](../element/multimedia/encode/README.md)输出channel_id相对应。例如，输入channel_id为20，使用编码器保存结果为本地视频时，文件名为20.avi。

一个图片文件夹表示一个视频，按frame_id命名，例如
```bash
IMG_DIR/
  ├── ****1.jpg
  ├── ****2.jpg
  ├── ****3.jpg
  ├── ****4.jpg
  ├── ****5.jpg
  ............
  └──******.jpg
```

> **注意**：
>1. 输入RTSP数据流的URL须以`rtsp://`开头
>2. 输入RTMP数据流的URL须以`rtmp://`开头
>3. 假设输入BASE64的URL为`/base64`，则http请求的格式需为「POST」(http://{host_ip}:{base64_port}/base64)，request body的data字段存储base64数据，如{"data": "{base64 string，不含头部(data:image/xxx;base64,)}"}
>4. 输入GB28181数据流的URL须以`gb28181://`开头

需要注意输入数据http_report/http_listen的设置，以[license_plate_recognition](license_plate_recognition/config/license_plate_recognition_demo.json)为例

```json
    "http_report": {
        "ip": "0.0.0.0",
        "port": 10001,
        "path": "/flask_test/"
    },
    "http_listen": {
        "ip": "0.0.0.0",
        "port": 8000,
        "path": "/task/test"
    },
```
|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
| ip | 字符串   | http_listen默认为"0.0.0.0"，http_report默认无| 上报/监听的ip地址，report时上报请求到此ip，listen时监听此ip的post请求 |
| port | 整数 | http_listen默认为8000，http_report默认无 | 上报/监听的端口号，report时上报请求到此port，listen时监听此port的post请求 |
|path | 字符串  | http_listen默认为"/task/test"，http_report默认无  | 上报/监听的路由，report时上报请求到此path，listen时监听此path的post请求。对于监听请求来说，此字段留空即可 |

> **注意**：
>1. http_report字段必须完整，否则不会进行上报，默认不上报。

## 3. 功能

samples支持通过http请求动态增加或停止一路码流的功能。在设置了上面的`http_listen`字段之后，可以使用如下代码增加一路码流：

```python
# add a channel
url = "http://localhost:8000/stream/addChannel"
payload = {"channel_id": 3,"url": "../yolov5/data/videos/test_car_person_1080P.avi","source_type": "VIDEO","sample_interval": 1,"loop_num": 1,"fps": 1}
headers = {'Content-Type': 'application/json'}
response = requests.request("POST", url, headers=headers, data=json.dumps(payload))
print(response)

# stop a channel
url = "http://localhost:8000/stream/stopChannel"
payload = {"channel_id": 3}
headers = {'Content-Type': 'application/json'}
response = requests.request("POST", url, headers=headers, data=json.dumps(payload))
print(response)
```

需要注意，目前bytetrack、osd、encode、http_push等插件的工作示例数量是依照预设的线程数来分配。因此，使用动态增加码流功能时，需要保证前述插件的线程数大于等于增加后的码流总数。
---

# 各示例详细文档

---

## bird_dwa_blend_encode

# dwa_blend_encode Demo

## 目录
- [dwa\_blend\_encode Demo](#dwa_blend_encode-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备数据](#3-准备数据)
  - [4. 环境准备](#4-环境准备)
    - [4. SoC平台](#4-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 SoC平台](#51-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)
  - [8. web ui使用](#8-web-ui使用)
    - [8.1 安装nodejs](#81-安装nodejs)
    - [8.2 web ui编译](#82-web-ui编译)
    - [8.3 运行web ui](#83-运行web-ui)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建鸟瞰应用。

本例程中，鸟瞰拼接算法的鱼眼展开分别在四个element上进行运算，鸟瞰拼接在三个element上运算。

## 2. 特性

* 支持BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备数据

​在`scripts`目录下提供了相关数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。
```bash
.
├── gridinfo
│   ├── 0grid_info_bev_60_60_2920_60_60_dst_1920x1920_src_1920x1080.dat
│   ├── 1grid_info_bev_60_60_2381_60_60_dst_1920x1920_src_1920x1080.dat
│   ├── 2grid_info_bev_60_60_2772_60_60_dst_1920x1920_src_1920x1080.dat
│   ├── 3grid_info_bev_60_60_2904_60_60_dst_1920x1920_src_1920x1080.dat
│   ├── Dgrid_info_64_16_1024_64_16_dst_2048x512_src_992x3072.dat
│   ├── Lgrid_info_16_96_1536_16_96_dst_512x3072_src_2240x2240.dat
│   ├── Lgrid_info_68_68_4624_70_70_dst_2240x2240_src_2240x2240.dat
│   ├── Rgrid_info_16_96_1536_16_96_dst_512x3072_src_2240x2240.dat
│   ├── Rgrid_info_68_67_4556_70_70_dst_2240x2240_src_2240x2240.dat
│   └── Ugrid_info_64_16_1024_64_16_dst_2048x512_src_992x3072.dat
├── images
│   ├── 0
│   │   └── sensor00.jpg
│   ├── 1
│   │   └── sensor1.jpg
│   ├── 2
│   │   └── sensor2.jpg
│   ├── 3
│   │   └── sensor3.jpg
│   ├── left
│   │   └── dc_src_2240x2240_L.png
│   ├── right
│   │   └── dc_src_2240x2240_R.png
│   ├── v2l
│   │   └── dc_srcl.jpg
│   └── v2r
│       └── dc_src.jpg
├── param
│   ├── 04E10_fisheye_dual.bin
│   ├── 04a10_zhiyuan_zhang.bin
│   ├── 04a10_zhiyuan_zhang01-12.bin
│   ├── A2_04A10_linear_rgb.bin
│   ├── cvi_sdr_bin
│   ├── cvi_sdr_bin16.bin
│   ├── duan-ce-04a10.bin
│   ├── os04a10-sdr-test.bin
│   ├── zbcv.bin
│   └── zhiyuan.zhang-04a10-2.bin
├── test_img
│   ├── 1920x1080.jpg
│   ├── dc_src_2240x2240_L.png
│   ├── dc_src_2240x2240_L.yuv
│   ├── dc_src_2240x2240_R.png
│   ├── dc_src_2240x2240_R.yuv
│   ├── sensor0.ppm
│   ├── sensor1.ppm
│   ├── sensor2.ppm
│   └── sensor3.ppm
└── wgt
    ├── alpha_weight_0.bin
    ├── alpha_weight_1.bin
    ├── alpha_weight_2.bin
    ├── beta_weight_0.bin
    ├── beta_weight_1.bin
    ├── beta_weight_2.bin
    ├── c01_alpha_444p_m2__0_2240x32.bin
    ├── c01_alpha_444p_m2__0_3072x32.bin
    ├── c01_beta_444p_m2__0_2240x32.bin
    └── c01_beta_444p_m2__0_3072x32.bin
```

## 4. 环境准备

### 4. SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。
(1) 安装驱动
安装驱动需要进入到超级权限，接着系统驱动目录，安装驱动：

```bash

sudo -s
insmod /mnt/system/ko/v4l2_pr2100.ko force_bus=1,1,1,1,-1,-1  force_i2caddr=0x5F,0x5F,0x5C,0x5C,0x5F,0x5F force_slave=0,0,1,1,0,0
```

## 5. 程序编译

### 5.1 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

dwa_blend_encode demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine.json                 # sophon-stream graph配置，需要分别配置dwa、blend、resize、encode等文件
├── camera_dwa_blend_encode_demo.json          # demo按sensor输入的配置文件
├── dwa_blend_encode_demo.json            # demo按图片输入的配置文件
├── dwa_0.json                  # 左侧输入的鱼眼展开配置文件
├── dwa_1.json                  # 右侧输入的鱼眼展开配置文件
├── dwa_2.json                  # 上侧输入的鱼眼展开配置文件
├── dwa_3.json                  # 下侧输入的鱼眼展开配置文件
├── blend1.json                  # 拼接1配置文件
├── blend2.json                  # 拼接2配置文件
├── blend3.json                  # 拼接3配置文件
├── encode.json                  # 编码配置
└── resize.json                 # 尺寸缩放配置文件
```

其中，[camera_dwa_blend_encode_demo.json](./config/camera_dwa_blend_encode_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。


### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。


1. 运行可执行文件,sensor的出图需要root权限
```bash
./main --demo_config_path=../bird_dwa_blend_encode/config/camera_dwa_blend_encode_demo.json
```

## 7. 性能测试

目前，鱼眼拼接算法只支持在BM1688 SOC模式下进行推理。按照默认设置可以达到25fps。

## 8. web ui使用
### 8.1 安装nodejs
访问https://nodejs.org/en/download/，根据说明完成nodejs的安装，推荐使用node-v20.11.1版本。

### 8.2 web ui编译
进入sophon-stream/sample/dwa_dpu_encode/web_ui目录，执行以下命令：
```bash
npm install --force
npm run build
```
编译完成后会在该目录下生产build文件夹。
### 8.3 运行web ui
进入sophon-stream/sample/dwa_dpu_encode/web_ui/build目录，执行以下命令：
```bash
python3 -m http.server 3000
```
其中，3000是web ui的端口号，可以根据需要修改。
在浏览器中访问http://localhost:3000/，即可打开web ui界面。（localhost更改为运行环境的ip）



---

## bytetrack

# ByteTrack Demo

[English](README_EN.md) | 简体中文

## 目录
- [ByteTrack Demo](#bytetrack-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)


## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标跟踪应用。

本例程插件的连接方式如下图所示:

![elements.jpg](pics/tracker.png)

ByteTrack是一个简单、快速、强大的多目标跟踪器，且不依赖特征提取模型。

**论文** (https://arxiv.org/abs/2110.06864)

**源代码** (https://github.com/ifzhang/ByteTrack)

## 2. 特性
* 支持BM1684X(x86 PCIe、SoC)和BM1684(x86 PCIe、SoC、arm PCIe)
* 支持检测模块和跟踪模块解耦，可适配各种检测器，本例程主要以YOLOX作为检测器
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本[download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：
```bash
./data/models
├── BM1684
│   ├── yolox_s_fp32_1b.bmodel    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel    # 用于BM1684的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel    # 用于BM1684的INT8 BModel，batch_size=4
├── BM1684X
│   ├── yolox_s_fp32_1b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=1
└── └── yolox_s_int8_4b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=4
```
下载的数据包括：
```bash
./data/videos
└──  test_car_person_1080P.avi                 # 测试视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。


### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。


## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明
bytetrack demo中各部分参数位于[config](../bytetrack/config/)目录，结构如下所示

```bash
./config
   ├── bytetrack_demo.json       # bytetrack demo 配置
   ├── bytetrack.json            # bytetrack目标跟踪器参数配置
   ├── decoder.json              # 解码配置
   ├── engine.json               # sophon-stream graph配置
   ├── infer.json                # 目标检测器推理配置
   ├── post.json                 # 目标检测器后处理配置
   └── pre.json                  # 目标检测器前处理配置
```

其中，[bytetrack_demo.json](../bytetrack/config/bytetrack_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "download_image": true,
  "draw_func_name": "draw_bytetrack_results",
  "engine_config_path": "../bytetrack/config/engine.json"
}
```
[engine.json](../bytetrack/config/engine.json)包含对每一张graph的配置信息。这里摘取一部分作为示例：在一张图内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。
```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "bytetrack",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../bytetrack/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../bytetrack/config/yolox_pre.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5002,
                "element_config": "../bytetrack/config/yolox_infer.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5003,
                "element_config": "../bytetrack/config/yolox_post.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5004,
                "element_config": "../bytetrack/config/bytetrack.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5002,
                "dst_port": 0
            },
            {
                "src_element_id": 5002,
                "src_port": 0,
                "dst_element_id": 5003,
                "dst_port": 0
            },
            {
                "src_element_id": 5003,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            }
        ]
    }
]
```
[bytetrack.json](../bytetrack/config/bytetrack.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，thread_number是element内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。
```json
{
    "configure": {
        "track_thresh": 0.5,
        "high_thresh": 0.6,
        "match_thresh": 0.7,
        "frame_rate": 30,
        "track_buffer": 30
    },
    "shared_object": "../../build/lib/libbytetrack.so",
    "name": "bytetrack",
    "side": "sophgo",
    "thread_number": 2
}
```

### 6.2 运行
对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../bytetrack/config/bytetrack_demo.json
```

2路视频流运行结果如下
```bash
total time cost 5246889 us.
frame count is 1422 | fps is 271.018 fps.
```

##  7. 性能测试

测试视频`elevator-1080p-25fps-4000kbps.h264`，编译选项为Release模式，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|系统内存(M)|系统内存峰值(M)|TPU利用率(%)|设备内存(M)|设备内存峰值(M)|平均FPS|峰值FPS|
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|4-4-4|505.98|208.26|216.78|99.94|1432.99|1623.00|318.06|332.26|
|SE5-16|4|4-4-4|238.21|118.36|119.81|94.40|1258.89|1397.00|130.36|143.47|
|SE5-8|3|3-3-3|160.97|96.19|97.71|92.69|993.08|1128.00|82.01|91.24|

> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；

### bytetrack - README_EN.md

# ByteTrack Demo

English | [简体中文](README.md)

## Catalogs
- [ByteTrack Demo](#bytetrack-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)


## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object track application.

The connection method for this example plugin is shown in the following diagram.

![elements.jpg](pics/tracker.png)

Bytetrack is a simple, fast and powerful multi-target tracker, and does not rely on feature extraction models.

**paper** (https://arxiv.org/abs/2110.06864)

**source code** (https://github.com/ifzhang/ByteTrack)

## 2. Features

* Supports BM1684X(x86 PCIe、SoC) and BM1684(x86 PCIe、SoC、arm PCIe)
* Support the decoupling of detection module and tracking module, can be adapted to a variety of detectors, this routine is mainly used as a detector YOLOX
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh)。

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./data/models
├── BM1684
│   ├── yolox_s_fp32_1b.bmodel    # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel    # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_s_int8_1b.bmodel    # INT8 BModel for BM1684，batch_size=1
│   └── yolox_s_int8_4b.bmodel    # INT8 BModel for BM1684，batch_size=4
├── BM1684X
│   ├── yolox_s_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel    # FP32 BModel for BM1684X，batch_size=4
│   ├── yolox_s_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
└── └── yolox_s_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
```

The downloaded data include:
```bash
./data/videos
└──  test_car_person_1080P.avi                 # test video
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the ByteTrack demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config
   ├── bytetrack_demo.json       # bytetrack demo configuration
   ├── bytetrack.json            # bytetrack tracker configuration
   ├── decoder.json              # decoding configuration
   ├── engine.json               # sophon-stream graph configuration
   ├── infer.json                # detector inference configuration
   ├── post.json                 # detector post-process configuration
   └── pre.json                  # detector pre-process configuration
```

Indeed, [bytetrack_demo.json](./config/bytetrack_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../bytetrack/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "download_image": true,
  "draw_func_name": "draw_bytetrack_results",
  "engine_config_path": "../bytetrack/config/engine.json"
}
```

[engine.json](./config/engine.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "bytetrack",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../bytetrack/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../bytetrack/config/yolox_pre.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5002,
                "element_config": "../bytetrack/config/yolox_infer.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5003,
                "element_config": "../bytetrack/config/yolox_post.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5004,
                "element_config": "../bytetrack/config/bytetrack.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5002,
                "dst_port": 0
            },
            {
                "src_element_id": 5002,
                "src_port": 0,
                "dst_element_id": 5003,
                "dst_port": 0
            },
            {
                "src_element_id": 5003,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            }
        ]
    }
]
```

[bytetrack.json](./config/bytetrack.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

```json
{
    "configure": {
        "track_thresh": 0.5,
        "high_thresh": 0.6,
        "match_thresh": 0.7,
        "frame_rate": 30,
        "track_buffer": 30
    },
    "shared_object": "../../build/lib/libbytetrack.so",
    "name": "bytetrack",
    "side": "sophgo",
    "thread_number": 2
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../bytetrack/config/bytetrack_demo.json
```

The running results of two video streams are as follows
```bash
total time cost 5246889 us.
frame count is 1422 | fps is 271.018 fps.
```

## 7. Performance Testing

The tested video is `elevator-1080p-25fps-4000kbps.h264`. The compilation was done in Release mode. The results are as follows:

|Device|Number of Channels|Algorithm Thread Count|CPU Utilization(%)|System Memory(M)|Peak System Memory(M)|TPU Utilization(%)|Device Memory(M)|Peak Device Memory(M)|Average FPS|Peak FPS|
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|4-4-4|505.98|208.26|216.78|99.94|1432.99|1623.00|318.06|332.26|
|SE5-16|4|4-4-4|238.21|118.36|119.81|94.40|1258.89|1397.00|130.36|143.47|
|SE5-8|3|3-3-3|160.97|96.19|97.71|92.69|993.08|1128.00|82.01|91.24|

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
---

## dwa_blend_encode

# dwa_blend_encode Demo

## 目录
- [dwa\_blend\_encode Demo](#dwa_blend_encode-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备数据](#3-准备数据)
  - [4. 环境准备](#4-环境准备)
    - [4. SoC平台](#4-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 SoC平台](#51-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)
  - [8. web ui使用](#8-web-ui使用)
    - [8.1 安装nodejs](#81-安装nodejs)
    - [8.2 web ui编译](#82-web-ui编译)
    - [8.3 运行web ui](#83-运行web-ui)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建深度估计应用。

本例程中，鱼眼拼接算法的鱼眼展开、鱼眼拼接分别在两个element上进行运算，element内部可以开启多个线程，保证了一定的运行效率。下图是广角拼接应用的流程图：
![dwa_pipeline](pic/image.jpg)

## 2. 特性

* 支持BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备数据

​在`scripts`目录下提供了相关数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。
```bash
.
├── gridinfo # 用于dwa模块的参数文件
├── images   # 测试图片
├── wgt     # 用于拼接的权重文件
└── videos   # 测试视频
```

## 4. 环境准备

### 4. SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。
(1) 安装驱动
安装驱动需要进入到超级权限，接着系统驱动目录，安装驱动：
```bash
sudo -s
cd /mnt/system/ko/
insmod v4l2_os04e10.ko
```
（2）isp参数文件配置

```bash
sudo -s
mkdir -p /mnt/cfg/param
cp data/cvi_sdr_bin /mnt/cfg/param
```


## 5. 程序编译

### 5.1 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

dwa_blend_encode demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine.json                 # sophon-stream graph配置，需要分别配置dwa、blend、resize、encode等文件
├── camera_dwa_blend_encode_demo.json          # demo按sensor输入的配置文件
├── dwa_blend_encode_demo.json            # demo按图片输入的配置文件
├── dwa_L.json                  # 左侧输入的鱼眼展开配置文件
├── dwa_R.json                  # 右侧输入的鱼眼展开配置文件
├── blend.json                  # 拼接配置文件
└── resize.json                 # 尺寸缩放配置文件
```

其中，[camera_dwa_blend_encode_demo.json](./config/camera_dwa_blend_encode_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。


### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。


1. 运行可执行文件,sensor的出图需要root权限
```bash
./main --demo_config_path=../dwa_blend_encode/config/camera_dwa_blend_encode_demo.json
```

## 7. 性能测试

目前，鱼眼拼接算法只支持在BM1688 SOC模式下进行推理。按照默认设置可以达到25fps。

## 8. web ui使用
### 8.1 安装nodejs

访问https://nodejs.org/en/download/，根据说明完成nodejs的安装，推荐使用node-v20.11.1版本。

### 8.2 web ui编译

进入sophon-stream/sample/dwa_dpu_encode/web/ui目录，执行以下命令：
```bash
npm install --force
npm run build
```
编译完成后会在该目录下产生build文件夹。最后，将编译生成的文件拷贝到SoC平台中运行。
### 8.3 运行web ui

进入sophon-stream/sample/dwa_dpu_encode/web/ui/build目录，执行以下命令：
```bash
python3 -m http.server 3000
```
其中，3000是web ui的端口号，可以根据需要修改。
在浏览器中访问http://localhost:3000/，即可打开web ui界面。（localhost更改为运行环境的ip）



### dwa_blend_encode - gridinfo_and_wgt.md

# Calibration

## 目录
- [Calibration](#calibration)
  - [目录](#目录)
  - [环境准备](#环境准备)
  - [拍摄照片](#拍摄照片)
  - [调参说明](#调参说明)
  - [参数文件更新](#参数文件更新)


## 环境准备
（1）安装04e10驱动，参照[使用手册](README.md)
（2）加载isp参数，参照[使用手册](README.md)

（3）打开CviIspTool.sh，执行以下命令：
```bash
cd /opt/sophon/sophon-soc-libisp_1.0.0/bin
```
将cfg.json中的"dev-num":改为2，保证可以读取两路的视频。
```bash
./CviIspTool.sh
```

（4）在window打开pqtool工具，输入主机ip，即可连接。下载链接如下：
```bash
python3 -m dfss --url=open@sophgo.com:/sophon-stream/dwa_dpu_encode/CviPQtool_20240111.zip
```
（5）在window打开鱼眼拼接调参工具，下载链接如下：
```bash
python3 -m dfss --url=open@sophgo.com:/sophon-stream/dwa_blend_encode/stitchtool_circular_fisheye_v240103.7.zip
```

## 拍摄照片

（1）分别用两个相机各拍摄一张具有重叠区域的照片（打开CviPQTool工具，点击Preview，再点击Get Single Image和Save来拍摄并保存图像），分别命名为l和r保存在L和R文件夹下（l为拼接图中的左图，r为拼接图中的右图），手动旋转为正方向。
[![9b8c262d0b85997994ef5dc685de28fa.png](https://s1.imagehub.cc/images/2024/02/20/9b8c262d0b85997994ef5dc685de28fa.png)](https://www.imagehub.cc/image/1hKM2j)

下面是两张样图：

[![794a97b2ef03d46232a9602bb8dbd1cf.png](https://s1.imagehub.cc/images/2024/02/20/794a97b2ef03d46232a9602bb8dbd1cf.png)](https://www.imagehub.cc/image/1hKweg)[![34d47fb2fdb2f810516df641a0630803.png](https://s1.imagehub.cc/images/2024/02/20/34d47fb2fdb2f810516df641a0630803.png)](https://www.imagehub.cc/image/1hKeso)

（2）或者用vlc++的截图功能，分别截取左右摄像头的图像，下载链接如下：
```bash
python3 -m dfss --url=open@sophgo.com:/sophon-stream/dwa_blend_encode/vlc++.zip
```
在vlc++中输入两路视频流地址，点击右上角的截图按钮。
[![c76b8a584e600c2c7ccfc7d54ba983ba.png](https://s1.imagehub.cc/images/2024/02/20/c76b8a584e600c2c7ccfc7d54ba983ba.png)](https://www.imagehub.cc/image/1hKWiO)

## 调参说明
（1）运行下列命令（鱼眼拼接调参工具目录下readme.txt中step1命令）来将鱼眼图像展开为平面图像（命令中图像后缀要与第1步保存的图片后缀一致）(在stitchtool_circular_fisheye_xxx目录下，打开cmd)。
[![23ee60cca7190a5f36700e4edda3a106.png](https://s1.imagehub.cc/images/2024/02/20/23ee60cca7190a5f36700e4edda3a106.png)](https://www.imagehub.cc/image/1hKEH6)

命令说明如下：
```bash
gen_gridinfo.exe mode data_folder/ image_path do_flip map_y_shift theta_x theta_y theta_z

data_folder：放置_mapx.bin及_mapy.bin的地方
image_path：使用影像档的绝对路径
map_x_shift：map矩阵要移动的pixel
map_y_shift：map矩阵要移动的pixel
theta_x, theta_y, theta_z: three angles in degree
```

如：
```bash
gen_gridinfo.exe 0 ./231204/L ./231204/L/l.bmp 0 0 0 0 0
gen_gridinfo.exe 0 ./231204/R ./231204/R/r.bmp 0 0 0 0 0
```
通过移动窗口的滑杆选择合适的单位视球球心和半径，以此将鱼眼图像还原到单位球面上，从而投影为平面图像（注意：找圆时尽量往内圈一点点，不要圈到边缘的黑边）。

[![9f8fc6298cd9bf992d5aa9a42ccb0e70.png](https://s1.imagehub.cc/images/2024/02/20/9f8fc6298cd9bf992d5aa9a42ccb0e70.png)](https://www.imagehub.cc/image/1hKRbJ)[![3e26f31a6233ecec4ad5c6f677aa7466.png](https://s1.imagehub.cc/images/2024/02/20/3e26f31a6233ecec4ad5c6f677aa7466.png)](https://www.imagehub.cc/image/1hKcEe)

（2）运行下列命令（鱼眼拼接调参工具目录下readme.txt中step2命令）产生DWA要使用的gridinfo文件及拼接要用的类似DWA的输出结果 dc_src.jpg （这个命令一般无需改动）：
命令说明如下：
```bash
gen_gridinfo.exe mode data_folder dc_src.jpg_path
```
如：
```bash
gen_gridinfo.exe 2 ./231204/L ./231204/L/dc_src.jpg
gen_gridinfo.exe 2 ./231204/R ./231204/R/dc_src.jpg
```

(3) 运行下列命令（鱼眼拼接调参工具目录下readme.txt中step3命令）按照默认参数将两张图片拼接得到拼接图（3/c01_stitch_444p_c2.png）。
命令说明如下：
```bash
gen_gridinfo.exe mode left_image_path right_image_path 融合区起始位置 融合区宽度
```
如：
```bash
gen_gridinfo.exe 1 ./231204/3/ ./231204/L/mask_result.png ./231204/R/mask_result.png 2144 32
```
注意：（注意：最后两个数值需要被32整除，相加不需要等于2240，融合时，左图右边缘会直接取右图，如示例中，2240 – 2144 – 32 = 64，即左图右边缘64pixel直接取右图）。
[![177b45c44ae6dad8915dcd4314ff70a1.png](https://s1.imagehub.cc/images/2024/02/20/177b45c44ae6dad8915dcd4314ff70a1.png)](https://www.imagehub.cc/image/1hKsuZ)

（4）根据拼接图来调整拼接参数，重复234步尝试找到最佳拼接效果

注意：一般固定右图不动，通过调节左图位置来对齐。

X方向（map_x_shift）：map矩阵要移动的pixel，正数代表向右移动图片，负数代表向左移动图片，以此可以对图片在X方向上的位置进行微调。

Y方向（map_y_shift）：map矩阵要移动的pixel，正数代表向下移动图片，负数代表向上移动图片，以此可以对图片在Y方向上的位置进行微调。
如：
```bash
gen_gridinfo.exe 0 ./231204/L ./231204/L/l.bmp -16 6 0 0 0
```
该命令的含义是：左图整体向左移动16个pixel，向下移动6个pixel。
上述命令中最后三个数字分别可以调整图像theta_x 、theta_y 、theta_z三个角度，其效果分别如下图所示(一般不使用这三个参数，默认设置为0)。下面是效果演示：
首先，原图为：
[![bcd976c4c80f5271526e07216cc9df94.png](https://s1.imagehub.cc/images/2024/02/20/bcd976c4c80f5271526e07216cc9df94.png)](https://www.imagehub.cc/image/1hKJih)

设置theta_x = 3，theta_y = 0，theta_z = 0，效果如下：
[![2f558fac322adcf054bf5120c8f947b2.png](https://s1.imagehub.cc/images/2024/02/20/2f558fac322adcf054bf5120c8f947b2.png)](https://www.imagehub.cc/image/1hKilr)

设置theta_x = 0，theta_y = 3，theta_z = 0，效果如下：
[![4045c79506e89c80beb32dfd6dd7e711.png](https://s1.imagehub.cc/images/2024/02/20/4045c79506e89c80beb32dfd6dd7e711.png)](https://www.imagehub.cc/image/1hKkHv)

设置theta_x = 0，theta_y = 0，theta_z = 3，效果如下：
[![1229d68bdba61af016dda4264c710342.png](https://s1.imagehub.cc/images/2024/02/20/1229d68bdba61af016dda4264c710342.png)](https://www.imagehub.cc/image/1hKyot)

注意：theta_x 表示绕横轴旋转，theta_y 表示绕纵轴旋转，theta_z表示垂直纸面旋转。

(5)下列命令中的两个参数可以调整两张图像X方向的拼接融合区域大小，其中2144为左图起始拼接位置，32为融合区域大小。在默认参数基础上以32为最小调节单位来进行拼接时X方向上的调整，选择效果最好的拼接图作为最终结果。
```bash
gen_gridinfo.exe 1 ./231204/3/ ./231204/L/mask_result.png ./231204/R/mask_result.png 2144 32
```
调整后的拼接图如下图所示:

[![0e6aeae69a2ea2c4f2e7b8906a3faee1.png](https://s1.imagehub.cc/images/2024/02/20/0e6aeae69a2ea2c4f2e7b8906a3faee1.png)](https://www.imagehub.cc/image/1hKQbS)

(7)拼接完成后保存以下文件（alpha、beta权重文件以及静态拼接图像在3文件夹中，Lgrid_info和Rgrid_info文件在231204文件中）。

[![07bd58bc0789514a0f96d43a39683a1c.png](https://s1.imagehub.cc/images/2024/02/20/07bd58bc0789514a0f96d43a39683a1c.png)](https://www.imagehub.cc/image/1hKaEL)


## 参数文件更新
将生成的权重文件和girdinfo文件替换sophon-stream/sample/dwa_blend_encode/data下相应的文件，并修改对应的dwa和blend的json文件即可（修改说明见对应插件的README）。

[![cf7b8955fad073f099fff6c4227e71ae.png](https://s1.imagehub.cc/images/2024/02/20/cf7b8955fad073f099fff6c4227e71ae.png)](https://www.imagehub.cc/image/1hKuvB)
---

## dwa_dpu_encode

# dwa_dpu_encode Demo

## 目录
- [dwa\_dpu\_encode Demo](#dwa_dpu_encode-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备数据](#3-准备数据)
  - [4. 环境准备](#4-环境准备)
    - [4. SoC平台](#4-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 SoC平台](#51-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)
  - [8. web ui使用](#8-web-ui使用)
    - [8.1 安装nodejs](#81-安装nodejs)
    - [8.2 web ui编译](#82-web-ui编译)
    - [8.3 运行web ui](#83-运行web-ui)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建深度估计应用。

本例程中，深度估计算法的镜头畸变矫正、深度估计、染色分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的运行效率。下图是深度估计应用的流程图：
![dpu_pipeline](pic/image.png)

## 2. 特性

* 支持BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备数据

​在`scripts`目录下提供了相关数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。
```bash
.
├── gridinfo # 用于dwa模块的参数文件
├── images   # 测试图片
├── maps     # 用于ive模块的染色文件
└── videos   # 测试视频
```

## 4. 环境准备

### 4. SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。
(1) 安装驱动
安装驱动需要进入到超级权限，接着系统驱动目录，安装驱动：
```bash
sudo -s
insmod /mnt/system/ko/v4l2_os04a10_sync.ko
```

（2）isp参数文件配置,需要在当前dwa_dpu_encode目录下

```bash
sudo -s
mkdir -p /mnt/cfg/param
cp ./data/cvi_sdr_bin /mnt/cfg/param/
```
备注：如需标定，请参考[摄像头标定](Calibration.md)
## 5. 程序编译

### 5.1 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

dwa_dpu_encode demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── encode.json                 # 编码配置
├── engine_dwa_dpu_ive_resize.json          # sophon-stream graph配置，需要分别配置dwa、dpu、ive_resize、encode等文件
├── dwa_dpu_encode_demo_imgs.json           # demo按图片输入的配置文件
├── camera_dwa_dpu_encode_demo.json        # demo按sensor输入的配置文件
├── dwa_L.json                  # 左侧输入对应的畸变矫正配置文件
├── dwa_R.json                  # 右侧输入对应的畸变矫正配置文件
├── dpu.json                    # 深度估计配置文件
├── resize.json                 # 尺寸缩放配置文件
└── ive.json                    # 深度估计配置文件

```

其中，[camera_dwa_dpu_encode_demo.json](./config/camera_dwa_dpu_encode_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。此例程不支持download选项。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。


### 6.2 运行

对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。


1. 运行可执行文件,sensor的出图需要root权限
```bash
./main --demo_config_path=../dwa_dpu_encode/config/camera_dwa_dpu_encode_demo.json
```

## 7. 性能测试

目前，深度估计算法只支持在BM1688 SOC模式下进行推理。按sensor输入按照默认设置可以达到30fps。可通过运行中打印的log确认fps是否正常。


## 8. web ui使用
### 8.1 安装nodejs

访问https://nodejs.org/en/download/，根据说明完成nodejs的安装，推荐使用node-v20.11.1版本。

### 8.2 web ui编译

进入sophon-stream/sample/dwa_dpu_encode/web/ui目录，执行以下命令：
```bash
npm install --force
npm run build
```
编译完成后会在该目录下产生build文件夹。最后，将编译生成的文件拷贝到SoC平台中运行。
### 8.3 运行web ui

进入sophon-stream/sample/dwa_dpu_encode/web/ui/build目录，执行以下命令：
```bash
python3 -m http.server 3000
```
其中，3000是web ui的端口号，可以根据需要修改。
在浏览器中访问http://localhost:3000/，即可打开web ui界面。（localhost更改为运行环境的ip）

备注：

1.注意本例程web界面使用dpu选项，不支持blend选项

2.推荐使用google或者edge浏览器，不推荐firefox浏览器

### dwa_dpu_encode - Calibration.md

# Calibration

## 目录
- [Calibration](#calibration)
  - [目录](#目录)
  - [整体标定流程](#整体标定流程)
  - [环境准备](#环境准备)
  - [标定工具说明](#标定工具说明)
  - [单目标定](#单目标定)
  - [双目标定](#双目标定)

## 整体标定流程
整体的标定流程分为两部分，首先进行单目的标定，然后进行双目转共面实现左右摄像头对应的特征点在统一水平线上。
[![5a279cd06e55ca7330dbd606924d7118.png](https://s1.imagehub.cc/images/2024/02/20/5a279cd06e55ca7330dbd606924d7118.png)](https://www.imagehub.cc/image/1hHShT)

## 环境准备
（1）安装04a10驱动，参照[使用手册](README.md)

（2）加载isp参数，参照[使用手册](README.md)

（3）进入root账户，打开CviIspTool.sh，执行以下命令：
```bash
sudo -s
cd /opt/sophon/sophon-soc-libisp_1.0.0/bin
```
将cfg.json中的"dev-num":改为2，保证可以读取两路的视频。
```bash
./CviIspTool.sh
```
（4）在window打开pqtool工具，输入主机ip，即可连接。下载链接如下：
```bash
python3 -m dfss --url=open@sophgo.com:/sophon-stream/dwa_dpu_encode/CviPQtool_20240111.zip
```

## 标定工具说明
（1）点击菜单栏的calibration，选择下方的Distrotion calibration。
[![9374cc476fafc40500f6315d48b5cfb7.png](https://s1.imagehub.cc/images/2024/02/20/9374cc476fafc40500f6315d48b5cfb7.png)](https://www.imagehub.cc/image/1hJd5O)

（2）参与矫正的图片来源有2种，分别是：从板端获取和从电脑本地加载。从板端获取和从电脑本地加载标定图片的唯一区别是：当点击“capture”按键时，则从板端获取。不点击“capture”按键而点击“Import Image”按键时，则从电脑本地获取。如下图：
[![7a88a2419a9df821bbee3711b0aa746e.png](https://s1.imagehub.cc/images/2024/02/20/7a88a2419a9df821bbee3711b0aa746e.png)](https://www.imagehub.cc/image/1hJbm6)

下面以本地加载标定的方式来说明单目标定，双目标定流程。

## 单目标定
（1）先选择类型是左目sensor还是右目sensor（左右目是哪一个sensor，根据情况由你自己来决定），比如此时选择类型为左目sensor，即Left eye，如下图：
[![f8395681a1f448d87de2840068793b7b.png](https://s1.imagehub.cc/images/2024/02/20/f8395681a1f448d87de2840068793b7b.png)](https://www.imagehub.cc/image/1hJMRJ)

（2）通过“Input”窗口的“Import Image”按键导入左目sensor的本地标定图片目录，导入后的结果如下图：
[![e000000ddd2ced96d1d09cf456c0aec0.png](https://s1.imagehub.cc/images/2024/02/20/e000000ddd2ced96d1d09cf456c0aec0.png)](https://www.imagehub.cc/image/1hJgVe)

特别说明：对于本地左右单目，及双目标定图片所在目录，其目录只能存放一种类型名的图片，即本地左目目录下的图片名均是以“left_xx”、 右目则为“right_xx”、双目则为“stereo_xx”为前缀的jpg图片，否则会导入失败。具体名字形式如下图：
[![74554bab1e926da434ef1df6a9320543.png](https://s1.imagehub.cc/images/2024/02/20/74554bab1e926da434ef1df6a9320543.png)](https://www.imagehub.cc/image/1hJlts)

（3）图片导入后，需要设定标定前的参数选项，具体是下面所提到的参数。下图中的“Pattern Info”参数设置，取决于所拍标定板的类型。
[![da38132aee740ea4341dbce480c49fbc.png](https://s1.imagehub.cc/images/2024/02/20/da38132aee740ea4341dbce480c49fbc.png)](https://www.imagehub.cc/image/1hJBKr)

下图中的“Image Calibrate Param”参数设置，取决于sensor镜头和实际需求的opencv参数选项。
[![e414d59992d0da76796ac4fc654390e7.png](https://s1.imagehub.cc/images/2024/02/20/e414d59992d0da76796ac4fc654390e7.png)](https://www.imagehub.cc/image/1hJtRS)

上两图的每个参数的详细设置说明，也可将鼠标移动到对应的参数名位置，停留片刻后，其会自动弹出对应说明。

（4）在启动标定之前，需选择标定时产生数据存放的位置，否则会保存到默认路径下，点击“Save Root Path”按键即可选择。如下图：

[![a362205c9e68d300cb048c52ba464843.png](https://s1.imagehub.cc/images/2024/02/20/a362205c9e68d300cb048c52ba464843.png)](https://www.imagehub.cc/image/1hJGtq)

（5）以上设置完成后，即可点击“Start calibration”按键进行标定，待标定完成后，右侧画面会自动跳转到标定后的效果界面。即“Output”窗口。如下图：
[![10aa57e34d98d941f740dcc1d6d2bd6d.png](https://s1.imagehub.cc/images/2024/02/20/10aa57e34d98d941f740dcc1d6d2bd6d.png)](https://www.imagehub.cc/image/1hJRA0)

（6）标定完成后，如果要保存标定后的结果图片，只需要点击上图右上角的“Save”按键即可完成保存，保存的位置，皆在选择的目录下。

标定后产生的数据文件分布情况如下图所示，以左目标定目录为例：
[![be12fa00ead877c237aaec3080b79bb4.png](https://s1.imagehub.cc/images/2024/02/20/be12fa00ead877c237aaec3080b79bb4.png)](https://www.imagehub.cc/image/1hJJgo)

右目标定流程，同左目流程，但要按照下图先选择“Right eye”。如下图。之后通过“Import Image”按键导入参与右目标定的图片。
[![215a9826ca04fa9b96d777c44c77d0b0.png](https://s1.imagehub.cc/images/2024/02/20/215a9826ca04fa9b96d777c44c77d0b0.png)](https://www.imagehub.cc/image/1hJUhb)

## 双目标定

（1）“Stereo eye”标定流程也同左目标定流程，需先选择“Stereo eye”，如下图，之后通过“Import Image”按键导入参与“Stereo eye”标定的图片。

[![4de6809f96ffcd02ae043348b628f69a.png](https://s1.imagehub.cc/images/2024/02/20/4de6809f96ffcd02ae043348b628f69a.png)](https://www.imagehub.cc/image/1hJQAa)

特别地，在进行“Stereo eye”标定前，需要先将左目和右目都标定完后或者已经存在由左右目标定时产生的标定数据，方可进行“Stereo eye”标定，否则无法进行此标定，同时工具会有相关警告提示。

（2）标定完成后，点击右上方的save，会将矫正后的图片全部存放在指定目录的undistort下。
[![5de7dedd7b980d351fd477d4657468d7.png](https://s1.imagehub.cc/images/2024/02/20/5de7dedd7b980d351fd477d4657468d7.png)](https://www.imagehub.cc/image/1hJunO)


下面是经过双目标定后生成的gridinfo、矫正图片以及矫正参数。

[![397bac344e58a2408245bf92fd9d3187.png](https://s1.imagehub.cc/images/2024/02/20/397bac344e58a2408245bf92fd9d3187.png)](https://www.imagehub.cc/image/1hJvKA)

下面的gridinfo文件即dwa的输入文件。
[![eb07276c3864ffe71721ba3eba8d2d7f.png](https://s1.imagehub.cc/images/2024/02/20/eb07276c3864ffe71721ba3eba8d2d7f.png)](https://www.imagehub.cc/image/1hJ9zk)

打开cameraParams.yaml，可以看到每张图重投影后的误差情况。
[![cae9075bd1c01d461767ac3e55fa330c.png](https://s1.imagehub.cc/images/2024/02/20/cae9075bd1c01d461767ac3e55fa330c.png)](https://www.imagehub.cc/image/1hJVw6)

至此，完成了双目相机的标定。
---

## dwa_lightstereo_encode

# dwa_lightstereo_encode Demo

## 目录
- [dwa\_lightstereo\_encode Demo](#dwa_lightstereo_encode-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备数据](#3-准备数据)
  - [4. 环境准备](#4-环境准备)
    - [4. SoC平台](#4-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 SoC平台](#51-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建深度估计应用。

本例程中，深度估计算法的镜头畸变矫正、深度估计、染色分别在不同的element上进行运算，element内部可以开启多个线程，保证了一定的运行效率。下图是深度估计应用的流程图：
![pipeline](pic/image.png)

其中，输入的`/dev/video0`是指左目摄像头，`/dev/video1`是指右目摄像头，encode element的输出默认是rtsp视频流，对应原视频流的视差。

## 2. 特性

* 支持BM1688(SoC)

## 3. 准备数据

​在`scripts`目录下提供了相关数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，目录结构如下：
```bash
.
├── cvi_sdr_bin # isp参数文件
├── gridinfo # 用于dwa模块的参数文件
└── models   # lightstereo的bmodel，来源可参考https://github.com/sophgo/sophon-demo/sample/LightStereo
```

## 4. 环境准备

### 4. SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。
(1) 安装驱动
安装驱动需要进入到超级权限，接着系统驱动目录，安装驱动：
```bash
sudo -s
insmod /mnt/system/ko/v4l2_os04a10_sync.ko
```

（2）isp参数文件配置,需要在当前dwa_lightstereo_encode目录下

```bash
sudo -s
mkdir -p /mnt/cfg/param
cp ./data/cvi_sdr_bin /mnt/cfg/param/
```
备注：如需标定，请参考[摄像头标定](../dwa_dpu_encode/Calibration.md)
## 5. 程序编译

### 5.1 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

dwa_lightstereo_encode demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── encode.json                 # 编码配置
├── engine.json                 # sophon-stream graph配置
├── camera_dwa_lightstereo_encode_demo.json # demo配置文件
├── dwa_L.json                  # 左侧输入对应的畸变矫正配置文件
├── dwa_R.json                  # 右侧输入对应的畸变矫正配置文件
├── lightstereo_pre.json        # lightstereo深度估计配置文件-前处理
├── lightstereo_infer.json      # lightstereo深度估计配置文件-推理
└── lightstereo_post.json       # lightstereo深度估计配置文件-后处理
```

其中，[camera_dwa_lightstereo_encode_demo.json](./config/camera_dwa_lightstereo_encode_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持2路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。此例程不支持download选项。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。


### 6.2 运行

对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。


1. 参考[rtsp使用说明](../../element/multimedia/encode/README.md#3-rtsp使用说明)，在SoC平台上运行rtsp服务器。
  
2. 运行可执行文件,sensor的出图需要root权限
```bash
sudo -s
./main --demo_config_path=../dwa_lightstereo_encode/config/camera_dwa_lightstereo_encode_demo.json
```

3. 在可以ping通SoC平台的主机上，使用VLC拉流。

## 7. 性能测试

目前，深度估计算法只支持在BM1688 SOC模式下进行推理。按sensor输入按照默认设置可以达到25fps。可通过运行中打印的log确认fps是否正常。
---

## gdwa_blend_encode

# gdwa_blend_encode Demo

## 目录
- [gdwa\_blend\_encode Demo](#gdwa_blend_encode-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备数据](#3-准备数据)
  - [4. 环境准备](#4-环境准备)
    - [4. SoC平台](#4-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 SoC平台](#51-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)
  - [8. web ui使用](#8-web-ui使用)
    - [8.1 安装nodejs](#81-安装nodejs)
    - [8.2 web ui编译](#82-web-ui编译)
    - [8.3 运行web ui](#83-运行web-ui)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建深度估计应用。

本例程中，广角拼接算法的广角展开、广角拼接分别在两个element上进行运算，element内部可以开启多个线程，保证了一定的运行效率。下图是广角拼接应用的流程图：
![gdwa_pipeline](pic/image.jpg)
## 2. 特性

* 支持BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备数据

​在`scripts`目录下提供了相关数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。
```bash
.
├── gridinfo # 用于dwa模块的参数文件
├── images   # 测试图片
├── wgt     # 用于拼接的权重文件
└── videos   # 测试视频
```

## 4. 环境准备

### 4. SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。
(1) 安装驱动
安装驱动需要进入到超级权限，接着系统驱动目录，安装驱动：
```bash
sudo -s
cd /mnt/system/ko/
insmod v4l2_os04a10_sync.ko
```
（2）isp参数文件配置

```bash
sudo -s
mkdir -p /mnt/cfg/param
cp data/cvi_sdr_bin /mnt/cfg/param
```


## 5. 程序编译

### 5.1 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

gdwa_blend_encode demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                  # 解码配置
├── engine_dwa_blend_resize.json # sophon-stream graph配置，需要分别配置dwa、blend、resize、encode等文件
├── camera_dwa_blend_encode_demo.json          # demo按sensor输入的配置文件
├── dwa_blend_encode_demo_imgs.json            # demo按图片输入的配置文件
├── dwa_L.json                  # 左侧输入的广角展开配置文件
├── dwa_R.json                  # 右侧输入的广角展开配置文件
├── blend.json                  # 拼接配置文件
├── encode.json                 # 编码配置
└── resize.json                 # 尺寸缩放配置文件
```

其中，[camera_dwa_blend_encode_demo.json](./config/camera_dwa_blend_encode_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。


### 6.2 运行

对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。


1. 运行可执行文件,sensor的出图需要root权限
```bash
./main --demo_config_path=../gdwa_blend_encode/config/camera_dwa_blend_encode_demo.json
```

## 7. 性能测试

目前，广角拼接算法只支持在BM1688 SOC模式下进行推理。按照默认设置可以达到25fps。

## 8. web ui使用
### 8.1 安装nodejs

访问 https://nodejs.org/en/download/ ，根据说明完成nodejs的安装，推荐使用node-v20.11.1版本。

### 8.2 web ui编译

进入sophon-stream/sample/dwa_dpu_encode/web/ui目录，执行以下命令：
```bash
npm install --force
npm run build
```
编译完成后会在该目录下产生build文件夹。最后，将编译生成的文件拷贝到SoC平台中运行。
### 8.3 运行web ui

进入sophon-stream/sample/dwa_dpu_encode/web/ui/build目录，执行以下命令：
```bash
python3 -m http.server 3000
```
其中，3000是web ui的端口号，可以根据需要修改。
在浏览器中访问 http://localhost:3000/ ，即可打开web ui界面。（localhost更改为运行环境的ip）


---

## license_area_intrusion

# license_area_intrusion Demo

[English](README_EN.md) | 简体中文

## 目录

- [license_area_intrusion Demo](#license_area_intrusion-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe 平台](#41-x86arm-pcie平台)
    - [4.2 SoC 平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe 平台](#51-x86arm-pcie平台)
    - [5.2 SoC 平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json 配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用 sophon-stream 快速构建基于 yolov5 的车牌检测和基于 lprnet 的车牌识别,可以设置区域。

**LPRNET 车牌检测源代码**(https://github.com/sirius-ai/LPRNet_Pytorch)

本例程中，yolov5、lprnet 算法的前处理、推理、后处理均分别在三个 element 上进行运算，element 内部可以开启多个线程，保证了一定的检测效率。

## 2. 特性

- 支持 BM1684X、BM1684(x86 PCIe、SoC)、BM1688(SoC)
- 支持多路视频流
- 支持多线程

## 3. 准备模型与数据

​ 在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装7z、unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install p7zip
sudo apt install p7zip-full
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`dataset`、`models`目录，其中，`dataset`存放车辆数据集，`models`存放 yolov5 和 lprnet 的模型文件。

下载的模型包括：

```bash
./models/
├── lprnet
│   ├── BM1684
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1684X
│   │   ├── lprnet_fp16_1b.bmodel
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1688
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_new_int8_4b_2core.bmodel
│   ├── onnx
│   │   ├── lprnet_1b.onnx
│   │   └── lprnet_4b.onnx
│   └── torch
│       ├── Final_LPRNet_model.pth
│       └── LPRNet_model.torchscript
└── yolov5s-licensePLate
    ├── BM1684
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
    ├── BM1684X
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
    └── BM1688
        ├── yolov5s_v6.1_license_3output_fp32_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_4b_2core.bmodel
        └── yolov5s_v6.1_license_3output_int8_4b.bmodel

```

模型说明:

以上 lprnet 模型移植于[LNRNet_Pytorch](https://github.com/sirius-ai/LPRNet_Pytorch) , yolov5s-licensePLate 模型基于绿色车牌数据集训练。

下载的数据包括：

```bash
./datasets
├── coco.names
└── 1080_1920_5s.mp4 // 用于测试的车辆视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe 平台

如果您在 x86/arm 平台安装了 PCIe 加速卡（如 SC 系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装 libsophon、sophon-opencv 和 sophon-ffmpeg，具体步骤可参考[x86-pcie 平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie 平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC 平台

如果您使用 SoC 平台（如 SE、SM 系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的 libsophon、sophon-opencv 和 sophon-ffmpeg 运行库包，可直接使用它作为运行环境。通常还需要一台 x86 主机作为开发环境，用于交叉编译 C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe 平台

可以直接在 PCIe 平台上编译程序，具体请参考[sophon-stream 编译](../../docs/HowToMake.md)。

### 5.2 SoC 平台

通常在 x86 主机上交叉编译程序，您需要在 x86 主机上使用 SOPHON SDK 搭建交叉编译环境，将程序所依赖的头文件和库文件打包至 sophon_sdk_soc 目录中，具体请参考[sophon-stream 编译](../../docs/HowToMake.md)。本例程主要依赖 libsophon、sophon-opencv 和 sophon-ffmpeg 运行库包。

## 6. 程序运行

### 6.1 Json 配置说明

license_area_intrusion demo 中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config
├── converger.json
├── decode.json
├── distributor_time_class.json
├── engine_group.json
├── engine.json
├── license_area_intrusion_demo.json
├── lprnet_group.json
└── yolov5_group.json

```

其中，[license_area_intrusion.json](./config/license_area_intrusion.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels 参数配置输入的路数，sample_interval 设置跳帧数，loop_num 设置循环播放次数，channel 中包含码流 url 等信息。

配置文件中不指定`channel_id`属性的情况，会在 demo 中对每一路数据的`channel_id`从 0 开始默认赋值。

```json
{
    "channels": [
      {
        "channel_id": 0,
        "url": "../license_area_intrusion/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 1,
        "url": "../license_area_intrusion/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 2,
        "url": "../license_area_intrusion/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 3,
        "url": "../license_area_intrusion/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      }
    ],
    "class_names": "../license_area_intrusion/data/coco.names",
    "download_image": true,
    "draw_func_name": "draw_license_area_intrusion_results",
    "engine_config_path": "../license_area_intrusion/config/engine_group.json"
}
```

[engine.json](./config/engine.json)包含对 graph 的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即删除`channels`的部分元素，再进行测试。

在该文件内，需要初始化每个 element 的信息和 element 之间的连接方式。element_id 是唯一的，起到标识身份的作用。element_config 指向该 element 的详细配置文件地址，port_id 是该 element 的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src 标志当前端口是否是整张图的输入端口，is_sink 标识当前端口是否是整张图的输出端口。
connection 是所有 element 之间的连接方式，通过 element_id 和 port_id 确定。

[lprnet_pre.json](./config/lprnet_pre.json)等配置文件是对具体某个 element 的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

```json
{
    "configure": {
        "model_path": "../license_area_intrusion/models/lprnet/BM1684X/lprnet_fp32_1b.bmodel",
        "stage": [
            "pre"
        ]
    },
    "shared_object": "../../build/lib/liblprnet.so",
    "name": "lprnet",
    "side": "sophgo",
    "thread_number": 1
}
```

[filter.json](../license_area_intrusion/config/filter.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。
其中，thread_number是element内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。
具体请参考[filter.json](../../element/tools/filter/README.md)
```json
{
  "configure": {
    "rules": [
      {
        "channel_id": 0,
        "filters": [
          {
            "alert_first_frames": 0,
            "alert_frame_skip_nums": 10000,
            "areas": [
              [
                {
                  "left": 1000,
                  "top": 0
                },
                {
                  "left": 1000,
                  "top": 1080
                }
              ]
            ],
            "classes": [
              0
            ],
            "times": [
              {
                "time_start": "00 00 00",
                "time_end": "23 59 59"
              }
            ],
            "type": 1
          }
        ]
      }
    ]
  },
  "shared_object": "../../build/lib/libfilter.so",
  "name": "filter",
  "side": "sophgo",
  "thread_number": 1
}
```

### 6.2 运行

对于 PCIe 平台，可以直接在 PCIe 平台上运行测试；对于 SoC 平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到 SoC 平台中测试。

SoC 平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始 sophon-stream 仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以 PCIe 模式进行介绍。

1. 运行可执行文件，注意给出区域
```bash
./main --demo_config_path=../license_area_intrusion/config/license_area_intrusion_demo.json
```

推理结果保存在/build/results路径下。

关闭图片保存时，1684X PCIe上推理 1 路图片运行结果如下，PCIe上的性能由于CPU的不同可能存在较大差异：

```bash
 total time cost 24724501 us.
frame count is 5007 | fps is 202.512 fps.
```

## 7. 性能测试

本例程只供流程参考，暂无最佳性能数据。


## 8. 使用application-web可视化

1.克隆 https://github.com/sophgo/application-web

2.根据application-web/README.md将application-web部署

3.参考[web_server](../../tools/web-server/README.md)搭建应用层程序

4.
```
cd ../../tools/web_server
bash start_server.sh
```
5.通过application-web下发任务，启动任务



### license_area_intrusion - README_EN.md

# license_plate_recognition Demo

English | [简体中文](README.md)

## Catalogs

- [license_plate_recognition Demo](#license_plate_recognition-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-compilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 Json Configuration Explanation](#61-json-configuration-explanation)
    - [6.2 Execution](#62-execution)
  - [7. Performance Testing](#7-performance-testing)


## 1. Introduction

This example is intended to illustrate how to use sophon-stream to quickly build license plate detection based on YOLOv5 and license plate recognition based on LPRNet.

**LPRNET License Plate Detection Source Code** (https://github.com/sirius-ai/LPRNet_Pytorch)

In this example, the preprocessing, inference, and post-processing of the YOLOv5 and LPRNet algorithms are separately computed on three elements. Multiple threads can be enabled within each element to ensure a certain level of detection efficiency.

## 2. Features

- Support for BM1684X, BM1684 (x86 PCIe, SoC), BM1688 (SoC)
- Support for multiple video streams
- Support for multi-threading

## 3. Prepare Models and Data

In the `scripts` directory, download scripts for relevant models and data are provided in [download.sh](./scripts/download.sh).

```bash
# Install 7z, unzip. Skip if already installed. For non-Ubuntu systems, use yum or other methods as appropriate.
sudo apt install p7zip
sudo apt install p7zip-full
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, the data and models directories will be generated in the current directory. The data directory contains the vehicle dataset, while the models directory contains the model files for YOLOv5 and LPRNet.

The downloaded models include:

```bash
./models/
├── lprnet
│   ├── BM1684
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1684X
│   │   ├── lprnet_fp16_1b.bmodel
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1688
│   │   ├── lprnet_fp32_1b.bmodel
│   │   └── lprnet_int8_1b.bmodel
│   ├── onnx
│   │   ├── lprnet_1b.onnx
│   │   └── lprnet_4b.onnx
│   └── torch
│       ├── Final_LPRNet_model.pth
│       └── LPRNet_model.torchscript
└── yolov5s-licensePLate
    ├── BM1684
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
    ├── BM1684X
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
    └── BM1688
        ├── yolov5s_v6.1_license_3output_fp32_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_4b_2core.bmodel
        └── yolov5s_v6.1_license_3output_int8_4b.bmodel
```

Model Description:

The LPRNet model mentioned above is ported from [LNRNet_Pytorch](https://github.com/sirius-ai/LPRNet_Pytorch). The yolov5s-licensePLate model is trained on the Green License Plate dataset.

Downloaded data includes:

```bash
./datasets
├── coco.names
└── test // Vehicle dataset for testing
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 Json Configuration Explanation

Various parameters in the license_plate_recognition demo are located in the [config](./config/) directory, structured as follows:

```bash
./config
├── converger.json
├── decode.json
├── distributor_time_class.json
├── engine_group.json
├── engine.json
├── license_plate_recognition_demo.json
├── lprnet_group.json
├── lprnet_infer.json
├── lprnet_post.json
├── lprnet_pre.json
├── yolov5_group.json
├── yolov5_infer.json
├── yolov5_post.json
└── yolov5_pre.json
```

Among them, [license_plate_recognition.json](./config/license_plate_recognition.json) is the overall configuration file for the example, managing information such as input streams. Multiple channels can be supported on a single image, with the channels parameter configuring the number of input channels, sample_interval setting the frame skip interval, and loop_num setting the number of loop plays. The channel section includes information such as stream URL.

In cases where the channel_id property is not specified in the configuration file, the demo will default to assigning channel_id for each data channel, starting from 0.

```json
{
    "channels": [
      {
        "channel_id": 0,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 1,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 2,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 3,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      }
    ],
    "class_names": "../license_plate_recognition/data/coco.names",
    "download_image": true,
    "draw_func_name": "draw_license_plate_recognition_results",
    "engine_config_path": "../license_plate_recognition/config/engine_group.json"
}
```

[engine.json](./config/engine.json) contains configuration information for the graph, and this configuration is unlikely to change once determined.

In this file, it is necessary to initialize information for each element and the connection between elements. The element_id is unique and serves as an identifier. The element_config points to the detailed configuration file address of the element, port_id is the input/output port number of the element, and in cases of multiple inputs or outputs, input/output numbers should not be duplicated. is_src flags whether the current port is the input port for the entire graph, and is_sink identifies whether the current port is the output port for the entire graph. 
Connection specifies the connection method between all elements, determined by element_id and port_id.

Configuration files like [lprnet_pre.json](./config/lprnet_pre.json) detail the configuration for a specific element, setting model parameters, dynamic library paths, thresholds, and other information. This configuration file does not need to specify the `id` and `device_id` fields; the example will pass the `element_id` and `device_id` specified in `engine.json`. Among them, `thread_number` is the number of working threads within the element; one thread corresponds to one data queue. In the case of multiple inputs, it is necessary to set the number of data queues reasonably to ensure that the thread workload is even and reasonable.

```json
{
    "configure": {
        "model_path": "../license_plate_recognition/models/lprnet/BM1684X/lprnet_fp32_1b.bmodel",
        "stage": [
            "pre"
        ]
    },
    "shared_object": "../../build/lib/liblprnet.so",
    "name": "lprnet",
    "side": "sophgo",
    "thread_number": 1
}
```

### 6.2 Execution

For the PCIe platform, you can directly run the test on the PCIe platform. For the SoC platform, you need to copy the dynamically linked libraries, executable files, required models, and test data generated by cross-compilation to the SoC platform for testing.

On the SoC platform, the directory structure of dynamic libraries, executable files, configuration files, models, and video data should be consistent with the original sophon-stream repository.

The parameters and running methods for testing are the same. The following mainly introduces the PCIe mode.

1. Run the executable file, pay attention to providing the region.
```bash
./main --demo_config_path=../license_plate_recognition/config/license_plate_recognition_demo.json
```

The inference results are saved in the /build/results path.

When saving images is disabled, the inference result for 1 image on 1684X PCIe is as follows. The performance on PCIe may vary significantly due to different CPUs:

```bash
 total time cost 24724501 us.
frame count is 5007 | fps is 202.512 fps.
```

## 7. Performance Testing
This example is for reference in the process and currently does not have optimal performance data.


---

## license_plate_recognition

# license_plate_recognition Demo

[English](README_EN.md) | 简体中文

## 目录

- [license\_plate\_recognition Demo](#license_plate_recognition-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe 平台](#41-x86arm-pcie-平台)
    - [4.2 SoC 平台](#42-soc-平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe 平台](#51-x86arm-pcie-平台)
    - [5.2 SoC 平台](#52-soc-平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json 配置说明](#61-json-配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用 sophon-stream 快速构建基于 yolov5 的车牌检测和基于 lprnet 的车牌识别。

**LPRNET 车牌检测源代码**(https://github.com/sirius-ai/LPRNet_Pytorch)

本例程中，yolov5、lprnet 算法的前处理、推理、后处理均分别在三个 element 上进行运算，element 内部可以开启多个线程，保证了一定的检测效率。

## 2. 特性

- 支持 BM1684X、BM1684(x86 PCIe、SoC)、BM1688(arm PCIe、SoC)
- 支持多路视频流
- 支持多线程

## 3. 准备模型与数据

​ 在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装7z、unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install p7zip
sudo apt install p7zip-full
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`、`models`目录，其中，`data`存放车辆数据集，`models`存放 yolov5 和 lprnet 的模型文件。

下载的模型包括：

```bash
./models/
├── lprnet
│   ├── BM1684
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1684X
│   │   ├── lprnet_fp16_1b.bmodel
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1688
│   │   ├── lprnet_fp32_1b.bmodel
│   │   └── lprnet_int8_1b.bmodel
│   ├── onnx
│   │   ├── lprnet_1b.onnx
│   │   └── lprnet_4b.onnx
│   └── torch
│       ├── Final_LPRNet_model.pth
│       └── LPRNet_model.torchscript
└── yolov5s-licensePLate
    ├── BM1684
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_4b.bmodel
    ├── BM1684X
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_4b.bmodel
    └── BM1688
        ├── yolov5s_v6.1_license_3output_fp32_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_4b_2core.bmodel
        └── yolov5s_v6.1_license_3output_int8_4b.bmodel
```

模型说明:

以上 lprnet 模型移植于[LNRNet_Pytorch](https://github.com/sirius-ai/LPRNet_Pytorch) , yolov5s-licensePLate 模型基于绿色车牌数据集训练。

下载的数据包括：

```bash
./datasets
├── coco.names
├── wqy-microhei.ttc // 字体文件
└── test // 用于测试的车辆数据集
```

## 4. 环境准备

### 4.1 x86/arm PCIe 平台

如果您在 x86/arm 平台安装了 PCIe 加速卡（如 SC 系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装 libsophon、sophon-opencv 和 sophon-ffmpeg，具体步骤可参考[x86-pcie 平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie 平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC 平台

如果您使用 SoC 平台（如 SE、SM 系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的 libsophon、sophon-opencv 和 sophon-ffmpeg 运行库包，可直接使用它作为运行环境。通常还需要一台 x86 主机作为开发环境，用于交叉编译 C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe 平台

可以直接在 PCIe 平台上编译程序，具体请参考[sophon-stream 编译](../../docs/HowToMake.md)。

### 5.2 SoC 平台

通常在 x86 主机上交叉编译程序，您需要在 x86 主机上使用 SOPHON SDK 搭建交叉编译环境，将程序所依赖的头文件和库文件打包至 sophon_sdk_soc 目录中，具体请参考[sophon-stream 编译](../../docs/HowToMake.md)。本例程主要依赖 libsophon、sophon-opencv 和 sophon-ffmpeg 运行库包。

## 6. 程序运行

### 6.1 Json 配置说明

license_plate_recognition demo 中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config
├── converger.json
├── decode.json
├── distributor_time_class.json
├── engine_group.json
├── engine.json
├── license_plate_recognition_demo.json
├── lprnet_group.json
├── lprnet_infer.json
├── lprnet_post.json
├── lprnet_pre.json
├── yolov5_group.json
├── yolov5_infer.json
├── yolov5_post.json
└── yolov5_pre.json
```

其中，[license_plate_recognition_demo.json](./config/license_plate_recognition_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels 参数配置输入的路数，loop_num 设置循环播放次数，channel 中包含码流 url 等信息。

配置文件中不指定`channel_id`属性的情况，会在 demo 中对每一路数据的`channel_id`从 0 开始默认赋值。

```json
{
    "channels": [
      {
        "channel_id": 0,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 1,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 2,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 3,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      }
    ],
    "class_names": "../license_plate_recognition/data/coco.names",
    "download_image": true,
    "draw_func_name": "draw_license_plate_recognition_results",
    "engine_config_path": "../license_plate_recognition/config/engine_group.json"
}
```

[engine.json](./config/engine.json)包含对 graph 的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即删除`channels`的部分元素，再进行测试。

在该文件内，需要初始化每个 element 的信息和 element 之间的连接方式。element_id 是唯一的，起到标识身份的作用。element_config 指向该 element 的详细配置文件地址，port_id 是该 element 的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src 标志当前端口是否是整张图的输入端口，is_sink 标识当前端口是否是整张图的输出端口。
connection 是所有 element 之间的连接方式，通过 element_id 和 port_id 确定。

[lprnet_pre.json](./config/lprnet_pre.json)等配置文件是对具体某个 element 的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

```json
{
    "configure": {
        "model_path": "../license_plate_recognition/models/lprnet/BM1684X/lprnet_fp32_1b.bmodel",
        "stage": [
            "pre"
        ]
    },
    "shared_object": "../../build/lib/liblprnet.so",
    "name": "lprnet",
    "side": "sophgo",
    "thread_number": 1
}
```

### 6.2 运行

对于 PCIe 平台，可以直接在 PCIe 平台上运行测试；对于 SoC 平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到 SoC 平台中测试。

SoC 平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始 sophon-stream 仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以 PCIe 模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../license_plate_recognition/config/license_plate_recognition_demo.json
```

推理结果保存在/build/results路径下。

关闭图片保存时，1684X PCIe上推理 1 路图片运行结果如下，PCIe上的性能由于CPU的不同可能存在较大差异：

```bash
 total time cost 24724501 us.
frame count is 5007 | fps is 202.512 fps.
```

## 7. 性能测试

本例程只供流程参考，暂无最佳性能数据。


### license_plate_recognition - README_EN.md

# license_plate_recognition Demo

English | [简体中文](README.md)

## Catalogs

- [license\_plate\_recognition Demo](#license_plate_recognition-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-compilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 Json Configuration Explanation](#61-json-configuration-explanation)
    - [6.2 Execution](#62-execution)
  - [7. Performance Testing](#7-performance-testing)


## 1. Introduction

This example is intended to illustrate how to use sophon-stream to quickly build license plate detection based on YOLOv5 and license plate recognition based on LPRNet.

**LPRNET License Plate Detection Source Code** (https://github.com/sirius-ai/LPRNet_Pytorch)

In this example, the preprocessing, inference, and post-processing of the YOLOv5 and LPRNet algorithms are separately computed on three elements. Multiple threads can be enabled within each element to ensure a certain level of detection efficiency.

## 2. Features

- Support for BM1684X, BM1684 (x86 PCIe, SoC), BM1688 (arm PCIe、SoC)
- Support for multiple video streams
- Support for multi-threading

## 3. Prepare Models and Data

In the `scripts` directory, download scripts for relevant models and data are provided in [download.sh](./scripts/download.sh).

```bash
# Install 7z, unzip. Skip if already installed. For non-Ubuntu systems, use yum or other methods as appropriate.
sudo apt install p7zip
sudo apt install p7zip-full
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, the data and models directories will be generated in the current directory. The data directory contains the vehicle dataset, while the models directory contains the model files for YOLOv5 and LPRNet.

The downloaded models include:

```bash
./models/
├── lprnet
│   ├── BM1684
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1684X
│   │   ├── lprnet_fp16_1b.bmodel
│   │   ├── lprnet_fp32_1b.bmodel
│   │   ├── lprnet_int8_1b.bmodel
│   │   └── lprnet_int8_4b.bmodel
│   ├── BM1688
│   │   ├── lprnet_fp32_1b.bmodel
│   │   └── lprnet_int8_1b.bmodel
│   ├── onnx
│   │   ├── lprnet_1b.onnx
│   │   └── lprnet_4b.onnx
│   └── torch
│       ├── Final_LPRNet_model.pth
│       └── LPRNet_model.torchscript
└── yolov5s-licensePLate
    ├── BM1684
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_4b.bmodel
    ├── BM1684X
    │   ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
    │   ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
    │   └── yolov5s_v6.1_license_3output_int8_4b.bmodel
    └── BM1688
        ├── yolov5s_v6.1_license_3output_fp32_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_1b.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_fp32_4b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b_2core.bmodel
        ├── yolov5s_v6.1_license_3output_int8_1b.bmodel
        ├── yolov5s_v6.1_license_3output_int8_4b_2core.bmodel
        └── yolov5s_v6.1_license_3output_int8_4b.bmodel
```

Model Description:

The LPRNet model mentioned above is ported from [LNRNet_Pytorch](https://github.com/sirius-ai/LPRNet_Pytorch). The yolov5s-licensePLate model is trained on the Green License Plate dataset.

Downloaded data includes:

```bash
./datasets
├── coco.names
└── test // Vehicle dataset for testing
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 Json Configuration Explanation

Various parameters in the license_plate_recognition demo are located in the [config](./config/) directory, structured as follows:

```bash
./config
├── converger.json
├── decode.json
├── distributor_time_class.json
├── engine_group.json
├── engine.json
├── license_plate_recognition_demo.json
├── lprnet_group.json
├── lprnet_infer.json
├── lprnet_post.json
├── lprnet_pre.json
├── yolov5_group.json
├── yolov5_infer.json
├── yolov5_post.json
└── yolov5_pre.json
```

Among them, [license_plate_recognition_demo.json](./config/license_plate_recognition_demo.json) is the overall configuration file for the example, managing information such as input streams. Multiple channels can be supported on a single image, with the channels parameter configuring the number of input channels, sample_interval setting the frame skip interval, and loop_num setting the number of loop plays. The channel section includes information such as stream URL.

In cases where the channel_id property is not specified in the configuration file, the demo will default to assigning channel_id for each data channel, starting from 0.

```json
{
    "channels": [
      {
        "channel_id": 0,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 1,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 2,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      },
      {
        "channel_id": 3,
        "url": "../license_plate_recognition/data/test",
        "source_type": "IMG_DIR",
        "loop_num": 1,
        "fps": -1
      }
    ],
    "class_names": "../license_plate_recognition/data/coco.names",
    "download_image": true,
    "draw_func_name": "draw_license_plate_recognition_results",
    "engine_config_path": "../license_plate_recognition/config/engine_group.json"
}
```

[engine.json](./config/engine.json) contains configuration information for the graph, and this configuration is unlikely to change once determined.

In this file, it is necessary to initialize information for each element and the connection between elements. The element_id is unique and serves as an identifier. The element_config points to the detailed configuration file address of the element, port_id is the input/output port number of the element, and in cases of multiple inputs or outputs, input/output numbers should not be duplicated. is_src flags whether the current port is the input port for the entire graph, and is_sink identifies whether the current port is the output port for the entire graph. 
Connection specifies the connection method between all elements, determined by element_id and port_id.

Configuration files like [lprnet_pre.json](./config/lprnet_pre.json) detail the configuration for a specific element, setting model parameters, dynamic library paths, thresholds, and other information. This configuration file does not need to specify the `id` and `device_id` fields; the example will pass the `element_id` and `device_id` specified in `engine.json`. Among them, `thread_number` is the number of working threads within the element; one thread corresponds to one data queue. In the case of multiple inputs, it is necessary to set the number of data queues reasonably to ensure that the thread workload is even and reasonable.

```json
{
    "configure": {
        "model_path": "../license_plate_recognition/models/lprnet/BM1684X/lprnet_fp32_1b.bmodel",
        "stage": [
            "pre"
        ]
    },
    "shared_object": "../../build/lib/liblprnet.so",
    "name": "lprnet",
    "side": "sophgo",
    "thread_number": 1
}
```

### 6.2 Execution

For the PCIe platform, you can directly run the test on the PCIe platform. For the SoC platform, you need to copy the dynamically linked libraries, executable files, required models, and test data generated by cross-compilation to the SoC platform for testing.

On the SoC platform, the directory structure of dynamic libraries, executable files, configuration files, models, and video data should be consistent with the original sophon-stream repository.

The parameters and running methods for testing are the same. The following mainly introduces the PCIe mode.

1. Run the executable file
```bash
./main --demo_config_path=../license_plate_recognition/config/license_plate_recognition_demo.json
```

The inference results are saved in the /build/results path.

When saving images is disabled, the inference result for 1 image on 1684X PCIe is as follows. The performance on PCIe may vary significantly due to different CPUs:

```bash
 total time cost 24724501 us.
frame count is 5007 | fps is 202.512 fps.
```

## 7. Performance Testing
This example is for reference in the process and currently does not have optimal performance data.


---

## line_crossing

# Line Crossing

## 目录
- [Line Crossing](#line-crossing)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建拌线检测算法应用；


## 2. 特性
* 检测模型使用yolox；
* 跟踪模型使用bytetrack；
* 支持BM1684X(x86 PCIe、SoC)，BM1684(x86 PCIe、SoC、arm PCIe)，BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本[download.sh](./scripts/download.sh)。

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

下载的模型包括：
```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684的INT8 BModel，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1688的bytetrack的FP32 BModel，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1688的bytetrack的INT8 BModel，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # 用于BM1688的INT8 BModel，batch_size=1
    └── yolox_s_int8_4b.bmodel              # 用于BM1688的INT8 BModel，batch_size=4
```
模型说明:

1.`yolox_bytetrack_s`系列模型移植于[bytetrack官方](https://github.com/ifzhang/ByteTrack)，插件配置`mean=[0,0,0]`，`std=[1,1,1]`，支持person类别的检测任务。

2.`yolox_s`系列模型移植于[yolox官方](https://github.com/Megvii-BaseDetection/YOLOX)，插件配置`mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`，支持COCO数据集的80分类检测任务。

下载的数据包括：
```bash
./data/test.mp4                           # 测试视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。


### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。


## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

配置文件位于 [./config](../line_crossing/config)

其中，[line_crossing.json](../line_crossing/config/line_crossing_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels中包含每一路码流url等信息。

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../data/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 2100000,
      "fps": 25,
      "sample_interval": 5
    }
  ],
  "engine_config_path": "../line_crossing/config/engine_group.json",
}
```

[engine.json](../line_crossing/config/engine.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即删去`channels`里的部分元素，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
  {
    "graph_id": 0,
    "device_id": 0,
    "graph_name": "yolox_osd_encode",
    "elements": [
      {
        "element_id": 5000,
        "element_config": "../tripwire/config/decode.json",
        "ports": {
          "input": [
            {
              "port_id": 0,
              "is_sink": false,
              "is_src": true
            }
          ]
        }
      },
      {
        "element_id": 5001,
        "element_config": "../tripwire/config/yolox_group.json",
        "inner_elements_id": [
          10001,
          10002,
          10003
        ]
      },
      {
        "element_id": 5004,
        "element_config": "../tripwire/config/bytetrack.json"
      },
      {
        "element_id": 5005,
        "element_config": "../tripwire/config/filter.json"
      },
      {
        "element_id": 5006,
        "element_config": "../tripwire/config/osd.json"
      },
      {
        "element_id": 5007,
        "element_config": "../tripwire/config/encode.json",
        "ports": {
          "output": [
            {
              "port_id": 0,
              "is_sink": true,
              "is_src": false
            }
          ]
        }
      }
    ],
    "connections": [
      {
        "src_element_id": 5000,
        "src_port": 0,
        "dst_element_id": 5001,
        "dst_port": 0
      },
      {
        "src_element_id": 5001,
        "src_port": 0,
        "dst_element_id": 5004,
        "dst_port": 0
      },
      {
        "src_element_id": 5004,
        "src_port": 0,
        "dst_element_id": 5005,
        "dst_port": 0
      },
      {
        "src_element_id": 5005,
        "src_port": 0,
        "dst_element_id": 5006,
        "dst_port": 0
      },
      {
        "src_element_id": 5006,
        "src_port": 0,
        "dst_element_id": 5007,
        "dst_port": 0
      }
    ]
  }
]
```

[filter.json](../line_crossing/config/filter.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。
其中，thread_number是element内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。
具体请参考[filter.json](../../element/tools/filter/README.md)
```json
{
  "configure": {
    "rules": [
      {
        "channel_id": 0,
        "filters": [
          {
            "alert_first_frame": 1,
            "alert_frame_skip_nums": 1,
            "areas": [
              [
                {
                  "left": 960,
                  "top": 0
                },
                {
                  "left": 960,
                  "top": 1080
                }
              ]
            ],
            "classes": [
              0
            ],
            "times": [
              {
                "time_start": "00 00 00",
                "time_end": "23 59 59"
              }
            ],
            "type": 1,
            "direction": [1,0],
            "trajectory_interval": 1
          }
        ]
      }
    ]
  },
  "shared_object": "../../build/lib/libfilter.so",
  "name": "filter",
  "side": "sophgo",
  "thread_number": 1
}
```

### 6.2 运行
对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件,注意要更改filter里面画出的线。
```bash
./main --demo_config_path=../line_crossing/config/line_crossing_demo.json
```

程序运行过程中会将触发越线警报的图片保存到`results`下，运行结果如下：
```bash
 total time cost 155246055 us.
frame count is 20 | fps is 0.128828 fps #表示这个视频总共有20帧触发了报警
```

## 7. 性能测试
由于涉及到筛选，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。
---

## multi_graph

# Multi-Graph Demo

[Englist](README_EN.md) | 简体中文

## 目录
- [Multi-Graph Demo](#Multi-Graph demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 运行](#4-运行)

## 1. 简介

本例程用于说明如何使用sophon-stream搭建多graph应用。

一般来说，sophon-stream的一个graph上所有输入码流只支持在相同的设备上执行相同的算法流程。因此当遇到不同码流需要做不同计算，或者不同码流在不同的设备上运行时，需要使用多graph来配置。

这里提供了一个简单的示例，Engine中包含两个Graph。Graph0流程为解码-检测-编码RTSP；Graph1流程为解码-检测-编码本地视频文件。

本例程插件连接方式如下图所示

![multi_graph](./pics/multi_graph.jpg)

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)，支持BM1688(SoC)
* BM1684X平台上，支持tpu_kernel后处理
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

本例程使用的数据来自yolov5例程，请参考[yolov5](../yolov5/README.md)下载数据

## 4. 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../multi_graph/config/multi_graph_demo.json
```


### multi_graph - README_EN.md

# Multi-Graph Demo

English | [简体中文](README.md)

## 目录
- [Multi-Graph Demo](#Multi-Graph demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Execute](#4-execute)

## 1. Introduction

This routine is used to illustrate how to build a multi-graph application using sophon-stream.

Generally speaking, all input streams on one graph of sophon-stream only support executing the same algorithm flow on the same device. Therefore, when encountering different code streams need to do different calculations, or different code streams run on different devices, you need to use multi-graph to configure it.

Here is a simple example, the Engine contains two Graphs: Graph0 is for decoding-detecting-encoding RTSP; Graph1 is for decoding-detecting-encoding local video files.

The plug-in connection of this routine is shown in the following figure

![multi_graph](./pics/multi_graph.jpg)

## 2. Feature

* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(SoC).
* Supports tpu_kernel post-process on BM1684X.
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

This sample uses data from the yolov5 sample, see [yolov5](../yolov5/README.md) to download the data

## 4. Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../multi_graph/config/multi_graph_demo.json
```

---

## openpose

# OPENPOSE Demo

[English](README_EN.md) | 简体中文

## 目录
- [OPENPOSE Demo](#openpose-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频姿态识别应用。

**源代码** (https://github.com/CMU-Perceptual-Computing-Lab/openpose) 

本例程中，openpose算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)
* 支持多路视频流
* 支持多线程
* BM1684X平台上，支持tpu_kernel后处理

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：

```bash
./models
├── BM1684
│   ├── pose_coco_fp32_1b.bmodel              # 使用TPU-NNTC编译，用于BM1684的FP32 BModel，batch_size=1，18个身体关键点识别
│   ├── pose_coco_int8_1b.bmodel              # 使用TPU-NNTC编译，用于BM1684的INT8 BModel，batch_size=1，18个身体关键点识别
│   ├── pose_coco_int8_4b.bmodel              # 使用TPU-NNTC编译，用于BM1684的INT8 BModel，batch_size=4，18个身体关键点识别
│   └── pose_body_25_fp32_1b.bmodel           # 使用TPU-NNTC编译，用于BM1684的FP32 BModel，batch_size=1，25个身体关键点识别
└── BM1684X
    ├── pose_coco_fp32_1b.bmodel              # 使用TPU-MLIR编译，用于BM1684X的FP32 BModel，batch_size=1，18个身体关键点识别
    ├── pose_coco_fp16_1b.bmodel              # 使用TPU-MLIR编译，用于BM1684X的FP16 BModel，batch_size=1，18个身体关键点识别
    ├── pose_coco_int8_1b.bmodel              # 使用TPU-MLIR编译，用于BM1684X的INT8 BModel，batch_size=1，18个身体关键点识别
    ├── pose_coco_int8_4b.bmodel              # 使用TPU-MLIR编译，用于BM1684X的INT8 BModel，batch_size=4，18个身体关键点识别
    ├── pose_body_25_fp32_1b.bmodel           # 使用TPU-MLIR编译，用于BM1684X的FP32 BModel，batch_size=1，25个身体关键点识别
    └── pose_body_25_fp16_1b.bmodel           # 使用TPU-MLIR编译，用于BM1684X的FP16 BModel，batch_size=1，25个身体关键点识别
```

下载的数据包括：
```bash
./videos
└── test.mp4                                  # 测试视频                                    
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

openpose demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine_group.json           # sophon-stream简化的graph配置
├── engine.json                 # sophon-stream graph配置，需要分别配置前处理、推理和后处理文件
├── openpose_demo.json          # demo输入配置文件
├── openpose_group.json         # 简化的openpose配置文件，将前处理、推理、后处理合到一个配置文件中
├── openpose_infer.json         # openpose 推理配置文件
├── openpose_post.json          # openpose 后处理配置文件
└── openpose_pre.json           # openpose 预处理配置文件
```

其中，[openpose_demo.json](./config/openpose_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    }
  ],
  "download_image": false,
  "draw_func_name": "draw_openpose_results",
  "engine_config_path": "../openpose/config/engine_group.json"
}
```

[engine.json](./config/engine.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "openpose",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../openpose/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../openpose/config/openpose_pre.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5002,
                "element_config": "../openpose/config/openpose_infer.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5003,
                "element_config": "../openpose/config/openpose_post.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5002,
                "dst_port": 0
            },
            {
                "src_element_id": 5002,
                "src_port": 0,
                "dst_element_id": 5003,
                "dst_port": 0
            }
        ]
    }
]
```

[openpose_pre.json](./config/openpose_pre.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

`use_tpu_kernel`为`true`时，会使用tpu_kernel后处理。tpu_kernel后处理只支持BM1684X设备。

```json
{
    "configure": {
        "model_path": "../openpose/data/models/BM1684X/pose_coco_int8_1b.bmodel",
        "threshold_nms": 0.05,
        "stage": [
            "pre"
        ],
        "use_tpu_kernel": true
    },
    "shared_object": "../../build/lib/libopenpose.so",
    "name": "openpose",
    "side": "sophgo",
    "thread_number": 2
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../openpose/config/openpose_demo.json
```

2路视频流运行结果如下
```bash
 total time cost 29882447 us.
frame count is 1432 | fps is 47.9211 fps.
```

## 7. 性能测试

目前，openpose例程支持在BM1684X和BM1684的PCIE、SOC模式下进行推理。

测试视频`test.mp4`，编译选项为Release模式，结果如下:

| 设备   | 路数 | 算法线程数 | CPU利用率(%) | 系统内存(M) | 系统内存峰值(M) | TPU利用率(%) | 设备内存(M) | 设备内存峰值(M) | 平均FPS | 峰值FPS | 模型                     |
| ------ | ---- | ---------- | ------------ | ----------- | ---------------- | ------------ | ----------- | --------------- | ------- | -------- | ------------------------ |
| SE7    | 8    | 8-8-8      | 453.26       | 329.96      | 471.59           | 99.36        | 1301.89     | 1330.00         | 91.12   | 115.94   | pose_coco_int8_1b.bmodel |
| SE7    | 4    | 4-4-4      | 430.31       | 192.94      | 278.14           | 97.72        | 715.61      | 730.00          | 88.84   | 105.93   | pose_coco_int8_1b.bmodel |
| SE7    | 2    | 2-2-2      | 383.83       | 117.92      | 165.36           | 91.80        | 422.17      | 429.00          | 81.00   | 92.73    | pose_coco_int8_1b.bmodel |
| SE5-16 | 4    | 4-4-4      | 571.45       | 100.10      | 101.43           | 99.78        | 703.38      | 743.00          | 61.16   | 68.92    | pose_coco_int8_4b.bmodel |
| SE5-8  | 3    | 3-3-3      | 324.26       | 80.91       | 82.16            | 99.68        | 522.47      | 553.00          | 36.14   | 39.37    | pose_coco_int8_4b.bmodel |
> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；


### openpose - README_EN.md

# OPENPOSE Demo

English | [简体中文](README.md)

## Catalogs

## 1. Introduction
- [OPENPOSE Demo](#openpose-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

This example demonstrates how to use sophon-stream to quickly build a video pose recognition application.

**source code** (https://github.com/CMU-Perceptual-Computing-Lab/openpose)

In this example, the pre-processing, inference, and post-processing of the openpose algorithm are computed on three separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Features

* Supports BM1684X, BM1684(x86, PCIe, SoC)
* Supports multiple video streams.
* Supports multi-threading.
* On the BM1684X platform, the TPU_kernel post-processing is supported.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models
├── BM1684
│   ├── pose_coco_fp32_1b.bmodel              # Compile with TPU-NNTC，FP32 BModel for BM1684，batch_size=1，18 body keypoints
│   ├── pose_coco_int8_1b.bmodel              # Compile with TPU-NNTC，INT8 BModel for BM1684，batch_size=1，18 body keypoints
│   ├── pose_coco_int8_4b.bmodel              # Compile with TPU-NNTC，INT8 BModel for BM1684，batch_size=4，18 body keypoints
│   └── pose_body_25_fp32_1b.bmodel           # Compile with TPU-NNTC，FP32 BModel for BM1684，batch_size=1，25 body keypoints
└── BM1684X
    ├── pose_coco_fp32_1b.bmodel              # Compile with TPU-MLIR，FP32 BModel for BM1684X，batch_size=1，18 body keypoints
    ├── pose_coco_fp16_1b.bmodel              # Compile with TPU-MLIR，FP16 BModel for BM1684X，batch_size=1，18 body keypoints
    ├── pose_coco_int8_1b.bmodel              # Compile with TPU-MLIR，INT8 BModel for BM1684X，batch_size=1，18 body keypoints
    ├── pose_coco_int8_4b.bmodel              # Compile with TPU-MLIR，INT8 BModel for BM1684X，batch_size=4，18 body keypoints
    ├── pose_body_25_fp32_1b.bmodel           # Compile with TPU-MLIR，FP32 BModel for BM1684X，batch_size=1，25 body keypoints
    └── pose_body_25_fp16_1b.bmodel           # Compile with TPU-MLIR，FP16 BModel for BM1684X，batch_size=1，25 body keypoints
```

The downloaded data include:

下载的数据包括：
```bash
./videos
└── test.mp4                                  # test video                               
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the openpose demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config/
├── decode.json                 # decoding configuration
├── engine_group.json           # sophon-stream simplified graph configuration
├── engine.json                 # sophon-stream graph configuration requires separate configuration for pre-processing, inference, and post-processing files.
├── openpose_demo.json          # input configuration file for the demo
├── openpose_group.json         # A simplified openpose configuration file that combines pre-processing, inference, and post-processing into one configuration file.
├── openpose_infer.json         # openpose inference configuration file
├── openpose_post.json          # openpose post-processing configuration file
└── openpose_pre.json           # openpose pre-processing configuration file
```

Indeed, [openpose_demo.json](./config/openpose_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../openpose/data/videos/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": -1
    }
  ],
  "download_image": false,
  "draw_func_name": "draw_openpose_results",
  "engine_config_path": "../openpose/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "openpose",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../openpose/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../openpose/config/openpose_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[openpose_group.json](./config/openpose_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

When `use_tpu_kernel` is set to `true`, it will utilize the tpu_kernel post-processing(using tpu to do post process). Note that tpu_kernel post-processing is only supported on BM1684X devices.

```json
{
    "configure": {
        "model_path": "../openpose/data/models/BM1684X/pose_coco_int8_1b.bmodel",
        "threshold_nms": 0.05,
        "stage": [
            "pre"
        ],
        "use_tpu_kernel": true
    },
    "shared_object": "../../build/lib/libopenpose.so",
    "name": "openpose",
    "side": "sophgo",
    "thread_number": 2
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../openpose/config/openpose_demo.json
```

The running results of two video streams are as follows
```bash
 total time cost 29882447 us.
frame count is 1432 | fps is 47.9211 fps.
```

## 7. Performance Testing

Currently, the openpose sample supports inference in the PCIE, SOC modes of the BM1684X and BM1684.

The tested video is `test.mp4`. The compilation was done in Release mode. The results are as follows:

| Device   | Number of Channels | Alorithm Thread Count | CPU Utilization(%) | System Memory(M) | Peak System Memory(M) | TPU Utilization(%) | Device Memory(M) | Peak Device Memory(M) | Average FPS | Peak FPS | Model                     |
| ------ | ---- | ---------- | ------------ | ----------- | ---------------- | ------------ | ----------- | --------------- | ------- | -------- | ------------------------ |
| SE7    | 8    | 8-8-8      | 453.26       | 329.96      | 471.59           | 99.36        | 1301.89     | 1330.00         | 91.12   | 115.94   | pose_coco_int8_1b.bmodel |
| SE7    | 4    | 4-4-4      | 430.31       | 192.94      | 278.14           | 97.72        | 715.61      | 730.00          | 88.84   | 105.93   | pose_coco_int8_1b.bmodel |
| SE7    | 2    | 2-2-2      | 383.83       | 117.92      | 165.36           | 91.80        | 422.17      | 429.00          | 81.00   | 92.73    | pose_coco_int8_1b.bmodel |
| SE5-16 | 4    | 4-4-4      | 571.45       | 100.10      | 101.43           | 99.78        | 703.38      | 743.00          | 61.16   | 68.92    | pose_coco_int8_4b.bmodel |
| SE5-8  | 3    | 3-3-3      | 324.26       | 80.91       | 82.16            | 99.68        | 522.47      | 553.00          | 36.14   | 39.37    | pose_coco_int8_4b.bmodel |

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
---

## ppocr

# PP-OCR Demo
[English](README_EN.md) | 简体中文
## 目录
- [PP-OCR Demo](#PP-OCR-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

PP-OCRv3，是百度飞桨团队开源的超轻量OCR系列模型，包含文本检测、文本分类、文本识别模型，是PaddleOCR工具库的重要组成之一。支持中英文数字组合识别、竖排文本识别、长文本识别，其性能及精度较之前的PP-OCR版本均有明显提升。本例程对[PaddleOCR-release-2.6](https://github.com/PaddlePaddle/PaddleOCR/tree/release/2.6)的`ch_PP-OCRv3_xx`系列模型和算法进行移植，使之能在SOPHON BM1684/BM1684X/BM1688上进行推理测试。


## 2. 特性
* 支持BM1684、BM1684X(x86 PCIe、SoC)，BM1688(SoC)
* 支持FP32、FP16模型编译和推理
* 支持图片数据集测试

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`、`datasets`

下载的模型包括：

```bash
├── BM1684
│   ├── ch_PP-OCRv3_det_fp32_1b.bmodel
│   ├── ch_PP-OCRv3_rec_fp32_1b_320.bmodel
│   └── ch_PP-OCRv3_rec_fp32_1b_640.bmodel
├── BM1684X
│   ├── ch_PP-OCRv3_det_fp16_1b.bmodel
│   ├── ch_PP-OCRv3_det_fp32_1b.bmodel
│   ├── ch_PP-OCRv3_rec_fp16_1b_320.bmodel
│   ├── ch_PP-OCRv3_rec_fp16_1b_640.bmodel
│   ├── ch_PP-OCRv3_rec_fp32_1b_320.bmodel
│   └── ch_PP-OCRv3_rec_fp32_1b_640.bmodel
└── BM1688
    ├── ch_PP-OCRv3_det_fp16_1b.bmodel
    ├── ch_PP-OCRv3_det_fp32_1b.bmodel
    ├── ch_PP-OCRv3_rec_fp16_1b_320.bmodel
    ├── ch_PP-OCRv3_rec_fp16_1b_640.bmodel
    ├── ch_PP-OCRv3_rec_fp32_1b_320.bmodel
    └── ch_PP-OCRv3_rec_fp32_1b_640.bmodel
```

下载的数据包括：
```bash
data
├── class.names
├── datasets
│   ├── ppocr_keys_v1.txt
│   ├── train_full_images_0
│   └── train_full_images_0.json
├── models
└── wqy-microhei.ttc

```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

ppocr demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config
├── converger.json                # 汇聚插件配置
├── decode.json                   # 解码插件配置
├── distributor_frame_class.json  # 分发插件配置
├── engine_group.json             # sophon-stream graph配置
├── ppocr_demo.json               # ppocr demo配置
├── ppocr_det_group.json          # 检测插件配置
└── ppocr_rec_group.json          # 识别插件配置
```

其中，[ppocr_demo.json](./config/ppocr_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "engine_config_path": "../ppocr/config/engine_group.json",
  "draw_func_name": "draw_ppocr_results",
  "download_image": true
}
```

[engine_group.json](./config/engine_group.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 1,
        "graph_name": "ppocr",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../ppocr/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../ppocr/config/ppocr_det_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../ppocr/config/distributor_frame_class.json"
            },
            {
                "element_id": 6001,
                "element_config": "../ppocr/config/ppocr_rec_group.json",
                "inner_elements_id": [20001, 20002, 20003]
            },
            {
                "element_id": 5005,
                "element_config": "../ppocr/config/converger.json",
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 1,
                "dst_element_id": 6001,
                "dst_port": 0
            },
            {
                "src_element_id": 6001,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 1
            }


        ]
    }
]
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../ppocr/config/ppocr_demo.json
```

2路视频流运行结果如下
```bash
 total time cost 376103640 us.
frame count is 10560 | fps is 28.0774 fps.
```

## 7. 性能测试
目前，ppocr例程支持在BM1684X和BM1684的PCIE、SOC模式下进行推理，支持BM1688 SOC模式下推理。

在不同的设备上可能需要修改json配置，例如模型路径、输入路数等。json的配置方法参考6.1节，程序运行方法参考上文6.2节。

由于PCIE设备cpu能力差距较大，性能数据没有参考意义，这里只给出SOC模式的测试结果。

测试图片集train_full_images_0，编译选项为Release模式，结果如下:

|设备|模型|路数|算法线程数|CPU利用率(%)|系统内存(M)|TPU利用率(%)|设备内存(M)|平均FPS|
|----|----|----|-----|-----|-----|-----|-----|-----|
|SE5-16|fp32|1|1-1-1|227.8|410.1|90|1066|23.14|
|SE7|fp32|1|1-1-1|313.6|438.0|88|1085|38.67|
|SE7|fp16|2|2-2-2|519.6|445.1 |52|1046|66.96|



> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；
3. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;
4. BM1688设备暂无性能测试。
5. 这个测试数据中，SE5 sdk版本为0.4.8，SE7 sdk版本为0.4.9，不同版本测试结果可能有差异。


### ppocr - README_EN.md

# PP-OCR Demo
English | [简体中文](README.md)
## Catalogs
- [PP-OCR Demo](#PP-OCR-demo)
  - [Directory](#Catalogs)
  - [1. Introduction](#1-Introduction)
  - [2. Feature](#2-Feature)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

PP-OCRv3 is an ultra-lightweight OCR series model open source by Baidu Feizhu team, including text detection, text classification and text recognition models, and is one of the important components of PaddleOCR tool library. It supports Chinese and English digit combination recognition, vertical text recognition, long text recognition, and its performance and accuracy are significantly improved compared with the previous PP-OCR version. This demo includes the model and algorithm from[PaddleOCR-release-2.6](https://github.com/PaddlePaddle/PaddleOCR/tree/release/2.6) with `ch_PP-OCRv3_xx`.


## 2. Feature
* Supports BM1684,BM1684X(x86 PCIe、SoC),BM1688(SoC)
* Support FP32, FP16 model compilation and inference
* Support image data set testing

## 3. Prepare Models and Data

The scripts directory contains download scripts for relevant models and data [download.sh](./scripts/download.sh)。

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, data directory will be generated in the current directory, containing two subdirectories: models and datasets

The downloaded models include：

```bash
├── BM1684
│   ├── ch_PP-OCRv3_det_fp32_1b.bmodel
│   ├── ch_PP-OCRv3_rec_fp32_1b_320.bmodel
│   └── ch_PP-OCRv3_rec_fp32_1b_640.bmodel
├── BM1684X
│   ├── ch_PP-OCRv3_det_fp16_1b.bmodel
│   ├── ch_PP-OCRv3_det_fp32_1b.bmodel
│   ├── ch_PP-OCRv3_rec_fp16_1b_320.bmodel
│   ├── ch_PP-OCRv3_rec_fp16_1b_640.bmodel
│   ├── ch_PP-OCRv3_rec_fp32_1b_320.bmodel
│   └── ch_PP-OCRv3_rec_fp32_1b_640.bmodel
└── BM1688
    ├── ch_PP-OCRv3_det_fp16_1b.bmodel
    ├── ch_PP-OCRv3_det_fp32_1b.bmodel
    ├── ch_PP-OCRv3_rec_fp16_1b_320.bmodel
    ├── ch_PP-OCRv3_rec_fp16_1b_640.bmodel
    ├── ch_PP-OCRv3_rec_fp32_1b_320.bmodel
    └── ch_PP-OCRv3_rec_fp32_1b_640.bmodel
```

The downloaded datasets include：
```bash
data
├── class.names
├── datasets
│   ├── ppocr_keys_v1.txt
│   ├── train_full_images_0
│   └── train_full_images_0.json
├── models
└── wqy-microhei.ttc

```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the ppocr demo, various parameters for each section are located in config directory, structured as follows:"

```bash
./config
├── converger.json                # converger configuration
├── decode.json                   # decoder configuration
├── distributor_frame_class.json  # distributor configuration
├── engine_group.json             # sophon-stream graph configuration
├── ppocr_demo.json               # ppocr demo configuration
├── ppocr_det_group.json          # detection configuration
└── ppocr_rec_group.json          # recognition configuration
```

[ppocr_demo.json](./config/ppocr_demo.json) is the overall configuration file of the routine, managing information such as input stream. Multiple data input can be supported on a single graph. The channels parameter configs the number of input channels, and the channel contains information such as stream url.

If the 'channel_id' attribute is not specified in the configuration file, the demo assigns a default value of 'channel_id' starting from 0 for each data line.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../ppocr/data/train_full_images_0",
      "source_type": "IMG_DIR",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "engine_config_path": "../ppocr/config/engine_group.json",
  "draw_func_name": "draw_ppocr_results",
  "download_image": true
}
```

[engine_group.json](./config/engine_group.json) contains the configuration information for the graph, which is basically unchanged after it is determined.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 1,
        "graph_name": "ppocr",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../ppocr/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../ppocr/config/ppocr_det_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../ppocr/config/distributor_frame_class.json"
            },
            {
                "element_id": 6001,
                "element_config": "../ppocr/config/ppocr_rec_group.json",
                "inner_elements_id": [20001, 20002, 20003]
            },
            {
                "element_id": 5005,
                "element_config": "../ppocr/config/converger.json",
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 1,
                "dst_element_id": 6001,
                "dst_port": 0
            },
            {
                "src_element_id": 6001,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 1
            }


        ]
    }
]
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../ppocr/config/ppocr_demo.json
```
## 7. Performance Testing

Currently, the ppocr example supports inference on BM1684X and BM1684 in PCIe and SOC modes, and BM1688 SOC mode.

Modifications in JSON configurations might be necessary when switching between different devices, such as adjusting model paths, input channels, etc. Refer to section 6.1 for JSON configuration methods and section 6.2 for program execution methods.

Due to significant differences in CPU capabilities among PCIe devices, performance data is not meaningful. Therefore, only provide the test results for SOC mode.

The test dataset is train_full_images_0，The compilation was done in Release mode. The results are as follows:

|Device|Model|Number of Channels|Algorithm Thread Count|CPU Utilization(%)|System Memory(M)|TPU Utilization(%)|Device Memory(M)|Average FPS|
|----|----|----|-----|-----|-----|-----|-----|-----|
|SE5-16|fp32|1|1-1-1|227.8|410.1|90|1066|23.14|
|SE7|fp32|1|1-1-1|313.6|438.0|88|1085|38.67|
|SE7|fp16|2|2-2-2|519.6|445.1 |52|1046|66.96|



> **Test Description**：
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
3. For the settings of input channels and algorithm thread count in the table, please refer to JSON configuration explanation. CPU utilization and system memory can be checked using the top command. TPU utilization and device memory can be checked using the bm-smi command. FPS can be obtained from the logs printed during program execution.
4. Performance testing is not currently available on the BM1688 device.
5. In this test data, SDK version on SE5 is 0.4.8; SDK version on SE7 is 0.4.9. Different version may bring different performance.
---

## resnet

# ResNet Demo

[English](README_EN.md) | 简体中文

## 目录
- [ResNet Demo](#resnet-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标分类应用。

深度残差网络（Deep residual network, ResNet）是由于Kaiming He等在2015提出的深度神经网络结构，它利用残差学习来解决深度神经网络训练退化的问题。

在此非常感谢Kaiming He, Xiangyu Zhang, Shaoqing Ren, Jian Sun等人的贡献。

**论文** (https://arxiv.org/abs/1512.03385)

## 2. 特性

* 支持BM1684X(x86 PCIe、SoC)和BM1684(x86 PCIe、SoC、arm PCIe)
* 支持FP32、FP16(BM1684X)、INT8模型编译和推理
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`、`videos`、`images`三个子目录。

下载的模型包括：

```bash
models/
├── BM1684
│   ├── resnet50_fp32_1b.bmodel                    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── resnet50_fp32_4b.bmodel                    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── resnet50_int8_1b.bmodel                    # 用于BM1684的INT8 BModel，batch_size=1
│   └── resnet50_int8_4b.bmodel                    # 用于BM1684的INT8 BModel，batch_size=4
└── BM1684X
    ├── resnet50_fp32_1b.bmodel                    # 用于BM1684X的FP32 BModel，batch_size=1
    ├── resnet50_fp32_4b.bmodel                    # 用于BM1684X的FP32 BModel，batch_size=4
    ├── resnet50_fp16_1b.bmodel                    # 用于BM1684X的FP16 BModel，batch_size=1
    ├── resnet50_int8_1b.bmodel                    # 用于BM1684X的INT8 BModel，batch_size=1
    └── resnet50_int8_4b.bmodel                    # 用于BM1684X的INT8 BModel，batch_size=4
```

下载的数据包括：
```bash
data/
├── images
│   ├── imagenet_val_1k                    # imagenet测试图片, 共1000张
│   ├── pedestrian_gender                  # 行人性别分类测试图片
│   └── vehicle_color                      # 车辆颜色分类测试图片
└── video
    ├── test_imagenet.mp4                  # imagenet测试视频
    └── test_vehicle_color.mp4             # 车辆颜色分类测试视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

resnet demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config
├── decode.json             # 解码配置
├── engine.json             # sophon-stream graph配置
├── resnet_demo.json        # resnet demo配置
├── resnet_roi.json         # resnet roi配置
└── resnet.json             # resnet 插件配置
```

其中，[resnet_demo.json](./config/resnet_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    }
  ],
  "engine_config_path": "../resnet/config/engine.json"
}
```

[engine.json](./config/engine.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "resnet",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../resnet/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../resnet/config/resnet.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../resnet/config/resnet_demo.json
```

2路视频流运行结果如下
```bash
 total time cost 1986302 us.
frame count is 1000 | fps is 803.448 fps.
```

## 7. 性能测试

目前，resnet例程支持在BM1684、BM1684X的PCIE、SOC模式下进行推理。

测试视频`videos/test_imagenet.mp4`，编译选项为Release模式，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|系统内存(M)|系统内存峰值(M)|TPU利用率(%)|设备内存(M)|设备内存峰值(M)|平均FPS|峰值FPS|
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|8|357.18|24.93|31.98|95.95|83.41|95.00|1990.34|2222.02|
|SE5-16|8| 8 |128.98|19.52|20.68|99.30|73.93|90.00|713.51|739.36|
|SE5-8|8| 4 |81.38|19.56|20.43|94.38|52.12|61.00|448.69|462.06|

> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；


### resnet - README_EN.md

# ResNet Demo

English | [简体中文](README.md)

## Catalogs
- [ResNet Demo](#resnet-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object classification application.

Deep residual network (ResNet) is a deep neural network architecture due to Kaiming He et al. in 2015, which utilizes residual learning to solve the problem of deep neural network training degradation.

The contributions of Kaiming He, Xiangyu Zhang, Shaoqing Ren, Jian Sun, and others are greatly appreciated.

**Paper** (https://arxiv.org/abs/1512.03385)

## 2. Features

* Supports BM1684X(x86 PCIe、SoC) and BM1684(x86 PCIe、SoC、arm PCIe)
* Supports FP32, FP16(BM1684X) and INT8 models compilation and inference.
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing three subdirectories: `models`, `videos` and `images`

The downloaded models include:

```bash
models/
├── BM1684
│   ├── resnet_pedestrian_gender_fp32_1b.bmodel    # FP32 BModel for BM1684，batch_size=1
│   ├── resnet_pedestrian_gender_fp32_4b.bmodel    # FP32 BModel for BM1684，batch_size=4
│   ├── resnet_pedestrian_gender_int8_1b.bmodel    # INT8 BModel for BM1684，batch_size=1
│   ├── resnet_pedestrian_gender_int8_4b.bmodel    # INT8 BModel for BM1684，batch_size=4
│   ├── resnet_vehicle_color_fp32_1b.bmodel        # FP32 BModel for BM1684，batch_size=1
│   ├── resnet_vehicle_color_fp32_4b.bmodel        # FP32 BModel for BM1684，batch_size=4
│   ├── resnet_vehicle_color_int8_1b.bmodel        # INT8 BModel for BM1684，batch_size=1
│   ├── resnet_vehicle_color_int8_4b.bmodel        # INT8 BModel for BM1684，batch_size=4
│   ├── resnet50_fp32_1b.bmodel                    # FP32 BModel for BM1684，batch_size=1
│   ├── resnet50_fp32_4b.bmodel                    # FP32 BModel for BM1684，batch_size=4
│   ├── resnet50_int8_1b.bmodel                    # INT8 BModel for BM1684，batch_size=1
│   └── resnet50_int8_4b.bmodel                    # INT8 BModel for BM1684，batch_size=4
└── BM1684X
    ├── resnet_pedestrian_gender_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
    ├── resnet_pedestrian_gender_fp32_4b.bmodel    # FP32 BModel for BM1684X，batch_size=4
    ├── resnet_pedestrian_gender_fp16_1b.bmodel    # FP16 BModel for BM1684X，batch_size=1
    ├── resnet_pedestrian_gender_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
    ├── resnet_pedestrian_gender_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
    ├── resnet_vehicle_color_fp32_1b.bmodel        # FP32 BModel for BM1684X，batch_size=1
    ├── resnet_vehicle_color_fp32_4b.bmodel        # FP32 BModel for BM1684X，batch_size=4
    ├── resnet_vehicle_color_fp16_1b.bmodel        # FP16 BModel for BM1684X，batch_size=1
    ├── resnet_vehicle_color_int8_1b.bmodel        # INT8 BModel for BM1684X，batch_size=1
    └── resnet_vehicle_color_int8_4b.bmodel        # INT8 BModel for BM1684X，batch_size=4
    ├── resnet50_fp32_1b.bmodel                    # FP32 BModel for BM1684X，batch_size=1
    ├── resnet50_fp32_4b.bmodel                    # FP32 BModel for BM1684X，batch_size=4
    ├── resnet50_fp16_1b.bmodel                    # FP16 BModel for BM1684X，batch_size=1
    ├── resnet50_int8_1b.bmodel                    # INT8 BModel for BM1684X，batch_size=1
    └── resnet50_int8_4b.bmodel                    # INT8 BModel for BM1684X，batch_size=4
```

The downloaded data include:

```bash
data/
├── images
│   ├── imagenet_val_1k                    # imagenet test images, total 1000 images
│   ├── pedestrian_gender                  # pedestrian sex classification test pictures
│   └── vehicle_color                      # vehicle color classification test pictures
└── video
    ├── test_imagenet.mp4                  # imagenet test video
    └── test_vehicle_color.mp4             # vehicle color classification test video
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the ResNet demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config
├── decode.json             # decoding configuration
├── engine.json             # sophon-stream graph configuration
├── resnet_demo.json        # resnet demo configuration
├── resnet_roi.json         # resnet roi configuration
└── resnet.json             # resnet plugin configuration
```

Indeed, [resnet_demo.json](./config/resnet_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. 

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../resnet/data/images/imagenet_val_1k",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    }
  ],
  "engine_config_path": "../resnet/config/engine.json"
}
```

[engine.json](./config/engine.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "resnet",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../resnet/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../resnet/config/resnet.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../resnet/config/resnet_demo.json
```

The running results of two video streams are as follows
```bash
 total time cost 1986302 us.
frame count is 1000 | fps is 803.448 fps.
```

## 7. Performance Testing

Currently, the YOLOv5 example supports inference on BM1684X and BM1684 in PCIe and SOC modes.

The tested video is `videos/test_imagenet.mp4`. The compilation was done in Release mode. The results are as follows:

|Device|Number of Channels|Algorithm Thread Count|CPU Utilization(%)|System Memory(M)|Peak System Memory(M)|TPU Utilization(%)|Device Memory(M)|Peak Device Memory(M)|AverageFPS|Peak FPS|
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|8|357.18|24.93|31.98|95.95|83.41|95.00|1990.34|2222.02|
|SE5-16|8| 8 |128.98|19.52|20.68|99.30|73.93|90.00|713.51|739.36|
|SE5-8|8| 4 |81.38|19.56|20.43|94.38|52.12|61.00|448.69|462.06|

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
---

## retinaface

# retinaface Demo

[English](README_EN.md) | 简体中文

## 目录
- [retinaface Demo](#retinaface-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标检测应用。

**源代码** (https://github.com/biubug6/Pytorch_Retinaface)

本例程中，retinaface算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)、BM1688(SoC)、CV186X(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

下载的模型包括：

```bash
./models/
│   ├── BM1684
│   │   ├── retinaface_mobilenet0.25_fp32_1b.bmodel # 用于BM1684的FP32 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_int8_1b.bmodel # 用于BM1684的INT8 BModel，batch_size=1，后处理在CPU上进行
│   │   └── retinaface_mobilenet0.25_int8_4b.bmodel # 用于BM1684的INT8 BModel，batch_size=4，后处理在CPU上进行
│   ├── BM1684X
│   │   ├── retinaface_mobilenet0.25_fp16_1b.bmodel # 用于BM1684X的FP16 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_fp32_1b.bmodel # 用于BM1684X的FP32 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_int8_1b.bmodel # 用于BM1684X的INT8 BModel，batch_size=1，后处理在CPU上进行
│   │   └── retinaface_mobilenet0.25_int8_4b.bmodel # 用于BM1684X的INT8 BModel，batch_size=4，后处理在CPU上进行
│   ├── BM1688
│   │   ├── retinaface_mobilenet0.25_fp16_1b_2core.bmodel # 用于BM1688的双核FP16 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_fp16_1b.bmodel       # 用于BM1688的FP16 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_fp32_1b_2core.bmodel # 用于BM1688的双核FP32 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_fp32_1b.bmodel       # 用于BM1688的FP32 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_int8_1b_2core.bmodel # 用于BM1688的双核INT8 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_int8_1b.bmodel       # 用于BM1688的INT8 BModel，batch_size=1，后处理在CPU上进行
│   │   ├── retinaface_mobilenet0.25_int8_4b_2core.bmodel # 用于BM1688的双核FP16 BModel，batch_size=4，后处理在CPU上进行
│   │   └── retinaface_mobilenet0.25_int8_4b.bmodel       # 用于BM1688的INT8 BModel，batch_size=4，后处理在CPU上进行
│   └── CV186X
│       ├── retinaface_mobilenet0.25_fp32_1b.bmodel       # 用于CV186X的FP32 BModel，batch_size=1，后处理在CPU上进行
│       └── retinaface_mobilenet0.25_int8_1b.bmodel       # 用于CV186X的INT8 BModel，batch_size=1，后处理在CPU上进行
```

模型说明:

以上模型移植于[Retinaface官方](https://github.com/biubug6/Pytorch_Retinaface)，插件配置`mean=[104,117,123]`，`std=[1,1,1]`。

下载的数据包括：

```bash
./images/
├── face            # 测试图片和视频
│   └── test
│       ├── face
│       └── videos
├── WIDERVAL
└── wind
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

retinaface demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
config/
├── decode.json                 # 解码配置
├── engine_group.json           # sophon-stream 简化的graph配置
├── engine.json                 # sophon-stream graph配置，需要分别配置前处理、推理和后处理文件
├── retinaface_demo.json        # demo输入配置文件
├── retinaface_group.json       # 简化的retinaface配置文件，将retinaface的前处理、推理、后处理合到一个配置文件中
├── retinaface_infer.json       # retinaface 推理配置文件
├── retinaface_post.json        # retinaface 后处理配置文件
└── retinaface_pre.json         # retinaface 前处理配置文件
```

其中，[retinaface_demo.json](./config/retinaface_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    }
  ],
  "download_image": true,
  "draw_func_name": "draw_retinaface_results",
  "engine_config_path": "../retinaface/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "retinaface",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../retinaface/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../retinaface/config/retinaface_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[retinaface_group.json](./config/retinaface_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

```json
{
    "configure": {
        "model_path": "../retinaface/data/models/BM1684X/retinaface_mobilenet0.25_fp32_1b.bmodel",
        "max_face_count":50,
        "score_threshold":0.1,
        "threshold_nms": 0.4,
        "bgr2rgb": false,
        "mean": [
            104,
            117,
            123
        ],
        "std": [
            1,
            1,
            1
        ]
    },
    "shared_object": "../../build/lib/libretinaface.so",
    "name": "retinaface",
    "side": "sophgo",
    "thread_number": 4
}
```


### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../retinaface/config/retinaface_demo.json
```

8路视频流运行结果如下:
```bash
total time cost 4798582 us.
frame count is 920 | fps is 191.723 fps.
```

## 7. 性能测试

目前，retinaface例程支持在BM1684X和BM1684的PCIE、SOC模式下进行推理，支持在BM1688和CV186X的SOC模式下进行推理。

测试数据`/data/images/wind`，编译选项为Release模式，使用int8模型，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|平均FPS|
|----|----|-----|-----|-----|
|SE7    |4  |4-4-4  |381  |428.797|
|SE9-16 |4  |4-4-4  |400  |302.932|

> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. SE5/SE7主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz，SE9-16为8核CA53@1.6GHz，SE9-8为6核CA53@1.6GH；

### retinaface - README_EN.md

# retinaface Demo

English | [简体中文](README.md)

## Catalogs
- [Retinaface Demo](#retinaface-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object detection application.

**source code** (https://github.com/biubug6/Pytorch_Retinaface)

In this example, the pre-processing, inference, and post-processing of the YOLOv5 algorithm are computed on three separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Features

* Supports BM1684X, BM1684(x86 PCIe、SoC), BM1688(SoC), CV186X(SoC).
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
│   ├── BM1684
│   │   ├── retinaface_mobilenet0.25_fp32_1b.bmodel # FP32 BModel for BM1684，batch_size=1，post-process on CPU
│   │   ├── retinaface_mobilenet0.25_int8_1b.bmodel # INT8 BModel for BM1684，batch_size=1，post-process on CPU
│   │   └── retinaface_mobilenet0.25_int8_4b.bmodel # INT8 BModel for BM1684，batch_size=4，post-process on CPU
│   ├── BM1684X
│   │   ├── retinaface_mobilenet0.25_fp16_1b.bmodel # FP16 BModel for BM1684X，batch_size=1，post-process on CPU
│   │   ├── retinaface_mobilenet0.25_fp32_1b.bmodel # FP32 BModel for BM1684X，batch_size=1，post-process on CPU
│   │   ├── retinaface_mobilenet0.25_int8_1b.bmodel # INT8 BModel for BM1684X，batch_size=1，post-process on CPU
│   │   └── retinaface_mobilenet0.25_int8_4b.bmodel # INT8 BModel for BM1684X，batch_size=4，post-process on CPU
│   ├── BM1688
│   │   ├── retinaface_mobilenet0.25_fp16_1b_2core.bmodel # FP16 2cores BModel for BM1688, batch_size=1, post-process on CPU
│   │   ├── retinaface_mobilenet0.25_fp16_1b.bmodel       # FP16 BModel for BM1688, batch_size=1, post-process on CPU
│   │   ├── retinaface_mobilenet0.25_fp32_1b_2core.bmodel # FP32 2cores BModel for BM1688, batch_size=1, post-process on CPU
│   │   ├── retinaface_mobilenet0.25_fp32_1b.bmodel       # FP32 BModel for BM1688, batch_size=1, post-process on CPU
│   │   ├── retinaface_mobilenet0.25_int8_1b_2core.bmodel # INT8 2cores BModel for BM1688, batch_size=1, post-process on CPU
│   │   ├── retinaface_mobilenet0.25_int8_1b.bmodel       # INT8 BModel for BM1688, batch_size=1, post-process on CPU
│   │   ├── retinaface_mobilenet0.25_int8_4b_2core.bmodel # FP16 2cores BModel for BM1688, batch_size=4, post-process on CPU
│   │   └── retinaface_mobilenet0.25_int8_4b.bmodel       # INT8 BModel for BM1688, batch_size=4, post-process on CPU
│   └── CV186X
│       ├── retinaface_mobilenet0.25_fp32_1b.bmodel       # FP32 BModel for BM1688, batch_size=1, post-process on CPU
│       └── retinaface_mobilenet0.25_int8_1b.bmodel       # INT8 BModel for BM1688, batch_size=1, post-process on CPU
```

The downloaded data include:

```bash
./images/
├── face            # images and video for test
│   └── test
│       ├── face
│       └── videos
├── WIDERVAL
└── wind
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the retinaface demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
config/
├── decode.json                 # decoding configuration
├── engine_group.json           # sophon-stream Simplified graph configuration
├── engine.json                 # sophon-stream graph configuration requires separate configuration for pre-processing, inference, and post-processing files.
├── retinaface_demo.json        # input configuration file for the demo
├── retinaface_group.json       # A simplified retinaface configuration file that combines pre-processing, inference, and post-processing into one configuration file.
├── retinaface_infer.json       # retinaface inference configuration file
├── retinaface_post.json        # retinaface post-process configuration file
└── retinaface_pre.json         # retinaface pre-process configuration file
```

Indeed, [retinaface_demo.json](./config/retinaface_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../retinaface/data/images/wind",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    }
  ],
  "download_image": true,
  "draw_func_name": "draw_retinaface_results",
  "engine_config_path": "../retinaface/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "retinaface",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../retinaface/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../retinaface/config/retinaface_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[retinaface_group.json](./config/retinaface_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

```json
{
    "configure": {
        "model_path": "../retinaface/data/models/BM1684X/retinaface_mobilenet0.25_fp32_1b.bmodel",
        "max_face_count":50,
        "score_threshold":0.5,
        "bgr2rgb": false,
        "mean": [
            104,
            117,
            123
        ],
        "std": [
            1,
            1,
            1
        ]
    },
    "shared_object": "../../build/lib/libretinaface.so",
    "name": "retinaface",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../retinaface/config/retinaface_demo.json
```

The running results of eight video streams are as follows
```bash
total time cost 4798582 us.
frame count is 920 | fps is 191.723 fps.
```

## 7. Performance Testing

Currently, the retinaface example supports inference on BM1684X and BM1684 in PCIe and SOC modes, BM1688 and CV186X in SoC.

The tested data is `/data/images/wind`. The compilation was done in Release mode. Using the int8 model, the results are as follows:

|Device|Number of Channels|Algorithm Thread Count|CPU Utilization(%)|Average FPS|
|----|----|-----|-----|-----|
|SE7    |4  |4-4-4  |381  |428.797|
|SE9-16 |4  |4-4-4  |400  |302.932|

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both SE5 and SE7 devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz. SE9-16 device utilizes an 8-core ARM A53 processor @ 1.6GHz, and SE9-8 device utilizes an 6-core ARM A53 processor @ 1.6GHz
---

## retinaface_distributor_resnet_faiss_converger

# 人脸检测-分发-识别 Demo

[English](README_EN.md) | 简体中文

## 目录
- [人脸检测-分发-识别 Demo](#人脸检测-分发-识别-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)
  - [8. 数据库生成方法](#8-数据库生成方法)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建包含了多算法和按类别发往不同分支的复杂应用。


## 2. 特性
* 检测模型使用retinaface；
* 人脸特征模型使用resnet；
* 人脸识别使用faiss；
* 支持BM1684X(x86 PCIe、SoC)、BM1688(SoC)；
* 支持多路视频流；
* 支持多线程。

备注：BM1688需要SDK版本1.8及以上

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`、`face_data`、`image`三个子目录。

下载的模型包括：

```bash
./models
├── BM1684X
│   ├── resnet_arcface_fp32_1b.bmodel # resnet人脸特征提取模型
│   └── retinaface_mobilenet0.25_fp32_1b.bmodel # BM1684X FP32人脸检测模型
└── BM1688
    ├── resnet_arcface_fp32_1b.bmodel # resnet人脸特征提取模型
    └── retinaface_mobilenet0.25_fp32_1b.bmodel # BM1688 FP32人脸检测模型
```

下载的数据包括：

```bash
├── class.names           # distributor 分类标签 
├── face_data 
│   ├── faiss_db_data.txt # 数据库人脸特征
│   └── faiss_index_label.name # 数据库的人脸label
├── images
│   ├── face_data_test  # 测试用的数据集
│   └── face_data_train # 生成数据库用的数据
```

## 4. 环境准备

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。


### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。


## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

配置文件位于 [./config](./config/)目录，结构如下所示：

```bash
./config
├── converger.json                                                    # 汇聚element配置
├── decode.json                                                       # 解码配置
├── distributor_class.json                                            # 每帧按类别分发
├── distributor_frame_class.json                                      # 跳帧按类别分发
├── distributor_frame.json                                            # 跳帧分发full frame
├── distributor_time_class.json                                       # 间隔时间按类别分发（默认）
├── distributor_time.json                                             # 间隔时间分发full frame
├── engine.json                                                       # graph配置
├── engine_group.json                                                 # 简化的graph配置
├── faiss.json                                                        # faiss配置
├── resnet_face.json                                                  # resnet 人脸分类
├── retinaface_distributor_resnet_faiss.json                          # demo配置
├── retinaface_group.json                                             # 简化的retinaface配置文件，管理前处理、推理、后处理三个步骤
├── retinaface_infer.json                                             # retinaface 推理配置
├── retinaface_post.json                                              # retinaface 后处理配置
└── retinaface_pre.json                                               # retinaface 前处理配置
```

其中，[retinaface_distributor_resnet_faiss.json](./config/retinaface_distributor_resnet_faiss.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 1,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 2,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    }
  ],
  "download_image": true,
  "draw_func_name": "draw_retinaface_distributor_resnet_faiss_converger_results",
  "engine_config_path": "../retinaface_distributor_resnet_faiss_converger/config/engine_group.json"
}
```

[engine.json](./config/engine.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

[retinaface_group.json](./config/retinaface_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../retinaface_distributor_resnet_faiss_converger/config/retinaface_distributor_resnet_faiss_converger.json
```

运行结果存放在`./build/results`目录下。本例程默认配置方式为每秒按类别发送到resnet分支，会在结果目录中每秒保存一帧绘制人脸label图像。


## 7. 性能测试
由于全流程依赖输入视频fps且画图速度慢，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。

## 8. 数据库生成方法
该例程中，通过对比resnet模型输出的向量与faiss索引库中已有的向量进行对比，返回人脸索引，然后通过索引获取对应的人脸标签。这里，我们也提供生成人脸数据库的方法。

此脚本依赖opencv和sophon-sail，请参考官网最新的SDK使用手册进行安装。

具体方法如下：

```bash
python3 scripts/resnet_opencv_faiss_write.py --input data/images/face_data_train --bmodel data/models/BM1684X/resnet_arcface_fp32_1b.bmodel --db_data faiss_db_data.txt --index_label faiss_index_label.name --dev_id 0 
```
参数说明如下：
```bash
usage:resnet_opencv_faiss_write.py [--input IMG_PATH] [--bmodel BMODEL] [--db_data DB_DATA] [--index_label INDEX_LABEL] [--dev_id DEV_ID]
--input: 推理图片路径，可输入整个图片文件夹的路径；
--bmodel: 用于推理的bmodel路径，默认使用stage 0的网络进行推理；
--db_data: 输出的人脸数据库；
--index_label: 输出数据库中索引对应的人脸标签文件；
--dev_id: 用于推理的tpu设备id。
```


### retinaface_distributor_resnet_faiss_converger - README_EN.md

# Face Detection-Distribution-Recognition Demo

English | [简体中文](README.md)

## Catalogs
- [Face Detection-Distribution-Recognition Demo](#face-detection-distribution-recognition-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)
  - [8. Database Generation](#8-database-generation)

## 1. Introduction

This sample is used to illustrate how to use sophon-stream to quickly build complex applications that contain multiple algorithms and send to different branches by classes.

## 2. Features
* Use retinaface for detection;
* Use resnet for face feature extraction;
* Use faiss for face recognition;
* Support BM1684X (x86 PCIe, SoC), BM1688(Soc);
* Supports multiple video streams;
* Support multi-threading.

Note: BM1688 needs SDK version >= 1.8

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing three subdirectories: `models`, `face_data` and `image`.

The downloaded models include:

```bash
./models
├── BM1684X
│   ├── resnet_arcface_fp32_1b.bmodel           # resnet for face feature extraction
│   └── retinaface_mobilenet0.25_fp32_1b.bmodel # retinaface for face detection
└── BM1688
    ├── resnet_arcface_fp32_1b.bmodel           # resnet for face feature extraction
    └── retinaface_mobilenet0.25_fp32_1b.bmodel # retinaface for face detection
```

The downloaded data include:

```bash
├── class.names           # distributor labels 
├── face_data 
│   ├── faiss_db_data.txt # database of features
│   └── faiss_index_label.name # database of label
├── images
│   ├── face_data_test  # dataset for test
│   └── face_data_train # dataset for generating the database
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

The configuration file is located in [./config](./config) directory with the following structure:

```bash
./config
├── converger.json                                                    # converger element configuration file
├── decode.json                                                       # decoding configuration
├── distributor_class.json                                            # distribute every frame in class
├── distributor_frame_class.json                                      # distribute frame with frame interval in class
├── distributor_frame.json                                            # distribute full frame with frame interval
├── distributor_time_class.json                                       # distribute frame with time interval (default)
├── distributor_time.json                                             # distribute full frame with time interval
├── engine.json                                                       # graph configuration
├── engine_group.json                                                 # 简化的graph配置 simplified graph configuration
├── faiss.json                                                        # faiss configuration
├── resnet_face.json                                                  # resnet for face feature extraction
├── retinaface_distributor_resnet_faiss.json                          # demo configuration
├── retinaface_group.json                                             # A simplified retinaface configuration file that combines pre-processing, inference, and post-processing into one configuration file
├── retinaface_infer.json                                             # retinaface inference configuration
├── retinaface_post.json                                              # retinaface post-process configuration
└── retinaface_pre.json                                               # retinaface pre-process configuration
```

Indeed, [retinaface_distributor_resnet_faiss.json](./config/retinaface_distributor_resnet_faiss.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 1,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 2,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../retinaface_distributor_resnet_faiss_converger/data/images/face_data_test",
      "source_type": "IMG_DIR",
      "loop_num": 1,
      "sample_interval": 1,
      "fps": -1
    }
  ],
  "download_image": true,
  "draw_func_name": "draw_retinaface_distributor_resnet_faiss_converger_results",
  "engine_config_path": "../retinaface_distributor_resnet_faiss_converger/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

[retinaface_group.json](./config/retinaface_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.


### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../retinaface_distributor_resnet_faiss_converger/config/retinaface_distributor_resnet_faiss_converger.json
```

The results are stored in the `./build/results` directory. This routine is configured by default to be sent to the resnet branch per second by category, and will save one frame per second of the drawn face label image in the results directory.

## 7. Performance Testing

Since the whole process depends on the input video fps and the drawing speed is slow, this routine does not provide performance test results for the time being, if you need the inference performance of each model, please go to the corresponding model routine to check.

## 8. Database Generation

In this sample, the face index is returned by comparing the vectors output from the resnet model with the vectors already in the faiss index library, and then the corresponding face labels are obtained through the index. Here, we also provide the method to generate the face database as follows:

```bash
python3 scripts/resnet_opencv_faiss_write.py --input data/images/face_data_train --bmodel data/models/BM1684X/resnet_arcface_fp32_1b.bmodel --db_data faiss_db_data.txt --index_label faiss_index_label.name --dev_id 0 
```

the parameters are as follow:

```bash
usage:resnet_opencv_faiss_write.py [--input IMG_PATH] [--bmodel BMODEL] [--db_data DB_DATA] [--index_label INDEX_LABEL] [--dev_id DEV_ID]
--input: image folder path;
--bmodel: bmodel path;
--db_data: face database;
--index_label: label file；
--dev_id: tpu id。
```

---

## structured_recognition

# structured_recognition Demo

[English](README_EN.md) | 简体中文

## 目录

- [structured\_recognition Demo](#structured_recognition-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe 平台](#41-x86arm-pcie-平台)
    - [4.2 SoC 平台](#42-soc-平台)
    - [4.3 在SoC平台安装可视化工具](#43-在soc平台安装可视化工具)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe 平台](#51-x86arm-pcie-平台)
    - [5.2 SoC 平台](#52-soc-平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json 配置说明](#61-json-配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用 sophon-stream 将一路码流分发给多个算法模块，快速构建一个结构化识别算法应用（机动车、非机动车、人、人脸、车牌）。  
本例程插件连接方式如下图所示。  
将一路video视频解码后，通过distributor插件分发给3个算法，yolov5进行人、机动车、非机动车识别，retinaface进行人脸识别，license_plate_recognition进行车牌识别。 
![structured_recognition](./pics/structured_recognition.png)  
本例程使用的模型来自[yolov5](../yolov5/README.md)、[retinaface](../retinaface/README.md)、[license_plate_recognition](../license_plate_recognition/README.md)例程

## 2. 特性

- 支持 BM1684X、BM1684(x86 PCIe、SoC)、BM1688(SoC)
- 支持多个算法检测一路视频流
- 支持多线程

## 3. 准备模型与数据

​ 在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中，`coco_lpr.names`为车牌检测yolov5s模型的分类集，`coco.names`为常规yolov5s模型的分类集，`models`存放模型文件。

下载的模型和数据包括：

```bash
.
├── data
│   ├── coco_lpr.names
│   ├── coco.names
│   ├── models
│   │   ├── lprnet
│   │   │   ├── BM1684
│   │   │   │   └── lprnet_int8_1b.bmodel
│   │   │   ├── BM1684X
│   │   │   │   └── lprnet_int8_1b.bmodel
│   │   │   └── BM1688
│   │   │       └── lprnet_int8_1b.bmodel
│   │   ├── retinaface
│   │   │   ├── BM1684
│   │   │   │   └── retinaface_mobilenet0.25_int8_1b.bmodel
│   │   │   ├── BM1684X
│   │   │   │   └── retinaface_mobilenet0.25_int8_1b.bmodel
│   │   │   └── BM1688
│   │   │       ├── retinaface_mobilenet0.25_int8_1b_2core.bmodel
│   │   │       └── retinaface_mobilenet0.25_int8_1b.bmodel
│   │   ├── yolov5s
│   │   │   ├── BM1684
│   │   │   │   └── yolov5s_v6.1_3output_int8_1b.bmodel
│   │   │   ├── BM1684X
│   │   │   │   └── yolov5s_v6.1_3output_int8_1b.bmodel
│   │   │   └── BM1688
│   │   │       ├── yolov5s_v6.1_3output_int8_1b_2core.bmodel
│   │   │       └── yolov5s_v6.1_3output_int8_1b.bmodel
│   │   └── yolov5s-licensePLate
│   │       ├── BM1684
│   │       │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
│   │       ├── BM1684X
│   │       │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
│   │       └── BM1688
│   │           ├── yolov5s_v6.1_license_3output_int8_1b_2core.bmodel
│   │           └── yolov5s_v6.1_license_3output_int8_1b.bmodel
│   └── videos
│       └── structs.mp4
└── tools
    └── application-web-linux_arm64.tgz
```
其中application-web-linux_arm64.tgz是在SoC平台运行的web可视化工具。

模型及数据说明: 为方便下载和测试，本例程仅采用int8精度模型，如需其他精度模型，可在[yolov5](../yolov5/README.md)、[retinaface](../retinaface/README.md)、[license_plate_recognition](../license_plate_recognition/README.md)例程下载。

数据包括：类别集合coco_lpr.names、coco.names以及测试视频structs.mp4


## 4. 环境准备

### 4.1 x86/arm PCIe 平台

如果您在 x86/arm 平台安装了 PCIe 加速卡（如 SC 系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装 libsophon、sophon-opencv 和 sophon-ffmpeg，具体步骤可参考[x86-pcie 平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie 平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC 平台

如果您使用 SoC 平台（如 SE、SM 系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的 libsophon、sophon-opencv 和 sophon-ffmpeg 运行库包，可直接使用它作为运行环境。通常还需要一台 x86 主机作为开发环境，用于交叉编译 C++程序。

### 4.3 在SoC平台安装可视化工具
在SoC平台依次执行安装命令：
```bash
tar -xzvf application-web-linux_arm64.tgz 
cd application_web/
sudo ./install.sh
```
安装完成后，使用浏览器输入`http://{ip}:8089`，打开页面，ip为SoC平台设备ip地址。用户名和密码均为`admin`  
![web](./pics/web.png)
## 5. 程序编译

### 5.1 x86/arm PCIe 平台

可以直接在 PCIe 平台上编译程序，具体请参考[sophon-stream 编译](../../docs/HowToMake.md)。

### 5.2 SoC 平台

通常在 x86 主机上交叉编译程序，您需要在 x86 主机上使用 SOPHON SDK 搭建交叉编译环境，将程序所依赖的头文件和库文件打包至 sophon_sdk_soc 目录中，具体请参考[sophon-stream 编译](../../docs/HowToMake.md)。本例程主要依赖 libsophon、sophon-opencv 和 sophon-ffmpeg 运行库包。

## 6. 程序运行

### 6.1 Json 配置说明

structured_recognition demo 中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
config/
├── converger.json
├── decode.json
├── distributor_frame.json
├── distributor_time_class.json
├── encode.json
├── engine_group.json
├── lprnet_group.json
├── retinaface_group.json
├── structured_recognition_demo.json
├── yolov5_group.json
└── yolov5_lpr_group.json
```


[engine_group.json](./config/engine_group.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，以及一路码流分配给多个算法检测。

在该文件内，需要初始化每个 element 的信息和 element 之间的连接方式。element_id 是唯一的，起到标识身份的作用。element_config 指向该 element 的详细配置文件地址，port_id 是该 element 的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src 标志当前端口是否是整张图的输入端口，is_sink 标识当前端口是否是整张图的输出端口。
connection 是所有 element 之间的连接方式，通过 element_id 和 port_id 确定。  

[engine_group.json](./config/engine_group.json)的配置如图所示：   
![engine_group](./pics/engine_group.png)  
其中1000-1009为每个`element`的id，配置文件中的`connections`表示连接`element`的箭头，每个`element`默认端口都为0；由于`distributor`需要将一帧图片分发给多个`element`，因此除了默认端口0以外，还有端口1-3，上图中，红色数字标注为`distributor`和`converger`插件的端口；`converger`插件是配合`distributor`实现数据汇集功能，接受端口需要和`distributor`发送的端口保持一致。  
最后通过`encode`插件将数据转为json格式，通过websock方式推流。


### 6.2 运行

对于 PCIe 平台，可以直接在 PCIe 平台上运行测试；对于 SoC 平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到 SoC 平台中测试。

SoC 平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始 sophon-stream 仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以 PCIe 模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../structured_recognition/config/structured_recognition_demo.json
```
2. 打开可视化工具  
打开推理结果页面，输入websock链接，格式为`ws://{ip}:{port}`,如`ws://192.168.0.101:9002`，其中端口值是由[encode.json](./config/encode.json)文件中`wss_port`字段决定，比如`wss_port`值为9000，[engine_group.json](./config/engine_group.json)中`channel_id`为2，那么这一路视频流结果地址就是9002。
<div style="text-align: center;">
  <img src="./pics/web1.png" alt="web1" style="width: 65%;">
</div>
点击播放，即可播放检测结果的图片流。
<div style="text-align: center;">
  <img src="./pics/res0.png" alt="res0" style="width: 65%;">
</div>
<div style="text-align: center;">
  <img src="./pics/res1.png" alt="res1" style="width: 65%;">
</div>
点击调试模式，可以在浏览器控制台打印每一帧结果的内容，可查看上报数据的json格式  
<div style="text-align: center;">
  <img src="./pics/res2.png" alt="res2" style="width: 65%;">
</div>  


## 7. 性能测试

由于全流程依赖输入视频fps且ws上传速度慢，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。

**注意** encode插件会将图片进行base64编码，速度比较慢；如果去掉encode插件，可跑满源视频的30fps。

### structured_recognition - README_EN.md

# structured_recognition Demo

English | [简体中文](README.md)

## Catalogs

- [structured\_recognition Demo](#structured_recognition-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
    - [4.3 Installing the Visualization Tool on the SoC Platform](#43-installing-the-visualization-tool-on-the-soc-platform)
  - [5. Program Compilation](#5-program-compilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 Json Configuration Explanation](#61-json-configuration-explanation)
    - [6.2 Execution](#62-execution)
  - [7. Performance Testing](#7-performance-testing)


## 1. Introduction

This example illustrates how to use sophon-stream to distribute a single stream to multiple algorithm modules, quickly building a structured recognition algorithm application (including motor vehicles, non-motor vehicles, people, faces, and license plates).
The plugin connection method for this example is shown in the following diagram.
After decoding a single video stream, it is distributed to three algorithms via the distributor plugin: yolov5 for recognizing people, motor vehicles, and non-motor vehicles; retinaface for face recognition; and license_plate_recognition for license plate recognition.

![structured_recognition](./pics/structured_recognition.png)

The models used in this example are from the [yolov5](../yolov5/README.md), [retinaface](../retinaface/README.md), and [license_plate_recognition](../license_plate_recognition/README.md) examples.


## 2. Features

- Supports BM1684X, BM1684 (x86 PCIe, SoC), BM1688 (SoC)
- Supports multiple algorithms detecting a single video stream
- Supports multithreading

## 3. Prepare Models and Data

In the `scripts` directory, download scripts for relevant models and data are provided in [download.sh](./scripts/download.sh).

```bash
chmod -R +x scripts/
./scripts/download.sh
```

After the script is executed, a `data` directory will be generated in the current directory. In this directory, `coco_lpr.names` contains the class set for the license plate detection yolov5s model, `coco.names` contains the class set for the regular yolov5s model, and `models` stores the model files.


The downloaded data include:

```bash
.
├── data
│   ├── coco_lpr.names
│   ├── coco.names
│   ├── models
│   │   ├── lprnet
│   │   │   ├── BM1684
│   │   │   │   └── lprnet_int8_1b.bmodel
│   │   │   ├── BM1684X
│   │   │   │   └── lprnet_int8_1b.bmodel
│   │   │   └── BM1688
│   │   │       └── lprnet_int8_1b.bmodel
│   │   ├── retinaface
│   │   │   ├── BM1684
│   │   │   │   └── retinaface_mobilenet0.25_int8_1b.bmodel
│   │   │   ├── BM1684X
│   │   │   │   └── retinaface_mobilenet0.25_int8_1b.bmodel
│   │   │   └── BM1688
│   │   │       ├── retinaface_mobilenet0.25_int8_1b_2core.bmodel
│   │   │       └── retinaface_mobilenet0.25_int8_1b.bmodel
│   │   ├── yolov5s
│   │   │   ├── BM1684
│   │   │   │   └── yolov5s_v6.1_3output_int8_1b.bmodel
│   │   │   ├── BM1684X
│   │   │   │   └── yolov5s_v6.1_3output_int8_1b.bmodel
│   │   │   └── BM1688
│   │   │       ├── yolov5s_v6.1_3output_int8_1b_2core.bmodel
│   │   │       └── yolov5s_v6.1_3output_int8_1b.bmodel
│   │   └── yolov5s-licensePLate
│   │       ├── BM1684
│   │       │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
│   │       ├── BM1684X
│   │       │   └── yolov5s_v6.1_license_3output_int8_1b.bmodel
│   │       └── BM1688
│   │           ├── yolov5s_v6.1_license_3output_int8_1b_2core.bmodel
│   │           └── yolov5s_v6.1_license_3output_int8_1b.bmodel
│   └── videos
│       └── structs.mp4
└── tools
    └── application-web-linux_arm64.tgz
```

The `application-web-linux_arm64.tgz` is a web visualization tool that runs on the SoC platform.

Model and Data Description: For convenience in downloading and testing, this example only uses int8 precision models. If other precision models are needed, they can be downloaded from the [yolov5](../yolov5/README.md), [retinaface](../retinaface/README.md), and [license_plate_recognition](../license_plate_recognition/README.md) examples.

The data includes: category sets `coco_lpr.names`, `coco.names`, and the test video `structs.mp4`.


## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

### 4.3 Installing the Visualization Tool on the SoC Platform
Execute the following installation commands sequentially on the SoC platform:
```bash
tar -xzvf application-web-linux_arm64.tgz 
cd application_web/
sudo ./install.sh
```
After the installation is complete, open a browser and enter `http://{ip}:8089` to open the page, where ip is the IP address of the SoC platform device. Both the username and password are `admin`.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 Json Configuration Explanation

Various parameters in the license_plate_recognition demo are located in the [config](./config/) directory, structured as follows:

```bash
config/
├── converger.json
├── decode.json
├── distributor_frame.json
├── distributor_time_class.json
├── encode.json
├── engine_group.json
├── lprnet_group.json
├── retinaface_group.json
├── structured_recognition_demo.json
├── yolov5_group.json
└── yolov5_lpr_group.json
```

The [engine_group.json](./config/engine_group.json) file is the overall configuration file for the example, managing information such as input streams. It supports multiple data inputs on one diagram and allows a single stream to be allocated to multiple algorithm detections.

In this file, you need to initialize the information for each element and the connections between elements. The `element_id` is unique and serves as an identifier. The `element_config` points to the detailed configuration file address for that element. The `port_id` is the input/output port number of the element. In cases of multiple inputs or outputs, the input/output numbers must not be duplicated. The `is_src` flag indicates whether the current port is an input port for the entire diagram, and the `is_sink` flag indicates whether the current port is an output port for the entire diagram.
The `connection` defines the connections between all elements, determined by `element_id` and `port_id`.

The configuration of [engine_group.json](./config/engine_group.json) is shown in the figure below:  

![engine_group](./pics/engine_group.png)

Where 1000-1009 are the IDs for each `element`. The `connections` in the configuration file represent the arrows connecting the `elements`, with each `element` having a default port of 0. Since the `distributor` needs to distribute a frame of image to multiple `elements`, it has ports 1-3 in addition to the default port 0. In the figure above, the red numbers indicate the ports of the `distributor` and `converger` plugins. The `converger` plugin works with the `distributor` to achieve data aggregation, and its receiving ports need to match the sending ports of the `distributor`.
Finally, the `encode` plugin converts the data into JSON format and streams it via WebSocket.

### 6.2 Execution

For the PCIe platform, you can directly run the test on the PCIe platform. For the SoC platform, you need to copy the dynamically linked libraries, executable files, required models, and test data generated by cross-compilation to the SoC platform for testing.

On the SoC platform, the directory structure of dynamic libraries, executable files, configuration files, models, and video data should be consistent with the original sophon-stream repository.

The parameters and running methods for testing are the same. The following mainly introduces the PCIe mode.

1. Run the executable file
```bash
./main --demo_config_path=../license_plate_recognition/config/license_plate_recognition_demo.json
```  
2. Open the Visualization Tool  
Open the inference result page and enter the WebSocket link in the format `ws://{ip}:{port}`, such as `ws://192.168.0.101:9002`. The port value is determined by the `wss_port` field in the [encode.json](./config/encode.json) file. For example, if the `wss_port` value is 9000 and the `channel_id` in [engine_group.json](./config/engine_group.json) is 2, then the address for this video stream result is 9002.
<div style="text-align: center;">
  <img src="./pics/web1.png" alt="web1" style="width: 65%;">
</div>
Click play to start playing the image stream of the detection results.
<div style="text-align: center;">
  <img src="./pics/res0.png" alt="res0" style="width: 65%;">
</div>
<div style="text-align: center;">
  <img src="./pics/res1.png" alt="res1" style="width: 65%;">
</div>
Click on debug mode to print the content of each frame result in the browser console, allowing you to view the JSON format of the reported data.
<div style="text-align: center;">
  <img src="./pics/res2.png" alt="res2" style="width: 65%;">
</div>  

## 7. Performance Testing

Due to the entire process relying on the input video FPS and the slow WebSocket upload speed, this example does not provide performance test results. For inference performance of each model, please refer to the corresponding model examples.

**Note**: The encode plugin will base64 encode the images, which is relatively slow; if you remove the encode plugin, the source video can run at 30fps.

---

## trinocular_panorama_stitch

# trinocular_panorama_stitch Demo

## 目录
- [trinocular\_panorama\_stitch Demo](#trinocular_panorama_stitch-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备数据](#3-准备数据)
  - [4. 环境准备](#4-环境准备)
    - [4. SoC平台](#4-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 SoC平台](#51-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建三目全景拼接应用，并通过distributor额外引出三路原始流。流程图如下：
![flowchart](./pics/flow_chart.png)

## 2. 特性

* 支持BM1688(SoC)
* 支持3路拼接

## 3. 准备数据

​在`scripts`目录下提供了相关数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。
```bash
.
├── config  #配置文件
├── cpp
│   ├── CMakeLists.txt
│   └── main.cpp #独立于stream的三目拼接功能测试代码
├── datasets
│   ├── 0106 #三目摄像机对应的图
│   └── mask #用于blending的权重
├── gridinfo #各个摄像机对应的gridinfo，需要根据实际情况更换。
│   ├── down_grid_info_bev_90_90_4589_90_90_dst_2880x2880_src_2560x1440.1.dat
│   ├── left_grid_info_bev_90_90_5078_90_90_dst_2880x2880_src_2560x2160.1.dat
│   └── right_grid_info_bev_90_90_4233_90_90_dst_2880x2880_src_2560x1440.1.dat
└── README.md
```

## 4. 环境准备

### 4. SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。
(1) 安装驱动
安装驱动需要进入到超级权限，接着系统驱动目录，安装驱动：

```bash
sudo -s
#请根据您实际使用的sensor安装相应驱动，这里只是示例：
insmod /mnt/system/ko/v4l2_pr2100.ko force_bus=1,1,1,1,-1,-1  force_i2caddr=0x5F,0x5F,0x5C,0x5C,0x5F,0x5F force_slave=0,0,1,1,0,0 
```

## 5. 程序编译

### 5.1 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

dwa_blend_encode demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine.json                 # sophon-stream graph配置，需要分别配置dwa、blend、encode等文件
├── distributor_full_frame.json # distributor配置文件
├── encode.json                 # distributor后接的编码配置文件
├── camera.json                 # demo按图片文件夹输入的配置文件
├── dwa_L.json                  # 左侧输入的几何畸变矫正配置文件
├── dwa_R.json                  # 右侧输入的几何畸变矫正配置文件
├── dwa_D.json                  # 下侧输入的几何畸变矫正配置文件
├── blend_left_right.json        # 左右图拼接配置文件
├── blend_up_down.json           # 上下图拼接配置文件
└── encode_dwa.json              # blend后接的编码配置文件
```

其中，[camera.json](./config/camera.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。

### 6.2 运行

对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

1. 参考[推流服务器使用方法](../../element/multimedia/encode/README.md#8-推流服务器)运行rtsp推流服务器。
   
2. 运行可执行文件， 如果使用sensor输入则需要运行`sudo -s`进入root环境，安装相关驱动，将--demo_config_path换成`config/camera_sensor.json`，并注意修改dwa和resize等配置文件的参数。
```bash
./main --demo_config_path=../samples/trinocular_panorama_stitch/config/camera.json
```
注：程序运行性能取决于dwa和blend插件的输入输出分辨率，使用本例程的配置和数据集的话，fps约等于12。

3. 通过VLC或其他方式拉取rtsp流。

拼接效果图：
![alt text](./pics/panorama.jpg)
---

## tripwire

# 拌线检测算法结果推送http请求Demo

[English](README_EN.md) | 简体中文

## 目录
- [拌线检测算法结果推送http请求Demo](#拌线检测算法结果推送http请求Demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建拌线检测算法结果推送http请求应用；


## 2. 特性
* 检测模型使用yolox；
* 跟踪模型使用bytetrack；
* 支持BM1684X(x86 PCIe、SoC)，BM1684(x86 PCIe、SoC、arm PCIe)，BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本[download.sh](./scripts/download.sh)。

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

下载的模型包括：
```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684的INT8 BModel，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1688的bytetrack的FP32 BModel，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1688的bytetrack的INT8 BModel，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # 用于BM1688的INT8 BModel，batch_size=1
    └── yolox_s_int8_4b.bmodel              # 用于BM1688的INT8 BModel，batch_size=4
```
模型说明:

1.`yolox_bytetrack_s`系列模型移植于[bytetrack官方](https://github.com/ifzhang/ByteTrack)，插件配置`mean=[0,0,0]`，`std=[1,1,1]`，支持person类别的检测任务。

2.`yolox_s`系列模型移植于[yolox官方](https://github.com/Megvii-BaseDetection/YOLOX)，插件配置`mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`，支持COCO数据集的80分类检测任务。

下载的数据包括：
```bash
./data/test.mp4                           # 测试视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。


### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。


## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

配置文件位于 [./config](../tripwire/config)

其中，[tripwire.json](../tripwire/config/tripwire_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels中包含每一路码流url等信息。

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../data/test.mp4",
      "source_type": "VIDEO",
      "loop_num": 2100000,
      "fps": 25,
      "sample_interval": 5
    }
  ],
  "engine_config_path": "../tripwire/config/engine_group.json",
  "http_report": {
    "ip": "0.0.0.0",
    "port": 10001,
    "path": "/flask_test/"
  }
}
```

[engine.json](../tripwire/config/engine.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即删去`channels`里的部分元素，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
  {
    "graph_id": 0,
    "device_id": 0,
    "graph_name": "yolox_osd_encode",
    "elements": [
      {
        "element_id": 5000,
        "element_config": "../yolox_bytetrack_osd_encode/config/decode.json",
        "ports": {
          "input": [
            {
              "port_id": 0,
              "is_sink": false,
              "is_src": true
            }
          ]
        }
      },
      {
        "element_id": 5001,
        "element_config": "../yolox_bytetrack_osd_encode/config/yolox_group.json",
        "inner_elements_id": [
          10001,
          10002,
          10003
        ]
      },
      {
        "element_id": 5004,
        "element_config": "../yolox_bytetrack_osd_encode/config/bytetrack.json"
      },
      {
        "element_id": 5005,
        "element_config": "../yolox_bytetrack_osd_encode/config/filter.json"
      },
      {
        "element_id": 5006,
        "element_config": "../yolox_bytetrack_osd_encode/config/http_push.json",
        "ports": {
          "output": [
            {
              "port_id": 0,
              "is_sink": true,
              "is_src": false
            }
          ]
        }
      }
    ],
    "connections": [
      {
        "src_element_id": 5000,
        "src_port": 0,
        "dst_element_id": 5001,
        "dst_port": 0
      },
      {
        "src_element_id": 5001,
        "src_port": 0,
        "dst_element_id": 5004,
        "dst_port": 0
      },
      {
        "src_element_id": 5004,
        "src_port": 0,
        "dst_element_id": 5005,
        "dst_port": 0
      },
      {
        "src_element_id": 5005,
        "src_port": 0,
        "dst_element_id": 5006,
        "dst_port": 0
      }
    ]
  }
]
```

[filter.json](../tripwire/config/filter.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。
其中，thread_number是element内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。
具体请参考[filter.json](../../element/tools/filter/README.md)
```json
{
  "configure": {
    "rules": [
      {
        "channel_id": 0,
        "filters": [
          {
            "alert_first_frames": 0,
            "alert_frame_skip_nums": 10000,
            "areas": [
              [
                {
                  "left": 1000,
                  "top": 0
                },
                {
                  "left": 1000,
                  "top": 1080
                }
              ]
            ],
            "classes": [
              0
            ],
            "times": [
              {
                "time_start": "00 00 00",
                "time_end": "23 59 59"
              }
            ],
            "type": 1
          }
        ]
      }
    ]
  },
  "shared_object": "../../build/lib/libfilter.so",
  "name": "filter",
  "side": "sophgo",
  "thread_number": 2
}
```

### 6.2 运行
对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件,注意要更改filter里面画出的线。
```bash
./main --demo_config_path=../tripwire/config/tripwire_demo.json
```

运行结果如下
```bash
total time cost 74520023 us.
frame count is 3077 | fps is 41.2909 fps.
```

## 7. 性能测试
由于涉及到筛选，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。

## 8. 使用application-web可视化

1.克隆 https://github.com/sophgo/application-web

2.根据application-web/README.md将application-web部署

3.参考[web_server](../../tools/web-server/README.md)搭建应用层程序

4.
```
cd ../../tools/web-server
bash start_server.sh 
```
5.通过application-web下发任务，启动任务

### tripwire - README_EN.md

# Detection-Track-UpStreaming Demo

English | [简体中文](README.md)

## Catalogs
- [Detection-Track-UpStreaming Demo](#detection-track-upstreaming-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This sample is used to illustrate how to quickly build a video target tracking application using sophon-stream and push stream the algorithm results to output;

The connection method for this example plugin is shown in the following diagram.

![elements.jpg](pics/dec_det_track_osd_enc.png)

## 2. Feature

* Use yolox for detection;
* Use bytetrack for track;
* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(SoC)
* Supports multiple video streams;
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp16_1b.bmodel    # FP16 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684X，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684X，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684X，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1688，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1688，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1688，batch_size=1
    └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1688，batch_size=4
```

Model description:

1.`yolox_s_bytetrack_` models are from [bytetrack](https://github.com/ifzhang/ByteTrack), `mean=[0,0,0]`，`std=[1,1,1]`, support for person category detection tasks.

2.`yolox_s` models are from [yolox](https://github.com/Megvii-BaseDetection/YOLOX), `mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`, support for 80 classes of COCO dataset.

The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the Detection-Track-UpStreaming Demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_01_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 3,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_03_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 20,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_06_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 30,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_08_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    }
  ],
  "engine_config_path": "../yolox_bytetrack_osd_encode/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox_osd_encode",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox_bytetrack_osd_encode/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox_bytetrack_osd_encode/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../yolox_bytetrack_osd_encode/config/bytetrack.json"
            },
            {
                "element_id": 5005,
                "element_config": "../yolox_bytetrack_osd_encode/config/osd.json"
            },
            {
                "element_id": 5006,
                "element_config": "../yolox_bytetrack_osd_encode/config/encode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5005,
                "src_port": 0,
                "dst_element_id": 5006,
                "dst_port": 0
            }
        ]
    }
]
```

[osd.json](./config/osd.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

```json
{
  "configure": {
    "osd_type": "TRACK",
    "class_names_file": "../yolox_bytetrack_osd_encode/data/coco.names",
    "draw_utils": "OPENCV",
    "draw_interval": false,
    "put_text": false
  },
  "shared_object": "../../build/lib/libosd.so",
  "name": "osd",
  "side": "sophgo",
  "thread_number": 1
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file,be careful to change the lines drawn in the filter.
```bash
./main --demo_config_path=../yolox_bytetrack_osd_encode/config/yolox_bytetrack_osd_encode_demo.json
```

The running results are as follows
```bash
total time cost 74520023 us.
frame count is 3077 | fps is 41.2909 fps.
```

If encode selects RTSP mode, you need to start the push streaming server. You can use VLC software to open the push streaming address to view the video algorithm results, see [encode plugin documentation](../../element/multimedia/encode/README.md) for details.

## 7. Performance Testing

Due to the slow drawing speed of Osd plugin, this sample does not provide performance test results for the time being. If you need the inference performance of each model, please go to the corresponding model sample to check.
---

## yolov5

# YOLOv5 Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOv5 Demo](#yolov5-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标检测应用。

本例程插件的连接方式如下图所示

![process](./pics/elements.jpg)

**源代码** (https://github.com/ultralytics/yolov5) v6.1版本

本例程中，yolov5算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)，支持BM1688(arm PCIe、SoC)
* BM1684X平台上，支持tpu_kernel后处理
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：

```bash
./models/
├── BM1684
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1684的FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1684的INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1684的INT8 BModel，batch_size=4，后处理在CPU上进行
├── BM1684X
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # 用于BM1684X的FP16 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1684X的FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1684X的INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1684X的INT8 BModel，batch_size=4，后处理在CPU上进行
├── BM1684X_tpukernel
│   ├── yolov5s_tpukernel_fp16_1b.bmodel            # 用于BM1684X的FP16 BModel，batch_size=1，后处理采用tpu_kernel
│   ├── yolov5s_tpukernel_fp32_1b.bmodel            # 用于BM1684X的FP32 BModel，batch_size=1，后处理采用tpu_kernel
│   ├── yolov5s_tpukernel_int8_1b.bmodel            # 用于BM1684X的INT8 BModel，batch_size=1，后处理采用tpu_kernel
│   └── yolov5s_tpukernel_int8_4b.bmodel            # 用于BM1684X的INT8 BModel，batch_size=4，后处理采用tpu_kernel
├── BM1688
│   ├── yolov5s_v6.1_3output_fp16_1b_2core.bmodel   # 用于BM1688的 FP16 BModel，batch_size=1，num_core=2
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # 用于BM1688的 FP16 BModel，batch_size=1，num_core=1
│   ├── yolov5s_v6.1_3output_fp32_1b_2core.bmodel   # 用于BM1688的 FP32 BModel，batch_size=1，num_core=2
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1688的 FP32 BModel，batch_size=1，num_core=1
│   ├── yolov5s_v6.1_3output_int8_1b_2core.bmodel   # 用于BM1688的 INT8 BModel，batch_size=1，num_core=2
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1688的 INT8 BModel，batch_size=1，num_core=1
│   ├── yolov5s_v6.1_3output_int8_4b_2core.bmodel   # 用于BM1688的 INT8 BModel，batch_size=4，num_core=2
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1688的 INT8 BModel，batch_size=4，num_core=1
└── CV186X
    ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # 用于CV186X的FP16 BModel，batch_size=1
    ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于CV186X的FP32 BModel，batch_size=1
    ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于CV186X的INT8 BModel，batch_size=1
    └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于CV186X的INT8 BModel，batch_size=4
```

模型说明:

以上模型移植于[yolov5官方](https://github.com/ultralytics/yolov5)，插件配置`mean=[0,0,0]`，`std=[255,255,255]`，支持COCO数据集的80分类检测任务。

下载的数据包括：

```bash
videos/
├── carvana_video.mp4   # 测试视频
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolov5 demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine_group.json           # sophon-stream 简化的graph配置
├── engine.json                 # sophon-stream graph配置，需要分别配置前处理、推理和后处理文件
├── yolov5_classthresh_roi_example.json  # yolov5按照类别设置阈值的参考配置文件，需要注意，按类别设置阈值仅支持非tpu_kernel的后处理模式
├── yolov5_demo.json            # demo输入配置文件
├── yolov5_group.json           # 简化的yolov5配置文件，将yolov5的前处理、推理、后处理合到一个配置文件中
├── yolov5_infer.json           # yolov5推理配置文件
├── yolov5_post.json            # yolov5后处理配置文件
└── yolov5_pre.json             # yolov5前处理配置文件
```

其中，[yolov5_demo.json](./config/yolov5_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov5/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov5_results",
  "engine_config_path": "../yolov5/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即调整`channels`参数，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov5",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolov5/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolov5/config/yolov5_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[yolov5_group.json](./config/yolov5_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine_group.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

`use_tpu_kernel`为`true`时，会使用tpu_kernel后处理。tpu_kernel后处理只支持BM1684X设备。

```json
{
    "configure": {
        "model_path": "../yolov5/data/models/BM1684X_tpukernel/yolov5s_tpukernel_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [
            0,
            0,
            0
        ],
        "std": [
            1,
            1,
            1
        ],
        "use_tpu_kernel": true
    },
    "shared_object": "../../build/lib/libyolov5.so",
    "name": "yolov5_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../yolov5/config/yolov5_demo.json
```

4路视频流运行结果如下
```bash
total time cost 12150217 us.
frame count is 2848 | fps is 234.399 fps.
```

**双核TPU推理**：

BM1688平台为双核TPU，可使用双核进行模型推理。当前版本的单核模型与双核模型的推理方式不同：

1. 对于单核模型，即目录`./data/models/BM1688`中`num_core=1`的模型，默认使用单核进行推理，此时的TPU利用率为50%，FPS约为120。

若您需要提高TPU利用率，可使用双核TPU进行推理，此时的TPU利用率可达90%左右，FPS约为160。
具体方法是修改文件`sophon-stream/element/algorithm/yolov5/src/yolov5_inference.cc`中的代码，将第178行和第191行
```cpp
178    ret = context->bmNetwork->forward<false>(inputTensors->tensors,
...
191    int ret = context->bmNetwork->forward<false>(
```
改为：
```cpp
178    ret = context->bmNetwork->forward<true>(inputTensors->tensors,
...
191    int ret = context->bmNetwork->forward<true>(
```
然后按照步骤[5. 程序编译](#5-程序编译)重新编译并运行。

2. 对于双核模型，即目录`./data/models/BM1688`中`num_core=2`的模型，默认使用双核进行推理，不需要修改代码，可直接运行。


## 7. 性能测试

目前，yolov5例程支持在BM1684X和BM1684的PCIE、SOC模式下进行推理，支持在BM1688 SOC模式下进行推理。

在不同的设备上可能需要修改json配置，例如模型路径、输入路数等。json的配置方法参考6.1节，程序运行方法参考上文6.2节。

由于PCIE设备cpu能力差距较大，性能数据没有参考意义，这里只给出SOC模式的测试结果。

测试视频`elevator-1080p-25fps-4000kbps.h264`，编译选项为Release模式，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|系统内存(M)|系统内存峰值(M)|TPU利用率(%)|设备内存(M)|设备内存峰值(M)|平均FPS|峰值FPS|
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|4-4-4|136.64|200.95|206.18|100.00|1857.94|2067.00|255.17|265.92|
|SE5-16|4|4-4-4|218.74|187.12|191.89|96.95|1897.06|2151.00|120.84|141.66|
|SE5-8|3|3-3-3|137.50|145.81|149.01|94.89|1273.70|1437.00|80.38|90.00|

> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；
3. 以上性能测试均基于int8模型给出；
4. 在BM1684设备上运行时，batch_size为4的模型可以达到更高的fps；
5. 在BM1684X设备上，使用batch_size为1的模型，并且开启tpu_kernel后处理，可以达到更高的fps；
6. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;
7. BM1688设备暂无性能测试数据。

### yolov5 - README_EN.md

# YOLOv5 Demo

English | [简体中文](README.md)

## Catalogs
- [YOLOv5 Demo](#yolov5-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Feature](#2-feature)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-compilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object detection application.

The connection method for this example plugin is shown in the following diagram.

![process](./pics/elements.jpg)

**source code** (https://github.com/ultralytics/yolov5) v6.1 version

In this example, the pre-processing, inference, and post-processing of the YOLOv5 algorithm are computed on three separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Feature

* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(arm PCIe、SoC).
* Supports tpu_kernel post-process on BM1684X.
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # FP32 BModel for BM1684, with a batch size of 1. Post-processing takes place on the CPU.
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # INT8 BModel for BM1684, with a batch size of 1. Post-processing takes place on the CPU.
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # INT8 BModel for BM1684, with a batch size of 4. Post-processing takes place on the CPU.
├── BM1684X
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # FP16 BModel for BM1684X, with a batch size of 1. Post-processing takes place on the CPU.
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # FP32 BModel for BM1684X, with a batch size of 1. Post-processing takes place on the CPU.
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # INT8 BModel for BM1684X, with a batch size of 1. Post-processing takes place on the CPU.
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # INT8 BModel for BM1684X, with a batch size of 4. Post-processing takes place on the CPU.
├── BM1684X_tpukernel
│   ├── yolov5s_tpukernel_fp16_1b.bmodel            # FP16 BModel for BM1684X, with a batch size of 1. Post-processing utilizes the tpu_kernel.
│   ├── yolov5s_tpukernel_fp32_1b.bmodel            # FP32 BModel for BM1684X, with a batch size of 1. Post-processing utilizes the tpu_kernel.
│   ├── yolov5s_tpukernel_int8_1b.bmodel            # INT8 BModel for BM1684X, with a batch size of 1. Post-processing utilizes the tpu_kernel.
│   └── yolov5s_tpukernel_int8_4b.bmodel            # INT8 BModel for BM1684X, with a batch size of 4. Post-processing utilizes the tpu_kernel.
├── BM1688
│   ├── yolov5s_v6.1_3output_fp16_1b_2core.bmodel   # FP16 BModel for BM1688, batch_size=1, num_core=2
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # FP16 BModel for BM1688, batch_size=1, num_core=1
│   ├── yolov5s_v6.1_3output_fp32_1b_2core.bmodel   # FP32 BModel for BM1688, batch_size=1, num_core=2
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # FP32 BModel for BM1688, batch_size=1, num_core=1
│   ├── yolov5s_v6.1_3output_int8_1b_2core.bmodel   # INT8 BModel for BM1688, batch_size=1, num_core=2
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # INT8 BModel for BM1688, batch_size=1, num_core=1
│   ├── yolov5s_v6.1_3output_int8_4b_2core.bmodel   # INT8 BModel for BM1688, batch_size=4, num_core=2
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # INT8 BModel for BM1688, batch_size=4, num_core=1
└── CV186X
    ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # FP16 BModel for CV186X，batch_size=1
    ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # FP32 BModel for CV186X，batch_size=1
    ├── yolov5s_v6.1_3output_int8_1b.bmodel         # INT8 BModel for CV186X，batch_size=1
    └── yolov5s_v6.1_3output_int8_4b.bmodel         # INT8 BModel for CV186X，batch_size=4
```

Model description:

The above models are ported from the official [yolov5 repository](https://github.com/ultralytics/yolov5). The plugin configuration includes `mean=[0,0,0]`, `std=[255,255,255]`, supporting 80-class detection tasks from the COCO dataset.


The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the YOLOv5 demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config/
├── decode.json                 # decoding configuration
├── engine_group.json           # sophon-stream Simplified graph configuration
├── engine.json                 # sophon-stream graph configuration requires separate configuration for pre-processing, inference, and post-processing files.
├── yolov5_classthresh_roi_example.json  # reference configuration file for setting thresholds per category in YOLOv5. Please note that setting thresholds per category is only supported in non-tpu_kernel post-processing mode
├── yolov5_demo.json            # input configuration file for the demo
├── yolov5_group.json           # A simplified YOLOv5 configuration file that combines pre-processing, inference, and post-processing into one configuration file.
├── yolov5_infer.json           # YOLOv5 inference configuration file
├── yolov5_post.json            # YOLOv5 post-processing configuration file
└── yolov5_pre.json             # YOLOv5 pre-processing configuration file
```

Indeed, [yolov5_demo.json](./config/yolov5_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolov5/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov5/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov5_results",
  "engine_config_path": "../yolov5/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov5",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../config/yolov5_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```


[yolov5_group.json](./config/yolov5_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

When `use_tpu_kernel` is set to `true`, it will utilize the tpu_kernel post-processing(using tpu to do post process). Note that tpu_kernel post-processing is only supported on BM1684X devices.

```json
{
    "configure": {
        "model_path": "../data/models/BM1684X_tpukernel/yolov5s_tpukernel_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [
            0,
            0,
            0
        ],
        "std": [
            1,
            1,
            1
        ],
        "use_tpu_kernel": true
    },
    "shared_object": "../../../build/lib/libyolov5.so",
    "name": "yolov5_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolov5/config/yolov5_demo.json
```

The running results of 4 video streams are as follows
```bash
total time cost 12150217 us.
frame count is 2848 | fps is 234.399 fps.
```

## 7. Performance Testing

Currently, the YOLOv5 example supports inference on BM1684X and BM1684 in PCIe and SoC modes, and supports inference on BM1688 in SoC mode.

Modifications in JSON configurations might be necessary when switching between different devices, such as adjusting model paths, input channels, etc. Refer to section 6.1 for JSON configuration methods and section 6.2 for program execution methods.

Due to significant differences in CPU capabilities among PCIe devices, performance data is not meaningful. Therefore, only provide the test results for SOC mode.

The tested video is `elevator-1080p-25fps-4000kbps.h264`. The compilation was done in Release mode. The results are as follows:


| Device | Number of Channels | Algorithm Thread Count | CPU Utilization (%) | System Memory (M) | Peak System Memory (M) | TPU Utilization (%) | Device Memory (M) | Peak Device Memory (M) | Average FPS | Peak FPS |
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|4-4-4|136.64|200.95|206.18|100.00|1857.94|2067.00|255.17|265.92|
|SE5-16|4|4-4-4|218.74|187.12|191.89|96.95|1897.06|2151.00|120.84|141.66|
|SE5-8|3|3-3-3|137.50|145.81|149.01|94.89|1273.70|1437.00|80.38|90.00|

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
3. All aforementioned performance tests are based on the INT8 model.
4. Running models with a batch size of 4 on the BM1684 device can achieve higher FPS.
5. On the BM1684X device, utilizing a batch size of 1 for the model and enabling the tpu_kernel for post-processing can yield higher FPS.
6. For the settings of input channels and algorithm thread count in the table, please refer to [JSON configuration explanation](#61-json-configuration). CPU utilization and system memory can be checked using the `top` command. TPU utilization and device memory can be checked using the `bm-smi` command. FPS can be obtained from the logs printed during program execution.
7. Performance testing is not currently available on the BM1688 device.
---

## yolov5_bytetrack_distributor_resnet_converger

# 目标检测-跟踪-分发-属性识别 Demo

[English](README_EN.md) | 简体中文

## 目录
- [目标检测-跟踪-分发-属性识别 Demo](#目标检测-跟踪-分发-属性识别-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建包含了多算法和按类别发往不同分支的复杂应用。

本例程插件的连接方式如下图所示：

![distributor.png](pics/distributor.png)

## 2. 特性
* 检测模型使用yolov5；
* 跟踪模型使用bytetrack；
* 分类模型使用resnet18；
* 支持BM1684X(x86 PCIe、SoC)和BM1684(x86 PCIe、SoC、arm PCIe)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：

```bash
./models
├── BM1684
│   ├── resnet50_fp32_1b.bmodel                     # 用于BM1684的RESNET50 FP32 Bmodel，batch_size=1，imagenet
│   ├── resnet50_fp32_4b.bmodel                     # 用于BM1684的RESNET50 FP32 Bmodel，batch_size=4，imagenet
│   ├── resnet50_int8_1b.bmodel                     # 用于BM1684的RESNET50 INT8 Bmodel，batch_size=1，imagenet
│   ├── resnet50_int8_4b.bmodel                     # 用于BM1684的RESNET50 INT8 Bmodel，batch_size=4，imagenet
│   ├── resnet_pedestrian_gender_fp32_1b.bmodel     # 用于BM1684的RESNET18 FP32 Bmodel，batch_size=1，行人性别分类
│   ├── resnet_pedestrian_gender_fp32_4b.bmodel     # 用于BM1684的RESNET18 FP32 Bmodel，batch_size=4，行人性别分类
│   ├── resnet_pedestrian_gender_int8_1b.bmodel     # 用于BM1684的RESNET18 INT8 Bmodel，batch_size=1，行人性别分类
│   ├── resnet_pedestrian_gender_int8_4b.bmodel     # 用于BM1684的RESNET18 INT8 Bmodel，batch_size=4，行人性别分类
│   ├── resnet_vehicle_color_fp32_1b.bmodel         # 用于BM1684的RESNET18 FP32 Bmodel，batch_size=1，车辆颜色分类
│   ├── resnet_vehicle_color_fp32_4b.bmodel         # 用于BM1684的RESNET18 FP32 Bmodel，batch_size=4，车辆颜色分类
│   ├── resnet_vehicle_color_int8_1b.bmodel         # 用于BM1684的RESNET18 INT8 Bmodel，batch_size=1，车辆颜色分类
│   ├── resnet_vehicle_color_int8_4b.bmodel         # 用于BM1684的RESNET18 INT8 Bmodel，batch_size=4，车辆颜色分类
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1684的YOLOV5 FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1684的YOLOV5 INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1684的YOLOV5 INT8 BModel，batch_size=4，后处理在CPU上进行
├── BM1684X
│   ├── resnet50_fp16_1b.bmodel                     # 用于BM1684X的RESNET50 FP16 Bmodel，batch_size=1，imagenet
│   ├── resnet50_fp32_1b.bmodel                     # 用于BM1684X的RESNET50 FP32 Bmodel，batch_size=1，imagenet
│   ├── resnet50_fp32_4b.bmodel                     # 用于BM1684X的RESNET50 FP32 Bmodel，batch_size=4，imagenet
│   ├── resnet50_int8_1b.bmodel                     # 用于BM1684X的RESNET50 INT8 Bmodel，batch_size=1，imagenet
│   ├── resnet50_int8_4b.bmodel                     # 用于BM1684X的RESNET50 INT8 Bmodel，batch_size=4，imagenet
│   ├── resnet_pedestrian_gender_fp16_1b.bmodel     # 用于BM1684X的RESNET18 FP16 Bmodel，batch_size=1，行人性别分类
│   ├── resnet_pedestrian_gender_fp32_1b.bmodel     # 用于BM1684X的RESNET18 FP32 Bmodel，batch_size=1，行人性别分类
│   ├── resnet_pedestrian_gender_fp32_4b.bmodel     # 用于BM1684X的RESNET18 FP32 Bmodel，batch_size=4，行人性别分类
│   ├── resnet_pedestrian_gender_int8_1b.bmodel     # 用于BM1684X的RESNET18 INT8 Bmodel，batch_size=1，行人性别分类
│   ├── resnet_pedestrian_gender_int8_4b.bmodel     # 用于BM1684X的RESNET18 INT8 Bmodel，batch_size=4，行人性别分类
│   ├── resnet_vehicle_color_fp16_1b.bmodel         # 用于BM1684X的RESNET18 FP16 Bmodel，batch_size=1，车辆颜色分类
│   ├── resnet_vehicle_color_fp32_1b.bmodel         # 用于BM1684X的RESNET18 FP32 Bmodel，batch_size=1，车辆颜色分类
│   ├── resnet_vehicle_color_fp32_4b.bmodel         # 用于BM1684X的RESNET18 FP32 Bmodel，batch_size=4，车辆颜色分类
│   ├── resnet_vehicle_color_int8_1b.bmodel         # 用于BM1684X的RESNET18 INT8 Bmodel，batch_size=1，车辆颜色分类
│   ├── resnet_vehicle_color_int8_4b.bmodel         # 用于BM1684X的RESNET18 INT8 Bmodel，batch_size=4，车辆颜色分类
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # 用于BM1684X的YOLOV5 FP16 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1684X的YOLOV5 FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=4，后处理在CPU上进行
└── BM1684X_tpukernel
    ├── yolov5s_tpukernel_fp16_1b.bmodel            # 用于BM1684X的YOLOV5 FP16 BModel，batch_size=1，后处理采用tpu_kernel
    ├── yolov5s_tpukernel_fp32_1b.bmodel            # 用于BM1684X的YOLOV5 FP32 BModel，batch_size=1，后处理采用tpu_kernel
    ├── yolov5s_tpukernel_int8_1b.bmodel            # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=1，后处理采用tpu_kernel
    └── yolov5s_tpukernel_int8_4b.bmodel            # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=4，后处理采用tpu_kernel
```

下载的数据包括：

```bash
./videos/                                           # 测试视频
├── carvana_video.mp4
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── test_car_person_1080P.avi
└── traffic.mp4
```

## 4. 环境准备

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。


### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。


## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

配置文件位于 [./config](./config/)目录，结构如下所示：

```bash
./config
├── bytetrack.json                                                    # bytetrack跟踪算法配置
├── converger.json                                                    # 汇聚element配置
├── decode.json                                                       # 解码配置
├── distributor_class.json                                            # 每帧按类别分发
├── distributor_frame_class.json                                      # 跳帧按类别分发
├── distributor_frame.json                                            # 跳帧分发full frame
├── distributor_time_class.json                                       # 间隔时间按类别分发（默认）
├── distributor_time.json                                             # 间隔时间分发full frame
├── engine.json                                                       # graph配置
├── engine_group.json                                                 # 简化的graph配置
├── resnet_car.json                                                   # resnet 车辆颜色分类
├── resnet_person.json                                                # resnet 行人性别分类
├── yolov5_bytetrack_distributor_resnet_converger_demo.json           # demo配置
├── yolov5_group.json                                                 # 简化的yolov5配置文件，将yolov5的前处理、推理、后处理合到一个配置文件中
├── yolov5_infer.json                                                 # yolov5 推理配置
├── yolov5_post.json                                                  # yolov5 后处理配置
└── yolov5_pre.json                                                   # yolov5 前处理配置
```

其中，[yolov5_bytetrack_distributor_resnet_converger_demo.json](./config/yolov5_bytetrack_distributor_resnet_converger_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，channel中包含码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 1,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 2,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 3,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    }
  ],
  "class_names": "../yolov5_bytetrack_distributor_resnet_converger/data/coco.names",
  "car_attributes": "../yolov5_bytetrack_distributor_resnet_converger/data/car.attributes",
  "person_attributes": "../yolov5_bytetrack_distributor_resnet_converger/data/person.attributes",
  "download_image": true,
  "draw_func_name": "draw_yolov5_bytetrack_distributor_resnet_converger_results",
  "engine_config_path": "../yolov5_bytetrack_distributor_resnet_converger/config/engine_group.json"
}
```

[engine.json](./config/engine.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

[yolov5_group.json](./config/yolov5_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

`use_tpu_kernel`为`true`时，会使用tpu_kernel后处理。tpu_kernel后处理只支持BM1684X设备。

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../yolov5_bytetrack_distributor_resnet_converger/config/yolov5_bytetrack_distributor_resnet_converger_demo.json
```

运行结果存放在`./build/results`目录下。本例程默认配置方式为每秒按类别发送到resnet分支，会在结果目录中每秒保存一帧绘制了目标box、track_id和具体属性的图像。

![result](./pics/result.jpg)

## 7. 性能测试
由于全流程依赖输入视频fps且画图速度慢，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。

### yolov5_bytetrack_distributor_resnet_converger - README_EN.md

# Target Detection-Tracking-Distribution-Attribute Recognition Demo

English | [简体中文](README.md)

## Catalogs
- [Target Detection-Tracking-Distribution-Attribute Recognition Demo](#target-detection-tracking-distribution-attribute-recognition-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a complex application that contain multiple algorithms and send to different branches by category.

The connection method for this example plugin is shown in the following diagram.

![distributor.png](pics/distributor.png)

## 2. Feature

* Use yolov5 for detection;
* Use bytetrack for track;
* Use resnet18 for classification;
* Supports BM1684X(x86 PCIe, SoC) and BM1684(x86 PCIe, SoC, arm PCIe);
* Supports multiple video streams;
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models
├── BM1684
│   ├── resnet50_fp32_1b.bmodel                     # RESNET50 FP32 Bmodel for BM1684，batch_size=1，imagenet
│   ├── resnet50_fp32_4b.bmodel                     # RESNET50 FP32 Bmodel for BM1684，batch_size=4，imagenet
│   ├── resnet50_int8_1b.bmodel                     # RESNET50 INT8 Bmodel for BM1684，batch_size=1，imagenet
│   ├── resnet50_int8_4b.bmodel                     # RESNET50 INT8 Bmodel for BM1684，batch_size=4，imagenet
│   ├── resnet_pedestrian_gender_fp32_1b.bmodel     # RESNET18 FP32 Bmodel for BM1684，batch_size=1，pedestrian gender classification
│   ├── resnet_pedestrian_gender_fp32_4b.bmodel     # RESNET18 FP32 Bmodel for BM1684，batch_size=4，pedestrian gender classification
│   ├── resnet_pedestrian_gender_int8_1b.bmodel     # RESNET18 INT8 Bmodel for BM1684，batch_size=1，pedestrian gender classification
│   ├── resnet_pedestrian_gender_int8_4b.bmodel     # RESNET18 INT8 Bmodel for BM1684，batch_size=4，pedestrian gender classification
│   ├── resnet_vehicle_color_fp32_1b.bmodel         # RESNET18 FP32 Bmodel for BM1684，batch_size=1，vehicle color classification
│   ├── resnet_vehicle_color_fp32_4b.bmodel         # RESNET18 FP32 Bmodel for BM1684，batch_size=4，vehicle color classification
│   ├── resnet_vehicle_color_int8_1b.bmodel         # RESNET18 INT8 Bmodel for BM1684，batch_size=1，vehicle color classification
│   ├── resnet_vehicle_color_int8_4b.bmodel         # RESNET18 INT8 Bmodel for BM1684，batch_size=4，vehicle color classification
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # YOLOV5 FP32 BModel for BM1684，batch_size=1，post-process on CPU
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # YOLOV5 INT8 BModel for BM1684，batch_size=1，post-process on CPU
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # YOLOV5 INT8 BModel for BM1684，batch_size=4，post-process on CPU
├── BM1684X
│   ├── resnet50_fp16_1b.bmodel                     # RESNET50 FP16 Bmodel for BM1684X，batch_size=1，imagenet
│   ├── resnet50_fp32_1b.bmodel                     # RESNET50 FP32 Bmodel for BM1684X，batch_size=1，imagenet
│   ├── resnet50_fp32_4b.bmodel                     # RESNET50 FP32 Bmodel for BM1684X，batch_size=4，imagenet
│   ├── resnet50_int8_1b.bmodel                     # RESNET50 INT8 Bmodel for BM1684X，batch_size=1，imagenet
│   ├── resnet50_int8_4b.bmodel                     # RESNET50 INT8 Bmodel for BM1684X，batch_size=4，imagenet
│   ├── resnet_pedestrian_gender_fp16_1b.bmodel     # RESNET18 FP16 Bmodel for BM1684X，batch_size=1，pedestrian gender classification
│   ├── resnet_pedestrian_gender_fp32_1b.bmodel     # RESNET18 FP32 Bmodel for BM1684X，batch_size=1，pedestrian gender classification
│   ├── resnet_pedestrian_gender_fp32_4b.bmodel     # RESNET18 FP32 Bmodel for BM1684X，batch_size=4，pedestrian gender classification
│   ├── resnet_pedestrian_gender_int8_1b.bmodel     # RESNET18 INT8 Bmodel for BM1684X，batch_size=1，pedestrian gender classification
│   ├── resnet_pedestrian_gender_int8_4b.bmodel     # RESNET18 INT8 Bmodel for BM1684X，batch_size=4，pedestrian gender classification
│   ├── resnet_vehicle_color_fp16_1b.bmodel         # RESNET18 FP16 Bmodel for BM1684X，batch_size=1，vehicle color classification
│   ├── resnet_vehicle_color_fp32_1b.bmodel         # RESNET18 FP32 Bmodel for BM1684X，batch_size=1，vehicle color classification
│   ├── resnet_vehicle_color_fp32_4b.bmodel         # RESNET18 FP32 Bmodel for BM1684X，batch_size=4，vehicle color classification
│   ├── resnet_vehicle_color_int8_1b.bmodel         # RESNET18 INT8 Bmodel for BM1684X，batch_size=1，vehicle color classification
│   ├── resnet_vehicle_color_int8_4b.bmodel         # RESNET18 INT8 Bmodel for BM1684X，batch_size=4，vehicle color classification
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # YOLOV5 FP16 BModel for BM1684X，batch_size=1，post-process on CPU
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # YOLOV5 FP32 BModel for BM1684X，batch_size=1，post-process on CPU
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # YOLOV5 INT8 BModel for BM1684X，batch_size=1，post-process on CPU
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # YOLOV5 INT8 BModel for BM1684X，batch_size=4，post-process on CPU
└── BM1684X_tpukernel
    ├── yolov5s_tpukernel_fp16_1b.bmodel            # YOLOV5 FP16 BModel for BM1684X，batch_size=1，post-process with tpu_kernel
    ├── yolov5s_tpukernel_fp32_1b.bmodel            # YOLOV5 FP32 BModel for BM1684X，batch_size=1，post-process with tpu_kernel
    ├── yolov5s_tpukernel_int8_1b.bmodel            # YOLOV5 INT8 BModel for BM1684X，batch_size=1，post-process with tpu_kernel
    └── yolov5s_tpukernel_int8_4b.bmodel            # YOLOV5 INT8 BModel for BM1684X，batch_size=4，post-process with tpu_kernel
```

The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

Configuration files are located on [config](./config/) directory, structured as follows:

```bash
./config
├── bytetrack.json                                                    # bytetrack configuration
├── converger.json                                                    # converger element configuration
├── decode.json                                                       # decoding configuration
├── distributor_class.json                                            # distribute every frame in class
├── distributor_frame_class.json                                      # distribute frame with frame interval in class
├── distributor_frame.json                                            # distribute full frame with frame interval
├── distributor_time_class.json                                       # distribute frame with time interval (default)
├── distributor_time.json                                             # distribute full frame with time interval
├── engine.json                                                       # graph configuration
├── engine_group.json                                                 # sophon-stream Simplified graph configuration
├── resnet_car.json                                                   # resnet vehicle color classification
├── resnet_person.json                                                # resnet pedestrian gender classification
├── yolov5_bytetrack_distributor_resnet_converger_demo.json           # demo configuration
├── yolov5_group.json                                                 # A simplified YOLOv5 configuration file that combines pre-processing, inference, and post-processing into one configuration file.
├── yolov5_infer.json                                                 # yolov5 inference configuration
├── yolov5_post.json                                                  # yolov5 post-process configuration
└── yolov5_pre.json                                                   # yolov5 pre-process configuration
```

Indeed, [yolov5_bytetrack_distributor_resnet_converger_demo.json](./config/yolov5_bytetrack_distributor_resnet_converger_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 1,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 2,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 3,
      "url": "../yolov5_bytetrack_distributor_resnet_converger/data/videos/traffic.mp4",
      "source_type": "VIDEO",
      "loop_num": 1,
      "fps": 25
    }
  ],
  "class_names": "../yolov5_bytetrack_distributor_resnet_converger/data/coco.names",
  "car_attributes": "../yolov5_bytetrack_distributor_resnet_converger/data/car.attributes",
  "person_attributes": "../yolov5_bytetrack_distributor_resnet_converger/data/person.attributes",
  "download_image": true,
  "draw_func_name": "draw_yolov5_bytetrack_distributor_resnet_converger_results",
  "engine_config_path": "../yolov5_bytetrack_distributor_resnet_converger/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

[yolov5_group.json](./config/yolov5_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

When `use_tpu_kernel` is set to `true`, it will utilize the tpu_kernel post-processing(using tpu to do post process). Note that tpu_kernel post-processing is only supported on BM1684X devices.

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolov5_bytetrack_distributor_resnet_converger/config/yolov5_bytetrack_distributor_resnet_converger_demo.json
```

The results are stored in the `./build/results` directory. This sample is configured by default to be sent to the resnet branch per second per category, and will save one frame per second in the results directory with the target box, track_id, and specific attributes drawn.

## 7. Performance Testing

Since the whole process depends on the input video fps and the drawing speed is slow, this routine does not provide performance test results for the time being, if you need the inference performance of each model, please go to the corresponding model routine to check.
---

## yolov5_fastpose_posec3d

# YOLOV5-FASTPOSE-POSEC3D Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOV5-FASTPOSE-POSEC3D Demo](#yolov5-fastpose_posec3d-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频姿态识别应用。

本例程插件的连接方式如下图所示

![process](./pics/posec3d.jpg)

**源代码** (https://github.com/MVIG-SJTU/AlphaPose和https://github.com/open-mmlab/mmaction2) 

本例程中，posec3d、fastpose和yolov5算法的前处理、推理、后处理分别在九个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X(x86 PCIe、SoC)
* YOLOv5的AlphaPose支持BM1684(x86 PCIe、SoC)、BM1684(x86 PCIe、SoC)
* 支持多路视频流
* 支持多线程
* BM1684X平台上，支持yolov5 tpu_kernel后处理

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

**注意：fastpose和posec3d BModel模型暂时只支持BM1684X平台。**

下载的模型包括：

```bash
./models
├── BM1684
│   ├── fast_res50_256x192_coco17_1b_fp32.bmodel    # 用于BM1684的FASTPOSE FP32 Bmodel，batch_size=1，17个关键点检测
│   ├── fast_res50_256x192_coco17_1b_int8.bmodel    # 用于BM1684的FASTPOSE INT8 Bmodel，batch_size=1，17个关键点检测
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1684的YOLOV5 FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1684的YOLOV5 INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1684的YOLOV5 INT8 BModel，batch_size=4，后处理在CPU上进行
├── BM1684X
│   ├── fast_res50_256x192_coco17_1b_fp16.bmodel    # 用于BM1684X的FASTPOSE FP16 Bmodel，batch_size=1，17个关键点检测
│   ├── fast_res50_256x192_coco17_1b_fp32.bmodel    # 用于BM1684X的FASTPOSE FP32 Bmodel，batch_size=1，17个关键点检测
│   ├── fast_res50_256x192_coco17_1b_int8.bmodel    # 用于BM1684X的FASTPOSE INT8 Bmodel，batch_size=1，17个关键点检测
│   ├── posec3d_gym_fp16.bmodel                     # 用于BM1684X的POSEC3D FP16 Bmodel，gym 99类识别
│   ├── posec3d_gym_fp32.bmodel                     # 用于BM1684X的POSEC3D FP32 Bmodel，gym 99类识别
│   ├── posec3d_ntu60_fp16.bmodel                   # 用于BM1684X的POSEC3D FP16 Bmodel，ntu 60类识别
│   ├── posec3d_ntu60_fp32.bmodel                   # 用于BM1684X的POSEC3D FP32 Bmodel，ntu 60类识别
│   ├── posec3d_ntu60_int8.bmodel                   # 用于BM1684X的POSEC3D INT8 Bmodel，ntu 60类识别
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # 用于BM1684X的YOLOV5 FP16 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # 用于BM1684X的YOLOV5 FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=4，后处理在CPU上进行
└── BM1684X_tpukernel
    ├── yolov5s_tpukernel_fp16_1b.bmodel            # 用于BM1684X的YOLOV5 FP16 BModel，batch_size=1，后处理采用tpu_kernel
    ├── yolov5s_tpukernel_fp32_1b.bmodel            # 用于BM1684X的YOLOV5 FP32 BModel，batch_size=1，后处理采用tpu_kernel
    ├── yolov5s_tpukernel_int8_1b.bmodel            # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=1，后处理采用tpu_kernel
    └── yolov5s_tpukernel_int8_4b.bmodel            # 用于BM1684X的YOLOV5 INT8 BModel，batch_size=4，后处理采用tpu_kernel
```

下载的数据包括：
```bash
./videos
├── demo_skeleton.mp4                         # 人体检测+关键检测+行为识别测试视频 
├── S017C001P003R001A001_rgb.avi              # 人体检测+关键检测+行为识别测试视频 
├── S017C001P003R002A008_rgb.avi              # 人体检测+关键检测+行为识别测试视频 
└── test.mp4                                  # 人体检测+关键检测测试视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolov5-fastpose demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                         # 解码配置
├── engine_group_alphapose.json         # sophon-stream 人体检测+关键检测graph配置
├── engine_group.json                   # sophon-stream 人体检测+关键检测+行为识别graph配置
├── fastpose_group.json                 # fastpose 配置
├── fastpose_infer.json                 # fastpose 推理配置
├── fastpose_post.json                  # fastpose 后处理配置
├── fastpose_pre.json                   # fastpose 前处理配置
├── posec3d_group.json                  # posec3d 配置
├── yolov5_fastpose_posec3d_demo.json   # yolov5-fastpose-posec3d demo配置
├── yolov5_group.json                   # yolov5 配置
├── yolov5_infer.json                   # yolov5 推理配置
├── yolov5_post.json                    # yolov5 后处理配置
└── yolov5_pre.json                     # yolov5 前处理配置
```

[engine_group_alphapose.json](./config/engine_group_alphapose.json)是伴随yolov5作为检测器的alphapose算法，输出人体检测框和人体关键点，输出如下图：

<img src="./pics/yolov5_fastpose.jpg" width="800">

在其基础上增加posec3d行为识别模型构成配置文件[engine_group.json](./config/engine_group.json)，输出人体检测框、人体关键点和行为类别，输出如下图

<img src="./pics/yolov5_fastpose_posec3d.jpg" width="800">

注意在该图中posec3d以72帧作为输入，最终类别标签打在第0帧上，该视频来自download.sh脚本下载的视频demo_skeleton.mp4。

其中，[yolov5_fastpose_posec3d_demo.json](./config/yolov5_fastpose_posec3d_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels中包含各路的码流url等信息。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

`heatmap_loss` 参数决定fastpose的后处理流程，目前基于官方的[模型配置](https://github.com/MVIG-SJTU/AlphaPose/blob/master/docs/MODEL_ZOO.md)，只支持了 `MSELoss`。

```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../yolov5_fastpose_posec3d/data/nturgb+d_rgb/S017C001P003R002A008_rgb.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov5_fastpose_posec3d/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov5_fastpose_posec3d_results",
  "engine_config_path": "../yolov5_fastpose_posec3d/config/engine_group.json",
  "heatmap_loss": "MSELoss"
}
```

[engine_group.json](./config/engine_group.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov5_fastpose_posec3d",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolov5_fastpose_posec3d/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolov5_fastpose_posec3d/config/yolov5_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 6001,
                "element_config": "../yolov5_fastpose_posec3d/config/fastpose_group.json",
                "inner_elements_id": [20001, 20002, 20003]
            },
            {
                "element_id": 7001,
                "element_config": "../yolov5_fastpose_posec3d/config/posec3d_group.json",
                "inner_elements_id": [30001, 30002, 30003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 6001,
                "dst_port": 0
            },
            {
                "src_element_id": 6001,
                "src_port": 0,
                "dst_element_id": 7001,
                "dst_port": 0
            }
        ]
    }
]
```

[fastpose_group.json](./config/fastpose_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

`use_tpu_kernel`为`true`时，会使用yolov5 tpu_kernel后处理。tpu_kernel后处理只支持BM1684X设备。

```json
{
    "configure": {
        "model_path": "../yolov5_fastpose_posec3d/data/models/BM1684X/halpe26_fast_res50_256x192_int8_1b.bmodel",
        "stage": [
            "pre"
        ],
        "heatmap_loss": "MSELoss",
        "area_thresh": 0.0
    },
    "shared_object": "../../build/lib/libfastpose.so",
    "name": "fastpose",
    "side": "sophgo",
    "thread_number": 1
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../yolov5_fastpose_posec3d/config/yolov5_fastpose_posec3d_demo.json
```

3路视频流运行结果如下
```bash
 total time cost 5453888 us.
frame count is 291 | fps is 53.3564 fps.
```

## 7. 性能测试

不同视频性能差别较大，以实际为准。


### yolov5_fastpose_posec3d - README_EN.md

# YOLOV5-FASTPOSE-POSEC3D Demo

English | [简体中文](README.md)

## Catalogs
- [YOLOV5-FASTPOSE-POSEC3D Demo](#yolov5-fastpose-posec3d-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video pose recognition application.

The connection method for this example plugin is shown in the following diagram.

![process](./pics/posec3d.jpg)

**Source Code** (https://github.com/MVIG-SJTU/AlphaPose和https://github.com/open-mmlab/mmaction2) 

In this example, the pre-processing, inference, and post-processing of the YOLOv5, fastpose and posec3d algorithm are computed on 9 separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Feature

* Supports BM1684X(x86 PCIe、SoC)
* AlphaPose with YOLOv5 supports BM1684X(x86 PCIe、SoC) and BM1684(x86 PCIe、SoC)
* Supports multiple video streams.
* Supports multi-threading.
* On the BM1684X platform, the TPU_kernel post-processing is supported.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

**Note: fastpose and posec3d BModel models are only supported on the BM1684X platform for now.**

The downloaded models include:

```bash
./models
├── BM1684
│   ├── fast_res50_256x192_coco17_1b_fp32.bmodel    # FASTPOSE FP32 Bmodel for BM1684，batch_size=1，17 key points
│   ├── fast_res50_256x192_coco17_1b_int8.bmodel    # FASTPOSE INT8 Bmodel for BM1684，batch_size=1，17 key points
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # YOLOV5 FP32 BModel for BM1684，batch_size=1，post process on CPU
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # YOLOV5 INT8 BModel for BM1684，batch_size=1，post process on CPU
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # YOLOV5 INT8 BModel for BM1684，batch_size=4，post process on CPU
├── BM1684X
│   ├── fast_res50_256x192_coco17_1b_fp16.bmodel    # FASTPOSE FP16 Bmodel for BM1684X，batch_size=1，17 key points
│   ├── fast_res50_256x192_coco17_1b_fp32.bmodel    # FASTPOSE FP32 Bmodel for BM1684X，batch_size=1，17 key points
│   ├── fast_res50_256x192_coco17_1b_int8.bmodel    # FASTPOSE INT8 Bmodel for BM1684X，batch_size=1，17 key points
│   ├── posec3d_gym_fp16.bmodel                     # POSEC3D FP16 Bmodel for BM1684X，gym 99 classes
│   ├── posec3d_gym_fp32.bmodel                     # POSEC3D FP32 Bmodel for BM1684X，gym 99 classes
│   ├── posec3d_ntu60_fp16.bmodel                   # POSEC3D FP16 Bmodel for BM1684X，ntu 60 classes
│   ├── posec3d_ntu60_fp32.bmodel                   # POSEC3D FP32 Bmodel for BM1684X，ntu 60 classes
│   ├── posec3d_ntu60_int8.bmodel                   # POSEC3D INT8 Bmodel for BM1684X，ntu 60 classes
│   ├── yolov5s_v6.1_3output_fp16_1b.bmodel         # YOLOV5 FP16 BModel for BM1684X，batch_size=1，post process on CPU
│   ├── yolov5s_v6.1_3output_fp32_1b.bmodel         # YOLOV5 FP32 BModel for BM1684X，batch_size=1，post process on CPU
│   ├── yolov5s_v6.1_3output_int8_1b.bmodel         # YOLOV5 INT8 BModel for BM1684X，batch_size=1，post process on CPU
│   └── yolov5s_v6.1_3output_int8_4b.bmodel         # YOLOV5 INT8 BModel for BM1684X，batch_size=4，post process on CPU
└── BM1684X_tpukernel
    ├── yolov5s_tpukernel_fp16_1b.bmodel            # YOLOV5 FP16 BModel for BM1684X，batch_size=1，post process with tpu_kernel
    ├── yolov5s_tpukernel_fp32_1b.bmodel            # YOLOV5 FP32 BModel for BM1684X，batch_size=1，post process with tpu_kernel
    ├── yolov5s_tpukernel_int8_1b.bmodel            # YOLOV5 INT8 BModel for BM1684X，batch_size=1，post process with tpu_kernel
    └── yolov5s_tpukernel_int8_4b.bmodel            # YOLOV5 INT8 BModel for BM1684X，batch_size=4，post process with tpu_kernel
```

The downloaded data include:

```bash
./videos
├── demo_skeleton.mp4                         # Human Detection + Key Point Detection + Behavior Recognition Test Video
├── S017C001P003R001A001_rgb.avi              # Human Detection + Key Point Detection + Behavior Recognition Test Video
├── S017C001P003R002A008_rgb.avi              # Human Detection + Key Point Detection + Behavior Recognition Test Video
└── test.mp4                                  # Human Detection + Key Point Detection Test Video
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the yolov5-fastpose demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config/
├── decode.json                         # decoding configuration
├── engine_group_alphapose.json         # sophon-stream human detect + key points detect graph configuration
├── engine_group.json                   # sophon-stream human detect + key points detect + behavior recognition graph configuration
├── fastpose_group.json                 # fastpose configuration
├── fastpose_infer.json                 # fastpose inference configuration
├── fastpose_post.json                  # fastpose post-process configuration
├── fastpose_pre.json                   # fastpose pre-process configuration
├── posec3d_group.json                  # posec3d configuration
├── yolov5_fastpose_posec3d_demo.json   # yolov5-fastpose-posec3d demo configuration
├── yolov5_group.json                   # yolov5 configuration
├── yolov5_infer.json                   # yolov5 inference configuration file
├── yolov5_post.json                    # yolov5 post-processing configuration file
└── yolov5_pre.json                     # yolov5 pre-processing configuration file
```

[engine_group_alphapose.json](./config/engine_group_alphapose.json) is the configuration file for the AlphaPose algorithm accompanying YOLOv5 as a detector. It outputs bounding boxes and key points of human bodies, as shown in the following image:

<img src="./pics/yolov5_fastpose.jpg" width="800">

Built on top of this, the posec3d behavior recognition model is added to form the configuration file [engine_group.json](./config/engine_group.json). It outputs human detection boxes, key points, and behavior categories, as illustrated below:

<img src="./pics/yolov5_fastpose_posec3d.jpg" width="800">

Note that in this image, posec3d takes 72 frames as input, and the final category label is placed on the 0th frame. The video is from the demo_skeleton.mp4 downloaded by the download.sh script.

Among them, [yolov5_fastpose_posec3d_demo.json](./config/yolov5_fastpose_posec3d_demo.json) is the overall configuration file for the demo, managing input streams and other information. Multiple streams of data can be supported on one image, and the `channels` section includes information such as the stream URL for each channel.

In cases where the `channel_id` property is not specified in the configuration file, the `channel_id` for each data stream in the demo is assigned a default value starting from 0.

The `heatmap_loss` parameter determines the post-processing flow of fastpose. Currently, based on the official [model configuration](https://github.com/MVIG-SJTU/AlphaPose/blob/master/docs/MODEL_ZOO.md), only `MSELoss` is supported.


```json
{
  "channels": [
    {
      "channel_id": 0,
      "url": "../yolov5_fastpose_posec3d/data/nturgb+d_rgb/S017C001P003R002A008_rgb.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov5_fastpose_posec3d/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov5_fastpose_posec3d_results",
  "engine_config_path": "../yolov5_fastpose_posec3d/config/engine_group.json",
  "heatmap_loss": "MSELoss"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov5_fastpose_posec3d",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolov5_fastpose_posec3d/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolov5_fastpose_posec3d/config/yolov5_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 6001,
                "element_config": "../yolov5_fastpose_posec3d/config/fastpose_group.json",
                "inner_elements_id": [20001, 20002, 20003]
            },
            {
                "element_id": 7001,
                "element_config": "../yolov5_fastpose_posec3d/config/posec3d_group.json",
                "inner_elements_id": [30001, 30002, 30003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 6001,
                "dst_port": 0
            },
            {
                "src_element_id": 6001,
                "src_port": 0,
                "dst_element_id": 7001,
                "dst_port": 0
            }
        ]
    }
]
```

[fastpose_group.json](./config/fastpose_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

When `use_tpu_kernel` is set to `true`, it will utilize the tpu_kernel post-processing(using tpu to do post process). Note that tpu_kernel post-processing is only supported on BM1684X devices.

```json
{
    "configure": {
        "model_path": "../yolov5_fastpose_posec3d/data/models/BM1684X/halpe26_fast_res50_256x192_int8_1b.bmodel",
        "stage": [
            "pre"
        ],
        "heatmap_loss": "MSELoss",
        "area_thresh": 0.0
    },
    "shared_object": "../../build/lib/libfastpose.so",
    "name": "fastpose",
    "side": "sophgo",
    "thread_number": 1
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolov5_fastpose_posec3d/config/yolov5_fastpose_posec3d_demo.json
```

The running results of three video streams are as follows
```bash
 total time cost 5453888 us.
frame count is 291 | fps is 53.3564 fps.
```

## 7. Performance Testing

Performance varies greatly from video to video, please subject to actual.
---

## yolov7

# YOLOv7 Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOv7 Demo](#yolov7-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标检测应用。

本例程插件的连接方式如下图所示

![process](./pics/elements.jpg)

**源代码** (https://github.com/WongKinYiu/yolov7) 

本例程中，yolov7算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)，支持BM1688(SoC)
* BM1684X平台上，支持tpu_kernel后处理
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：

```bash
./models/
├── BM1684
│   ├── yolov7_v0.1_3output_fp32_1b.bmodel         # 用于BM1684的FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov7_v0.1_3output_int8_1b.bmodel         # 用于BM1684的INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov7_v0.1_3output_int8_4b.bmodel         # 用于BM1684的INT8 BModel，batch_size=4，后处理在CPU上进行
├── BM1684X
│   ├── yolov7_v0.1_3output_fp16_1b.bmodel         # 用于BM1684X的FP16 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov7_v0.1_3output_fp32_1b.bmodel         # 用于BM1684X的FP32 BModel，batch_size=1，后处理在CPU上进行
│   ├── yolov7_v0.1_3output_int8_1b.bmodel         # 用于BM1684X的INT8 BModel，batch_size=1，后处理在CPU上进行
│   └── yolov7_v0.1_3output_int8_4b.bmodel         # 用于BM1684X的INT8 BModel，batch_size=4，后处理在CPU上进行
├── BM1684X_tpukernel
│   ├── yolov7_tpukernel_fp16_1b.bmodel            # 用于BM1684X的FP16 BModel，batch_size=1，后处理采用tpu_kernel
│   ├── yolov7_tpukernel_fp32_1b.bmodel            # 用于BM1684X的FP32 BModel，batch_size=1，后处理采用tpu_kernel
│   ├── yolov7_tpukernel_int8_1b.bmodel            # 用于BM1684X的INT8 BModel，batch_size=1，后处理采用tpu_kernel
│   └── yolov7_tpukernel_int8_4b.bmodel            # 用于BM1684X的INT8 BModel，batch_size=4，后处理采用tpu_kernel
└── BM1688
    ├── yolov7_v0.1_3output_int8_1b.bmodel         # 用于BM1688的INT8 单核BModel，batch_size=1
    ├── yolov7_v0.1_3output_int8_4b.bmodel         # 用于BM1688的INT8 单核BModel，batch_size=4
    ├── yolov7_v0.1_3output_int8_1b_2core.bmodel   # 用于BM1688的INT8 双核BModel，batch_size=1
    └── yolov7_v0.1_3output_int8_4b_2core.bmodel   # 用于BM1688的INT8 双核BModel，batch_size=4
```

模型说明:

以上模型移植于[yolov7官方](https://github.com/WongKinYiu/yolov7)，插件配置`mean=[0,0,0]`，`std=[255,255,255]`，支持COCO数据集的80分类检测任务。

下载的数据包括：

```bash
videos/
├── carvana_video.mp4   # 测试视频
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolov7 demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine_group.json           # sophon-stream 简化的graph配置
├── yolov7_classthresh_roi_example.json  # yolov7按照类别设置阈值的参考配置文件，需要注意，按类别设置阈值仅支持非tpu_kernel的后处理模式
├── yolov7_demo.json            # demo输入配置文件
└── yolov7_group.json           # 简化的yolov7配置文件，将yolov7的前处理、推理、后处理合到一个配置文件中

```

其中，[yolov7_demo.json](./config/yolov7_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov7/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov7_results",
  "engine_config_path": "../yolov7/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov7",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolov7/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolov7/config/yolov7_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[yolov7_group.json](./config/yolov7_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine_group.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

`use_tpu_kernel`为`true`时，会使用tpu_kernel后处理。tpu_kernel后处理只支持BM1684X设备。

```json
{
    "configure": {
        "model_path": "../yolov7/data/models/BM1684X_tpukernel/yolov7_tpukernel_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [
            0,
            0,
            0
        ],
        "std": [
            255,
            255,
            255
        ],
        "use_tpu_kernel": true
    },
    "shared_object": "../../build/lib/libyolov7.so",
    "name": "yolov7_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../yolov7/config/yolov7_demo.json
```

在1684x，pcie工作模式下，多路视频流运行结果大致如下:
```bash
 total time cost 26692057 us.
frame count is 2848 | fps is 106.698 fps.
```

## 7. 性能测试

目前，yolov7例程支持在BM1684X和BM1684的PCIE、SOC模式下进行推理，支持在BM1688 SOC模式下进行推理。

在不同的设备上可能需要修改json配置，例如模型路径、输入路数等。json的配置方法参考6.1节，程序运行方法参考上文6.2节。

由于PCIE设备cpu能力差距较大，性能数据没有参考意义，这里只给出SOC模式的测试结果。

测试视频`elevator-1080p-25fps-4000kbps.h264`，编译选项为Release模式，模型类型为int8结果如下:

|设备   |路数|算法线程数|CPU利用率(%)|系统内存(M)|系统内存峰值(M)|TPU利用率(%)|设备内存(M)|设备内存峰值(M)|平均FPS|
|-------|----|---------|------------|----------|--------------|------------|-----------|--------------|-------|
|SE9-16 |4   |4-4-4    | 247.1      | 178.02   | 193.64       | 94         | 1869      | 2700       | 49.79 |
|SE9-8  |4   |4-4-4    | 140.0      | 192.25   | 196.45       | 94         | 1655      | 2836         | 28.38 |
|SE7    |4   |4-4-4    | 70.40      | 121.48   | 130.70       | 100        |1853       | 1890         |107.96 |
|SE5-16 |4   |4-4-4    | 149.50     | 185.40   | 186.96       | 99         | 1400      | 1526         | 47.69 |
|SE5-8  |4   |4-4-4    | 111.90     | 171.13   | 178.56       | 98         | 1360      | 1501         | 30.64 |



> **测试说明**：
1. SE9使用的双核4batch模型，SE7使用单batch并开启tpu_kernel后处理，SE5使用4batch模型，性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；
3. 以上性能测试均基于int8模型给出；
4. 在BM1684设备上运行时，batch_size为4的模型可以达到更高的fps；
5. 在BM1684X设备上，使用batch_size为1的模型，并且开启tpu_kernel后处理，可以达到更高的fps；
6. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;
7. 上表中，所有数据均在不保存图片的情况下测试获得。

### yolov7 - README_EN.md

# YOLOv7 Demo

English | [简体中文](README.md)

## Catalogs
- [YOLOv7 Demo](#yolov7-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object detection application.

The connection method for this example plugin is shown in the following diagram.

![process](./pics/elements.jpg)

**source code** (https://github.com/WongKinYiu/yolov7) 

In this example, the pre-processing, inference, and post-processing of the YOLOv7 algorithm are computed on three separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Feature

* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(SoC)
* On the BM1684X platform, the TPU_kernel post-processing is supported.
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
│   ├── yolov7_v0.1_3output_fp32_1b.bmodel         # FP32 BModel for BM1684, with a batch size of 1. Post-processing takes place on the CPU.
│   ├── yolov7_v0.1_3output_int8_1b.bmodel         # INT8 BModel for BM1684, with a batch size of 1. Post-processing takes place on the CPU.
│   └── yolov7_v0.1_3output_int8_4b.bmodel         # INT8 BModel for BM1684, with a batch size of 4. Post-processing takes place on the CPU.
├── BM1684X
│   ├── yolov7_v0.1_3output_fp16_1b.bmodel         # FP16 BModel for BM1684X, with a batch size of 1. Post-processing takes place on the CPU.
│   ├── yolov7_v0.1_3output_fp32_1b.bmodel         # FP32 BModel for BM1684X, with a batch size of 1. Post-processing takes place on the CPU.
│   ├── yolov7_v0.1_3output_int8_1b.bmodel         # INT8 BModel for BM1684X, with a batch size of 1. Post-processing takes place on the CPU.
│   └── yolov7_v0.1_3output_int8_4b.bmodel         # INT8 BModel for BM1684X, with a batch size of 4. Post-processing takes place on the CPU.
├── BM1684X_tpukernel
│   ├── yolov7_tpukernel_fp16_1b.bmodel            # FP16 BModel for BM1684X, with a batch size of 1. Post-processing utilizes the tpu_kernel.
│   ├── yolov7_tpukernel_fp32_1b.bmodel            # FP32 BModel for BM1684X, with a batch size of 1. Post-processing utilizes the tpu_kernel.
│   ├── yolov7_tpukernel_int8_1b.bmodel            # INT8 BModel for BM1684X, with a batch size of 1. Post-processing utilizes the tpu_kernel.
│   └── yolov7_tpukernel_int8_4b.bmodel            # INT8 BModel for BM1684X, with a batch size of 4. Post-processing utilizes the tpu_kernel.
└── BM1688
    ├── yolov7_v0.1_3output_int8_1b.bmodel         # INT8 BModel for BM1688, with a batch size of 1.
    ├── yolov7_v0.1_3output_int8_4b.bmodel         # INT8 BModel for BM1688, with a batch size of 4.
    ├── yolov7_v0.1_3output_int8_1b_2core.bmodel   # INT8 2CORE-BModel for BM1688, with a batch size of 1.
    └── yolov7_v0.1_3output_int8_4b_2core.bmodel   # INT8 2CORE-BModel for BM1688, with a batch size of 4.
```

Model description:

The above models are ported from the official [yolov7 repository](https://github.com/WongKinYiu/yolov7) . The plugin configuration includes `mean=[0,0,0]`, `std=[255,255,255]`, supporting 80-class detection tasks from the COCO dataset.


The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the YOLOv7 demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config/
├── decode.json                 # decoding configuration
├── engine_group.json           # sophon-stream Simplified graph configuration
├── yolov7_classthresh_roi_example.json  # reference configuration file for setting thresholds per category in YOLOv7. Please note that setting thresholds per category is only supported in non-tpu_kernel post-processing mode
├── yolov7_demo.json            # input configuration file for the demo
└──  yolov7_group.json           # A simplified YOLOv7 configuration file that combines pre-processing, inference, and post-processing into one configuration file.
```

Indeed, [yolov7_demo.json](./config/yolov7_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolov7/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov7/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov7_results",
  "engine_config_path": "../yolov7/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov7",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../config/yolov7_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```


[yolov7_group.json](./config/yolov7_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

When `use_tpu_kernel` is set to `true`, it will utilize the tpu_kernel post-processing(using tpu to do post process). Note that tpu_kernel post-processing is only supported on BM1684X devices.

```json
{
    "configure": {
        "model_path": "../data/models/BM1684X_tpukernel/yolov7s_tpukernel_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [
            0,
            0,
            0
        ],
        "std": [
            255,
            255,
            255
        ],
        "use_tpu_kernel": true
    },
    "shared_object": "../../../build/lib/libyolov7.so",
    "name": "yolov7_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolov7/config/yolov7_demo.json
```

In 1684x, PCIe working mode, the running results of multi-channel video streams are roughly as follows:
```bash
 total time cost 26692057 us.
frame count is 2848 | fps is 106.698 fps.
```

## 7. Performance Testing

Currently, the YOLOv7 example supports inference on BM1684X and BM1684 in PCIe and SoC modes, and supports inference on BM1688 in SoC mode.

Modifications in JSON configurations might be necessary when switching between different devices, such as adjusting model paths, input channels, etc. Refer to section 6.1 for JSON configuration methods and section 6.2 for program execution methods.

Due to significant differences in CPU capabilities among PCIe devices, performance data is not meaningful. Therefore, only provide the test results for SOC mode.

The tested video is `elevator-1080p-25fps-4000kbps.h264`. The compilation was done in Release mode. The results are as follows:


| Device | Number of Channels | Algorithm Thread Count | CPU Utilization (%) | System Memory (M) | Peak System Memory (M) | TPU Utilization (%) | Device Memory (M) | Peak Device Memory (M) | Average FPS |
|-------|----|---------|------------|----------|--------------|------------|-----------|--------------|-------|
|SE9-16 |4   |4-4-4    | 247.1      | 178.02   | 193.64       | 94         |  1869     | 2700         | 49.79 |
|SE9-8  |4   |4-4-4    | 140.0      | 192.25   | 196.45       | 94         | 1655      | 2836         | 28.38 |
|SE7    |4   |4-4-4    | 70.40      | 121.48   | 130.70       | 100        |1853       | 1890         |107.96 |
|SE5-16 |4   |4-4-4    | 149.50     | 185.40   | 186.96       | 99         | 1400      | 1526         | 47.69 |
|SE5-8  |4   |4-4-4    | 111.90     | 171.13   | 178.56       | 98         | 1360      | 1501         | 30.64 |

> **Test Description**:
1. SE9 uses a dual-core 4batch model, SE7 uses a single batch and turns on tpu_kernel post-processing, and SE5 uses a 4batch model. The performance test results have a certain degree of volatility. It is recommended to average multiple tests;
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
3. All aforementioned performance tests are based on the INT8 model.
4. Running models with a batch size of 4 on the BM1684 device can achieve higher FPS.
5. On the BM1684X device, utilizing a batch size of 1 for the model and enabling the tpu_kernel for post-processing can yield higher FPS.
6. For the settings of input channels and algorithm thread count in the table, please refer to [JSON configuration explanation](#61-json-configuration). CPU utilization and system memory can be checked using the `top` command. TPU utilization and device memory can be checked using the `bm-smi` command. FPS can be obtained from the logs printed during program execution.
7. In the above table, all the data were obtained by testing without saving the images.
---

## yolov8

# YOLOv8 Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOv8 Demo](#yolov8-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标检测应用。

**源代码** (https://github.com/ultralytics/ultralytics)

本例程中，yolov8算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)、BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：

```bash
./models/
├── BM1684
|   ├── yolov8n_cls_fp32_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684的FP32 yolov8-cls BModel，batch_size=1
│   ├── yolov8n_pose_fp32_1b.bmodel # 使用TPU-MLIR编译，用于BM1684的FP32 yolov8-pose BModel，batch_size=1
│   ├── yolov8n_pose_int8_1b.bmodel # 使用TPU-MLIR编译，用于BM1684的INT8 yolov8-pose BModel，batch_size=1
│   ├── yolov8s_seg_fp32_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684的FP32 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_int8_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684的int8 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_getmask_32_fp32.bmodel  # 使用TPU-MLIR编译，用于BM1684的FP32 yolov8-seg TPU后处理BModel
│   ├── yolov8s_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684的FP32 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_int8_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684的INT8 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_int8_4b.bmodel   # 使用TPU-MLIR编译，用于BM1684的INT8 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_opt_fp32_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684的FP32 yolov8-detect BModel，batch_size=1，针对后处理做了优化
│   ├── yolov8s_opt_int8_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684的INT8 yolov8-detect BModel，batch_size=1，针对后处理做了优化
│   └── yolov8s_opt_int8_4b.bmodel  # 使用TPU-MLIR编译，用于BM1684的INT8 yolov8-detect BModel，batch_size=4，针对后处理做了优化
├── BM1684X
|   ├── yolov8n_cls_fp32_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov8-cls BModel，batch_size=1
│   ├── yolov8n_pose_fp32_1b.bmodel # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov8-pose BModel，batch_size=1
│   ├── yolov8n_pose_int8_1b.bmodel # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov8-pose BModel，batch_size=1
│   ├── yolov8s_seg_fp32_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_int8_1b.bmodel  # 使用TPU-MLIR编译，用于BM1684X的int8 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_getmask_32_fp32.bmodel  # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov8-seg TPU后处理BModel
│   ├── yolov8s_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP16 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_int8_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_int8_4b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_opt_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP32 BModel，batch_size=1，针对后处理做了优化
│   ├── yolov8s_opt_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP16 BModel，batch_size=1，针对后处理做了优化
│   ├── yolov8s_opt_int8_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 BModel，batch_size=1，针对后处理做了优化
│   └── yolov8s_opt_int8_4b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 BModel，batch_size=4，针对后处理做了优化
├── BM1688
|   ├── yolov8n_cls_fp32_1b.bmodel    # 使用TPU-MLIR编译，用于BM1688的FP32 yolov8-cls BModel，batch_size=1
│   ├── yolov8n_cls_fp32_1b_2core.bmodel    # 使用TPU-MLIR编译，用于BM1688的FP32 双核 yolov8-cls BModel，batch_size=1
│   ├── yolov8n_pose_fp32_1b_1core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 单核 yolov8-pose BModel，batch_size=1
│   ├── yolov8n_pose_fp32_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 双核 yolov8-pose BModel，batch_size=1
│   ├── yolov8n_pose_int8_1b_1core.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 单核 yolov8-pose BModel，batch_size=1
│   ├── yolov8n_pose_int8_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 双核 yolov8-pose BModel，batch_size=1
│   ├── yolov8s_seg_fp32_1b_1core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 单核 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_fp32_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 双核 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_int8_1b_1core.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 单核 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_int8_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 双核 yolov8-seg BModel，batch_size=1
│   ├── yolov8s_seg_getmask_32_fp32.bmodel  # 使用TPU-MLIR编译，用于BM1688的FP32 yolov8-seg TPU后处理BModel
│   ├── yolov8s_fp16_1b_2core.bmodel  # 使用TPU-MLIR编译，用于BM1688的FP16 双核 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_fp16_1b.bmodel        # 使用TPU-MLIR编译，用于BM1688的FP16 单核 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_fp16_4b_2core.bmodel  # 使用TPU-MLIR编译，用于BM1688的FP16 双核 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_fp16_4b.bmodel        # 使用TPU-MLIR编译，用于BM1688的FP16 单核 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_fp32_1b_2core.bmodel  # 使用TPU-MLIR编译，用于BM1688的FP32 双核 yolov8-detect BModel，batch_size=1
|   ├── yolov8s_fp32_1b.bmodel        # 使用TPU-MLIR编译，用于BM1688的FP32 单核 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_fp32_4b_2core.bmodel  # 使用TPU-MLIR编译，用于BM1688的FP32 双核 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_fp32_4b.bmodel        # 使用TPU-MLIR编译，用于BM1688的FP32 单核 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_int8_1b_2core.bmodel  # 使用TPU-MLIR编译，用于BM1688的INT8 双核 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_int8_1b.bmodel        # 使用TPU-MLIR编译，用于BM1688的INT8 单核 yolov8-detect BModel，batch_size=1
│   ├── yolov8s_int8_4b_2core.bmodel  # 使用TPU-MLIR编译，用于BM1688的INT8 双核 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_int8_4b.bmodel        # 使用TPU-MLIR编译，用于BM1688的INT8 单核 yolov8-detect BModel，batch_size=4
│   ├── yolov8s_opt_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 BModel，batch_size=1, num_core=1，针对后处理做了优化
│   ├── yolov8s_opt_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP16 BModel，batch_size=1, num_core=1，针对后处理做了优化
│   ├── yolov8s_opt_int8_1b.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 BModel，batch_size=1, num_core=1，针对后处理做了优化
│   ├── yolov8s_opt_int8_4b.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 BModel，batch_size=4, num_core=1，针对后处理做了优化
│   ├── yolov8s_opt_fp32_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 BModel，batch_size=1, num_core=2，针对后处理做了优化
│   ├── yolov8s_opt_fp16_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP16 BModel，batch_size=1, num_core=2，针对后处理做了优化
│   ├── yolov8s_opt_int8_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 BModel，batch_size=1, num_core=2，针对后处理做了优化
│   └── yolov8s_opt_int8_4b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的INT8 BModel，batch_size=4, num_core=2，针对后处理做了优化
└── CV186X
    ├── yolov8s_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的FP32 BModel，batch_size=1
    ├── yolov8s_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的FP16 BModel，batch_size=1
    ├── yolov8s_int8_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的INT8 BModel，batch_size=1
    ├── yolov8s_int8_4b.bmodel   # 使用TPU-MLIR编译，用于CV186X的INT8 BModel，batch_size=4
    ├── yolov8s_opt_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的FP32 BModel，batch_size=1，针对后处理做了优化
    ├── yolov8s_opt_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的FP16 BModel，batch_size=1，针对后处理做了优化
    ├── yolov8s_opt_int8_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的INT8 BModel，batch_size=1，针对后处理做了优化
    └── yolov8s_opt_int8_4b.bmodel   # 使用TPU-MLIR编译，用于CV186X的INT8 BModel，batch_size=4，针对后处理做了优化
```

模型说明:

以上模型移植于[yolov8官方](https://github.com/ultralytics/ultralytics)，插件配置`mean=[0,0,0]`，`std=[255,255,255]`。

其中，名为`yolov8n_cls`的模型支持基于`ImageNet`的1000类分类任务，名为`yolov8n_pose`的模型支持人体姿态检测任务，其它模型支持COCO数据集的80分类检测任务。

任务配置时，需要参考[yolov8_element](../../element/algorithm/yolov8/README.md)的说明来修改配置文件。

目前，默认的配置方式实现的是目标检测功能。如果希望运行姿态检测算法，则除了需要将模型和任务修改外，还需要将[yolov8_demo.json](./config/yolov8_demo.json)中可视化算法名称修改为`draw_yolov8_det_pose`。对于分类算法，因为分类任务没有可视化的结果，因此不需要配置可视化算法名称，也不需要保存图片，观察程序运行中的日志即可。对于分割算法，除了需要将模型和任务修改外，还需要将[yolov8_demo.json](./config/yolov8_demo.json)中可视化算法名称修改为`draw_yolov8_seg`，除此之外，yolov8_seg可以使用TPU做后处理，此时，需要将"seg_tpu_opt"设置为true，"mask_bmodel_path"设置为TPU后处理所需要的bmodel路径。

下载的数据包括：

```bash
videos/
├── demo_skeleton.mp4     # 姿态检测测试视频1
├── yaotou.mp4            # 姿态检测测试视频2
├── carvana_video.mp4     # 目标检测测试视频
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi

./pics/                     # 分类任务测试数据
├── bus.jpg
├── n01440764_10043.jpg
├── n01440764_10470.jpg
├── n01440764_10744.jpg
├── n01440764_10845.jpg
├── n01440764_11170.jpg
├── n01440764_12021.jpg
├── n01440764_12063.jpg
├── n01440764_12090.jpg
├── n01440764_12329.jpg
└── n01440764_12435.jpg
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolov8 demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine_group.json           # sophon-stream graph配置
├── yolov8_classthresh_roi_example.json  # yolov8按照类别设置阈值的参考配置文件，需要注意，按类别设置阈值仅支持非tpu_kernel的后处理模式
├── yolov8_demo.json            # demo输入配置文件
├── yolov8_group.json           # 简化的yolov8配置文件，将yolov8的前处理、推理、后处理合到一个配置文件中
```

其中，[yolov8_demo.json](./config/yolov8_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov8/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov8_results",
  "engine_config_path": "../yolov8/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov8",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolov8/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolov8/config/yolov8_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[yolov8_group.json](./config/yolov8_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine_group.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

```json
{
    "configure": {
        "model_path": "../yolov8/data/models/BM1684X/yolov8s_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [
            0,
            0,
            0
        ],
        "std": [
            255,
            255,
            255
        ]
    },
    "shared_object": "../../build/lib/libyolov8.so",
    "name": "yolov8_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../yolov8/config/yolov8_demo.json
```

2路视频流运行结果如下
```bash
 total time cost 13714673 us.
frame count is 1424 | fps is 103.83 fps.
```

## 7. 性能测试


目前，yolov8例程支持在BM1684X和BM1684的PCIe、SoC模式，BM1688的SoC模式下进行推理。

在不同的设备上可能需要修改json配置，例如模型路径、输入路数等。json的配置方法参考6.1节，程序运行方法参考上文6.2节。

由于PCIe设备cpu能力差距较大，性能数据没有参考意义，这里只给出SoC模式的测试结果。

测试视频`test_car_person_1080P.avi`，编译选项为Release模式，测试yolov8 检测模型性能，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|TPU利用率(%)|设备内存峰值(M)|平均FPS|
|----|----|-----|-----|-----|-----|---|
|SE7|6|6-6-6|672|60|3500|149.891|
|SE5-16|3|3-3-3|306|90|1012|95.19|
|SE9-16|4|4-4-4|504|85|2578|96.05|


> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；
3. 以上性能测试均基于int8 优化后处理的模型给出；
4. 在BM1684设备上运行时，batch_size为4的模型可以达到更高的fps；
5. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;


### yolov8 - README_EN.md

# YOLOv8 Demo

English | [简体中文](README.md)

## Catalogs
- [YOLOv8 Demo](#yolov8-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object detection application.

**source code** (https://github.com/ultralytics/ultralytics)

In this example, the pre-processing, inference, and post-processing of the YOLOv8 algorithm are computed on three separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Feature

* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(SoC)
* On the BM1684X platform, the TPU_kernel post-processing is supported.
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
|   ├── yolov8n_cls_fp32_1b.bmodel  # Compile with TPU-MLIR，FP32 yolov8-cls BModel for BM1684，batch_size=1
│   ├── yolov8n_pose_fp32_1b.bmodel # Compile with TPU-MLIR，FP32 yolov8-pose BModel for BM1684，batch_size=1
│   ├── yolov8n_pose_int8_1b.bmodel # Compile with TPU-MLIR，INT8 yolov8-pose BModel for BM1684，batch_size=1
│   ├── yolov8s_seg_fp32_1b.bmodel  # Compile with TPU-MLIR，FP32 yolov8-seg BModel for BM1684，batch_size=1
│   ├── yolov8s_seg_int8_1b.bmodel  # Compile with TPU-MLIR，INT8 yolov8-seg BModel for BM1684，batch_size=1
│   ├── yolov8s_seg_getmask_32_fp32.bmodel  # Compile with TPU-MLIR，FP32 yolov8-seg TPU post-processing BModel for BM1684
│   ├── yolov8s_fp32_1b.bmodel   # Compile with TPU-MLIR, FP32 BModel for BM1684, batch_size=1
│   ├── yolov8s_int8_1b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1684, batch_size=1
│   ├── yolov8s_int8_4b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1684, batch_size=4
│   ├── yolov8s_opt_fp32_1b.bmodel  # Compile with TPU-MLIR, FP32 yolov8-detect BModel for BM1684, batch_size=1, optimized the post-process
│   ├── yolov8s_opt_int8_1b.bmodel  # Compile with TPU-MLIR, INT8 yolov8-detect BModel for BM1684, batch_size=1, optimized the post-process
│   └── yolov8s_opt_int8_4b.bmodel  # Compile with TPU-MLIR, INT8 yolov8-detect BModel for BM1684, batch_size=4, optimized the post-process
├── BM1684X
|   ├── yolov8n_cls_fp32_1b.bmodel  # Compile with TPU-MLIR，FP32 yolov8-cls BModel for BM1684X，batch_size=1
│   ├── yolov8n_pose_fp32_1b.bmodel # Compile with TPU-MLIR，FP32 yolov8-pose BModel for BM1684X，batch_size=1
│   ├── yolov8n_pose_int8_1b.bmodel # Compile with TPU-MLIR，INT8 yolov8-pose BModel for BM1684X，batch_size=1
│   ├── yolov8s_seg_fp32_1b.bmodel  # Compile with TPU-MLIR，FP32 yolov8-seg BModel for BM1684X，batch_size=1
│   ├── yolov8s_seg_int8_1b.bmodel  # Compile with TPU-MLIR，INT8 yolov8-seg BModel for BM1684X，batch_size=1
│   ├── yolov8s_seg_getmask_32_fp32.bmodel  # Compile with TPU-MLIR，FP32 yolov8-seg TPU post-processing BModel for BM1684X
│   ├── yolov8s_fp32_1b.bmodel   # Compile with TPU-MLIR, FP32 BModel for BM1684X, batch_size=1
│   ├── yolov8s_fp16_1b.bmodel   # Compile with TPU-MLIR, FP16 BModel for BM1684X, batch_size=1
│   ├── yolov8s_int8_1b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1684X, batch_size=1
│   ├── yolov8s_int8_4b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1684X, batch_size=4
│   ├── yolov8s_opt_fp32_1b.bmodel   # Compile with TPU-MLIR, FP32 BModel for BM1684X, batch_size=1, optimized the post-process
│   ├── yolov8s_opt_fp16_1b.bmodel   # Compile with TPU-MLIR, FP16 BModel for BM1684X, batch_size=1, optimized the post-process
│   ├── yolov8s_opt_int8_1b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1684X, batch_size=1, optimized the post-process
│   └── yolov8s_opt_int8_4b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1684X, batch_size=4, optimized the post-process
├── BM1688
|   ├── yolov8n_cls_fp32_1b.bmodel    # Compile with TPU-MLIR，FP32 1 core yolov8-cls BModel for BM1688 batch_size=1
│   ├── yolov8n_cls_fp32_1b_2core.bmodel    # Compile with TPU-MLIR，FP32 2 core yolov8-cls BModel for BM1688，batch_size=1
│   ├── yolov8n_pose_fp32_1b_1core.bmodel   # Compile with TPU-MLIR, FP32 1core yolov8-pose BModel for BM1688, batch_size=1
│   ├── yolov8n_pose_fp32_1b_2core.bmodel   # Compile with TPU-MLIR, FP32 2core yolov8-pose BModel for BM1688, batch_size=1
│   ├── yolov8n_pose_int8_1b_1core.bmodel   # Compile with TPU-MLIR, INT8 1core yolov8-pose BModel for BM1688, batch_size=1
│   ├── yolov8n_pose_int8_1b_2core.bmodel   # Compile with TPU-MLIR, INT8 2core yolov8-pose BModel for BM1688, batch_size=1
│   ├── yolov8s_seg_fp32_1b_1core.bmodel   # Compile with TPU-MLIR, FP32 1core yolov8-seg BModel for BM1688, batch_size=1
│   ├── yolov8s_seg_fp32_1b_2core.bmodel   # Compile with TPU-MLIR, FP32 2core yolov8-seg BModel for BM1688, batch_size=1
│   ├── yolov8s_seg_int8_1b_1core.bmodel   # Compile with TPU-MLIR, INT8 1core yolov8-seg BModel for BM1688, batch_size=1
│   ├── yolov8s_seg_int8_1b_2core.bmodel   # Compile with TPU-MLIR, INT8 2core yolov8-seg BModel for BM1688, batch_size=1
│   ├── yolov8s_seg_getmask_32_fp32.bmodel  # Compile with TPU-MLIR，FP32 yolov8-seg TPU post-processing BModel for BM1688
│   ├── yolov8s_fp16_1b_2core.bmodel  # Compile with TPU-MLIR, FP16 2 core BModel for BM1688, batch_size=1
│   ├── yolov8s_fp16_1b.bmodel        # Compile with TPU-MLIR, FP16 1 core BModel for BM1688, batch_size=1
│   ├── yolov8s_fp16_4b_2core.bmodel  # Compile with TPU-MLIR, FP16 2 core BModel for BM1688, batch_size=4
│   ├── yolov8s_fp16_4b.bmodel        # Compile with TPU-MLIR, FP16 1 core BModel for BM1688, batch_size=4
│   ├── yolov8s_fp32_1b_2core.bmodel  # Compile with TPU-MLIR, FP32 2 core BModel for BM1688, batch_size=1
|   ├── yolov8s_fp32_1b.bmodel        # Compile with TPU-MLIR, FP32 1 core BModel for BM1688, batch_size=1
│   ├── yolov8s_fp32_4b_2core.bmodel  # Compile with TPU-MLIR, FP32 2 core BModel for BM1688, batch_size=4
│   ├── yolov8s_fp32_4b.bmodel        # Compile with TPU-MLIR, FP32 1 core BModel for BM1688, batch_size=4
│   ├── yolov8s_int8_1b_2core.bmodel  # Compile with TPU-MLIR, INT8 2 core BModel for BM1688, batch_size=1
│   ├── yolov8s_int8_1b.bmodel        # Compile with TPU-MLIR, INT8 1 core BModel for BM1688, batch_size=1
│   ├── yolov8s_int8_4b_2core.bmodel  # Compile with TPU-MLIR, INT8 2 core BModel for BM1688, batch_size=4
│   └── yolov8s_int8_4b.bmodel        # Compile with TPU-MLIR, INT8 1 core BModel for BM1688, batch_size=4
│   ├── yolov8s_opt_fp32_1b.bmodel   # Compile with TPU-MLIR, FP32 BModel for BM1688, batch_size=1, num_core=1, optimized the post-process
│   ├── yolov8s_opt_fp16_1b.bmodel   # Compile with TPU-MLIR, FP16 BModel for BM1688, batch_size=1, num_core=1, optimized the post-process
│   ├── yolov8s_opt_int8_1b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1688, batch_size=1, num_core=1, optimized the post-process
│   ├── yolov8s_opt_int8_4b.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1688, batch_size=4, num_core=1, optimized the post-process
│   ├── yolov8s_opt_fp32_1b_2core.bmodel   # Compile with TPU-MLIR, FP32 BModel for BM1688, batch_size=1, num_core=2, optimized the post-process
│   ├── yolov8s_opt_fp16_1b_2core.bmodel   # Compile with TPU-MLIR, FP16 BModel for BM1688, batch_size=1, num_core=2, optimized the post-process
│   ├── yolov8s_opt_int8_1b_2core.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1688, batch_size=1, num_core=2, optimized the post-process
│   └── yolov8s_opt_int8_4b_2core.bmodel   # Compile with TPU-MLIR, INT8 BModel for BM1688, batch_size=4, num_core=2, optimized the post-process
└── CV186X
    ├── yolov8s_fp32_1b.bmodel   # Compile with TPU-MLIR, FP32 BModel for CV186X, batch_size=1
    ├── yolov8s_fp16_1b.bmodel   # Compile with TPU-MLIR, FP16 BModel for CV186X, batch_size=1
    ├── yolov8s_int8_1b.bmodel   # Compile with TPU-MLIR, INT8 BModel for CV186X, batch_size=1
    ├── yolov8s_int8_4b.bmodel   # Compile with TPU-MLIR, INT8 BModel for CV186X, batch_size=4
    ├── yolov8s_opt_fp32_1b.bmodel   # Compile with TPU-MLIR, FP32 BModel for CV186X, batch_size=1, optimized the post-process
    ├── yolov8s_opt_fp16_1b.bmodel   # Compile with TPU-MLIR, FP16 BModel for CV186X, batch_size=1, optimized the post-process
    ├── yolov8s_opt_int8_1b.bmodel   # Compile with TPU-MLIR, INT8 BModel for CV186X, batch_size=1, optimized the post-process
    └── yolov8s_opt_int8_4b.bmodel   # Compile with TPU-MLIR, INT8 BModel for CV186X, batch_size=4, optimized the post-process
```

Model description:

The above models are ported from the official [yolov8 repository](https://github.com/ultralytics/ultralytics). The plugin configuration includes `mean=[0,0,0]`, `std=[255,255,255]`.

Among them, the model named `yolov8n_cls` supports a 1000-class classification task based on `ImageNet`, the model named `yolov8n_pose` supports a human pose detection task, and the other models support an 80-classification detection task for the COCO dataset.

For task configuration, you need to refer to [yolov8_element](../../element/algorithm/yolov8/README.md) for instructions on how to modify the configuration file.

Currently, the default configuration implements the target detection function. If you wish to run the attitude detection algorithm, you will need to change the name of the visualisation algorithm in [yolov8_demo.json](./config/yolov8_demo.json) in the visualisation algorithm name to `draw_yolov8_det_pose`. For the classification algorithm, since the classification task does not visualise the results, there is no need to configure the visualisation algorithm name or to save the image; it is sufficient to observe the logs as the program runs. For the segmentation algorithm, in addition to modifying the model and task, the visualization algorithm name in [yolov8_demo.json](./config/yolov8_demo.json) needs to be changed to `draw_yolov8_seg`. In addition, yolov8_seg can use TPU for post-processing. In this case, set "seg_tpu_opt" to true and "mask_bmodel_path" to the bmodel path required for post-processing of TPU.

The downloaded data include:

```bash
videos/
├── demo_skeleton.mp4     # test video 1 for pose models
├── yaotou.mp4            # test video 2 for pose models
├── carvana_video.mp4     # test videos for detect models
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi

./pics/                     # test images for classification models
├── bus.jpg
├── n01440764_10043.jpg
├── n01440764_10470.jpg
├── n01440764_10744.jpg
├── n01440764_10845.jpg
├── n01440764_11170.jpg
├── n01440764_12021.jpg
├── n01440764_12063.jpg
├── n01440764_12090.jpg
├── n01440764_12329.jpg
└── n01440764_12435.jpg
```
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the YOLOv8 demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config/
├── decode.json                 # decoding configuration
├── engine_group.json           # sophon-stream graph configuration
├── yolov8_classthresh_roi_example.json  # reference configuration file for setting thresholds per category in YOLOv8. Please note that setting thresholds per category is only supported in non-tpu_kernel post-processing mode
├── yolov8_demo.json            # input configuration file for the demo
├── yolov8_group.json           # A simplified YOLOv8 configuration file that combines pre-processing, inference, and post-processing into one configuration file.
```

Indeed, [yolov8_demo.json](./config/yolov8_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolov8/data/videos/test_car_person_1080P.avi",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolov8/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolov8_results",
  "engine_config_path": "../yolov8/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolov8",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolov8/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolov8/config/yolov8_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[yolov8_group.json](./config/yolov8_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

```json
{
    "configure": {
        "model_path": "../yolov8/data/models/BM1684X/yolov8s_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [
            0,
            0,
            0
        ],
        "std": [
            255,
            255,
            255
        ]
    },
    "shared_object": "../../build/lib/libyolov8.so",
    "name": "yolov8_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolov8/config/yolov8_demo.json
```

The running results of two video streams are as follows
```bash
 total time cost 13714673 us.
frame count is 1424 | fps is 103.83 fps.
```

## 7. Performance Testing

Currently, the YOLOv5 example supports inference on BM1684X and BM1684 in PCIe and SoC modes, and supports inference on BM1688 in SoC mode.

Modifications in JSON configurations might be necessary when switching between different devices, such as adjusting model paths, input channels, etc. Refer to section 6.1 for JSON configuration methods and section 6.2 for program execution methods.

Due to significant differences in CPU capabilities among PCIe devices, performance data is not meaningful. Therefore, only provide the test results for SOC mode.

The tested video is `elevator-1080p-25fps-4000kbps.h264`. The compilation was done in Release mode. Model is yolov8 det. The results are as follows:


| Device | Number of Channels | Algorithm Thread Count | CPU Utilization (%) |  TPU Utilization (%) |  Peak Device Memory (M) | Average FPS | 
|----|----|-----|-----|-----|-----|-----|
|SE7|6|6-6-6|672|60|3500|149.891|
|SE5-16|3|3-3-3|306|90|1012|95.19|
|SE9-16|4|4-4-4|504|85|2578|96.05|

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
3. All aforementioned performance tests are based on the INT8 model with optimized post-process.
4. Running models with a batch size of 4 on the BM1684 device can achieve higher FPS.
5. On the BM1684X device, utilizing a batch size of 1 for the model and enabling the tpu_kernel for post-processing can yield higher FPS.
6. For the settings of input channels and algorithm thread count in the table, please refer to [JSON configuration explanation](#61-json-configuration). CPU utilization and system memory can be checked using the `top` command. TPU utilization and device memory can be checked using the `bm-smi` command. FPS can be obtained from the logs printed during program execution.
7. Performance testing is not currently supported on the BM1688 device.
---

## yolov8_obb

# YOLOv8 obb Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOv8 obb Demo](#yolov8-obb-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建yolov8-obb旋转框目标检测应用。

**源代码** (https://github.com/ultralytics/ultralytics)

本例程中，yolov8_obb算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率

## 2. 特性

* 支持BM1684X、BM1684(x86 PCIe、SoC)、BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
chmod -R +x scripts/
./scripts/download.sh
```

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`datasets`两个子目录。

下载的模型包括：

```bash
./models
├── BM1684X
│   ├── yolov8s-obb_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolov8s-obb_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP16 BModel，batch_size=1
├── BM1688
│   ├── yolov8s-obb_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 BModel，batch_size=1, num_core=1
│   ├── yolov8s-obb_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP16 BModel，batch_size=1, num_core=1
│   ├── yolov8s-obb_fp32_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP32 BModel，batch_size=1, num_core=2
│   ├── yolov8s-obb_fp16_1b_2core.bmodel   # 使用TPU-MLIR编译，用于BM1688的FP16 BModel，batch_size=1, num_core=2
└── CV186X
   ├── yolov8s-obb_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的FP32 BModel，batch_size=1
   └── yolov8s-obb_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于CV186X的FP16 BModel，batch_size=1
```

模型说明:

bmodel的编译方法可见[sophon-demo/sample/YOLOv8_obb](https://github.com/sophgo/sophon-demo/tree/release/sample/YOLOv8_obb)。

下载的数据包括：

```bash
./videos
└── test.mp4 #一个十字路口的航拍视频
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolov8 demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine_obb_group.json       # sophon-stream graph配置
├── yolov8_obb_demo.json        # demo输入配置文件
└── yolov8_obb_group.json       # 简化的yolov8配置文件，将yolov8的前处理、推理、后处理合到一个配置文件中
```

其中，[yolov8_obb_demo.json](./config/yolov8_obb_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。
如果希望保存视频，可以把`yolov8_obb_demo.json`中的`engine_config_path`字段对应的值更改为：`../yolov8_obb/config/engine_obb_group_encode.json`。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolov8_obb/data/videos/test.mp4",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 3,
      "url": "../yolov8_obb/data/videos/test.mp4",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 4,
      "url": "../yolov8_obb/data/videos/test.mp4",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": 25
    },
    {
      "channel_id": 5,
      "url": "../yolov8_obb/data/videos/test.mp4",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": 25
    }
  ],
  "class_names": "../yolov8_obb/data/dotav1.names",
  "download_image": false,
  "draw_func_name": "draw_yolov8_obb_results",
  "engine_config_path": "../yolov8_obb/config/engine_obb_group.json"
}
```

[engine_obb_group.json](./config/engine_obb_group.json)包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，如果无法再减少，则可以调低`fps`，或者调低`sophon-stream/framework/src/datapipe.cc`中的宏定义`DEFAULT_DATA_PIPE_CAPACITY`再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
  {
    "graph_id": 0,
    "device_id": 0,
    "graph_name": "yolov8",
    "elements": [
      {
        "element_id": 5000,
        "element_config": "../yolov8_obb/config/decode.json",
        "ports": {
          "input": [
            {
              "port_id": 0,
              "is_sink": false,
              "is_src": true
            }
          ]
        }
      },
      {
        "element_id": 5001,
        "element_config": "../yolov8_obb/config/yolov8_obb_group.json",
        "inner_elements_id": [10001, 10002, 10003],
        "ports": {
          "output": [
            {
              "port_id": 0,
              "is_sink": true,
              "is_src": false
            }
          ]
        }
      }
    ],
    "connections": [
      {
        "src_element_id": 5000,
        "src_port": 0,
        "dst_element_id": 5001,
        "dst_port": 0
      }
    ]
  }
]
```

[yolov8_obb_group.json](./config/yolov8_obb_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine_group.json`中指定的`element_id`和`device_id`传入。其中，`thread_number`是`element`内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

```json
{
  "configure": {
    "model_path": "../yolov8_obb/data/models/BM1684X/yolov8s-obb_fp16_1b.bmodel",
    "threshold_conf": 0.25,
    "threshold_nms": 0.7,
    "bgr2rgb": true,
    "task_type": "Obb",
    "mean": [
      0,
      0,
      0
    ],
    "std": [
      255,
      255,
      255
    ]
  },
  "shared_object": "../../build/lib/libyolov8.so",
  "name": "yolov8_group",
  "side": "sophgo",
  "thread_number": 1
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

1. 运行可执行文件
```bash
./main --demo_config_path=../yolov8_obb/config/yolov8_obb_demo.json
```

2路视频流运行结果如下
```bash
 total time cost 197389382 us.
frame count is 4004 | fps is 20.2848 fps.
```


## 7. 性能测试


目前，yolov8_obb例程支持在BM1684X的PCIe、SoC模式，BM1688、CV186X的SoC模式下进行推理。

在不同的设备上可能需要修改json配置，例如模型路径、输入路数等。json的配置方法参考6.1节，程序运行方法参考上文6.2节。

由于PCIe设备cpu能力差距较大，性能数据没有参考意义，这里只给出SoC模式的测试结果。

测试视频`test.mp4`，编译选项为Release模式，测试`yolov8s-obb_fp16_1b.bmodel`模型(BM1688测`yolov8s-obb_fp16_1b_2core.bmodel`)性能，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|TPU利用率(%)|设备内存峰值(M)|平均FPS|
|----|----|-----    |-----      |-----      |-----         |---|
|SE7-32| 4 | 4-4-4  |    156    | 100%      |   1352       | 65.5  |
|SE9-16| 4 | 4-4-4  |    94    | 100%      |   1422       | 20.3  |
|SE9-8| 4 | 4-4-4  |    70    | 100%      |   1430       | 10.8  |


> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；
3. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;
4. 上述测试不带画框和存图。
5. 在SE9-8上，需要调整内存布局，npu heap >= 1280M，vpp heap >= 512M，参考[内存布局修改工具](https://doc.sophgo.com/bm1688_sdk-docs/v1.8/docs_latest_release/docs/BM1688_CV186AH_SophonSDK_doc/appendix/2_mem_edit_tools.html#id2)。
---

## yolov8_seg

# YOLOv8 Seg Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOv8 Seg Demo](#yolov8-seg-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频实例分割应用。

**源代码** (https://github.com/ultralytics/ultralytics)

本例程中，yolov8-seg算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率。

后处理参考 [sophon-demo YOLOv8_plus_seg](https://github.com/sophon-ai/sophon-demo) 的cpp示例实现，使用CPU完成mask的生成和渲染。可视化结果通过osd element输出，同时绘制检测框和分割掩码（mask）。

## 2. 特性

* 支持BM1684X(x86 PCIe、SoC)
* 支持多路视频流
* 支持多线程
* 支持检测框+分割掩码（mask）的可视化渲染
* 后处理与sophon-demo YOLOv8_plus_seg对齐

## 3. 准备模型与数据

模型来自sophon-demo的YOLOv8_plus_seg示例，可从sophon-demo仓库获取。

```bash
# 下载sophon-demo中的模型
# 从 open@sophgo.com:sophon-demo/YOLOv8_plus_seg/BM1684X.tar.gz 下载并解压
tar -xzf BM1684X.tar.gz
cp -r BM1684X/* ../yolov8_seg/data/models/BM1684X/
```

下载的模型包括：

```bash
./models/BM1684X/
├── yolov8s_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov8-seg BModel，batch_size=1
├── yolov8s_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP16 yolov8-seg BModel，batch_size=1
├── yolov8s_int8_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov8-seg BModel，batch_size=1
├── yolov8s_int8_4b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov8-seg BModel，batch_size=4
├── yolov9s_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov9-seg BModel，batch_size=1
├── yolov9s_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP16 yolov9-seg BModel，batch_size=1
├── yolov9c_fp32_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP32 yolov9-seg BModel，batch_size=1
├── yolov9c_fp16_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的FP16 yolov9-seg BModel，batch_size=1
├── yolov9c_int8_1b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov9-seg BModel，batch_size=1
└── yolov9c_int8_4b.bmodel   # 使用TPU-MLIR编译，用于BM1684X的INT8 yolov9-seg BModel，batch_size=4
```

模型说明：

以上模型移植于[yolov8官方](https://github.com/ultralytics/ultralytics)，插件配置`mean=[0,0,0]`，`std=[255,255,255]`。

模型输出包含2个tensor：
- 检测输出（transposed格式 `[1, 8400, 116]`）：包含bbox坐标（4）、类别分数（80）、mask系数（32）
- 分割输出（`[1, 32, 160, 160]`）：mask prototypes

> **注意**：与yolov8 detect模型不同，分割模型的检测输出采用transposed格式（anchor-major），后处理代码已自动适配。

测试数据与yolov8 sample共用，通过软链接指向 `../yolov8/data/` 目录。

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolov8_seg demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config/
├── decode.json                 # 解码配置
├── engine_group.json           # sophon-stream graph配置
├── yolov8_seg_demo.json        # demo输入配置文件
├── yolov8_group.json           # 简化的yolov8配置文件，将yolov8的前处理、推理、后处理合到一个配置文件中
└── osd.json                    # osd可视化配置
```

其中，[yolov8_seg_demo.json](./config/yolov8_seg_demo.json)是例程的整体配置文件，管理输入码流等信息。

Graph拓扑为 `decode → yolov8_group → osd → encode`，输出结果保存在 `./results` 目录下。

[yolov8_group.json](./config/yolov8_group.json)中配置task_type为"Seg"，模型使用FP32 BModel（推荐，FP32模型可输出更准确的置信度）：

```json
{
    "configure": {
        "model_path": "../yolov8_seg/data/models/BM1684X/yolov8s_fp32_1b.bmodel",
        "task_type": "Seg",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [0, 0, 0],
        "std": [255, 255, 255]
    },
    "shared_object": "../../build/lib/libyolov8.so",
    "name": "yolov8_group",
    "side": "sophgo",
    "thread_number": 4
}
```

[osd.json](./config/osd.json)中配置draw_utils为"OPENCV"以支持mask渲染，osd_type为"DET"以绘制检测框和分割掩码：

```json
{
    "configure": {
        "osd_type": "DET",
        "class_names_file": "../yolov8_seg/data/coco.names",
        "draw_utils": "OPENCV",
        "draw_interval": false,
        "put_text": true
    },
    "shared_object": "../../build/lib/libosd.so",
    "name": "osd",
    "side": "sophgo",
    "thread_number": 1
}
```

> **注意**：
> 1. 分割mask的渲染需要使用OPENCV draw_utils（BMCV模式不支持mask渲染）。
> 2. 当mSegmentedObjectMetadatas不为空时，osd会自动在检测框上叠加绘制分割掩码。
> 3. INT8模型在当前配置下可能产生较低的置信度，推荐使用FP32模型获得更好的分割效果。

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

1. 运行可执行文件
```bash
./main --demo_config_path=../yolov8_seg/config/yolov8_seg_demo.json
```

4路视频流运行结果如下
```bash
 total time cost 114256300 us.
frame count is 2848 | fps is 24.93 fps.
```

## 7. 性能测试

目前，yolov8_seg例程支持在BM1684X的PCIe、SoC模式下进行推理。

测试视频`test_car_person_1080P.avi`，编译选项为Release模式，测试yolov8 seg模型性能，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|TPU利用率(%)|设备内存峰值(M)|平均FPS|
|----|----|-----|-----|-----|-----|---|
|SE7|4|4-4-4|-|-|-|24.93|

> **测试说明**：
> 1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
> 2. 以上性能测试基于yolov8s_fp32_1b分割模型给出；
> 3. 分割后处理（CPU get_mask）及mask渲染（OpenCV）会占用额外的CPU时间，相比纯检测任务fps有所下降；
> 4. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;


### yolov8_seg - README_EN.md

# YOLOv8 Seg Demo

English | [简体中文](README.md)

## Table of Contents
- [YOLOv8 Seg Demo](#yolov8-seg-demo)
  - [Table of Contents](#table-of-contents)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Environment Setup](#4-environment-setup)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-compilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Running](#62-running)
  - [7. Performance Test](#7-performance-test)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video instance segmentation application.

**Source Code** (https://github.com/ultralytics/ultralytics)

In this example, the pre-processing, inference, and post-processing of the yolov8-seg algorithm are performed on three separate elements. Each element can open multiple threads internally, ensuring a certain detection efficiency.

The post-processing references the C++ example implementation of [sophon-demo YOLOv8_plus_seg](https://github.com/sophon-ai/sophon-demo) and uses CPU to complete mask generation and rendering. Visualization results are output through the osd element, which simultaneously draws detection boxes and segmentation masks.

## 2. Features

* Supports BM1684X (x86 PCIe, SoC)
* Supports multiple video streams
* Supports multi-threading
* Supports visualization of detection boxes + segmentation masks
* Post-processing aligned with sophon-demo YOLOv8_plus_seg

## 3. Prepare Models and Data

The models come from the sophon-demo YOLOv8_plus_seg example, available from the sophon-demo repository.

```bash
# Download models from sophon-demo
# Download and extract from open@sophgo.com:sophon-demo/YOLOv8_plus_seg/BM1684X.tar.gz
tar -xzf BM1684X.tar.gz
cp -r BM1684X/* ../yolov8_seg/data/models/BM1684X/
```

The downloaded models include:

```bash
./models/BM1684X/
├── yolov8s_fp32_1b.bmodel   # TPU-MLIR compiled, FP32 yolov8-seg BModel for BM1684X, batch_size=1
├── yolov8s_fp16_1b.bmodel   # TPU-MLIR compiled, FP16 yolov8-seg BModel for BM1684X, batch_size=1
├── yolov8s_int8_1b.bmodel   # TPU-MLIR compiled, INT8 yolov8-seg BModel for BM1684X, batch_size=1
├── yolov8s_int8_4b.bmodel   # TPU-MLIR compiled, INT8 yolov8-seg BModel for BM1684X, batch_size=4
├── yolov9s_fp32_1b.bmodel   # TPU-MLIR compiled, FP32 yolov9-seg BModel for BM1684X, batch_size=1
├── yolov9s_fp16_1b.bmodel   # TPU-MLIR compiled, FP16 yolov9-seg BModel for BM1684X, batch_size=1
├── yolov9c_fp32_1b.bmodel   # TPU-MLIR compiled, FP32 yolov9-seg BModel for BM1684X, batch_size=1
├── yolov9c_fp16_1b.bmodel   # TPU-MLIR compiled, FP16 yolov9-seg BModel for BM1684X, batch_size=1
├── yolov9c_int8_1b.bmodel   # TPU-MLIR compiled, INT8 yolov9-seg BModel for BM1684X, batch_size=1
└── yolov9c_int8_4b.bmodel   # TPU-MLIR compiled, INT8 yolov9-seg BModel for BM1684X, batch_size=4
```

Model description:

The above models are ported from [yolov8 official](https://github.com/ultralytics/ultralytics). Plugin configuration: `mean=[0,0,0]`, `std=[255,255,255]`.

The model output contains 2 tensors:
- Detection output (transposed format `[1, 8400, 116]`): contains bbox coordinates (4), class scores (80), mask coefficients (32)
- Segmentation output (`[1, 32, 160, 160]`): mask prototypes

> **Note**: Unlike yolov8 detect models, segmentation models use a transposed format (anchor-major). The post-processing code has been adapted accordingly.

The test data is shared with the yolov8 sample via a symlink pointing to the `../yolov8/data/` directory.

## 4. Environment Setup

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as SC series accelerator cards) on an x86/arm platform, you can directly use it as both the development and runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For detailed steps, refer to [x86-pcie platform development and runtime environment setup](../../docs/EnvironmentInstallGuide.md#3-x86-pcie-platform-development-and-runtime-environment-setup) or [arm-pcie platform development and runtime environment setup](../../docs/EnvironmentInstallGuide.md#5-arm-pcie-platform-development-and-runtime-environment-setup).

### 4.2 SoC Platform

If you are using an SoC platform (such as SE, SM series edge devices), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/` after flashing. You can use it directly as the runtime environment. Typically, an x86 host is also needed as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
Programs can be compiled directly on the PCIe platform. For details, refer to [sophon-stream compilation](../../docs/HowToMake.md).

### 5.2 SoC Platform
Programs are typically cross-compiled on an x86 host. You need to set up a cross-compilation environment on the x86 host using the SOPHON SDK, and package the required header files and library files into the sophon_sdk_soc directory. For details, refer to [sophon-stream compilation](../../docs/HowToMake.md). This example mainly depends on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

The parameters for each part of the yolov8_seg demo are located in the [config](./config/) directory, structured as follows:

```bash
./config/
├── decode.json                 # Decode configuration
├── engine_group.json           # sophon-stream graph configuration
├── yolov8_seg_demo.json        # Demo input configuration file
├── yolov8_group.json           # Simplified yolov8 configuration, combining pre-processing, inference, and post-processing into one config file
└── osd.json                    # OSD visualization configuration
```

[yolov8_seg_demo.json](./config/yolov8_seg_demo.json) is the overall configuration file for the example, managing input stream information.

The graph topology is `decode → yolov8_group → osd → encode`, with output saved in the `./results` directory.

Configure task_type as "Seg" in [yolov8_group.json](./config/yolov8_group.json). FP32 BModel is recommended (FP32 models produce more accurate confidence scores):

```json
{
    "configure": {
        "model_path": "../yolov8_seg/data/models/BM1684X/yolov8s_fp32_1b.bmodel",
        "task_type": "Seg",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "mean": [0, 0, 0],
        "std": [255, 255, 255]
    },
    "shared_object": "../../build/lib/libyolov8.so",
    "name": "yolov8_group",
    "side": "sophgo",
    "thread_number": 4
}
```

Configure draw_utils as "OPENCV" to support mask rendering, and osd_type as "DET" to draw detection boxes and segmentation masks in [osd.json](./config/osd.json):

```json
{
    "configure": {
        "osd_type": "DET",
        "class_names_file": "../yolov8_seg/data/coco.names",
        "draw_utils": "OPENCV",
        "draw_interval": false,
        "put_text": true
    },
    "shared_object": "../../build/lib/libosd.so",
    "name": "osd",
    "side": "sophgo",
    "thread_number": 1
}
```

> **Note**:
> 1. Segmentation mask rendering requires OPENCV draw_utils (BMCV mode does not support mask rendering).
> 2. When mSegmentedObjectMetadatas is not empty, osd automatically overlays segmentation masks on top of detection boxes.
> 3. INT8 models may produce lower confidence scores under the current configuration. FP32 models are recommended for better segmentation results.

### 6.2 Running

For PCIe platforms, tests can be run directly on the PCIe platform. For SoC platforms, copy the cross-compiled dynamic libraries, executable files, required models, and test data to the SoC platform for testing.

1. Run the executable
```bash
./main --demo_config_path=../yolov8_seg/config/yolov8_seg_demo.json
```

Sample output for 4 video streams:
```bash
 total time cost 114256300 us.
frame count is 2848 | fps is 24.93 fps.
```

## 7. Performance Test

Currently, the yolov8_seg example supports inference on BM1684X in both PCIe and SoC modes.

Test video `test_car_person_1080P.avi`, compiled in Release mode, testing yolov8 seg model performance:

| Device | Streams | Algorithm Threads | CPU Usage(%) | TPU Usage(%) | Peak Device Memory(M) | Average FPS |
|--------|---------|-------------------|--------------|--------------|----------------------|-------------|
| SE7    | 4       | 4-4-4             | -            | -            | -                    | 24.93       |

> **Test Notes**:
> 1. Performance test results have some variability; it is recommended to run multiple tests and take the average;
> 2. The above performance tests are based on the yolov8s_fp32_1b segmentation model;
> 3. Segmentation post-processing (CPU get_mask) and mask rendering (OpenCV) consume additional CPU time, resulting in lower fps compared to pure detection tasks;
> 4. In the table above, input streams and algorithm thread settings refer to [JSON Configuration](#61-json-configuration). CPU utilization and system memory can be checked with the `top` command, TPU utilization and device memory with the `bm-smi` command, and fps from the program's printed logs;

---

## yolox

# YOLOX Demo

[English](README_EN.md) | 简体中文

## 目录
- [YOLOX Demo](#yolox-demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)
## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标检测应用。

本例程插件的连接方式如下图所示

![process](./pics/elements.jpg)

yolox由旷视提出，是基于YOLO系列的改进

**论文** (https://arxiv.org/abs/2107.08430)

**源代码** (https://github.com/Megvii-BaseDetection/YOLOX)

本例程中，yolox算法的前处理、推理、后处理分别在三个element上进行运算，element内部可以开启多个线程，保证了一定的检测效率；

## 2. 特性

* 支持BM1684X(x86 PCIe、SoC)，BM1684(x86 PCIe、SoC、arm PCIe)，BM1688(SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本 [download.sh](./scripts/download.sh)。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```
脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

下载的模型包括：

```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684的INT8 BModel，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp16_1b.bmodel    # 用于BM1684X的FP16 BModel，batch_size=1
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=4
└── BM1688_2cores
    ├── yolox_s_int8_1b.bmodel              # 用于BM1688的INT8 BModel，batch_size=1
    └── yolox_s_int8_4b.bmodel              # 用于BM1688的INT8 BModel，batch_size=4
```
模型说明:

1.`yolox_bytetrack_s`系列模型移植于[bytetrack官方](https://github.com/ifzhang/ByteTrack)，插件配置`mean=[0,0,0]`，`std=[1,1,1]`，支持person类别的检测任务。

2.`yolox_s`系列模型移植于[yolox官方](https://github.com/Megvii-BaseDetection/YOLOX)，插件配置`mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`，支持COCO数据集的80分类检测任务。

下载的数据包括：

```bash
./videos/
├── carvana_video.mp4           # 测试视频
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

## 5. 程序编译

### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

yolox demo中各部分参数位于 [config](./config/) 目录，结构如下所示：

```bash
./config
├── decode.json             # 解码配置
├── engine_group.json       # sophon-stream 简化的graph配置
├── engine.json             # sophon-stream graph配置，需要分别配置前处理、推理和后处理文件
├── yolox_classthresh_roi_example.json # yolox按照类别设置阈值的参考配置文件
├── yolox_demo.json         # yolox demo配置
├── yolox_group.json        # 简化的yolox配置文件，将yolox的前处理、推理、后处理合到一个配置文件中
├── yolox_infer.json        # yolox 推理配置
├── yolox_post.json         # yolox 后处理配置
└── yolox_pre.json          # yolox 前处理配置
```

其中，[yolox_demo.json](./config/yolox_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels参数配置输入的路数，sample_interval设置跳帧数，loop_num设置循环播放次数，channel中包含码流url等信息。download_image控制是否保存推理结果，若为false则不保存，若为true，则会保存在/build/results目录下。

配置文件中不指定`channel_id`属性的情况，会在demo中对每一路数据的`channel_id`从0开始默认赋值。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolox/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolox_results",
  "engine_config_path": "../yolox/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[yolox_group.json](./config/yolox_group.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。其中，thread_number是element内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。

```json
{
    "configure": {
      "model_path": "../yolox/data/models/BM1684X/yolox_s_int8_1b.bmodel",
      "threshold_conf": 0.5,
      "threshold_nms": 0.5,
      "bgr2rgb": true,
      "mean": [
        0,
        0,
        0
      ],
      "std": [
        1,
        1,
        1
      ]
    },
    "shared_object": "../../build/lib/libyolox.so",
    "name": "yolox_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 运行

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../yolox/config/yolox_demo.json
```

2路视频流运行结果如下
```bash
 total time cost 5272393 us.
frame count is 1422 | fps is 269.707 fps.
```

## 7. 性能测试

目前，yolox例程支持在BM1684、BM1684X的PCIe、SoC模式下进行推理，支持在BM1688 SoC模式下进行推理。

在不同的设备上可能需要修改json配置，例如模型路径、输入路数等。json的配置方法参考6.1节，程序运行方法参考上文6.2节。

由于PCIE设备cpu能力差距较大，性能数据没有参考意义，这里只给出SOC模式的测试结果。

测试视频`elevator-1080p-25fps-4000kbps.h264`，编译选项为Release模式，结果如下:

|设备|路数|算法线程数|CPU利用率(%)|系统内存(M)|系统内存峰值(M)|TPU利用率(%)|设备内存(M)|设备内存峰值(M)|平均FPS|峰值FPS|
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|4-4-4|204.26|195.29|201.29|99.45|1373.88|1611.00|319.44|329.85|
|SE5-16|4|4-4-4|78.39|122.59|124.93|94.60|1252.69|1424.00|130.26|140.32|
|SE5-8|3|3-3-3|51.42|96.97|98.46|92.77|992.12|1116.00|82.03|91.62|

> **测试说明**：
1. 性能测试结果具有一定的波动性，建议多次测试取平均值；
2. BM1684/1684X SoC的主控CPU均为8核 ARM A53 42320 DMIPS @2.3GHz；
3. 以上性能测试均基于int8模型给出；
4. 在BM1684设备上运行时，batch_size为4的模型可以达到更高的fps；
5. 在BM1684X设备上，使用batch_size为1的模型可以达到更高的fps；
6. 上表中，输入路数和算法线程数的设置请参考[json配置说明](#61-json配置说明)，CPU利用率和系统内存使用top命令可查，TPU利用率和设备内存使用bm-smi命令可查，fps可以从运行程序打印的log中获得;
7. BM1688设备暂无性能测试。

### yolox - README_EN.md

# YOLOX Demo

English | [简体中文](README.md)

## Catalogs
- [YOLOX Demo](#yolox-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This example demonstrates how to use sophon-stream to quickly build a video object detection application.

The connection method for this example plugin is shown in the following diagram.

![process](./pics/elements.jpg)

yolox is proposed by Megvii and is based on the improvement of the YOLO series.

**Paper** (https://arxiv.org/abs/2107.08430)

**Source Code** (https://github.com/Megvii-BaseDetection/YOLOX)

In this example, the pre-processing, inference, and post-processing of the YOLOX algorithm are computed on three separate elements, allowing multiple threads to be utilized within each element, ensuring a certain level of detection efficiency.

## 2. Feature

* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(SoC)
* Supports multiple video streams.
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp16_1b.bmodel    # FP16 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684X，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684X，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684X，batch_size=4
└── BM1688_2cores
    ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1688，batch_size=1
    └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1688，batch_size=4
```

Model description:

1.`yolox_s_bytetrack_` models are from [bytetrack](https://github.com/ifzhang/ByteTrack), `mean=[0,0,0]`，`std=[1,1,1]`, support for person category detection tasks.

2.`yolox_s` models are from [yolox](https://github.com/Megvii-BaseDetection/YOLOX), `mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`, support for 80 classes of COCO dataset.

The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the YOLOX demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```bash
./config
├── decode.json             # decoding configuration
├── engine_group.json       # sophon-stream Simplified graph configuration
├── engine.json             # sophon-stream graph configuration requires separate configuration for pre-processing, inference, and post-processing files.
├── yolox_classthresh_roi_example.json # reference configuration file for setting thresholds per category in YOLOX. Please note that setting thresholds per category is only supported in non-tpu_kernel post-processing mode
├── yolox_demo.json         # input configuration file for the demo
├── yolox_group.json        # A simplified YOLOX configuration file that combines pre-processing, inference, and post-processing into one configuration file.
├── yolox_infer.json        # yolox inference configuration file
├── yolox_post.json         # yolox post-processing configuration file
└── yolox_pre.json          # yolox pre-processing configuration file
```

Indeed, [yolox_demo.json](./config/yolox_demo.json) is the overall configuration file for the example, managing input streams and other information. Multiple data inputs can be supported on a single graph, where the `channels` parameter configures the number of input channels, `sample_interval` sets the frame skipping rate, and `loop_num` sets the number of looped plays. The `channel` section contains video stream information such as the URL. The `download_image` parameter controls whether to save the inference results. If set to `false`, results will not be saved. If set to `true`, they will be saved in the `/build/results` directory.

In the configuration file, when the `channel_id` attribute is not specified, the demo will assign default `channel_id` values starting from 0 for each data channel.

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 3,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 20,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    },
    {
      "channel_id": 30,
      "url": "../yolox/data/videos/elevator-1080p-25fps-4000kbps.h264",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 1,
      "fps": -1
    }
  ],
  "class_names": "../yolox/data/coco.names",
  "download_image": true,
  "draw_func_name": "draw_yolox_results",
  "engine_config_path": "../yolox/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003],
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            }
        ]
    }
]
```

[yolox_group.json](./config/yolox_group.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

```json
{
    "configure": {
      "model_path": "../yolox/data/models/BM1684X/yolox_s_int8_1b.bmodel",
      "threshold_conf": 0.5,
      "threshold_nms": 0.5,
      "bgr2rgb": true,
      "mean": [
        0,
        0,
        0
      ],
      "std": [
        1,
        1,
        1
      ]
    },
    "shared_object": "../../build/lib/libyolox.so",
    "name": "yolox_group",
    "side": "sophgo",
    "thread_number": 4
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolox/config/yolox_demo.json
```

The running results of two video streams are as follows
```bash
 total time cost 5272393 us.
frame count is 1422 | fps is 269.707 fps.
```

## 7. Performance Testing

Currently, the YOLOX example supports inference on BM1684X and BM1684 in PCIe and SoC modes, and supports inference on BM1688 in SoC mode.

Modifications in JSON configurations might be necessary when switching between different devices, such as adjusting model paths, input channels, etc. Refer to section 6.1 for JSON configuration methods and section 6.2 for program execution methods.

Due to significant differences in CPU capabilities among PCIe devices, performance data is not meaningful. Therefore, only provide the test results for SOC mode.

The tested video is `elevator-1080p-25fps-4000kbps.h264`. The compilation was done in Release mode. The results are as follows:

| Device | Number of Channels | Algorithm Thread Count | CPU Utilization (%) | System Memory (M) | Peak System Memory (M) | TPU Utilization (%) | Device Memory (M) | Peak Device Memory (M) | Average FPS | Peak FPS |
|----|----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
|SE7|8|4-4-4|204.26|195.29|201.29|99.45|1373.88|1611.00|319.44|329.85|
|SE5-16|4|4-4-4|78.39|122.59|124.93|94.60|1252.69|1424.00|130.26|140.32|
|SE5-8|3|3-3-3|51.42|96.97|98.46|92.77|992.12|1116.00|82.03|91.62|

> **Test Description**:
1. Performance test results exhibit certain fluctuations; it's advisable to conduct multiple tests and calculate the average.
2. Both BM1684 and BM1684X SoC devices utilize an 8-core ARM A53 processor, offering 42320 DMIPS @ 2.3GHz.
3. All aforementioned performance tests are based on the INT8 model.
4. Running models with a batch size of 4 on the BM1684 device can achieve higher FPS.
5. On the BM1684X device, utilizing a batch size of 1 for the model can yield higher FPS.
6. For the settings of input channels and algorithm thread count in the table, please refer to [JSON configuration explanation](#61-json-configuration). CPU utilization and system memory can be checked using the `top` command. TPU utilization and device memory can be checked using the `bm-smi` command. FPS can be obtained from the logs printed during program execution.
7. Performance testing is not currently available on the BM1688 device.
---

## yolox_bytetrack_osd_encode

# 目标跟踪算法结果推流Demo

[English](README_EN.md) | 简体中文

## 目录
- [目标跟踪算法结果推流Demo](#目标跟踪算法结果推流demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标跟踪应用，并将算法结果推流输出；

本例程插件的连接方式如下图所示:

![elements.jpg](pics/dec_det_track_osd_enc.png)

## 2. 特性
* 检测模型使用yolox；
* 跟踪模型使用bytetrack；
* 支持BM1684X(x86 PCIe、SoC)，BM1684(x86 PCIe、SoC、arm PCIe)，BM1688(arm PCIe、SoC)
* 支持多路视频流
* 支持多线程

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本[download.sh](./scripts/download.sh)。

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

下载的模型包括：
```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684的INT8 BModel，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1688的bytetrack的FP32 BModel，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1688的bytetrack的INT8 BModel，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # 用于BM1688的INT8 BModel，batch_size=1
    └── yolox_s_int8_4b.bmodel              # 用于BM1688的INT8 BModel，batch_size=4
```
模型说明:

1.`yolox_bytetrack_s`系列模型移植于[bytetrack官方](https://github.com/ifzhang/ByteTrack)，插件配置`mean=[0,0,0]`，`std=[1,1,1]`，支持person类别的检测任务。

2.`yolox_s`系列模型移植于[yolox官方](https://github.com/Megvii-BaseDetection/YOLOX)，插件配置`mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`，支持COCO数据集的80分类检测任务。

下载的数据包括：
```bash
./data/videos                             # 测试视频
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
└── sample_1080p_h265.mp4
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。


### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。


## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

配置文件位于 [./config](../yolox_bytetrack_osd_encode/config)

其中，[yolox_bytetrack_osd_encode_demo.json](../yolox_bytetrack_osd_encode/config/yolox_bytetrack_osd_encode_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels中包含每一路码流url等信息。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_01_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 3,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_03_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 20,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_06_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 30,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_08_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    }
  ],
  "engine_config_path": "../yolox_bytetrack_osd_encode/config/engine_group.json"
}
```

[engine.json](../yolox_bytetrack_osd_encode/config/engine.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即删去`channels`里的部分元素，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox_osd_encode",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox_bytetrack_osd_encode/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox_bytetrack_osd_encode/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../yolox_bytetrack_osd_encode/config/bytetrack.json"
            },
            {
                "element_id": 5005,
                "element_config": "../yolox_bytetrack_osd_encode/config/osd.json"
            },
            {
                "element_id": 5006,
                "element_config": "../yolox_bytetrack_osd_encode/config/encode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5005,
                "src_port": 0,
                "dst_element_id": 5006,
                "dst_port": 0
            }
        ]
    }
]
```

[osd.json](../yolox_bytetrack_osd_encode/config/osd.json)等配置文件是对具体某个element的配置细节，设置了模型参数、动态库路径、阈值等信息。该配置文件不需要指定`id`字段和`device_id`字段，例程会将`engine.json`中指定的`element_id`和`device_id`传入。
其中，thread_number是element内部的工作线程数量，一个线程会对应一个数据队列，多路输入情况下，需要合理设置数据队列数目，来保证线程工作压力均匀且合理。
```json
{
  "configure": {
    "osd_type": "TRACK",
    "class_names_file": "../yolox_bytetrack_osd_encode/data/coco.names",
    "draw_utils": "OPENCV",
    "draw_interval": false,
    "put_text": false
  },
  "shared_object": "../../build/lib/libosd.so",
  "name": "osd",
  "side": "sophgo",
  "thread_number": 1
}
```

### 6.2 运行
对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

测试的参数及运行方式是一致的，下面主要以PCIe模式进行介绍。

运行可执行文件
```bash
./main --demo_config_path=../yolox_bytetrack_osd_encode/config/yolox_bytetrack_osd_encode_demo.json
```

运行结果如下
```bash
total time cost 74520023 us.
frame count is 3077 | fps is 41.2909 fps.
```
如果encode选择RTSP模式，需要启动推流服务器。您可以使用vlc软件打开推流地址查看视频算法结果，详细说明查看[encode插件文档](../../element/multimedia/encode/README.md)说明。

## 7. 性能测试
由于Osd插件画图速度慢，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。

### yolox_bytetrack_osd_encode - README_EN.md

# Detection-Track-UpStreaming Demo

English | [简体中文](README.md)

## Catalogs
- [Detection-Track-UpStreaming Demo](#detection-track-upstreaming-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Feature](#2-feature)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-compilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This sample is used to illustrate how to quickly build a video target tracking application using sophon-stream and push stream the algorithm results to output;

The connection method for this example plugin is shown in the following diagram.

![elements.jpg](pics/dec_det_track_osd_enc.png)

## 2. Feature

* Use yolox for detection;
* Use bytetrack for track;
* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(arm PCIe、SoC)
* Supports multiple video streams;
* Supports multi-threading.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp16_1b.bmodel    # FP16 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684X，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684X，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684X，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1688，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1688，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1688，batch_size=1
    └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1688，batch_size=4
```

Model description:

1.`yolox_s_bytetrack_` models are from [bytetrack](https://github.com/ifzhang/ByteTrack), `mean=[0,0,0]`，`std=[1,1,1]`, support for person category detection tasks.

2.`yolox_s` models are from [yolox](https://github.com/Megvii-BaseDetection/YOLOX), `mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`, support for 80 classes of COCO dataset.

The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the Detection-Track-UpStreaming Demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_01_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 3,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_03_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 20,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_06_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 30,
      "url": "../yolox_bytetrack_osd_encode/data/videos/mot17_08_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    }
  ],
  "engine_config_path": "../yolox_bytetrack_osd_encode/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox_osd_encode",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox_bytetrack_osd_encode/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox_bytetrack_osd_encode/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../yolox_bytetrack_osd_encode/config/bytetrack.json"
            },
            {
                "element_id": 5005,
                "element_config": "../yolox_bytetrack_osd_encode/config/osd.json"
            },
            {
                "element_id": 5006,
                "element_config": "../yolox_bytetrack_osd_encode/config/encode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": false
                        }
                    ],
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5005,
                "src_port": 0,
                "dst_element_id": 5006,
                "dst_port": 0
            }
        ]
    }
]
```

[osd.json](./config/osd.json) and similar configuration files detail the configuration specifics for a particular element, setting parameters such as model settings, dynamic library paths, thresholds, and more. These configuration files don't require specifying the `id` or `device_id` fields, as the demo will pass in the `element_id` and `device_id` specified in the engine_group.json.

Among these configurations, `thread_number` specifies the number of working threads within the `element`. Each thread corresponds to a data queue. In scenarios with multiple inputs, it's essential to set the number of data queues reasonably to ensure an even and adequate workload distribution across threads.

```json
{
  "configure": {
    "osd_type": "TRACK",
    "class_names_file": "../yolox_bytetrack_osd_encode/data/coco.names",
    "draw_utils": "OPENCV",
    "draw_interval": false,
    "put_text": false
  },
  "shared_object": "../../build/lib/libosd.so",
  "name": "osd",
  "side": "sophgo",
  "thread_number": 1
}
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

The parameters for testing and the method of execution remain consistent. Therefore, it'll primarily explain in terms of PCIe mode.

Run the executable file
```bash
./main --demo_config_path=../yolox_bytetrack_osd_encode/config/yolox_bytetrack_osd_encode_demo.json
```

The running results are as follows
```bash
total time cost 74520023 us.
frame count is 3077 | fps is 41.2909 fps.
```

If encode selects RTSP mode, you need to start the push streaming server. You can use VLC software to open the push streaming address to view the video algorithm results, see [encode plugin documentation](../../element/multimedia/encode/README.md) for details.

## 7. Performance Testing

Due to the slow drawing speed of Osd plugin, this sample does not provide performance test results for the time being. If you need the inference performance of each model, please go to the corresponding model sample to check.
---

## yolox_bytetrack_osd_qt

# 目标跟踪算法结果显示Demo

[English](README_EN.md) | 简体中文

## 目录
- [目标跟踪算法结果显示Demo](#目标跟踪算法结果显示demo)
  - [目录](#目录)
  - [1. 简介](#1-简介)
  - [2. 特性](#2-特性)
  - [3. 准备模型与数据](#3-准备模型与数据)
  - [4. 环境准备](#4-环境准备)
    - [4.1 x86/arm PCIe平台](#41-x86arm-pcie平台)
    - [4.2 SoC平台](#42-soc平台)
  - [5. 程序编译](#5-程序编译)
    - [5.1 x86/arm PCIe平台](#51-x86arm-pcie平台)
    - [5.2 SoC平台](#52-soc平台)
  - [6. 程序运行](#6-程序运行)
    - [6.1 Json配置说明](#61-json配置说明)
    - [6.2 运行](#62-运行)
  - [7. 性能测试](#7-性能测试)

## 1. 简介

本例程用于说明如何使用sophon-stream快速构建视频目标跟踪应用，并将算法结果显示输出；

## 2. 特性
* 检测模型使用yolox；
* 跟踪模型使用bytetrack；
* 支持BM1684X(x86 PCIe、SoC)，BM1684(x86 PCIe、SoC、arm PCIe)，BM1688(SoC)
* 支持多路视频流
* 支持多线程
* 支持qt显示

## 3. 准备模型与数据

​在`scripts`目录下提供了相关模型和数据的下载脚本[download.sh](./scripts/download.sh)。

脚本执行完毕后，会在当前目录下生成`data`目录，其中包含`models`和`videos`两个子目录。

```bash
# 安装unzip，若已安装请跳过，非ubuntu系统视情况使用yum或其他方式安装
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

下载的模型包括：
```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684的INT8 BModel，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # 用于BM1684X的INT8 BModel，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # 用于BM1684X的FP32 BModel，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # 用于BM1684X的INT8 BModel，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # 用于BM1688的bytetrack的FP32 BModel，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # 用于BM1688的bytetrack的INT8 BModel，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # 用于BM1688的INT8 BModel，batch_size=1
    └── yolox_s_int8_4b.bmodel              # 用于BM1688的INT8 BModel，batch_size=4
```
模型说明:

1.`yolox_bytetrack_s`系列模型移植于[bytetrack官方](https://github.com/ifzhang/ByteTrack)，插件配置`mean=[0,0,0]`，`std=[1,1,1]`，支持person类别的检测任务。

2.`yolox_s`系列模型移植于[yolox官方](https://github.com/Megvii-BaseDetection/YOLOX)，插件配置`mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`，支持COCO数据集的80分类检测任务。

下载的数据包括：
```bash
./data/videos                             # 测试视频
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
└── sample_1080p_h265.mp4
```

## 4. 环境准备

### 4.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](../../docs/EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

除此之外还需要安装公版QT：

```bash
sudo apt install qtbase5-dev
```

### 4.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

#### 4.2.1 BM1684/BM1684X
如果您使用的是BM1684/BM1684X设备，您需要在您进行交叉编译的设备上，通过以下命令下载并解压sophon-qt以进行后续的交叉编译：

```bash
python3 -m dfss --url=open@sophgo.com:sophon-demo/MultiYolov5/qt-5.14-amd64-aarch64-fl2000fb_v1.1.0.tar.xz
tar -xaf qt-5.14-amd64-aarch64-fl2000fb_v1.1.0.tar.xz
```
#### 4.2.2 BM1688
如果您使用的是BM1688设备，您需要在您进行交叉编译的设备上，通过以下命令下载并解压arm的公版qt以进行后续的交叉编译：
```bash
python3 -m dfss --url=open@sophgo.com:sophon-pipeline/a2_bringup/qtbase.zip
unzip qtbase.zip
```

## 5. 程序编译
程序运行前需要编译可执行文件。
### 5.1 x86/arm PCIe平台
可以直接在PCIe平台上编译程序，具体请参考[sophon-stream编译](../../docs/HowToMake.md)

### 5.2 SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](../../docs/HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

## 6. 程序运行

### 6.1 Json配置说明

配置文件位于 [./config](../yolox_bytetrack_osd_qt/config)

其中，[yolox_bytetrack_osd_qt_demo.json](../yolox_bytetrack_osd_qt/config/yolox_bytetrack_osd_qt_demo.json)是例程的整体配置文件，管理输入码流等信息。在一张图上可以支持多路数据的输入，channels中包含每一路码流url等信息。

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_01_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 3,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_03_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 20,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_06_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 30,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_08_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    }
  ],
  "engine_config_path": "../yolox_bytetrack_osd_qt/config/engine_group.json"
}
```

[engine.json](../yolox_bytetrack_osd_qt/config/engine.json) 包含对graph的配置信息，这部分配置确定之后基本不会发生更改。

需要注意，部署环境下的NPU等设备内存大小会显著影响例程运行的路数。如果默认的输入路数运行中出现了申请内存失败等错误，可以考虑把输入路数减少，即删去`channels`里的部分元素，再进行测试。

这里摘取配置文件的一部分作为示例：在该文件内，需要初始化每个element的信息和element之间的连接方式。element_id是唯一的，起到标识身份的作用。element_config指向该element的详细配置文件地址，port_id是该element的输入输出端口编号，多输入或多输出的情况下，输入/输出编号也不可以重复。is_src标志当前端口是否是整张图的输入端口，is_sink标识当前端口是否是整张图的输出端口。
connection是所有element之间的连接方式，通过element_id和port_id确定。

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox_osd_qt_display",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox_bytetrack_osd_qt/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox_bytetrack_osd_qt/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../yolox_bytetrack_osd_qt/config/bytetrack.json"
            },
            {
                "element_id": 5005,
                "element_config": "../yolox_bytetrack_osd_qt/config/osd.json"
            },
            {
                "element_id": 5006,
                "element_config": "../yolox_bytetrack_osd_qt/config/qt_display.json",
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5005,
                "src_port": 0,
                "dst_element_id": 5006,
                "dst_port": 0
            }
        ]
    }
]
```

### 6.2 运行
对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库、可执行文件、所需的模型和测试数据拷贝到SoC平台中测试。

SoC平台上，动态库、可执行文件、配置文件、模型、视频数据的目录结构关系应与原始sophon-stream仓库中的关系保持一致。

PCIE模式下运行可执行文件
```bash
sudo ./main --demo_config_path=../yolox_bytetrack_osd_qt/config/yolox_bytetrack_osd_qt_demo.json
```

SoC模式下，如果桌面程序正在运行，需要先停止服务 
```bash
sudo systemctl stop SophonHDMI.service
```

然后在scripts目录下运行run_hdmi_show.sh脚本
```bash
cd scripts
sudo ./run_hdmi_show.sh
```

运行结果如下
```bash
total time cost 60697616 us.
frame count is 3773 | fps is 62.1606 fps.
```

## 7. 性能测试
由于Osd插件画图速度慢，本例程暂不提供性能测试结果，如需各模型推理性能，请到对应模型例程查看。

### yolox_bytetrack_osd_qt - README_EN.md

# Detection-Track-QT-Display Demo

English | [简体中文](README.md)

## Catalogs
- [Detection-Track-UpStreaming Demo](#detection-track-upstreaming-demo)
  - [Catalogs](#catalogs)
  - [1. Introduction](#1-introduction)
  - [2. Features](#2-features)
  - [3. Prepare Models and Data](#3-prepare-models-and-data)
  - [4. Prepare Environment](#4-prepare-environment)
    - [4.1 x86/arm PCIe Platform](#41-x86arm-pcie-platform)
    - [4.2 SoC Platform](#42-soc-platform)
  - [5. Program Compilation](#5-program-pompilation)
    - [5.1 x86/arm PCIe Platform](#51-x86arm-pcie-platform)
    - [5.2 SoC Platform](#52-soc-platform)
  - [6. Program Execution](#6-program-execution)
    - [6.1 JSON Configuration](#61-json-configuration)
    - [6.2 Execute](#62-execute)
  - [7. Performance Testing](#7-performance-testing)

## 1. Introduction

This sample is used to illustrate how to quickly build a video target tracking application using sophon-stream and push stream the algorithm results to output;

## 2. Feature

* Use yolox for detection;
* Use bytetrack for track;
* Supports BM1684X, BM1684(x86 PCIe、SoC), supports BM1688(SoC)
* Supports multiple video streams;
* Supports multi-threading.
* Supports qt display.

## 3. Prepare Models and Data

The `scripts` directory contains download scripts for relevant models and data. [download.sh](./scripts/download.sh).

```bash
# Install unzip. Skip this step if already installed. If not Ubuntu systems, use yum or other methods as needed.
sudo apt install unzip
chmod -R +x scripts/
./scripts/download.sh
```

After the script execution, `data` directory will be generated in the current directory, containing two subdirectories: `models` and `videos`

The downloaded models include:

```bash
./models/
├── BM1684
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_fp32_4b.bmodel    # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp16_1b.bmodel    # FP16 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684X，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684X，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684X，batch_size=4
├── BM1684X
│   ├── yolox_bytetrack_s_fp16_1b.bmodel    # FP16 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1684X，batch_size=1
│   ├── yolox_bytetrack_s_int8_4b.bmodel    # INT8 BModel for BM1684X，batch_size=4
│   ├── yolox_s_fp32_1b.bmodel              # FP32 BModel for BM1684X，batch_size=1
│   ├── yolox_s_fp32_4b.bmodel              # FP32 BModel for BM1684X，batch_size=4
│   ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1684X，batch_size=1
│   └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1684X，batch_size=4
└── BM1688
    ├── yolox_bytetrack_s_fp32_1b.bmodel    # FP32 BModel for BM1688，batch_size=1
    ├── yolox_bytetrack_s_int8_1b.bmodel    # INT8 BModel for BM1688，batch_size=1
    ├── yolox_s_int8_1b.bmodel              # INT8 BModel for BM1688，batch_size=1
    └── yolox_s_int8_4b.bmodel              # INT8 BModel for BM1688，batch_size=4
```

Model description:

1.`yolox_s_bytetrack_` models are from [bytetrack](https://github.com/ifzhang/ByteTrack), `mean=[0,0,0]`，`std=[1,1,1]`, support for person category detection tasks.

2.`yolox_s` models are from [yolox](https://github.com/Megvii-BaseDetection/YOLOX), `mean=[0,0,0]`，`std=[0.0039216,0.0039216,0.0039216]`, support for 80 classes of COCO dataset.

The downloaded data include:

```bash
videos/
├── carvana_video.mp4   # test video
├── elevator-1080p-25fps-4000kbps.h264
├── mot17_01_frcnn.mp4
├── mot17_03_frcnn.mp4
├── mot17_06_frcnn.mp4
├── mot17_07_frcnn.mp4
├── mot17_08_frcnn.mp4
├── mot17_12_frcnn.mp4
├── mot17_14_frcnn.mp4
├── sample_1080p_h265.mp4
└── test_car_person_1080P.avi
```

## 4. Prepare Environment

### 4.1 x86/arm PCIe Platform

If you have installed a PCIe accelerator card (such as the SC series card) on an x86/arm platform, you can directly use it as the development or runtime environment. You need to install libsophon, sophon-opencv, and sophon-ffmpeg. For specific steps, please refer to [the setup guide for x86-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#3-x86-pcie-platform-development-and-runtime-environment-construction) or [setup guide for arm-pcie platform](../../docs/EnvironmentInstallGuide_EN.md#5-arm-pcie-platform-development-and-runtime-environment-construction).

Besides, QT with official version is needed:

```bash
sudo apt install qtbase5-dev
```


### 4.2 SoC Platform

If you are using the SoC platform (such as SE or SM series edge devices), after flashing(Upgrade the operating system by SD card.), the corresponding libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages are pre-installed under `/opt/sophon/`, which can be directly used as the runtime environment. Typically, you would also need an x86 machine as the development environment for cross-compiling C++ programs.

#### 4.2.1 BM1684/BM1684X
If you are using BM1684/BM1684X device, you will need to download and unzip sophon-qt on the device where you cross-compile for subsequent process:

```bash
python3 -m dfss --url=open@sophgo.com:sophon-demo/MultiYolov5/qt-5.14-amd64-aarch64-fl2000fb_v1.1.0.tar.xz
tar -xaf qt-5.14-amd64-aarch64-fl2000fb_v1.1.0.tar.xz
```
#### 4.2.2 BM1688
If you are using a BM1688 device, you will need to download and extract the public qt version of arm for subsequent cross-compilation on the device you are cross-compiling on:

```bash
python3 -m dfss --url=open@sophgo.com:sophon-pipeline/a2_bringup/qtbase.zip
unzip qtbase.zip
```

## 5. Program Compilation

### 5.1 x86/arm PCIe Platform
You can directly compile programs on the PCIe platform. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md).

### 5.2 SoC Platform
Typically, programs are cross-compiled on an x86 computer. You need to set up a cross-compilation environment using SOPHON SDK on the x86 computer. Package the necessary include files and library files for the program into the `sophon_sdk_soc` directory. For specifics, please refer to [sophon-stream compilation](../../docs/HowToMake_EN.md). This example mainly dependes on the libsophon, sophon-opencv, and sophon-ffmpeg runtime library packages.

## 6. Program Execution

### 6.1 JSON Configuration

In the Detection-Track-QT-Display Demo, various parameters for each section are located in [config](./config/) directory, structured as follows:

```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_01_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 3,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_03_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 20,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_06_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    },
    {
      "channel_id": 30,
      "url": "../yolox_bytetrack_osd_qt/data/videos/mot17_08_frcnn.mp4",
      "source_type": "VIDEO",
      "loop_num": 1
    }
  ],
  "engine_config_path": "../yolox_bytetrack_osd_qt/config/engine_group.json"
}
```

[engine_group.json](./config/engine_group.json) contains configuration information for graphs, which, once set, typically remain unchanged.

Here's an excerpt from the configuration file as an example: Within this file, it's necessary to initialize information for each element and specify the connections between elements. The `element_id` serves as a unique identifier. `element_config` points to the detailed configuration file for that element. `port_id` denotes the input/output port number for the element. And in cases of multiple inputs or outputs, these numbers should not be duplicated. `is_src` denotes whether the current port is the input port for the entire graph, while `is_sink` identifies whether the port is the output for the whole graph. `connection` determines how elements are connected, using `element_id` and `port_id` for identification.

```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "yolox_osd_qt_display",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../yolox_bytetrack_osd_qt/config/decode.json",
                "ports": {
                    "input": [
                        {
                            "port_id": 0,
                            "is_sink": false,
                            "is_src": true
                        }
                    ]
                }
            },
            {
                "element_id": 5001,
                "element_config": "../yolox_bytetrack_osd_qt/config/yolox_group.json",
                "inner_elements_id": [10001, 10002, 10003]
            },
            {
                "element_id": 5004,
                "element_config": "../yolox_bytetrack_osd_qt/config/bytetrack.json"
            },
            {
                "element_id": 5005,
                "element_config": "../yolox_bytetrack_osd_qt/config/osd.json"
            },
            {
                "element_id": 5006,
                "element_config": "../yolox_bytetrack_osd_qt/config/qt_display.json",
                "ports": {
                    "output": [
                        {
                            "port_id": 0,
                            "is_sink": true,
                            "is_src": false
                        }
                    ]
                }
            }
        ],
        "connections": [
            {
                "src_element_id": 5000,
                "src_port": 0,
                "dst_element_id": 5001,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 0,
                "dst_element_id": 5004,
                "dst_port": 0
            },
            {
                "src_element_id": 5004,
                "src_port": 0,
                "dst_element_id": 5005,
                "dst_port": 0
            },
            {
                "src_element_id": 5005,
                "src_port": 0,
                "dst_element_id": 5006,
                "dst_port": 0
            }
        ]
    }
]
```

### 6.2 Execute

For PCIe platforms, you can directly run tests on the PCIe platform. For SoC platforms, you'll need to copy the dynamically linked libraries, executable files, required models, and test data generated from cross-compilation to the SoC platform for testing.

On the SoC platform, maintain a directory structure for dynamic libraries, executable files, configuration files, models, and video data consistent with the original sophon-stream repository's structure.

In PCIE mode:

Run the executable file
```bash
./main --demo_config_path=../yolox_bytetrack_osd_qt/config/yolox_bytetrack_osd_qt_demo.json
```

In SoC mode:

If SophonHDMI is running, stop it first.
```bash
sudo systemctl stop SophonHDMI.service
```

Then, run the run_hdmi_show.sh script under the scripts directory.

```bash
cd scripts
sudo ./run_hdmi_show.sh
```

The output should be like:
```bash
total time cost 60697616 us.
frame count is 3773 | fps is 62.1606 fps.
```


## 7. Performance Testing

Due to the slow drawing speed of Osd plugin, this sample does not provide performance test results for the time being. If you need the inference performance of each model, please go to the corresponding model sample to check.
