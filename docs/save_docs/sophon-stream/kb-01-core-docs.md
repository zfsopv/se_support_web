# Sophon-Stream 项目核心文档

> 本文档由以下原始文档合并而成，方便导入知识库。
> 生成时间: 2026-05-22 15:04:00

---

## 项目 README

# sophon-stream

[English](README_EN.md) | 简体中文

## 1 简介

sophon-stream是面向算丰开发平台的数据流处理工具。本软件基于插件化的思想，使用C++17开发了一套支持多路数据流并发处理的流水线框架。基于现有的接口，sophon-stream对用户具有易使用、易二次开发的优点，可以大大简化用户配置工程或添加插件的复杂度。sophon-stream基于SophonSDK，可以充分发挥算丰硬件的编解码能力及深度学习算法的推理能力，从而获得较高的性能。

目前，本仓库已开源到github：https://github.com/sophgo/sophon-stream

教学视频已发布到Bilibili：https://www.bilibili.com/video/BV1ZpvDeXEQw

欢迎star、issue、pr！

主要目录结构和模块说明：

| 目录      | 模块                                             | 功能说明  |
| ------------------------|-------------------------------------------------------------------|---------------------| 
| [framework](./framework)| [framework](./framework)                                          | 框架                        |
| [element](./element)    | [yolov5](./element/algorithm/yolov5)                              | yolov5插件             |
|                         | [yolov7](./element/algorithm/yolov7)                              | yolov7插件             |
|                         | [yolov8](./element/algorithm/yolov8)                              | yolov8插件，支持检测、姿态、分类 |
|                         | [yolox](./element/algorithm/yolox)                                | yolox插件               |
|                         | [bytetrack](./element/algorithm/bytetrack)                        | bytetrack插件       |
|                         | [resnet](./element/algorithm/resnet)                              | resnet插件，支持分类、抽取特征  |
|                         | [openpose](./element/algorithm/openpose)                          | openpose插件       |
|                         | [retinaface](./element/algorithm/retinaface)                      | retinaface插件     |
|                         | [lprnet](./element/algorithm/lprnet)                              | lprnet插件            |
|                         | [decode](./element/multimedia/decode)                             | 解码插件               |
|                         | [encode](./element/multimedia/encode)                             | 编码插件               |
|                         | [osd](./element/multimedia/osd)                                   | 算法结果可视化插件       |
|                         | [distributor](./element/tools/distributor)                        | 数据分发插件       |
|                         | [converger](./element/tools/converger)                            | 数据汇聚插件       |
|                         | [faiss](./element/tools/faiss)                                    | faiss数据库插件         |
|                         | [blank](./element/tools/blank)                                    | 空白插件                |
| [samples](./samples)    | [yolov5](./samples/yolov5)                                        | yolov5 demo                             |
|                         | [yolov7](./samples/yolov7)                                        | yolov7 demo                            |
|                         | [yolov8](./samples/yolov8/)                                       | yolov8 demo                             |
|                         | [yolov8_obb](./samples/yolov8_obb/)                               | yolov8 obb demo                         |
|                         | [yolox](./samples/yolox)                                          | yolox demo                              |
|                         | [bytetrack](./samples/bytetrack)                                  | bytetrack demo                          |
|                         | [resnet](./samples/resnet)                                        | resnet demo                             |
|                         | [openpose](./samples/openpose)                                    | openpose demo                           |
|                         | [retinaface](./samples/retinaface)                                | retinaface demo                         |
|                         | [yolox_bytetrack_osd_encode](./samples/yolox_bytetrack_osd_encode)| 目标检测-跟踪-算法结果推流demo |
|                         | [yolov5_bytetrack_distributor_resnet_converger](./samples/yolov5_bytetrack_distributor_resnet_converger)| 目标检测-跟踪-分发-属性识别demo |
|                         | [retinaface_distributor_resnet_faiss_converger](./samples/retinaface_distributor_resnet_faiss_converger)| 人脸检测-分发-人脸识别demo |
|                         | [license_plate_recognition](./samples/license_plate_recognition/) | 车牌检测-车牌识别demo |
|                         | [ppocr](./samples/ppocr/)                                         | PPOCR demo |
|                         | [yolov5_fastpose_posec3d](./samples/yolov5_fastpose_posec3d/)     | 姿态识别-行为识别demo |
|                         | [bird_dwa_blend_encode](./samples/bird_dwa_blend_encode/)         | 鸟瞰拼接demo |
|                         | [dwa_blend_encode](./samples/dwa_blend_encode/)                   | 鱼眼拼接demo |
|                         | [dwa_dpu_encode](./samples/dwa_dpu_encode/)                       | 双目深度估计demo |
|                         | [dwa_lightstereo_encode](./samples/dwa_lightstereo_encode/)       | 轻型立体拼接demo |
|                         | [gdwa_blend_encode](./samples/gdwa_blend_encode/)                 | 广角拼接demo |
|                         | [license_area_intrusion](./samples/license_area_intrusion/)       | 区域入侵demo |
|                         | [line_crossing](./samples/line_crossing/)                         | 越线检测demo |
|                         | [multi_graph](./samples/multi_graph/)                             | 多graph功能demo |
|                         | [structured_recognition](./samples/structured_recognition/)       | 单路码流配置不同算法demo |
|                         | [trinocular_panorama_stitch](./samples/trinocular_panorama_stitch/)| 三目全景拼接demo |
|                         | [tripwire](./samples/tripwire/)                                   | 越线检测demo |
|                         | [yolox_bytetrack_osd_qt](./samples/yolox_bytetrack_osd_qt/)       | 目标检测-跟踪-绘图-HDMI显示demo |

## 2 快速入门
请参考[sophon-stream用户文档](./docs/Sophon_Stream_User_Guide.md)

## 3 FAQ
请参考[sophon-stream常见问题及解答](./docs/FAQ.md)

---

## 用户手册

# 算能 sophon-stream 用户手册

[English](Sophon_Stream_User_Guide_EN.md) | 简体中文

## 目录
- [算能 sophon-stream 用户手册](#算能-sophon-stream-用户手册)
  - [目录](#目录)
  - [1. 快速入门](#1-快速入门)
    - [1.1 安装和配置环境](#11-安装和配置环境)
      - [1.1.1 x86/arm PCIe平台](#111-x86arm-pcie平台)
      - [1.1.2 SoC平台](#112-soc平台)
    - [1.2 编译命令](#12-编译命令)
      - [1.2.1 x86/arm PCIe平台](#121-x86arm-pcie平台)
      - [1.2.2 SoC平台](#122-soc平台)
    - [1.3 编译结果](#13-编译结果)
  - [2. 概述](#2-概述)
    - [2.1 sophon-stream优势](#21-sophon-stream优势)
    - [2.2 sophon-stream软件栈](#22-sophon-stream软件栈)
  - [3. 框架](#3-框架)
    - [3.1 Element](#31-element)
    - [3.2 Graph](#32-graph)
    - [3.3 Engine](#33-engine)
    - [3.4 Connector](#34-connector)
    - [3.5 ObjectMetadata](#35-objectmetadata)
    - [3.6 Frame](#36-frame)
    - [3.7 Group](#37-group)
  - [4. 插件](#4-插件)
    - [4.1 algorithm](#41-algorithm)
      - [4.1.1 概述](#411-概述)
      - [4.1.2 yolox](#412-yolox)
      - [4.1.3 yolov5](#413-yolov5)
      - [4.1.4 bytetrack](#414-bytetrack)
      - [4.1.5 openpose](#415-openpose)
      - [4.1.6 lprnet](#416-lprnet)
      - [4.1.7 retinaface](#417-retinaface)
    - [4.2 multimedia](#42-multimedia)
      - [4.2.1 decode](#421-decode)
      - [4.2.2 encode](#422-encode)
      - [4.2.3 osd](#423-osd)
    - [4.3 tools](#43-tools)
      - [4.3.1 distributor](#431-distributor)
      - [4.3.2 converger](#432-converger)
      - [4.3.3 blank](#433-blank)
      - [4.3.4 faiss](#434-faiss)
  - [5. 应用程序](#5-应用程序)
    - [5.1 例程概述](#51-例程概述)
    - [5.2 配置文件](#52-配置文件)
    - [5.3 入口程序](#53-入口程序)
    - [5.4 用户侧信息](#54-用户侧信息)

## 1. 快速入门

### 1.1 安装和配置环境

#### 1.1.1 x86/arm PCIe平台

如果您在x86/arm平台安装了PCIe加速卡（如SC系列加速卡），可以直接使用它作为开发环境和运行环境。您需要安装libsophon、sophon-opencv和sophon-ffmpeg，具体步骤可参考[x86-pcie平台的开发和运行环境搭建](EnvironmentInstallGuide.md#3-x86-pcie平台的开发和运行环境搭建)或[arm-pcie平台的开发和运行环境搭建](EnvironmentInstallGuide.md#5-arm-pcie平台的开发和运行环境搭建)。

#### 1.1.2 SoC平台

如果您使用SoC平台（如SE、SM系列边缘设备），刷机后在`/opt/sophon/`下已经预装了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，可直接使用它作为运行环境。通常还需要一台x86主机作为开发环境，用于交叉编译C++程序。

### 1.2 编译命令

需要注意，如果您的主机上没有安装boost库，则需要使用如下命令进行安装。

```bash
sudo apt-get update 
sudo apt-get install libboost-all-dev
```

完成环境配置后，用户可以参考 [sophon-stream编译指南](./HowToMake.md)，使用如下命令编译。

* 需要注意，编译命令在sophon-stream目录下进行。

#### 1.2.1 x86/arm PCIe平台

```bash
mkdir build
cd build 
cmake ..
make -j
```

#### 1.2.2 SoC平台

通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中，具体请参考[sophon-stream编译](./HowToMake.md)。本例程主要依赖libsophon、sophon-opencv和sophon-ffmpeg运行库包。

```bash
mkdir build
cd build 
cmake .. -DTARGET_ARCH=soc -DSOPHON_SDK_SOC=${path_to_sophon_soc_sdk}
make -j
```

注意：交叉编译时，${path_to_sophon_soc_sdk} 变量指运行交叉编译命令的x86主机上打包好的sophon_sdk_soc目录。

### 1.3 编译结果

sophon-stream中，除了sample以外的每个模块都以插件的形式参与运行。完成 [1.2 编译命令](#12-编译命令) 后，用户可以在 ./build/lib/ 目录下看到每个参与编译的插件对应的动态库文件。

samples中的源文件，其编译结果是`samples/build`目录下的可执行程序。

## 2. 概述

sophon-stream是面向算丰开发平台的数据流处理工具。本软件基于插件化的思想，使用C++17开发了一套支持多路数据流并发处理的流水线框架。基于现有的接口，sophon-stream对用户具有易使用、易二次开发的优点，可以大大简化用户配置工程或添加插件的复杂度。sophon-stream基于SophonSDK，可以充分发挥算丰硬件的编解码能力及深度学习算法的推理能力，从而获得较高的性能。

### 2.1 sophon-stream优势

sophon-stream具有以下优点：
  
 - 稳健灵活的基础框架。在保证sophon-stream基础框架的稳健性的同时，它也具有相当大的灵活性。用户可以简单地配置json文件，从而准确方便地搭建复杂的业务流水线。
 - 完备的软硬件生态体系。sophon-stream基于算丰处理器的底层特点，包含了编解码硬件加速、常规的图像处理加速以及推理加速功能，可以充分发挥算丰处理器的性能优势，极大地提升整体的吞吐效率。
 - 丰富的算法库。sophon-stream支持多种目标检测及跟踪算法，例如yolox、yolov5、bytetrack等。
 - 便于部署。sophon-stream适用于算丰BM1684、BM1684X、BM1688处理器，可以在PCIE、SOC模式下灵活部署。

### 2.2 sophon-stream软件栈

sophon-stream基于SophonSDK设计。SophonSDK是算能科技基于自主研发的深度学习处理器所定制的深度学习SDK，涵盖了神经网络推理阶段所需的模型优化、高效运行时支持等能力，为深度学习应用开发和部署提供易用、高效的全栈式解决方案。

![stream_and_sdk](./pics/stream_sdk.png)

sophon-stream由framework和element两部分组成，framework是整体的框架，作为底层决定了sophon-stream的运行方式，如图的构建、数据传输等。element是所有图节点的统称，它们由同一个抽象基类派生而来，负责基于SophonSDK提供某项特定功能，如视频编解码、图像处理等。

## 3. 框架

sophon-stream框架包含三层结构，分别是Engine，Graph和Element。三者之间的层次关系如下图所示。

![engine](./pics/engine.png)

Engine是sophon-stream中最外层的结构，向外部工程提供接口。Engine管理着多个Graph，而每个Graph是一张独立的有向无环图，管理着多个Element。

### 3.1 Element

element类是sophon-stream的通用基类，用户二次开发的插件也都基于element。作为一个抽象类，element类统一规定了所有派生类的主要接口和成员，包括数据如何传递、线程如何管理、两个element之间通过何种方式连接等。

一个element的结构如下图所示: 

![element](./pics/element.png)

Element与外部的数据传递通过connector来进行，每个输入或输出port都有一个connector与之对应。从输入connector获取数据之后，element在run()方法中调用doWork()来对数据进行处理，然后分发给outputPort对应的connector，即下一个element的输入connector。

element基类的主要成员变量:

```cpp
int mId;     // element id，用于在engine及graph中确定element的身份，在graph中具有唯一性
int mDeviceId;  // device id，涉及tpu操作时使用的设备id。pcie模式下可以按需设置，soc模式下应设置为0
int mThreadNumber; // element内部工作的线程数，也等于InputConnector中的DataPipe数目

// 管理输入和输出Connector的映射，key是输入或输出的port_id，value是指向Connector的指针
std::map<int, std::shared_ptr<framework::Connector>> mInputConnectorMap;
std::map<int, std::weak_ptr<framework::Connector>> mOutputConnectorMap;

/* 管理输出SinkHandler的映射，key是输出的port_id，value是一个签名为void(std::shared_ptr<void>)的函数。SinkHandler为graph末尾的元素提供数据处理功能，一般包括绘图等。 */
std::map<int, SinkHandler> mSinkHandlerMap;
```

主要的成员函数: 
```cpp
/* static方法，连接两个element。设置srcElement的输出port和dstElement的输入port，并把dstElement的inputConnector注册到srcElement的mOutputConnectorMap */
static void connect(Element& srcElement, int srcElementPort, Element& dstElement, int dstElementPort);

// 从配置文件初始化element的通用属性，例如element id、thread number等
common::ErrorCode init(const std::string& json)
// 启停element
common::ErrorCode start();
common::ErrorCode stop();
// push数据，用于启动DecoderElement的解码任务
common::ErrorCode pushInputData(int inputPort, int dataPipeId, std::shared_ptr<void> data);

// 线程函数，负责循环调用doWork()并分配处理器时间片资源
void run(int dataPipeId)

// 纯虚函数，派生类中用于初始化自定义的属性，例如算法相关内容
virtual common::ErrorCode initInternal(const std::string& json) = 0;

// 纯虚函数，派生类中自定义具体的算法逻辑，一般为[pop数据——组batch——运行算法——push数据]等
virtual common::ErrorCode doWork(int dataPipeId) = 0;
// 循环调用doWork()，线程资源调度
void run(int dataPipeId);

// 将已处理完的数据push到输出Connector。特别地，如果当前element是sink element，则执行SinkHandler。
common::ErrorCode pushOutputData(int outputPort, int dataPipeId, std::shared_ptr<void> data);
```

### 3.2 Graph

graph类的实例由engine管理，它提供接口给engine调用，主要在初始化或析构一张图时起作用。各graph可配置在不同设备运行，不推荐graph中各element配置在不同设备运行。graph类对外的接口主要包括：

```cpp
// 初始化及反初始化当前graph
common::ErrorCode init(const std::string& json);
void uninit();
// 启停当前graph
common::ErrorCode start();
common::ErrorCode stop();
// 向source element推入数据，用于启动DecoderElement的解码任务
common::ErrorCode pushSourceData(int elementId, int inputPort,
                                std::shared_ptr<void> data);
// 为sink element的sinkPort设置数据处理函数，例如绘图、发送等
void setSinkHandler(int elementId, int outputPort, SinkHandler sinkHandler);
```
### 3.3 Engine

engine类是一个单例，一个进程中只存在一个engine。engine类对外的接口主要包括：

```cpp
// 启停某个graph
common::ErrorCode start(int graphId);
common::ErrorCode stop(int graphId);
// 添加一个graph
common::ErrorCode addGraph(const std::string& json);
// 向某个graph中的source element推入数据。用于启动解码功能。
common::ErrorCode pushSourceData(int graphId, int elementId, int inputPort,
                                std::shared_ptr<void> data);
// 为某个graph的sink element的sinkPort设置数据处理函数，例如绘图、发送等。
void setSinkHandler(int graphId, int elementId, int outputPort,
                    SinkHandler sinkHandler);
```

### 3.4 Connector

Connector是在两个element之间传递数据的桥梁。一个connector的实例可以管理多个datapipe。

Connector类的主要成员如下: 

```cpp
class Connector : public ::sophon_stream::common::NoCopyable {
 public:
  
  // 获取编号为id的队列头部的数据，并将其弹出
  std::shared_ptr<void> popDataWithId(int id);
  
  // 将data push到编号为id的队列
  common::ErrorCode pushDataWithId(int id, std::shared_ptr<void> data);

  // 获取connector中队列的数目
  int getCapacity() const;
 private:
  
  // 多个DataPipe 
  std::vector<std::shared_ptr<DataPipe>> mDataPipes;
};
```

Connector类的成员方法都由id获取某个datapipe，然后调用该datapipe的对应方法来实现。

### 3.5 ObjectMetadata

ObjectMetadata是sophon-stream的通用数据结构，所有element中的功能都基于此结构设计。

ObjectMetadata的主要成员包括: 

```cpp
std::shared_ptr<common::Packet> mPacket; // 储存解码前信息
std::shared_ptr<common::Frame> mFrame;   // 储存解码后信息: bm_image、frame_id、EndOfStream标识等
std::shared_ptr<bmTensors> mInputBMtensors; // 当前frame经过预处理得到的inputTensor
std::shared_ptr<bmTensors> mOutputBMtensors; // 当前frame经过推理得到的outputTensor

// 嵌套的objectMetadata，储存当前图上的子结构
std::vector<std::shared_ptr<ObjectMetadata> > mSubObjectMetadatas; 
// detect相关信息，例如box坐标
std::shared_ptr<common::DetectedObjectMetadata> mDetectedObjectMetadata; 
// track相关信息，例如track_id
std::shared_ptr<common::TrackedObjectMetadata> mTrackedObjectMetadata;
```

### 3.6 Frame

Frame是ObjectMetadata中储存了图像信息的结构，其主要成员包括：

```cpp
int mChannelId;                         // 指定了推流服务中对应码流的url，不在配置文件中指定的情况下，默认从0开始赋值
int mChannelIdInternal;                 // 内部channel_id，从0开始赋值，用于计算connector中的数据流向
std::int64_t mFrameId;                  // 解码得到的帧id，在一路数据中递增
bool mEndOfStream;                      // 数据流结束的标识
std::shared_ptr<bm_image> mSpData;      // 存放原始bm_image
std::shared_ptr<bm_image> mSpDataOsd;   // 存放osd插件绘图之后的bm_image
```

### 3.7 Group

Group是一个为了统一管理算法的前处理、推理、后处理三个阶段而设计的特殊的模板类。

Group继承自Element基类，其模板参数为某一个具体的算法类。当Group被实例化时，将自动构造模板参数对应的前处理、推理、后处理Element，在初始化流程中配置它们的属性，确定连接关系，并将内部的三个Element与外部相连。

Group Element本身的`doWork()`方法本身不执行任何算法逻辑，其工作由内部的三个Element完成。Group Element的作用是将算法插件的配置文件由三个减少到一个，大大提高了使用的便利性。

## 4. 插件

sophon-stream中，所有算法或多媒体功能都以插件的形式存放于 sophon_stream/element/ 目录中。

sophon-stream/element/algorithm 目录是算法插件的集合，目前包括yolox、yolov5、bytetrack、resnet算法。

sophon-stream/element/multimedia 目录是多媒体插件的集合，目前包括编解码和OSD(On-Screen Display)功能。

sophon-stream/element/tools 目录是功能性插件的集合，包括数据分发、数据汇聚element，以及一个供用户进行调试的空白element等。

### 4.1 algorithm

#### 4.1.1 概述

算法插件是基于SophonSDK中BMCV和BMRuntime库实现的具有图像处理和推理功能的模块，包括前处理、推理、后处理三个部分。用户根据业务需求，只需要载入对应的模型，即可调用硬件启动相应的功能。

算法插件具有以下特性：

 - element每个线程都与输入connector的一个datapipe绑定。组batch发生在doWork()函数的开始，从当前线程对应的datapipe中获取数据
 - 发送数据时，保证下游element各个线程负载均衡
 - 如果两个模块之间只有模型内部参数的差异，前处理、推理、后处理的流程完全相同时，可以复用前处理和后处理element
 - 支持将前处理、推理和后处理分别配置在不同的element上。如此配置的目的是充分利用各项资源，提高算法效率

#### 4.1.2 yolox

yolox是旷视提出的目标检测算法，具有较高的性能。

yolox的配置文件形如：

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
配置参数的详细介绍请参见 [yolox插件介绍](../element/algorithm/yolox/README.md)

yolox demo请参考 [yolox demo](../samples/yolox/README.md)

#### 4.1.3 yolov5

yolov5是世界上最受欢迎的视觉模型，使用十分广泛。

yolov5的配置文件形如：
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

配置参数的详细介绍请参见 [yolov5插件介绍](../element/algorithm/yolov5/README.md)

yolov5 demo请参考 [yolov5 demo](../samples/yolov5/README.md)

#### 4.1.4 bytetrack

bytetrack是华中科技大学、香港大学和字节跳动联合提出的一个简单、快速、强大的多目标跟踪器。

其配置文件形如：
```json
{
    "configure": {
        "track_thresh": 0.5,
        "high_thresh": 0.6,
        "match_thresh": 0.7,
        "min_box_area": 10,
        "frame_rate": 30,
        "track_buffer": 30
    },
    "shared_object": "../../build/lib/libbytetrack.so",
    "name": "bytetrack",
    "side": "sophgo",
    "thread_number": 8
}
```

配置参数的详细介绍请参见 [bytetrack插件介绍](../element/algorithm/bytetrack/README.md)

bytetrack demo请参考 [bytetrack demo](../samples/bytetrack/README.md)

#### 4.1.5 openpose

openpose是一个强大的姿态估计网络。

其配置文件形如：
```json
{
    "configure": {
      "model_path": "../openpose/data/models/BM1684X/pose_coco_int8_1b.bmodel",
      "threshold_nms": 0.05,
      "use_tpu_kernel": true
    },
    "shared_object": "../../build/lib/libopenpose.so",
    "name": "openpose_group",
    "side": "sophgo",
    "thread_number": 2
}
```

配置参数的详细介绍请参见 [openpose插件介绍](../element/algorithm/openpose/README.md)

openpose demo请参考 [openpose demo](../samples/openpose/README.md)

#### 4.1.6 lprnet

lprnet是一个用于车牌识别的网络。

其配置文件形如：
```json
{
    "configure": {
      "model_path": "../license_plate_recognition/models/lprnet/BM1684X/lprnet_fp32_1b.bmodel"
    },
    "shared_object": "../../build/lib/liblprnet.so",
    "name": "lprnet_group",
    "side": "sophgo",
    "thread_number": 1
}
```

配置参数的详细介绍请参见 [lprnet插件介绍](../element/algorithm/lprnet/README.md)

lprnet没有提供单独的demo，而是提供了一个yolov5车辆检测与lprnet车牌识别级联的demo。请参考 [license_plate_recognition demo](../samples/license_plate_recognition/README.md)

#### 4.1.7 retinaface

retinaface是一个用于人脸检测的网络。

其配置文件形如：
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
    "name": "retinaface_group",
    "side": "sophgo",
    "thread_number": 4
}
```

配置参数的详细介绍请参见 [retinaface插件介绍](../element/algorithm/retinaface/README.md)

retinaface没有提供单独的demo，而是提供了一个人脸检测与基于arcface与faiss的人脸识别的demo。请参考 [retinaface_distributor_resnet_faiss_converger demo](../samples/retinaface_distributor_resnet_faiss_converger/README.md)

### 4.2 multimedia

#### 4.2.1 decode

decode是sophon-stream的起始模块，起到从各种类型的输入获得ObjectMetadata，并发送往下游element的作用。

目前，DecoderElement支持的数据源类型包括: 
 - 本地视频文件
 - 本地图片文件夹
 - RTSP流
 - RTMP流

作为sophon-stream的起始模块，decode在触发任务时具有一定的特殊性。在graph构建完毕后，需要向decode element发送一个启动任务的信号，才会使decode开始工作，后续的element才有数据流入。

```cpp
nlohmann::json channel_config = yolox_json.channel_config;

channel_config["channel_id"] = channel_id;

auto channelTask = 
    std::make_shared<sophon_stream::element::decode::ChannelTask>();

channelTask->request.operation = 
    sophon_stream::element::decode::ChannelOperateRequest::ChannelOperate::START;

channelTask->request.json = channel_config.dump();

sophon_stream::common::ErrorCode errorCode = 
    engine.pushInputData(graph_id, 
                        src_id_port.first, 
                        src_id_port.second, 
                        std::static_pointer_cast<void>(channelTask));
```

可以看到，在构造channelTask时，还需要设置当前输入码流的channel_id。channel_id作为码流的标识，起到了确定ObjectMetadata在connector中流向的作用。

decode的配置文件包括以下内容: 

```json
{
  "configure": {},
  "shared_object": "../../build/lib/libdecode.so",
  "id": 0,
  "device_id": 0,
  "name": "decode",
  "side": "sophgo",
  "thread_number": 1
}
```

配置参数的详细介绍请参见 [decode介绍](../element/multimedia/decode/README.md)

#### 4.2.2 encode

 encode一般作为sophon-stream的尾部模块使用，用于将处理后的图像信息编码为各类视频格式。

 目前，encode支持的目标类型包括: 
  - RTSP、RTMP、本地视频文件
  - H.264/H.265编码格式
  - I420、NV12等像素格式

encode的配置文件包括以下内容:

```json
{
  "configure": {
    "encode_type": "RTSP",
    "rtsp_port": "8554",
    "rtmp_port": "1935",
    "enc_fmt": "h264_bm",
    "pix_fmt": "I420"
  },
  "shared_object": "../../build/lib/libencode.so",
  "id": 0,
  "device_id": 0,
  "name": "encode",
  "side": "sophgo",
  "thread_number": 4
}
```

配置参数的详细介绍请参见 [encode介绍](../element/multimedia/encode/README.md)

#### 4.2.3 osd

osd插件是可视化插件，目前支持目标检测、目标跟踪算法结果的可视化。

osd插件的配置文件包括:

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

其中，"osd_type" 字段标识了算法类型，可以设置为 "DET" 或 "TRACK" 

配置参数的详细介绍请参见 [osd介绍](../element/multimedia/osd/README.md)

### 4.3 tools

#### 4.3.1 distributor

distributor插件是数据分发插件，可以实现将检测到的不同类别目标分发到不同的下游分支的功能。目前，distributor支持的分发规则包括按时间间隔分发和按帧间隔分发两种，在配置具体的分发规则时，可以将上述两种方式搭配使用。

distributor的配置文件包括：

```json
{
    "configure": {
        "default_port": 0,
        "rules": [
            {
                "time_interval": 1,
                "routes": [
                    {
                        "classes":["car"],
                        "port": 1
                    },
                    {
                        "classes":["person"],
                        "port" : 2
                    }
                ]
            }
        ],
        "class_names_file": "../xxx/data/coco.names"
    },
    "shared_object": "../../build/lib/libdistributor.so",
    "name": "distributor",
    "side": "sophgo",
    "thread_number": 1
}
```

distributor插件必须搭配converger插件使用，详细说明请参考[distributor介绍](../element/tools/distributor/README.md)

#### 4.3.2 converger

converger是数据汇聚插件，可以将经由distributor延伸的各分支element传来的数据进行汇总，然后输出到下游element。

converger的配置文件包括：

```json
{
    "configure": {
        "default_port": 0
    },
    "shared_object": "../../build/lib/libconverger.so",
    "name": "converger",
    "side": "sophgo",
    "thread_number": 1
}
```

converger插件必须搭配distributor插件使用，详细说明请参考[converger介绍](../element/tools/converger/README.md)

#### 4.3.3 blank

blank是一个空白插件，可以连接在任意两个element之间，不会对pipeline有任何影响。

该插件提供了一个完整的插件模板，但没有实现任何配置参数或工作逻辑。用户可以自行编写相关代码，以起到调试等作用。

#### 4.3.4 faiss

faiss是一个数据库召回插件，在 BM1684X 上实现了Faiss::IndexFlatIP.search()。考虑 BM1684X 上 TPU 的连续内存, 针对 100W 底库, 可以在单处理器上一次查询最多约 512 个 256 维的输入。

## 5. 应用程序

基于sophon-stream创建应用程序，其实是基于sophon-stream的framework和element搭建业务流水线。

在实现了基础的算法功能之后，只需要编写配置文件和相应的入口程序，就可以完成流水线的搭建。

接下来，本文以一个典型的应用程序介绍配置文件和入口程序的编写要点。

### 5.1 例程概述

一个典型的pipeline一般包括如下操作:
  - 数据源解码
  - 目标检测
  - 目标跟踪
  - 绘制跟踪结果
  - 编码输出

本节以 [参考例程](../samples/yolox_bytetrack_osd_encode/config/yolox_bytetrack_osd_encode_demo.json) 为例进行讲解。该例程构建的graph如下图所示:

![dec_det_track_osd_enc](./pics/dec_det_track_osd_enc.png)

### 5.2 配置文件

首先，需要配置上图中各个element的信息。在实际配置业务时，为了保证检测算法的效率，可以将检测算法的前处理、推理和后处理三个阶段分别配置在三个element上执行。本例程中采取这种配置方式，因此，该流水线包括decode、pre_process、inference、post_process、track、osd、encode共七个element。

该例程的配置文件包括：

```bash
./config/
├── bytetrack.json                          # 跟踪
├── decode.json                             # 解码
├── encode.json                             # 编码
├── engine.json                             # graph的总体配置
├── engine_group.json                       # 简化的graph总体配置
├── osd.json                                # osd模块
├── yolox_bytetrack_osd_encode_demo.json    # demo的总体配置
├── yolox_group.json                        # 统一管理的yolox配置
├── yolox_infer.json                        # yolox推理
├── yolox_post.json                         # yolox后处理
└── yolox_pre.json                          # yolox预处理
```

其中，各个element的配置文件在上文对应的章节已经进行了阐述，这里对demo和graph配置文件中的内容进行说明。

yolox_bytetrack_osd_encode_demo.json 是该demo的总体配置，其形如：

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

demo的配置文件包括两个属性。一是`channels` ，在一个list中记录所有输入的`url`、`channel_id`和`source_type`信息。需要注意的是：`source_type`需要参考 [decode配置](../element/multimedia/decode/README.md) 准确设置。

该配置文件中，`channel_id`标记了encode插件启动推流服务器时，输出视频流的URL。推流URL配置请参考 [#4.2.2 encode](#422-encode)。如果不需要该功能，可以不设置`channel_id`，在demo中默认从0开始赋值。

engine.json 是当前demo程序中构造的graph信息，储存了每个graph内包含的element及element之间如何连接等信息。其形如：

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
                "element_config": "../yolox_bytetrack_osd_encode/config/osd.json",
            },
            {
                "element_id": 5006,
                "element_config": "../config/encode.json",
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

其中，需要重点关注的是 "elements" 和 "connections" 部分。"elements" 是graph内所有element的列表，对于每个element，需要配置element_id、对应的配置文件路径和端口信息。同一个graph内不同的element应具有不同的element_id。element的端口包括输入和输出端口，同一种类的不同端口之间同样应该由不同的port_id区分开。每个端口都具有 "is_src" 和 "is_sink" 属性，标志着当前是否是整张graph的输入或输出端口。

一般只有decode element才会具有输入端口。对于此element，需要在应用程序中为其发送channelTask，以启动pipeline的工作。不同的是，输出端口不要求element的类型，任何element都可以具有输出端口，具体应该参考工程需求进行配置。对于具有输出端口的element，应为其设置SinkHandler，即正确处理输出数据的回调函数。

### 5.3 入口程序

对于不同的demo，其差异主要在配置文件方面，入口程序基本是一致的。

入口程序一般包括以下几个部分：

 - 解析demo的配置文件
 - 解析engine的配置文件
 - 调用engine.addGraph()，初始化所有element及其connection
 - 设置sink element的SinkHandler
 - 发送channelTask，触发decode element的工作任务
 - 等候所有码流处理完毕，结束任务
 - 统计fps等信息

### 5.4 用户侧信息

运行一个例程时，命令行中会依序打印如下信息：

 - graph的配置信息
 ```bash
 [info] [/sophon-stream/framework/src/engine.cc:95] Add graph start, json: {"connections":[{"dst_id":5001,"dst_port":0,"src_id":5000,"src_port":0},{"dst_id":5002,"dst_port":0,"src_id":5001,"src_port":0},{"dst_id":5003,"dst_port":0,"src_id":5002,"src_port":0}],"elements":[{"configure":{},"device_id":0,"id":5000,"name":"decode","shared_object":"../../../build/lib/libdecode.so","side":"sophgo","thread_number":1},{"configure":{"model_path":"../data/models/BM1684X/yolox_s_int8_4b.bmodel","stage":["pre"],"threshold_conf":0.5,"threshold_nms":0.5},"device_id":0,"id":5001,"name":"yolox","shared_object":"../../../build/lib/libyolox.so","side":"sophgo","thread_number":2},{"configure":{"model_path":"../data/models/BM1684X/yolox_s_int8_4b.bmodel","stage":["infer"],"threshold_conf":0.5,"threshold_nms":0.5},"device_id":0,"id":5002,"name":"yolox","shared_object":"../../../build/lib/libyolox.so","side":"sophgo","thread_number":2},{"configure":{"model_path":"../data/models/BM1684X/yolox_s_int8_4b.bmodel","stage":["post"],"threshold_conf":0.5,"threshold_nms":0.5},"device_id":0,"id":5003,"is_sink":true,"name":"yolox","shared_object":"../../../build/lib/libyolox.so","side":"sophgo","thread_number":2}],"graph_id":0}
 ```

 - element的配置信息

 ```bash
 [info] [/sophon-stream/framework/src/element.cc:45] Init start, json: {"configure":{"model_path":"../data/models/BM1684X/yolox_s_int8_4b.bmodel","stage":["pre"],"threshold_conf":0.5,"threshold_nms":0.5},"device_id":0,"id":5001,"name":"yolox","shared_object":"../../../build/lib/libyolox.so","side":"sophgo","thread_number":2}
[BMRT][bmcpu_setup:349] INFO:cpu_lib 'libcpuop.so' is loaded.
bmcpu init: skip cpu_user_defined
open usercpu.so, init user_cpu_init
[BMRT][load_bmodel:1079] INFO:Loading bmodel from [../data/models/BM1684X/yolox_s_int8_4b.bmodel]. Thanks for your patience...
[BMRT][load_bmodel:1023] INFO:pre net num: 0, load net num: 1
*** Run in PCIE mode ***

########################
NetName: yolox_s_bmnetp
---- stage 0 ----
  Input 0) 'x.1' shape=[ 4 3 640 640 ] dtype=INT8 scale=0.498161
  Output 0) '15' shape=[ 4 8400 85 ] dtype=FLOAT32 scale=1
########################
 ```

 - connection信息

 ```bash
 [debug] [/sophon-stream/framework/src/element.cc:26] InputConnector initialized, mId = 5001, inputPort = 0, dataPipeNum = 2
 ```

 - 启动graph、element

 ```bash
[info] [/sophon-stream/framework/src/graph.cc:107] Start graph thread start, graph id: 0
[info] [/sophon-stream/framework/src/element.cc:125] Start element thread start, element id: 5000
[info] [/sophon-stream/framework/src/element.cc:140] Start element thread finish, element id: 5000
[info] [/sophon-stream/framework/src/element.cc:125] Start element thread start, element id: 5001
[info] [/sophon-stream/framework/src/element.cc:140] Start element thread finish, element id: 5001
[info] [/sophon-stream/framework/src/element.cc:125] Start element thread start, element id: 5002
[info] [/sophon-stream/framework/src/element.cc:140] Start element thread finish, element id: 5002
[info] [/sophon-stream/framework/src/element.cc:125] Start element thread start, element id: 5003
[info] [/sophon-stream/framework/src/element.cc:140] Start element thread finish, element id: 5003
[info] [/sophon-stream/framework/src/graph.cc:127] Start graph thread finish, graph id: 0
 ```

 - 设置sink handler

 ```bash
 [info] [/sophon-stream/framework/src/engine.cc:143] Set sink handler, graph id: 0, element id: 5003, output port: 0
 ```

 - 设置channel_task，启动decode

 ```bash
 [info] [/sophon-stream/element/multimedia/decode/src/decode.cc:127] add one channel task
 [info] [/sophon-stream/element/multimedia/decode/src/decode.cc:163] channel info decoder address: 0x7f7814000b70
 ```

 - element工作状态

```bash
[engine] [debug] [/sophon-stream/framework/src/element.cc:232] send data, element id: 5000, output port: 0, data:0x7f781cab01d0
[engine] [debug] [/sophon-stream/framework/src/element.cc:232] send data, element id: 5000, output port: 0, data:0x7f781cad9110
[engine] [debug] [/sophon-stream/framework/src/element.cc:232] send data, element id: 5000, output port: 0, data:0x7f781c0681b0
[engine] [debug] [/sophon-stream/framework/src/element.cc:232] send data, element id: 5002, output port: 0, data:0x7f781c0127d0
[engine] [debug] [/sophon-stream/framework/src/element.cc:232] send data, element id: 5002, output port: 0, data:0x7f781c067480
[engine] [debug] [/sophon-stream/framework/src/element.cc:232] send data, element id: 5002, output port: 0, data:0x7f781c0daf90
```

 - 解码至文件尾

 ```bash
 [h264_bm @ 0x7f7814007f80] av_read_frame ret(-541478725) maybe eof...
 ```
 - engine、graph、element线程终止
 ```bash
 [engine] [info] [/sophon-stream/framework/src/engine.cc:35] Engine stop graph thread start, graph id: 0
 [engine] [info] [/sophon-stream/framework/src/engine.cc:50] Engine stop graph thread finish, graph id: 0
 [engine] [info] [/sophon-stream/framework/src/graph.cc:132] Stop graph thread start, graph id: 0
 [engine] [info] [/sophon-stream/framework/src/element.cc:145] Stop element thread start, element id: 5000
 [engine] [info] [/sophon-stream/element/multimedia/decode/src/decode.cc:52] Decode stop...
 [engine] [info] [/sophon-stream/framework/src/element.cc:159] Stop element thread finish, element id: 5000
 [engine] [info] [/sophon-stream/framework/src/element.cc:145] Stop element thread start, element id: 5001
 [engine] [info] [/sophon-stream/framework/src/element.cc:159] Stop element thread finish, element id: 5001
 [engine] [info] [/sophon-stream/framework/src/element.cc:145] Stop element thread start, element id: 5002
 [engine] [info] [/sophon-stream/framework/src/element.cc:159] Stop element thread finish, element id: 5002
 [engine] [info] [/sophon-stream/framework/src/element.cc:145] Stop element thread start, element id: 5003
 [engine] [info] [/sophon-stream/framework/src/element.cc:159] Stop element thread finish, element id: 5003
 [engine] [info] [/sophon-stream/framework/src/graph.cc:150] Stop graph thread finish, graph id: 0
 ```

 - 统计耗时、帧数、fps

```bash
total time cost 5286871 us.
frame count is 1422 | fps is 268.968 fps.
```
---

## 环境安装指南

# sophon-stream环境安装指南

[English](EnvironmentInstallGuide_EN.md) | 简体中文

## 目录
- [sophon-stream环境安装指南](#sophon-stream环境安装指南)
  - [目录](#目录)
  - [1 TPU-MLIR环境搭建](#1-tpu-mlir环境搭建)
  - [2 x86 PCIe平台的开发和运行环境搭建](#2-x86-pcie平台的开发和运行环境搭建)
    - [2.1 安装libsophon](#21-安装libsophon)
    - [2.2 安装sophon-ffmpeg和sophon-opencv](#22-安装sophon-ffmpeg和sophon-opencv)
  - [3 SoC平台的开发和运行环境搭建](#3-soc平台的开发和运行环境搭建)
    - [3.1 交叉编译环境搭建](#31-交叉编译环境搭建)
  - [4 arm PCIe平台的开发和运行环境搭建](#4-arm-pcie平台的开发和运行环境搭建)
    - [4.1 安装libsophon](#41-安装libsophon)
    - [4.2 安装sophon-ffmpeg和sophon-opencv](#42-安装sophon-ffmpeg和sophon-opencv)

Sophon Stream所依赖的环境主要包括用于编译和量化模型的TPU-NNTC、TPU-MLIR环境，用于编译C++程序的开发环境以及用于部署程序的运行环境。

## 1 TPU-MLIR环境搭建
如果您使用BM1684X处理器，建议使用TPU-MLIR编译BModel。通常需要在x86主机上安装TPU-MLIR环境，x86主机已安装Ubuntu16.04/18.04/20.04系统，并且运行内存在12GB以上。TPU-MLIR环境安装步骤主要包括：

1. 安装Docker

   若已安装docker，请跳过本节。
    ```bash
    # 安装docker
    sudo apt-get install docker.io
    # docker命令免root权限执行
    # 创建docker用户组，若已有docker组会报错，没关系可忽略
    sudo groupadd docker
    # 将当前用户加入docker组
    sudo usermod -aG docker $USER
    # 切换当前会话到新group或重新登录重启X会话
    newgrp docker​
    ```
    > **提示**：需要logout系统然后重新登录，再使用docker就不需要sudo了。

2. 下载并解压TPU-MLIR

    从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载TPU-MLIR的压缩包，命名如tpu-mlir_vx.y.z-hash-date.tar.gz，x.y.z表示版本号，并进行解压。
    ```bash
    tar zxvf tpu-mlir_vx.y.z-<hash>-<date>.tar.gz
    ```

3. 创建并进入docker

    TPU-MLIR使用的docker是sophgo/tpuc_dev:latest, docker镜像和tpu-mlir有绑定关系，少数情况下有可能更新了tpu-mlir，需要新的镜像。
    ```bash
    # 如果当前系统没有对应镜像，会自动从docker hub上下载
    # 这里将本级目录映射到docker内的/workspace目录,用户需要根据实际情况将stream的目录映射到docker里面
    # myname只是举个名字的例子, 请指定成自己想要的容器的名字
    docker run --name myname -v $PWD:/workspace -it sophgo/tpuc_dev:latest
    # 此时已经进入docker，并在/workspace目录下
    # 初始化软件环境
    cd /workspace/tpu-mlir_vx.y.z-<hash>-<date>
    source ./envsetup.sh
    ```
此镜像仅用于编译和量化模型，程序编译和运行请在开发和运行环境中进行。更多TPU-MLIR的教程请参考[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)的《TPU-MLIR快速入门手册》和《TPU-MLIR开发参考手册》。

## 2 x86 PCIe平台的开发和运行环境搭建
如果您在x86平台安装了PCIe加速卡，开发环境与运行环境可以是统一的，您可以直接在宿主机上搭建开发和运行环境。

### 2.1 安装libsophon
从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载libsophon安装包，包括:
* sophon-driver_x.y.z_amd64.deb
* sophon-libsophon_x.y.z_amd64.deb
* sophon-libsophon-dev_x.y.z_amd64.deb

其中：x.y.z表示版本号；sophon-driver包含了PCIe加速卡驱动；sophon-libsophon包含了运行时环境（库文件、工具等）；sophon-libsophon-dev包含了开发环境（头文件等）。如果只是在部署环境上安装，则不需要安装 sophon-libsophon-dev。
```bash
# 安装依赖库，只需要执行一次
sudo apt install dkms libncurses5
# 安装libsophon
sudo dpkg -i sophon-*amd64.deb
# 在终端执行如下命令，或者登出再登入当前用户后即可使用bm-smi等命令：
source /etc/profile
```

更多libsophon信息请参考《LIBSOPHON使用手册.pdf》。

### 2.2 安装sophon-ffmpeg和sophon-opencv
从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载sophon-mw安装包，包括:
* sophon-mw-sophon-ffmpeg_x.y.z_amd64.deb
* sophon-mw-sophon-ffmpeg-dev_x.y.z_amd64.deb
* sophon-mw-sophon-opencv_x.y.z_amd64.deb
* sophon-mw-sophon-opencv-dev_x.y.z_amd64.deb

其中：x.y.z表示版本号；sophon-ffmpeg/sophon-opencv包含了ffmpeg/opencv运行时环境（库文件、工具等）；sophon-ffmpeg-dev/sophon-opencv-dev包含了开发环境（头文件、pkgconfig、cmake等）。如果只是在部署环境上安装，则不需要安装 sophon-ffmpeg-dev/sophon-opencv-dev。

sophon-mw-sophon-ffmpeg依赖sophon-libsophon包，而sophon-mw-sophon-opencv依赖sophon-mw-sophon-ffmpeg，因此在安装次序上必须
先安装libsophon, 然后sophon-mw-sophon-ffmpeg, 最后安装sophon-mw-sophon-opencv。

如果运行环境中使用的libstdc++库使用GCC5.1之前的旧版本ABI接口（典型的有CENTOS系统），请使用sophon-mw-sophon-opencv-abi0相关安装包。

```bash
# 安装sophon-ffmpeg
sudo dpkg -i sophon-mw-sophon-ffmpeg_*amd64.deb sophon-mw-sophon-ffmpeg-dev_*amd64.deb
# 安装sophon-opencv
sudo dpkg -i sophon-mw-sophon-opencv_*amd64.deb sophon-mw-sophon-opencv-dev_*amd64.deb
# 在终端执行如下命令，或者logout再login当前用户后即可使用安装的工具
source /etc/profile
```

更多sophon-mw信息请参考《MULTIMEDIA使用手册.pdf》、《MULTIMEDIA开发参考手册.pdf》。


## 3 SoC平台的开发和运行环境搭建
对于SoC平台，安装好SophonSDK(>=v22.09.02)后内部已经集成了相应的libsophon、sophon-opencv和sophon-ffmpeg运行库包，位于`/opt/sophon/`下，可直接用于运行环境。通常在x86主机上交叉编译程序，使之能够在SoC平台运行。SophonSDK固件刷新方法可参考[FAQ文档](./FAQ.md#12-soc模式下如何使用sd卡刷更新固件).

### 3.1 交叉编译环境搭建
需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至soc-sdk目录中。

需要注意的是，对于SE9设备，下列命令中的sophon-mw目录应改为sophon-media。


1. 安装交叉编译工具链
    ```bash
    sudo apt-get install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
    ```
    如果报错：`/lib/aarch64-linux-gnu/libc.so.6: version 'GLIBC_2.33' not found`。
    这是由于您主机上的交叉编译工具链版本太高导致，可以通过如下命令重新安装：
    ```bash
    sudo apt remove cpp-*-aarch64-linux-gnu
    sudo apt-get install gcc-7-aarch64-linux-gnu g++-7-aarch64-linux-gnu
    sudo ln -s /usr/bin/aarch64-linux-gnu-gcc-7 /usr/bin/aarch64-linux-gnu-gcc
    sudo ln -s /usr/bin/aarch64-linux-gnu-g++-7 /usr/bin/aarch64-linux-gnu-g++
    ```

2. 打包libsophon

    从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载sophon-img安装包，其中包括libsophon_soc_x.y.z_aarch64.tar.gz，x.y.z表示版本号，并进行解压。

    ```bash
    # 创建依赖文件的根目录
    mkdir -p soc-sdk
    # 解压libsophon_soc_x.y.z_aarch64.tar.gz
    tar -zxf libsophon_soc_${x.y.z}_aarch64.tar.gz
    # 将相关的库目录和头文件目录拷贝到依赖文件根目录下
    cp -rf libsophon_soc_${x.y.z}_aarch64/opt/sophon/libsophon-${x.y.z}/lib ${soc-sdk}
    cp -rf libsophon_soc_${x.y.z}_aarch64/opt/sophon/libsophon-${x.y.z}/include ${soc-sdk}
    ```

3. 打包sophon-ffmpeg和sophon-opencv

    从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载sophon-mw安装包，其中包括sophon-mw-soc_x.y.z_aarch64.tar.gz，x.y.z表示版本号，并进行解压。
    ```bash
    # 解压sophon-mw-soc_x.y.z_aarch64.tar.gz
    tar -zxf sophon-mw-soc_${x.y.z}_aarch64.tar.gz
    # 将ffmpeg和opencv的库目录和头文件目录拷贝到soc-sdk目录下
    cp -rf sophon-mw-soc_${x.y.z}_aarch64/opt/sophon/sophon-ffmpeg_${x.y.z}/lib ${soc-sdk}
    cp -rf sophon-mw-soc_${x.y.z}_aarch64/opt/sophon/sophon-ffmpeg_${x.y.z}/include ${soc-sdk}
    cp -rf sophon-mw-soc_${x.y.z}_aarch64/opt/sophon/sophon-opencv_${x.y.z}/lib ${soc-sdk}
    cp -rf sophon-mw-soc_${x.y.z}_aarch64/opt/sophon/sophon-opencv_${x.y.z}/include ${soc-sdk}
    ```

这里，交叉编译环境已经搭建完成，接下来可以使用打包好的soc-sdk编译需要在SoC平台上运行的程序。更多交叉编译信息请参考《LIBSOPHON使用手册.pdf》。

## 4 arm PCIe平台的开发和运行环境搭建
如果您在arm平台安装了PCIe加速卡，开发环境与运行环境可以是统一的，您可以直接在宿主机上搭建开发和运行环境。
这里提供银河麒麟v10机器的环境安装方法，其他类型机器具体请参考官网开发手册。
### 4.1 安装libsophon
从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载libsophon安装包，
安装包由一个文件构成，其中“$arch”为当前机器的硬件架构，使用以下命令可以获取当前服务器的arch：
```
uname -m
```
通常x86_64机器对应的硬件架构为x86_64，arm64机器对应的硬件架构为aarch64：
```
libsophon_x.y.z_$arch.tar.gz，x.y.z表示版本号
```
可以通过如下步骤安装：

**注意：如果有旧版本，先参考下面的卸载方式步骤卸载旧版本。**
```
tar -xzvf libsophon_${x.y.z}_aarch64.tar.gz
sudo cp -r libsophon_${x.y.z}_aarch64/* /
sudo ln -s /opt/sophon/libsophon-${x.y.z} /opt/sophon/libsophon-current
```
接下来请先按照您所使用Linux发行版的要求搭建驱动编译环境，然后做如下操作：
```
sudo ln -s /opt/sophon/driver-${x.y.z}/$bin /lib/firmware/bm1684x_firmware.bin
sudo ln -s /opt/sophon/driver-${x.y.z}/$bin /lib/firmware/bm1684_ddr_firmware.bin
sudo ln -s /opt/sophon/driver-${x.y.z}/$bin /lib/firmware/bm1684_tcm_firmware.bin
cd /opt/sophon/driver-${x.y.z}
```
此处“$bin”是带有版本号的bin文件全名, 对于bm1684x板卡，为a53lite_pkg.bin，对于bm1684板卡，如bm1684_ddr.bin_v3.1.1-63a8614d-220906和bm1684_tcm.bin_v3.1.1-63a8614d-220906。

之后就可以编译驱动了（这里不依赖于dkms）：
```
sudo make SOC_MODE=0 PLATFORM=asic SYNC_API_INT_MODE=1 \
          TARGET_PROJECT=sg_pcie_device FW_SIMPLE=0 \
          PCIE_MODE_ENABLE_CPU=1
sudo cp ./bmsophon.ko /lib/modules/$(uname -r)/kernel/
sudo depmod
sudo modprobe bmsophon
```
最后是一些配置工作：

添加库和可执行文件路径：
```
sudo cp /opt/sophon/libsophon-current/data/libsophon.conf /etc/ld.so.conf.d/
sudo ldconfig
sudo cp /opt/sophon/libsophon-current/data/libsophon-bin-path.sh /etc/profile.d/
```
在终端执行如下命令，或者登出再登入当前用户后即可使用bm-smi等命令：
```
source /etc/profile
```
添加cmake config文件：
```
sudo mkdir -p /usr/lib/cmake/libsophon
sudo cp /opt/sophon/libsophon-current/data/libsophon-config.cmake /usr/lib/cmake/libsophon/
```
卸载方式：
```
sudo rm -f /etc/ld.so.conf.d/libsophon.conf
sudo ldconfig
sudo rm -f /etc/profile.d/libsophon-bin-path.sh
sudo rm -rf /usr/lib/cmake/libsophon
sudo rmmod bmsophon
sudo rm -f /lib/modules/$(uname -r)/kernel/bmsophon.ko
sudo depmod
sudo rm -f /lib/firmware/bm1684x_firmware.bin
sudo rm -f /lib/firmware/bm1684_ddr_firmware.bin
sudo rm -f /lib/firmware/bm1684_tcm_firmware.bin
sudo rm -f /opt/sophon/libsophon-current
sudo rm -rf /opt/sophon/libsophon-0.4.6
sudo rm -rf /opt/sophon/driver-0.4.6
```
其他平台机器请参考[libsophon安装教程](https://doc.sophgo.com/sdk-docs/v24.04.01/docs_latest_release/docs/libsophon/guide/html/index.html)。
更多libsophon信息请参考《LIBSOPHON使用手册.pdf》

### 4.2 安装sophon-ffmpeg和sophon-opencv
从[算能官网](https://developer.sophgo.com/site/index/material/88/all.html)上下载sophon-mw安装包，
安装包由一个文件构成：
```
sophon-mw_x.y.z_aarch64.tar.gz，x.y.z表示版本号
```
可以通过如下步骤安装：

先按照《LIBSOPHON使用手册》安装好libsophon包，然后，
```
tar -xzvf sophon-mw_${x.y.z}_aarch64.tar.gz
sudo cp -r sophon-mw_${x.y.z}_aarch64/* /
sudo ln -s /opt/sophon/sophon-ffmpeg_${x.y.z} /opt/sophon/sophon-ffmpeg-latest
sudo ln -s /opt/sophon/sophon-opencv_${x.y.z} /opt/sophon/sophon-opencv-latest
sudo ln -s /opt/sophon/sophon-sample_${x.y.z} /opt/sophon/sophon-sample-latest
sudo sed -i "s/usr\/local/opt\/sophon\/sophon-ffmpeg-latest/g" /opt/sophon/sophon-ffmpeg-latest/lib/pkgconfig/*.pc
sudo sed -i "s/^prefix=.*$/prefix=\/opt\/sophon\/sophon-opencv-latest/g" /opt/sophon/sophon-opencv-latest/lib/pkgconfig/opencv4.pc
```
最后，**安装bz2 libc6 libgcc依赖库**（这部分需要根据操作系统不同，选择对应的安装包，这里不统一介绍）
然后是一些配置工作：

添加库和可执行文件路径：
```
sudo cp /opt/sophon/sophon-ffmpeg-latest/data/01_sophon-ffmpeg.conf /etc/ld.so.conf.d/
sudo cp /opt/sophon/sophon-opencv-latest/data/02_sophon-opencv.conf /etc/ld.so.conf.d/
sudo ldconfig
sudo cp /opt/sophon/sophon-ffmpeg-latest/data/sophon-ffmpeg-autoconf.sh /etc/profile.d/
sudo cp /opt/sophon/sophon-opencv-latest/data/sophon-opencv-autoconf.sh /etc/profile.d/
sudo cp /opt/sophon/sophon-sample-latest/data/sophon-sample-autoconf.sh /etc/profile.d/
source /etc/profile
```
其他平台机器请参考[libsophon安装教程](https://doc.sophgo.com/sdk-docs/v24.04.01/docs_latest_release/docs/libsophon/guide/html/index.html)。
更多sophon-mw信息请参考《MULTIMEDIA使用手册.pdf》、《MULTIMEDIA开发参考手册.pdf》。


---

## 编译指南

# 编译指南

[English](HowToMake_EN.md) | 简体中文

- [编译指南](#编译指南)
  - [使用开发镜像编译](#使用开发镜像编译)
  - [x86/arm PCIe平台](#x86arm-pcie平台)
  - [SoC平台](#soc平台)
  - [编译结果](#编译结果)

* 需要注意，编译需要在sophon-stream目录下进行。

## 使用开发镜像编译

* 原则上stream编译不强依赖docker镜像；
* 如果您使用的主机部分环境不兼容，且不方便更改本机环境，可以使用我们提供的docker镜像进行编译；
* 请注意，不要将下文的stream_dev镜像和用于模型编译的tpuc_dev镜像混用。

通过dfss下载：
```bash
pip3 install dfss
python3 -m dfss --url=open@sophgo.com:/sophon-stream/docker/stream_dev.tar
```

如果设备为bm1688/cv186ah，而且SDK版本大于等于1.9，则需要拉取如下镜像：
```bash
pip3 install dfss
python3 -m dfss --url=open@sophgo.com:/sophon-stream/docker/stream_dev_22.04.tar
```

如果是首次使用Docker, 可执行下述命令进行安装和配置(仅首次执行):
```bash
sudo apt install docker.io
sudo systemctl start docker
sudo systemctl enable docker
sudo groupadd docker
sudo usermod -aG docker $USER
newgrp docker
```

在下载好的镜像目录中加载镜像
```bash
docker load -i stream_dev.tar
```
可以通过`docker images`查看加载好的镜像。需要注意镜像版本号，stream_dev_22.04镜像版本号为0.2。

创建容器
```bash
docker run --privileged --name stream_dev -v $PWD:/workspace  -it stream_dev:latest
# stream_dev只是举个名字的例子, 请指定成自己想要的容器的名字
```
容器中的`workspace`目录会挂载到您运行`docker run`时所在的宿主机目录，您可以在此容器中编译项目


## x86/arm PCIe平台
```bash
# 以下命令需要在sophon-stream项目根目录执行
mkdir build
cd build
cmake ..
make -j4
```
如果需要qt显示，请先下载公版qt，否则将默认不编译qt组件。

可以通过以下命令下载qt：
```bash
sudo apt install qtbase5-dev
```

如果需要在http_push插件中使用https，请先下载公版OpenSSL，否则将默认不支持https。

可以通过以下命令下载OpenSSL：
```bash
sudo apt install libssl-dev
```


## SoC平台
通常在x86主机上交叉编译程序，您需要在x86主机上使用SOPHON SDK搭建交叉编译环境，将程序所依赖的头文件和库文件打包至sophon_sdk_soc目录中。您可以下载SOPHON SDK自行打包，也可以下载我们打包好的文件(根据您的SOC环境选择一个即可)。

下列文件，分别对应官网BM1684/BM1684X SDK的v23.03.01、v23.05.01、v23.07.01、v23.10.01、v25.03.01版本。
```bash
pip3 install dfss
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/soc0301.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/soc0501.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/soc0701.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/soc1001.tar.gz
python3 -m dfss --url=open@sophgo.com:/soc-sdk-allin/v25.03.01/soc-sdk-allin.tgz
```

下列文件，分别对应BM1688 SDK的1.7、1.8、1.9版本。
```bash
pip3 install dfss
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/1688_1.7.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/1688_1.8.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/1688_1.9.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/1688_2.0.tar.gz
python3 -m dfss --url=open@sophgo.com:/sophon-stream/soc-sdk/1688_2.1.tar.gz
```

如果需要使用qt，只需要在x86上下载用于交叉编译的Qt。盒子上环境已经是齐全的，不需要重新下载/安装Qt，并在编译时用`QTPATH`参数指定qt的路径：（如果不需要使用QT，可以忽略这部分，并且不添加交叉编译时的`QTPATH`参数）

BM1684/BM1684X设备：
```bash
python3 -m dfss --url=open@sophgo.com:sophon-demo/MultiYolov5/qt-5.14-amd64-aarch64-fl2000fb_v1.1.0.tar.xz
```

BM1688/CV186AH设备：
```bash
python3 -m dfss --url=open@sophgo.com:sophon-pipeline/a2_bringup/qtbase.zip
```

如果需要在http_push插件中使用https，只需要在x86上下载用于交叉编译的openssl，并在编译时使用`OPENSSL_PATH`参数指定openssl的路径：（如果不需要使用https，可以忽略这部分，并且不添加交叉编译时的`OPENSSL_PATH`参数）
```bash
python3 -m dfss --dflag=openssl_1.1.1f_aarch64
```

如果使用BM1688 1.9及之后的SDK版本，由于系统从ubuntu20.04升级到22.04，所以需要使用如下命令下载较新的openssl。
```bash
python3 -m dfss --url=open@sophgo.com:sophon-stream/soc-sdk/openssl_3_aarch64.tar.gz
```

交叉编译时，`SOPHON_SDK_SOC`、`QTPATH`，`OPENSSL_PATH`需要填写绝对路径

```bash
# 以下命令需要在sophon-stream项目根目录执行
mkdir build
cd build
cmake ../ -DTARGET_ARCH=soc -DSOPHON_SDK_SOC=/path/to/sophon_sdk_soc -DQTPATH=/path/to/qt -DOPENSSL_PATH=/path/to/openssl
make -j4
```

如果您需要的SDK版本上文未提供，需要自己打包soc-sdk，可以参考以下流程进行打包。需要注意的是，对于SE9设备，下列命令中的sophon-mw目录名或压缩包文件名应改为sophon-media。

 1. 解压SDK目录下，sophon-img包里的libsophon_soc_<x.y.z>_aarch64.tar.gz，将lib和include的所有内容分别拷贝到您的soc-sdk目录
 ```bash
 cd sophon-img_<date>_<hash>
# 创建依赖文件的根目录
mkdir -p soc-sdk
# 解压sophon-img release包里的libsophon_soc_${x.y.z}_aarch64.tar.gz，其中x.y.z为版本号
tar -zxf libsophon_soc_<x.y.z>_aarch64.tar.gz
# 将相关的库目录和头文件目录拷贝到依赖文件根目录下
cp -rf libsophon_soc_<x.y.z>_aarch64/opt/sophon/libsophon-<x.y.z>/lib ${soc-sdk}
cp -rf libsophon_soc_<x.y.z>_aarch64/opt/sophon/libsophon-<x.y.z>/include ${soc-sdk}
 ```
 2. 解压sophon-mw包里的sophon-mw-soc_<x.y.z>_aarch64.tar.gz，将sophon-mw下lib和include的所有内容拷贝到您的soc-sdk目录。
 ```bash
 cd sophon-mw_<date>_<hash>
# 解压sophon-mw包里的sophon-mw-soc_<x.y.z>_aarch64.tar.gz，其中x.y.z为版本号
tar -zxf sophon-mw-soc_<x.y.z>_aarch64.tar.gz
# 将ffmpeg和opencv的库目录和头文件目录拷贝到依赖文件根目录下
cp -rf sophon-mw-soc_<x.y.z>_aarch64/opt/sophon/sophon-ffmpeg_<x.y.z>/lib ${soc-sdk}
cp -rf sophon-mw-soc_<x.y.z>_aarch64/opt/sophon/sophon-ffmpeg_<x.y.z>/include ${soc-sdk}
cp -rf sophon-mw-soc_<x.y.z>_aarch64/opt/sophon/sophon-opencv_<x.y.z>/lib ${soc-sdk}
cp -rf sophon-mw-soc_<x.y.z>_aarch64/opt/sophon/sophon-opencv_<x.y.z>/include ${soc-sdk}
 ```

## 编译结果
1.`framework`和`element`会在`build/lib`中生成动态链接库

2.`samples`会在`samples/build`文件夹生成可执行文件

对于PCIe平台，可以直接在PCIe平台上运行测试；对于SoC平台，需将交叉编译生成的动态链接库和可执行文件拷贝到SoC平台中测试。

3.交叉编译完成后，将编译结果scp到盒子上时，需要保证目录结构不变。

可以使用如下命令直接将主机上的stream目录拷贝到盒子上：

```bash
scp -r ./sophon-stream linaro@<your ip>:<your path>
```

其中，盒子的ip和文件目录您可以根据实际情况进行设置。

4.登录目标盒子，添加环境变量，以确保运行`sample`中的程序时能找到动态链接库：
```bash
echo 'export LD_LIBRARY_PATH=<your path>/sophon-stream/build/lib/:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc
```

其中，`<your path>`替换为目标盒子中`sophon-stream`的绝对路径。

---

## 常见问题及解答 (FAQ)

# 使用问题及解答

[English](FAQ_EN.md) | 简体中文

#### 1. 初始化阶段报错`Can not find element maker, name: xxx`

这个原因是修改了某个element的json文件里的"name"项，这是Factory识别Element的依据，需要保持原样。


#### 2. 编译报错`fatal error: boost/version.hpp: No such file or directory`
请使用如下命令安装该库：
```bash
sudo apt-get update
sudo apt-get install libboost-all-dev
```

#### 3. 交叉编译之后在盒子上运行报错 `undefined symbol: _ZN5boost6system15system_categoryEv`

先检查boost库的版本：`dpkg -S /usr/include/boost/version.hpp` 1.71版本可以正常编译运行，较旧的版本可能报如上错误。

如果不便安装该版本boost，可以使用我们提供的[docker镜像进行编译](./HowToMake.md#使用开发镜像编译)

#### 4. 交叉编译之后在盒子上运行报错`/lib/aarch64-linux-gnu/libc.so.6: version 'GLIBC_2.33' not found`
这是由于您主机上的交叉编译工具链版本太高导致，可以通过如下命令重新安装：
```bash
sudo apt remove cpp-*-aarch64-linux-gnu
sudo apt-get install gcc-7-aarch64-linux-gnu g++-7-aarch64-linux-gnu
sudo ln -s /usr/bin/aarch64-linux-gnu-gcc-7 /usr/bin/aarch64-linux-gnu-gcc
sudo ln -s /usr/bin/aarch64-linux-gnu-g++-7 /usr/bin/aarch64-linux-gnu-g++
```

如果不便安装该版本交叉编译器，可以使用我们提供的[docker镜像进行编译](./HowToMake.md#使用开发镜像编译)

#### 5. 编译阶段报错`找不到tpu_kernel相关定义`

启用tpu_kernel相关功能需要03.01及之后的SDK，可以参考官网进行升级。

#### 6. 交叉编译之后在盒子上运行报错`error while loading shared libraries: libxxx.so: cannot open shared object file: No such file or directory`

首先检查sophon-stream/build/lib目录下是否有这个so文件，如果没有，需要从交叉编译的主机上scp到盒子；如果文件路径无误但无法找到，需要执行以下命令：

```bash
export LD_LIBRARY_PATH=path-to/sophon-stream/build/lib/:$LD_LIBRARY_PATH
```

#### 7. 编译阶段报错`找不到-lframework、-livslogger`

一般是编译路径有误，例如在element目录或samples目录编译等。需要回到sophon-stream项目目录进行编译。

#### 8. 运行报错`[BM_CHECK][error] BM_CHECK_RET fail`，详细报错如下：
```
[bmlib_memory][error] bm_alloc_gmem failed, dev_id = 0, size = 0x20
[BM_CHECK][error] BM_CHECK_RET fail /workspace/libsophon/bmlib/src/bmlib_memory.cpp: bm_malloc_device_byte_heap_mask: 705
MAT Allocate Err: dims = 2, size = [1, 8], type = 5
terminate called after throwing an instance of 'cv::Exception'
[bmlib_memory][error] bm_alloc_gmem failed, dev_id = 0, size = 0x10
  what():  OpenCV(4.1.0) /workspace/middleware-soc/bm_opencv/modules/core/src/matrix.cpp:448: error: (-215:Assertion failed) u != 0 in function 'create'

[BM_CHECK][error] BM_CHECK_RET fail /workspace/libsophon/bmlib/src/bmlib_memory.cpp: bm_malloc_device_byte_heap_mask: 705
Aborted (core dumped)
```
可能是linux文件句柄数量限制导致的，某些设备上默认句柄数量限制为1024，可尝试将其设置为20480
```
ulimit -n 20480
```

#### 9. 有的图片没有识别/检测结果

图片/视频存在少数的漏检误检是正常情况，因为原模型精度也无法达到100%。stream暂时未提供精度评估功能，观察大部分图片识别/检测结果正常即可。

#### 10. 推流失败

推流失败最常见的原因是未开启流服务器，此种情况下，终端会有如下打印：

```bash
[tcp @ 0x7f6cbe4a00] Connection to tcp://localhost:1935?tcp_nodelay=0 failed: Connection refused
[rtmp @ 0x7f6cbe4580] Cannot open connection tcp://localhost:1935?tcp_nodelay=0
```

或：

```bash
[tcp @ 0x7f40104eb0] Connection to tcp://localhost:8554?timeout=0 failed: Connection refused
```

此种情况下，请参考 [encode element](../element/multimedia/encode/README.md)，开启流服务器后重试。
---

## 自定义插件指南

# 自定义插件指南

* 本指南旨在讲解sophon-stream element的工作原理，帮助用户快速开发支持自定义功能的插件。

## 1. sophon-stream插件工作原理

`sophon-stream` 框架中，`element` 是实际工作的最小数据结构。所有 `element` 都是继承自同一个抽象基类 `sophon_stream::framework::Element`。该基类提供了通用的初始化接口 `init()` 和多线程启停与工作逻辑，也提供了主要的纯虚函数 `initInternal()` 和 `doWork()`，分别用于自定义初始化和自定义线程函数。

因此，添加自定义的`element`，实际上就是添加一个新的派生类。它继承自 `sophon_stream::framework::Element` 基类，需要实现独有的初始化函数 `initInternal()` 及工作函数 `doWork()`，以及一些可能的其它自定义功能。

## 2. 添加自定义插件

目前，`sophon-stream` 中已经支持了较多常见的CV模型或算法，具体可以参见 [algorithm](../element/algorithm/) 目录。若用户的目标算法暂不在支持列表中，可以参考如下方式进行自定义开发。

### 2.1 添加深度学习算法插件

`sophon-stream` 仓库中提供了快速添加深度学习算法插件的方法。仅需要按照如下命令操作，即可在 `sophon-stream/element/algorithm` 目录下生成一个由用户自定义算法命名的子目录。

```bash
cd sophon-stream/element/algorithm/
./algorithm_maker.sh <your_alg_name>
```

其中，`<your_alg_name>` 是用户自定义的插件名。

下文将以 `yolov3` 算法为例进行说明。

运行 `./algorithm_make.sh yolov3` 命令，将在当前目录下生成 `yolov3` 子目录，其目录结构为：

```bash
yolov3
├── CMakeLists.txt
├── include
│   ├── yolov3_context.h        # 声明context结构，存放当前算法的只读参数。例如模型路径、后处理阈值等
│   ├── yolov3.h                # Element的派生类，在对应的.cc文件中实现initInternal()及doWork()方法
│   ├── yolov3_inference.h      # 声明推理类
│   ├── yolov3_post_process.h   # 声明后处理类
│   └── yolov3_pre_process.h    # 声明预处理类
├── README.md
└── src
    ├── yolov3.cc               # 实现算法Element的初始化和工作逻辑，常规CV模型工作逻辑一般不需要修改，只需要添加自定义初始化功能
    ├── yolov3_inference.cc     # 实现推理功能，一般不需要额外修改
    ├── yolov3_post_process.cc  # 实现后处理功能
    └── yolov3_pre_process.cc   # 实现预处理功能
```

接下来，需要实现算法的初始化功能、数据处理逻辑以及具体的预处理和后处理算法。

其中，初始化功能即 `yolov3.cc` 文件内的 `initInternal()` 函数，该函数以一个json字符串为输入，由用户自定义的规则进行解析。一般此处会解析如模型路径、预处理的均值、方差、后处理的阈值等参数。

数据处理逻辑即 `yolov3.cc` 文件内的 `doWork()` 函数。该函数主要实现了从输入队列弹出数据、凑batch、处理、push进输出队列的过程。对于常规的CV模型来说，模板代码中的该部分已经可满足需求，一般不需要额外修改。

预处理算法和后处理算法分别在 `yolov3_pre_process.cc` 和 `yolov3_post_process.cc` 中实现，该部分实现细节可以参考算法源码和 [BMCV用户手册](https://doc.sophgo.com/sdk-docs/v23.09.01-lts/docs_latest_release/docs/bmcv/reference/html/index.html) 等文档。

需要注意，由于预处理和后处理一般都是只针对一张或一组数据进行操作，因此原则上不应存在线程冲突，也就不需要使用互斥量进行保护。

#### 2.1.1 yolov3_context.h

* 本节说明 `yolov3_context.h` 文件中应涉及的内容。

`yolov3_context.h` 文件中应包含一个继承了 `sophon_stream::framework::Context` 的类，用于定义模型的预处理、推理、后处理相关参数。

使用 `algorithm_maker.sh` 生成的模板应如下所示：

```cpp
class Yolov3Context : public ::sophon_stream::framework::Context {
 public:
  int deviceId;  // 设备ID

  std::shared_ptr<BMNNContext> bmContext;
  std::shared_ptr<BMNNNetwork> bmNetwork;
  bm_handle_t handle;

  std::vector<float> mean;  // 前处理均值， 长度为3，顺序为rgb
  std::vector<float> stdd;  // 前处理方差， 长度为3，顺序为rgb
  bool bgr2rgb;             // 是否将bgr图像转成rgb推理

  /**
   * @brief 最小的置信度阈值。详细说明请参考README
   */
  float thresh_conf_min = -1;
  /**
   * @brief 置信度阈值，key：类名，value：阈值
   * 该参数支持对不同的类别设置不同的阈值
   */
  std::unordered_map<std::string, float> thresh_conf;
  /**
   * @brief NMS IOU阈值
   */
  float thresh_nms;
  std::vector<std::string> class_names;
  /**
   * @brief 决定是否启用类别阈值
   */
  bool class_thresh_valid = false;

  /**
   * @brief
   * 类别数量，从model中读取。需要和thresh_conf、class_names的长度做校验
   */
  int class_num = 80;
  int m_frame_h, m_frame_w;
  int net_h, net_w, m_net_channel;
  int max_batch;
  int input_num;
  int output_num;
  int min_dim;
  bmcv_convert_to_attr converto_attr;

  /**
   * @brief json文件中定义的ROI，若此项生效，则只对ROI划定的区域做算法
   */
  bmcv_rect_t roi;
  bool roi_predefined = false;
};
```

上面包含了深度学习算法中大部分常见的参数。由于涵盖范围较广，很多参数并不一定是每个算法都必须的。例如，对于目标检测算法，往往会需要后处理的类别置信度阈值和NMS的IOU阈值，但对于分类算法（如 `Resnet` ），则不需要这两个参数，可以将其从成员中删去。

#### 2.1.2 yolov3.h

* 本节说明 `yolov3.h` 文件中应包含的内容。

`yolov3.h` 文件声明了继承自 `sophon_stream::framework::Element` 的派生类。其形式如：

```cpp
class Yolov3 : public ::sophon_stream::framework::Element {
   public:
    Yolov3();
    ~Yolov3() override;

    const static std::string elementName;

    /**
     * @brief
     * 解析configure，初始化派生element的特有属性；调用initContext初始化算法相关参数
     * @param json json格式的配置文件
     * @return common::ErrorCode
     * 成功返回common::ErrorCode::SUCCESS，失败返回common::ErrorCode::PARSE_CONFIGURE_FAIL
     */
    common::ErrorCode initInternal(const std::string& json) override;

    /**
     * @brief
     * element的功能在这里实现。例如，算法模块需要实现组batch、调用算法、发送数据等功能
     * @param dataPipeId pop数据时对应的dataPipeId
     * @return common::ErrorCode 成功返回common::ErrorCode::SUCCESS
     */
    common::ErrorCode doWork(int dataPipeId) override;

    // 以下是element管理接口
    void setContext(
        std::shared_ptr<::sophon_stream::framework::Context> context);
    void setPreprocess(
        std::shared_ptr<::sophon_stream::framework::PreProcess> pre);
    void setInference(
        std::shared_ptr<::sophon_stream::framework::Inference> infer);
    void setPostprocess(
        std::shared_ptr<::sophon_stream::framework::PostProcess> post);
    void setStage(bool pre, bool infer, bool post);
    void initProfiler(std::string name, int interval);
    std::shared_ptr<::sophon_stream::framework::Context> getContext() {
      return mContext;
    }
    std::shared_ptr<::sophon_stream::framework::PreProcess> getPreProcess() {
      return mPreProcess;
    }
    std::shared_ptr<::sophon_stream::framework::Inference> getInference() {
      return mInference;
    }
    std::shared_ptr<::sophon_stream::framework::PostProcess> getPostProcess() {
      return mPostProcess;
    }

    /**
     * @brief 从json文件读取的配置项，应按需增删
     */
    static constexpr const char* CONFIG_INTERNAL_STAGE_NAME_FIELD = "stage";
    static constexpr const char* CONFIG_INTERNAL_MODEL_PATH_FIELD =
        "model_path";
    static constexpr const char* CONFIG_INTERNAL_THRESHOLD_CONF_FIELD =
        "threshold_conf";
    static constexpr const char* CONFIG_INTERNAL_THRESHOLD_NMS_FIELD =
        "threshold_nms";
    static constexpr const char* CONFIG_INTERNAL_THRESHOLD_BGR2RGB_FIELD =
        "bgr2rgb";
    static constexpr const char* CONFIG_INTERNAL_THRESHOLD_MEAN_FIELD = "mean";
    static constexpr const char* CONFIG_INTERNAL_THRESHOLD_STD_FIELD = "std";
    static constexpr const char* CONFIG_INTERNAL_CLASS_NAMES_FILE_FIELD =
        "class_names_file";
    static constexpr const char* CONFIG_INTERNAL_ROI_FILED = "roi";
    static constexpr const char* CONFIG_INTERNAL_LEFT_FILED = "left";
    static constexpr const char* CONFIG_INTERNAL_TOP_FILED = "top";
    static constexpr const char* CONFIG_INTERNAL_WIDTH_FILED = "width";
    static constexpr const char* CONFIG_INTERNAL_HEIGHT_FILED = "height";

   private:
    std::shared_ptr<Yolov3Context> mContext;          // context对象
    std::shared_ptr<Yolov3PreProcess> mPreProcess;    // 预处理对象
    std::shared_ptr<Yolov3Inference> mInference;      // 推理对象
    std::shared_ptr<Yolov3PostProcess> mPostProcess;  // 后处理对象

    bool use_pre = false;
    bool use_infer = false;
    bool use_post = false;

    std::string mFpsProfilerName;
    ::sophon_stream::common::FpsProfiler mFpsProfiler;

    common::ErrorCode initContext(const std::string& json);
    void process(common::ObjectMetadatas& objectMetadatas);
  };
```

#### 2.1.3 yolov3.cc

* 本节说明 `yolov3.cc` 文件中应包含的内容。

首先，`yolov3.cc` 文件中必须实现 `initInternal()` 和 `doWork()` 两个函数。

模板自动生成的内容如下所示：

```cpp
common::ErrorCode Yolov3::initInternal(const std::string& json) {
    common::ErrorCode errorCode = common::ErrorCode::SUCCESS;
    do {
      // json是否正确
      auto configure = nlohmann::json::parse(json, nullptr, false);
      if (!configure.is_object()) {
        errorCode = common::ErrorCode::PARSE_CONFIGURE_FAIL;
        break;
      }
      // 判断当前element运行算法的预处理/推理/后处理阶段
      auto stageNameIt = configure.find(CONFIG_INTERNAL_STAGE_NAME_FIELD);
      if (configure.end() != stageNameIt && stageNameIt->is_array()) {
        std::vector<std::string> stages =
            stageNameIt->get<std::vector<std::string>>();
        if (std::find(stages.begin(), stages.end(), "pre") != stages.end()) {
          use_pre = true;
          mFpsProfilerName = "fps_yolov3_pre";
        }
        if (std::find(stages.begin(), stages.end(), "infer") != stages.end()) {
          use_infer = true;
          mFpsProfilerName = "fps_yolov3_infer";
        }
        if (std::find(stages.begin(), stages.end(), "post") != stages.end()) {
          use_post = true;
          mFpsProfilerName = "fps_yolov3_post";
        }

        mFpsProfiler.config(mFpsProfilerName, 100);
      }
      // 新建context,预处理,推理和后处理对象
      mContext = std::make_shared<Yolov3Context>();
      mPreProcess = std::make_shared<Yolov3PreProcess>();
      mInference = std::make_shared<Yolov3Inference>();
      mPostProcess = std::make_shared<Yolov3PostProcess>();

      if (!mPreProcess || !mInference || !mPostProcess || !mContext) {
        break;
      }

      mContext->deviceId = getDeviceId();
      // 初始化上一节中定义的context
      initContext(configure.dump());
      // 前处理初始化
      mPreProcess->init(mContext);
      // 推理初始化
      mInference->init(mContext);
      // 后处理初始化
      mPostProcess->init(mContext);

    } while (false);
    return errorCode;
  }
```

在这里，算法插件的自定义初始化放在了 `initContext()` 函数中。这样设置与直接在 `initInternal()` 中初始化没有本质上的区别。

`initContext()` 函数形如：

```cpp
common::ErrorCode Yolov3::initContext(const std::string& json) {
    common::ErrorCode errorCode = common::ErrorCode::SUCCESS;
    do {
      auto configure = nlohmann::json::parse(json, nullptr, false);
      if (!configure.is_object()) {
        errorCode = common::ErrorCode::PARSE_CONFIGURE_FAIL;
        break;
      }
      // 从json中解析模型路径。解析其它参数可以参考此行代码编写。
      auto modelPathIt = configure.find(CONFIG_INTERNAL_MODEL_PATH_FIELD);
    } while (false);
    return common::ErrorCode::SUCCESS;
  }
```

这里只演示了从json中解析模型路径的部分。实际开发时，应该还需要包含读入模型、解析模型输入输出形状参数等步骤。

`doWork()` 函数形如：

```cpp
common::ErrorCode Yolov3::doWork(int dataPipeId) {
    common::ErrorCode errorCode = common::ErrorCode::SUCCESS;

    common::ObjectMetadatas objectMetadatas;
    std::vector<int> inputPorts = getInputPorts();
    // 默认算法插件是单输入，如果有特殊设计可以修改
    int inputPort = inputPorts[0];
    int outputPort = 0;
    if (!getSinkElementFlag()) {
      std::vector<int> outputPorts = getOutputPorts();
      outputPort = outputPorts[0];
    }

    common::ObjectMetadatas pendingObjectMetadatas;

    while (objectMetadatas.size() < mContext->max_batch &&
           (getThreadStatus() == ThreadStatus::RUN)) {
      // pop数据凑batch，如果队列为空则等待
      auto data = popInputData(inputPort, dataPipeId);
      if (!data) {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        continue;
      }
      // 判断是否有跳帧
      auto objectMetadata =
          std::static_pointer_cast<common::ObjectMetadata>(data);
      if (!objectMetadata->mFilter) objectMetadatas.push_back(objectMetadata);

      pendingObjectMetadatas.push_back(objectMetadata);

      if (objectMetadata->mFrame->mEndOfStream) {
        break;
      }
    }
    // 实际处理的函数。调用预处理/推理/后处理过程
    process(objectMetadatas);

    for (auto& objectMetadata : pendingObjectMetadatas) {
      int channel_id_internal = objectMetadata->mFrame->mChannelIdInternal;
      int outDataPipeId =
          getSinkElementFlag()
              ? 0
              : (channel_id_internal % getOutputConnectorCapacity(outputPort));
      // 向后面的队列push数据
      errorCode =
          pushOutputData(outputPort, outDataPipeId,
                         std::static_pointer_cast<void>(objectMetadata));
      if (common::ErrorCode::SUCCESS != errorCode) {
        IVS_WARN(
            "Send data fail, element id: {0:d}, output port: {1:d}, data: "
            "{2:p}",
            getId(), outputPort, static_cast<void*>(objectMetadata.get()));
      }
    }
    mFpsProfiler.add(objectMetadatas.size());

    return common::ErrorCode::SUCCESS;
  }
```

从这里看，代码模板中的 `doWork()` 函数已经包含了一个线程函数应该有的功能：从前一个队列中pop数据、处理数据、再向后面的队列push数据。因此，这段代码一般不需要用户修改。`doWork()` 函数中提供了 `process()` 函数用于调用实际的预处理/推理/后处理过程，用户可以只专注于该部分的开发。

#### 2.1.4 yolov3_pre_process.h && yolov3_pre_process.cc

`yolov3_pre_process.h` 文件包含对预处理类的声明。

```cpp
class Yolov3PreProcess : public ::sophon_stream::framework::PreProcess {
 public:
  /**
   * @brief 对一个batch的数据做预处理
   * @param context context指针
   * @param objectMetadatas 一个batch的数据
   * @return common::ErrorCode
   * common::ErrorCode::SUCCESS，中间过程失败会中断执行
   */
  common::ErrorCode preProcess(std::shared_ptr<Yolov3Context> context,
                               common::ObjectMetadatas& objectMetadatas);
  void init(std::shared_ptr<Yolov3Context> context);
 private:
  /**
   * @brief 为一个batch的数据初始化设备内存
   * @param context context指针
   * @param objectMetadatas 一个batch的数据
   */
  void initTensors(std::shared_ptr<Yolov3Context> context,
                   common::ObjectMetadatas& objectMetadatas);
};
```

这里只涉及两个函数。`preProcess()` 是算法预处理阶段实际调用的函数，它从输入的 `objectMetadatas` 中获取一个batch上的图像信息，然后经过处理将其转化为 `tensor` 送给NPU推理。`initTensors()`函数是对每个batch的图像数据的初始化操作。它按照当前 `element` 初始化阶段获取到的模型信息，为每帧图像申请推理所需的设备内存，并使用智能指针管理其生命周期和析构函数。

目前，`initTensors()` 函数的具体实现已经比较通用，基本不需要特异性的更改。用户只需要在 `preProcess()` 函数中填入自己的预处理逻辑即可完成该模块的开发。

#### 2.1.5 yolov3_inference.h && yolov3_inference.cc

`yolov3_inference.h` 文件包含对推理类的声明。

```cpp
class Yolov3Inference : public ::sophon_stream::framework::Inference {
 public:
  ~Yolov3Inference() override;
  /**
   * @brief init device and engine
   * @param[in] context: model path,inputs and outputs name...
   */
  void init(std::shared_ptr<Yolov3Context> context);
  /**
   * @brief network predict output
   * @param[in] context: inputData and outputData
   */
  common::ErrorCode predict(std::shared_ptr<Yolov3Context> context,
                            common::ObjectMetadatas& objectMetadatas);
 private:
  /**
   * @brief 组合inputTensor，batchsize==1时不调用
   * @param context context指针
   * @param objectMetadatas 一个batch的数据
   * @return std::shared_ptr<sophon_stream::common::bmTensors>
   * 组合的inputTensors
   */
  std::shared_ptr<sophon_stream::common::bmTensors> mergeInputDeviceMem(
      std::shared_ptr<Yolov3Context> context,
      common::ObjectMetadatas& objectMetadatas);
  /**
   * @brief 申请outputTensors
   * @param context context指针
   * @return std::shared_ptr<sophon_stream::common::bmTensors>
   * 申请的outputTensors
   */
  std::shared_ptr<sophon_stream::common::bmTensors> getOutputDeviceMem(
      std::shared_ptr<Yolov3Context> context);
  /**
   * @brief
   * 将更新的outputTensors分配到每一个ObjectMetadata上，batchsize==1时不调用
   * @param context context指针
   * @param objectMetadatas 一个batch的数据
   * @param outputTensors 经过推理，更新的outputTensors
   */
  void splitOutputMemIntoObjectMetadatas(
      std::shared_ptr<Yolov3Context> context,
      common::ObjectMetadatas& objectMetadatas,
      std::shared_ptr<sophon_stream::common::bmTensors> outputTensors);
};
```

推理模块相关的函数如上所示。其中，外层 `process()` 函数直接调用的是 `predict()` 函数。该函数以一个batch的数据为输入，先将输入内存转化为连续并为其申请输出内存，然后进行推理，再把推理得到的连续内存分块到batch内的各帧图像上。特别地，当 `batch_size` 为1时，将直接申请输出内存并进行推理，省略了转化连续内存和将连续内存分块的操作。

一般来说，推理模块的代码已经具有一定的通用性，用户不需要修改。

#### 2.1.6 yolov3_post_process.h && yolov3_post_process.cc

`yolov3_post_process.h` 文件包含对推理类的声明。

```cpp
class Yolov3PostProcess : public ::sophon_stream::framework::PostProcess {
 public:
  void init(std::shared_ptr<Yolov3Context> context);
  /**
   * @brief 对一个batch的数据做后处理
   * @param context context指针
   * @param objectMetadatas 一个batch的数据
   */
  void postProcess(std::shared_ptr<Yolov3Context> context,
                   common::ObjectMetadatas& objectMetadatas);
 private:
};
```

如上所示，`yolov3_post_process.h` 中需要用户自己实现的函数只有 `postProcess()` 一个。该函数以一个batch的数据为输入，经过自定义的后处理，获得对应的输出结果，例如检测框的位置、置信度、类别等。

### 2.2 添加非深度学习的插件

非深度学习的插件和深度学习算法插件本质上是一致的，都符合 [插件工作原理](#1-sophon-stream插件工作原理) 的描述。但一般来说，基于深度学习的CV算法可以概括性地区分为预处理、推理和后处理三个阶段，因此可以使用一个统一的形式来组织其目录结构。而非深度学习的插件则不一定具有如上所述的阶段。依据插件的具体功能，如跟踪、绘图、编解码、向量召回、发送http请求等，其代码组织形式可以相对灵活，因此本仓库没有为非深度学习的插件提供统一的模板。

不过，添加非深度学习的插件时，也可以参考其它插件的基本框架，然后填入自己需要的功能。例如，可以参考 [空白插件](../element/tools/blank/)，这是一个实验性的插件，没有实现具体功能，因此代码行数最少，只override了基类的纯虚函数来保证编译可以通过。用户可以按照该插件的形式，实现自己的初始化及工作逻辑。

具体地，可以参考上文 [yolov3.cc](#213-yolov3cc)。

### 3 编译及测试

插件开发完成后，需要将其联合 `sophon-stream` 的框架及其它插件一起编译，并测试效果。

#### 3.1 编译

首先，在 [CMakeLists.txt](../CMakeLists.txt) 中增加一行 `checkAndAddElement(element/algorithm/yolov3)`，该命令效果是将 `yolov3` 插件加入 `sophon-stream` 的编译流程。

然后参考 [HowToMake.md](../docs/HowToMake.md) 编译即可。

编译完成后，在 `../build/lib` 目录下可以看到 `libyolov3.so` 文件。

#### 3.2 测试

最后，需要将新添加的插件与其它插件连接起来测试功能。

这里以 `yolov3` 插件为例。这类深度学习算法插件测试功能的方式一般是相似的，分为以下几步。

1. 编写配置文件。首先应参考 [yolov5例程](../samples/yolov5/)，新建一个 `samples/yolov3` 例程的测试目录。然后需要正确编写 `yolov3_demo.json`，`engine.json`，`decode.json`，`yolov3.json` 四个配置文件，分别对应输入数据、graph定义、解码插件、yolov3插件四个部分。

2. 编写绘图函数。绘图函数主要作用是可视化地验证结果正确性。绘图函数位于 [draw_funcs.h](../samples/include/draw_funcs.h)。对于检测任务来说，`draw_funcs.h` 文件中已经包含了比较通用的绘图函数，可以直接调用，无需重复开发。如果 `draw_funcs.h` 中未提供需要的绘图函数，用户自己实现后还应该在 [main.cc](../samples/src/main.cc) 中指明配置文件中 `draw_func_name` 字段和绘图函数的关系，以便通过 `json` 文件调用到预期的绘图函数。

3. 运行，观察结果。正确编写配置文件和绘图函数后，应实际运行来验证结果是否正确。运行方式可以参考 [samples](../samples) 目录下的各个例程，即统一以 `main` 二进制文件作为入口，由传入的 `json` 文件决定 `graph` 如何搭建。例程运行过程中的日志一般包括数据传递、各个插件的fps等。运行结束后，可以在 `results` 目录下看到保存的图片。

4. 除结果正确性之外，还应关注插件的安全性。即是否存在内存泄漏等。具体地，可以在程序运行时分别通过 `top` 命令和`bm-smi` 命令观察系统内存和设备内存是否会持续上涨。若有上涨，则说明插件中某处缺少内存释放逻辑，需要排查解决。

若非深度学习算法插件，其测试方法与上文基本相同，但配置文件中可能涉及的 `element` 及其连接规则需要结合插件功能来具体设置。运行结束后，应将结果与预期结果对比，判断功能是否正常。

