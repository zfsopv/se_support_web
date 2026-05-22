# Sophon-Stream 插件 (Element) 参考文档

> 本文档由 element/ 目录下所有中文 README.md 合并而成。
> 生成时间: 2026-05-22 15:04:00

## 目录

### 算法插件 (algorithm)
- [bytetrack](#bytetrack)
- [fastpose](#fastpose)
- [lightstereo](#lightstereo)
- [lprnet](#lprnet)
- [openpose](#openpose)
- [posec3d](#posec3d)
- [ppocr](#ppocr)
- [resnet](#resnet)
- [retinaface](#retinaface)
- [yolov5](#yolov5)
- [yolov7](#yolov7)
- [yolov8](#yolov8)
- [yolox](#yolox)

### 多媒体插件 (multimedia)
- [decode](#decode)
- [encode](#encode)
- [osd](#osd)

### 工具插件 (tools)
- [blank](#blank)
- [blend](#blend)
- [converger](#converger)
- [distributor](#distributor)
- [dpu](#dpu)
- [dwa](#dwa)
- [faiss](#faiss)
- [filter](#filter)
- [http_push](#http_push)
- [ive](#ive)
- [qt_display](#qt_display)
- [resize](#resize)
- [stitch](#stitch)

---

# 算法插件 (algorithm)

---

## bytetrack

# sophon-stream bytetrack element

[English](README_EN.md) | 简体中文

sophon-stream bytetrack element是sophon-stream框架中的一个插件，是一个简单、快速、强大的多目标跟踪器，且不依赖特征提取模型。本项目已提供此插件例程，详情请参见[ByteTrack Demo](../../../samples/bytetrack/README.md)

## 1. 特性
* 支持检测模块和跟踪模块解耦，可适配各种检测器
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream bytetrack插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "track_thresh": 0.5,
        "high_thresh": 0.6,
        "match_thresh": 0.7,
        "min_box_area": 10,
        "frame_rate": 30,
        "track_buffer": 30,
        "correct_box": true,
        "agnostic": true
    },
    "shared_object": "../../../build/lib/libbytetrack.so",
    "device_id": 0,
    "id": 0,
    "name": "bytetrack",
    "side": "sophgo",
    "thread_number": 4
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  track_thresh  |   浮点数   | 0.5 | 目标跟踪检测阈值，与目标检测阈值相关联。如果目标检测阈值偏低，可以适当调低此参数 |
|  high_thresh   |   浮点数   | 0.6 | 在暂未匹配的检测目标中初始化新轨迹的阈值，不做重点调整 |
|  match_thresh  |   浮点数   | 0.7 | 目标跟踪匹配阈值，用于判断检测框的关联性。如果同一目标检测对象容易被匹配为不同目标跟踪的对象，可以适当调高此参数 |
|  frame_rate    |   浮点数   | 30  | 视频帧率，影响跟踪目标的最大消失时间，超过此时间的目标将被移除，计算方式为(max_time_lost = frame_rate / 30.0 * track_buffer) |
|  min_box_area  |   整数    |  10 | 过滤掉h*w小于min_box_area的跟踪框 |
|  track_buffer  |   整数    |  30 | 目标跟踪缓存，与最大消失时间关联 |
|  correct_box   |   布尔值  | true | 是否使用卡尔曼滤波矫正追踪框，值为false时使用原始目标检测框 |
|    agnostic    |   布尔值  | true | 是否进行无类别跟踪，值为false时不同类别的box将偏移不同的偏移量，然后计算iou，偏移量为类别id乘7000|
|  shared_object |   字符串   |  "../../../build/lib/libbytetrack.so"  | libbytetrack 动态库路径 |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     id      |    整数       | 0  | element id |
|     name    |    字符串     | "bytetrack" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 无 | 启动线程数，需要保证和处理码流数一致 |

> **注意**：
需要保证插件线程数和处理码流数一致

---

## fastpose

# sophon-stream fastpose element

[English](README_EN.md) | 简体中文

sophon-stream fastpose element是sophon-stream框架中的一个插件，是一个简单、快速、强大的姿态识别模型。本项目已提供此插件例程，详情请参见 [yolov5_fastpose Demo](../../../samples/yolov5_fastpose/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream fastpose插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "model_path": "../data/models/fastpose/BM1684X/halpe26_fast_res50_256x192_int8_1b.bmodel",
        "stage": [
            "pre"
        ],
        "heatmap_loss": "MSELoss",
        "area_thresh": 0.0
    },
    "shared_object": "../../../build/lib/libfastpose.so",
    "name": "fastpose",
    "side": "sophgo",
    "thread_number": 2
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/fastpose/BM1684X/halpe26_fast_res50_256x192_int8_1b.bmodel" | fastpose模型路径 |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
|  heatmap_loss  |   字符串   | "MSELoss" | 姿态识别训练所使用的损失函数，暂只支持MSELoss |
|  area_thresh |   浮点数   |  0.0  | 姿态识别中的阈值 |
|  shared_object |   字符串   |  "../../../build/lib/libfastpose.so"  | libfastpose 动态库路径 |
|     name    |    字符串     | "fastpose" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 2 | 启动线程数 |

> **注意**：
1. stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。
2. fastpose依赖前序的检测器，需要搭配具有类别“person”检测能力的检测器一起使用。

---

## lightstereo

# sophon-stream lightstereo element

sophon-stream lightstereo element是sophon-stream框架中的一个插件，是一个简单、快速、强大的立体匹配模型。本项目已提供此插件例程，详情请参见 [dwa_lightstereo_encode Demo](../../../samples/dwa_lightstereo_encode/README.md)

## 1. 特性
* 支持2路输入
* 支持BM1688 SoC

## 2. 配置参数
sophon-stream lightstereo插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
      "model_path": "../dwa_lightstereo_encode/data/models/BM1688/LightStereo-S-SceneFlow_int8_1b_480x736.bmodel",
      "bgr2rgb": true,
      "mean": [0.485, 0.456, 0.406],
      "std": [0.229, 0.224, 0.225],
      "stage": ["pre"]
    },
    "shared_object": "../../build/lib/liblightstereo.so",
    "name": "lightstereo",
    "side": "sophgo",
    "thread_number": 1
}

```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../dwa_lightstereo_encode/data/models/BM1688/LightStereo-S-SceneFlow_int8_1b_480x736.bmodel" | lightstereo模型路径 |
|  bgr2rgb  |   bool   | true | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | [0.229,0.224,0.225] | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | [0.485,0.456,0.406] | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
|  stage |   字符串数组   |  pre  | 处理类型，分别有pre、infer、post，表示当前element做前处理、推理或后处理 |
|  shared_object |   字符串   |  "../../../build/lib/liblightstereo.so"  | liblightstereo.so 动态库路径 |
|     name    |    字符串     | "lightstereo" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数，目前只支持每个element一个线程 |

---

## lprnet

# sophon-stream lprnet element

[English](README_EN.md) | 简体中文

sophon-stream lprnet element 是 sophon-stream 框架中的一个插件，是一个简单、快速、强大的车牌识别模型。本项目已提供此 yolo 识别+lprnet 检测车牌的插件例程，详情请参见 [license_plate_recognition](../../../samples/license_plate_recognition/README.md)

## 特性

- 支持多路视频流
- 支持多线程处理

## 2. 配置参数

sophon-stream lprnet 插件分为预处理、推理、后处理三个部分，均具有一些可配置的参数，可以根据需求进行设置。以推理为例，是一些常用的参数：

```json
{
  "configure": {
    "model_path": "../models/BM1684/lprnet_fp32_1b.bmodel",
    "stage": ["infer"]
  },
  "shared_object": "../../../build/lib/liblprnet.so",
  "name": "lprnet",
  "side": "sophgo",
  "thread_number": 4
}
```

|    参数名     |  类型  |                  默认值                  |               说明               |
| :-----------: | :----: | :--------------------------------------: | :------------------------------: |
|  model_path   | 字符串 | "../models/BM1684/lprnet_fp32_1b.bmodel" |         lprnet 模型路径          |
|     stage     |  列表  |                 ["pre"]                  | 标志前处理、推理、后处理三个阶段 |
| shared_object | 字符串 |    "../../../build/lib/liblprnet.so"     |       liblprnet 动态库路径       |
|     name      | 字符串 |                 "lprnet"                 |           element 名称           |
|     side      | 字符串 |                 "sophgo"                 |             设备类型             |
| thread_number |  整数  |                    1                     |            启动线程数            |

> **注意**：

1. stage 参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接 element。将三个阶段分配在三个 element 上的目的是充分利用各项资源，提高检测效率。

---

## openpose

# sophon-stream openpose element

[English](README_EN.md) | 简体中文

sophon-stream openpose element是sophon-stream框架中的一个插件，是一个简单、快速、强大的姿态识别模型。本项目已提供此插件例程，详情请参见 [openpose Demo](../../../samples/openpose/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream openpose插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "model_path": "../data/models/BM1684X/pose_coco_int8_1b.bmodel",
        "threshold_nms": 0.05,
        "stage": [
            "pre"
        ]
    },
    "shared_object": "../../../build/lib/libopenpose.so",
    "name": "openpose",
    "side": "sophgo",
    "thread_number": 4
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/BM1684X/pose_coco_int8_1b.bmodel" | openpose模型路径 |
|  threshold_nms  |   浮点数   | 0.05 | 姿态识别NMS IOU阈值 |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
|  shared_object |   字符串   |  "../../../build/lib/libopenpose.so"  | libopenpose 动态库路径 |
|     name    |    字符串     | "openpose" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |

> **注意**：
1. stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。

---

## posec3d

# sophon-stream posec3d element

[English](README_EN.md) | 简体中文

sophon-stream posec3d element 是 sophon-stream 框架中的一个插件，是一个简单、快速、强大的行为识别模型。本项目已提供Alphapose + PoseC3D 检测和识别行为的插件例程，详情请参见 [yolov5_fastpose_posec3d](../../../samples/yolov5_fastpose_posec3d/README.md)

## 特性

- 支持多路视频流
- 支持多线程处理

## 2. 配置参数

sophon-stream posec3d 插件分为预处理、推理、后处理三个部分，均具有一些可配置的参数，可以根据需求进行设置。以推理为例，是一些常用的参数：

```json
{
  "configure": {
    "model_path": "../yolov5_fastpose_posec3d/data/models/BM1684X/posec3d_ntu60_int8.bmodel",
    "class_names_file": "../yolov5_fastpose_posec3d/data/label_map_ntu60.txt",
    "frames_num": 72
  },
  "shared_object": "../../build/lib/libposec3d.so",
  "name": "posec3d_group",
  "side": "sophgo",
  "thread_number": 3
}
```

|    参数名         |  类型  |                  默认值                                                    |               说明                |
| :--------------: | :----: | :------------------------------------------------------------------------: | :------------------------------: |
|  model_path      | 字符串 | "../yolov5_fastpose_posec3d/data/models/BM1684X/posec3d_ntu60_int8.bmodel" |         posec3d 模型路径          |
| class_names_file | 字符串 |      "../yolov5_fastpose_posec3d/data/label_map_ntu60.txt"                 |            行为类别名文件          |
|    frames_num    |  整数  |                    72                                                      |       行为识别时一起处理的帧数      |
|  shared_object   | 字符串 |    "../../build/lib/libposec3d.so"                                         |       libposec3d 动态库路径        |
|     name         | 字符串 |                 "posec3d_group"                                            |           element 名称            |
|     side         | 字符串 |                 "sophgo"                                                   |             设备类型             |
| thread_number    |  整数  |                    1                                                       |            启动线程数            |

> **注意**：

1. 按前处理-推理-后处理的顺序连接 element。将三个阶段分配在三个 element 上的目的是充分利用各项资源，提高检测效率。

---

## ppocr

# sophon-stream ppocr element

[English](README_EN.md) | 简体中文

sophon-stream ppocr element 是 sophon-stream 框架中的一个插件，是一个简单、快速、强大的文字检测识别模型。本项目已提供此插件例程，详情请参见 [PPOCR Demo](../../../samples/ppocr/README.md)

## 特性

- 支持多路视频流
- 支持多线程处理

## 2. 配置参数

sophon-stream ppocr 插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

检测部分：
```json
{
    "configure": {
        "model_path": "../ppocr/data/models/BM1684X/ch_PP-OCRv3_det_fp16_1b.bmodel",
        "bgr2rgb": false,
        "mean": [
            123.675,
            116.28,
            103.53
        ],
        "std": [
            58.395,
            57.12,
            57.375
        ]
    },
    "shared_object": "../../build/lib/libppocr_det.so",
    "name": "ppocr_det_group",
    "side": "sophgo",
    "thread_number": 4
}
```

|    参数名         |  类型  |                  默认值                                                    |               说明                |
| :--------------: | :----: | :------------------------------------------------------------------------: | :------------------------------: |
|  model_path      | 字符串 | "../ppocr/data/models/BM1684X/ch_PP-OCRv3_det_fp16_1b.bmodel"             |         检测模型路径               |
| bgr2rgb          | bool |      false                                                                   |            解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式                  |
|    mean         |  浮点数组  |                    无                                                      |       图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r      |
|    std           |  浮点数组  |                    无                                                      |       图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r      |
|  shared_object   | 字符串 |    "../../build/lib/libppocr_det.so"                                       |       libppocr_det 动态库路径      |
|     name         | 字符串 |                 "ppocr_det_group"                                            |           element 名称            |
|     side         | 字符串 |                 "sophgo"                                                   |             设备类型             |
| thread_number    |  整数  |                    1                                                       |            启动线程数            |

识别部分：
```json
{
  "configure": {
    "model_path": "../ppocr/data/models/BM1684X/ch_PP-OCRv3_rec_fp16_1b_320.bmodel",
    "beam_search": false,
    "beam_width": 3,
    "class_names_file": "../ppocr/data/datasets/ppocr_keys_v1.txt"
  },
  "shared_object": "../../build/lib/libppocr_rec.so",
  "name": "ppocr_rec_group",
  "side": "sophgo",
  "thread_number": 4
}

```

|    参数名         |  类型  |                  默认值                                                    |               说明                |
| :--------------: | :----: | :------------------------------------------------------------------------: | :------------------------------: |
|  model_path      | 字符串 | ".../ppocr/data/models/BM1684X/ch_PP-OCRv3_rec_fp16_1b_320.bmodel"         |         识别模型路径          |
| beam_search     | bool |                                     false                                    |            bean_search          |
| beam_width      | 整数 |                                         3                                      |            search宽度          |
| class_names_file | 字符串 |      "../ppocr/data/datasets/ppocr_keys_v1.txt"                              |            类别名文件          |
|  shared_object   | 字符串 |    "../../build/lib/libppocr_rec.so"                                         |       libppocr_rec 动态库路径        |
|     name         | 字符串 |                 "ppocr_rec_group"                                            |           element 名称            |
|     side         | 字符串 |                 "sophgo"                                                   |             设备类型             |
| thread_number    |  整数  |                    1                                                       |            启动线程数            |



---

## resnet

# sophon-stream resnet element

[English](README_EN.md) | 简体中文

sophon-stream resnet element是sophon-stream框架中的一个插件，是一个简单、快速、强大的分类模型。本项目已提供此插件例程，详情请参见 [ResNet Demo](../../../samples/resnet/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream resnet插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "model_path": "../data/models/BM1684X/resnet50_int8_4b.bmodel",
    "bgr2rgb": true,
    "mean": [
      0.229,
      0.224,
      0.225
    ],
    "std": [
      0.485,
      0.456,
      0.406
    ],
    "roi": {
      "left": 600,
      "top": 400,
      "width": 800,
      "height": 600
    },
    "task_type": "SingleLabel",
    "class_thresh": [0.5, 0.3, 0.7]
  },
  "shared_object": "../../../build/lib/libresnet.so",
  "name": "resnet",
  "side": "sophgo",
  "thread_number": 1
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/BM1684X/resnet_car_int8_4b.bmodel" | resnet模型路径 |
|  bgr2rgb  |   bool   | true | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | [0.229,0.224,0.225] | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | [0.485,0.456,0.406] | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
| roi | map | 无 | 预设的ROI，配置了此参数时，只会对ROI框取的区域进行处理 |
| task_type | 字符串 | "SingleLabel" | resnet的工作方式，`SingleLabel`表示输出分值最大的标签；`FeatureExtract`表示抽取特征向量，不进行分类；`MultiLabel`表示多标签输出，需要搭配`class_thresh`字段使用 |
| class_thresh | list | 无 | 当`task_type`为`MultiLabel`时生效，配置了每个类别的过滤阈值。如果不设置，则默认所有类别阈值均为0.5 |
|  shared_object |   字符串   |  "../../../build/lib/libresnet.so"  | libresnet 动态库路径 |
|     id      |    整数       | 0  | element id |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     name    |    字符串     | "resnet" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |

---

## retinaface

# sophon-stream retinaface element

[English](README_EN.md) | 简体中文

sophon-stream retinaface element是sophon-stream框架中的一个插件，利用额外监督(extra-supervised)和自监督(self-supervised)结合的多任务学习(multi-task learning)，对不同尺寸的人脸进行像素级定位。本项目已提供此插件例程，详情请参见 [retinaface Demo](../../../samples/retinaface/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream retinaface插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "model_path": "../data/models/BM1684X/retinaface_mobilenet0.25_fp32_1b.bmodel",
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
        ],
        "stage": [
            "pre"
        ]
    },
    "shared_object": "../../../build/lib/libretinaface.so",
    "name": "retinaface",
    "side": "sophgo",
    "thread_number": 1
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/retinaface_mobilenet0.25_int8_1b.bmodel" | retinaface模型路径 |
|  max_face_count   |   整数   | 50 | 最大的人脸数量 |
|  score_threshold  |   浮点数   | 0.1 | 目标检测置信度阈值 |
|  threshold_nms  |  浮点数 | 0.4 | 目标检测NMS IOU阈值 |
|  bgr2rgb  |   bool   | false | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | 无 | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | 无 | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
|  shared_object |   字符串   |  "../../../build/lib/libretinaface.so"  | libretinaface 动态库路径 |
|     id      |    整数       | 0  | element id |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     name    |    字符串     | "retinaface" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |

> **注意**：
1. stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。

---

## yolov5

# sophon-stream yolov5 element

[English](README_EN.md) | 简体中文

sophon-stream yolov5 element是sophon-stream框架中的一个插件，是一个简单、快速、强大的检测模型。本项目已提供此插件例程，详情请参见 [YOLOv5 Demo](../../../samples/yolov5/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream yolov5插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure":{
        "model_path":"../data/models/yolov5s_tpukernel_int8_4b.bmodel",
        "threshold_conf":0.5,
        "threshold_nms":0.5,
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
        "use_tpu_kernel": true,
        "roi": {
            "left": 600,
            "top": 400,
            "width": 800,
            "height": 600
        },
        "maxdet":1280,
        "mindet":50
    },
    "shared_object":"../../../build/lib/libyolov5.so",
    "id":0,
    "device_id":0,
    "name":"yolov5_group",
    "side":"sophgo",
    "thread_number":1
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/yolov5s_tpukernel_int8_4b.bmodel" | yolov5模型路径 |
|  threshold_conf   |   浮点数或map   | 0.5 | 目标检测物体置信度阈值，设置为浮点数时，所有类别共用同一个阈值；设置为map时，不同类别可以使用不同阈值，此时还需要正确设置class_names_file |
|  threshold_nms  |   浮点数   | 0.5 | 目标检测NMS IOU阈值 |
|  bgr2rgb  |   bool   | true | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | 无 | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | 无 | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
| roi | map | 无 | 预设的ROI，配置了此参数时，只会对ROI框取的区域进行处理 |
|  use_tpu_kernel  |   布尔值    |  true | 是否启用tpu_kernel后处理 |
| class_names_file | 字符串 | 无 | threshold_conf为浮点数时不生效，可以不设置；当threshold_conf为map时启用，class name文件的路径 |
|  shared_object |   字符串   |  "../../../build/lib/libyolov5.so"  | libyolov5 动态库路径 |
|     id      |    整数       | 0  | element id |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     name    |    字符串     | "yolov5" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |
|   maxdet    |    整数     | MAX_INT| 仅接受宽高都小于maxdet的检测框 |
|   mindet    |    整数     | 0 | 仅接受宽高都大于mindet的检测框 |

> **注意**：
1. stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。
3. tpu_kernel后处理仅适配BM1684X设备，若不启用，则需要设置为false


## 3. 动态修改参数

目前，yolov5插件支持在stream运行时通过外部http请求修改某些参数。代码中提供了动态修改置信度阈值的功能，可以使用如下python脚本进行验证：

```python
import requests
import json
import sys

url = "http://localhost:8000/yolov5/SetConfThreshold/10003"
payload = {"value": 1.0}
headers = {'Content-Type': 'application/json'}
response = requests.request("POST", url, headers=headers, data=json.dumps(payload))
print(response)
```

其中，10003为实际运行时yolov5插件的id；请求中value字段表示期望设置的置信度阈值。例如，上述请求会将置信度改为1.0，也就是说几乎任何情况都无法检测到目标。

目前设置的此置信度阈值，只有启用cpu后处理时生效。

> **需要注意：启用动态修改参数功能，需要参考 [README.md](../../../samples/README.md) 设置监听的ip和端口**
---

## yolov7

# sophon-stream yolov7 element

[English](README_EN.md) | 简体中文

sophon-stream yolov7 element是sophon-stream框架中的一个插件，是一个简单、快速、强大的检测模型。本项目已提供此插件例程，详情请参见 [YOLOv7 Demo](../../../samples/yolov7/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream yolov7插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure":{
        "model_path":"../data/models/yolov7s_tpukernel_int8_4b.bmodel",
        "threshold_conf":0.5,
        "threshold_nms":0.5,
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
        "stage": [
            "pre"
        ],
        "use_tpu_kernel": true,
        "roi": {
            "left": 600,
            "top": 400,
            "width": 800,
            "height": 600
    }
    },
    "shared_object":"../../../build/lib/libyolov7.so",
    "id":0,
    "device_id":0,
    "name":"yolov7",
    "side":"sophgo",
    "thread_number":1
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/yolov7s_tpukernel_int8_4b.bmodel" | yolov7模型路径 |
|  threshold_conf   |   浮点数或map   | 0.5 | 目标检测物体置信度阈值，设置为浮点数时，所有类别共用同一个阈值；设置为map时，不同类别可以使用不同阈值，此时还需要正确设置class_names_file |
|  threshold_nms  |   浮点数   | 0.5 | 目标检测NMS IOU阈值 |
|  bgr2rgb  |   bool   | true | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | 无 | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | 无 | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
| roi | map | 无 | 预设的ROI，配置了此参数时，只会对ROI框取的区域进行处理 |
|  use_tpu_kernel  |   布尔值    |  true | 是否启用tpu_kernel后处理 |
| class_names_file | 字符串 | 无 | threshold_conf为浮点数时不生效，可以不设置；当threshold_conf为map时启用，class name文件的路径 |
|  shared_object |   字符串   |  "../../../build/lib/libyolov7.so"  | libyolov7 动态库路径 |
|     id      |    整数       | 0  | element id |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     name    |    字符串     | "yolov7" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |

> **注意**：
1. stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。
3. tpu_kernel后处理仅适配BM1684X设备，若不启用，则需要设置为false

---

## yolov8

# sophon-stream yolov8 element

[English](README_EN.md) | 简体中文

sophon-stream yolov8 element是sophon-stream框架中的一个插件，是一个简单、快速、强大的检测模型。本项目已提供此插件例程，详情请参见 [yolov8 Demo](../../../samples/yolov8/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream yolov8插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "model_path": "../yolov8/data/models/BM1684X/yolov8s_int8_1b.bmodel",
        "threshold_conf": 0.5,
        "threshold_nms": 0.5,
        "bgr2rgb": true,
        "task_type": "Pose",
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
        "roi": {
            "left": 600,
            "top": 400,
            "width": 800,
            "height": 600
        }
    },
    "shared_object": "../../build/lib/libyolov8.so",
    "name": "yolov8_group",
    "side": "sophgo",
    "thread_number": 4
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/BM1684X/yolov8s_int8_1b.bmodel" | yolov8模型路径 |
|  threshold_conf   |   浮点数或map   | 0.5 | 目标检测物体置信度阈值，设置为浮点数时，所有类别共用同一个阈值；设置为map时，不同类别可以使用不同阈值，此时还需要正确设置class_names_file |
|  threshold_nms  |   浮点数   | 0.5 | 目标检测NMS IOU阈值 |
|  task_type   | 字符串 | "Detect" | yolov8算法类型，支持了 "Detect", "Cls", "Pose", "Seg"和"obb" |
|  bgr2rgb  |   bool   | true | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | 无 | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | 无 | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
| roi | map | 无 | 预设的ROI，配置了此参数时，只会对ROI框取的区域进行处理 |
| class_names_file | 字符串 | 无 | threshold_conf为浮点数时不生效，可以不设置；当threshold_conf为map时启用，class name文件的路径 |
|  shared_object |   字符串   |  "../../../build/lib/libyolov8.so"  | libyolov8 动态库路径 |
|     id      |    整数       | 0  | element id |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     name    |    字符串     | "yolov8" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |
| seg_tpu_opt |    bool     | false | yolov8_seg是否使用TPU后处理 |
| mask_bmodel_path |    字符串     | 无 | 当启用seg_tpu_opt时，后处理的bmodel路径 |

> **注意**：
1. stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。


---

## yolox

# sophon-stream yolox element

[English](README_EN.md) | 简体中文

sophon-stream yolox element是sophon-stream框架中的一个插件，是一个简单、快速、强大的检测模型。本项目已提供此插件例程，详情请参见 [YOLOX Demo](../../../samples/yolox/README.md)

## 1. 特性
* 支持多路视频流
* 支持多线程处理

## 2. 配置参数
sophon-stream yolox插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "model_path": "../data/models/BM1684X/yolox_s_int8_4b.bmodel",
    "threshold_conf": 0.5,
    "threshold_nms": 0.5,
    "bgr2rgb": true,
    "mean": [
      0,
      0,
      0
    ],
    "std": [
      0.0039216,
      0.0039216,
      0.0039216
    ],
    "stage": [
      "pre"
    ],
    "roi": {
      "left": 600,
      "top": 400,
      "width": 800,
      "height": 600
    }
  },
  "shared_object": "../../../build/lib/libyolox.so",
  "id": 0,
  "device_id": 0,
  "name": "yolox",
  "side": "sophgo",
  "thread_number": 2
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  model_path  |   字符串   | "../data/models/BM1684X/yolox_s_int8_4b.bmodel" | yolox模型路径 |
|  threshold_conf   |   浮点数或map   | 0.5 | 目标检测物体置信度阈值，设置为浮点数时，所有类别共用同一个阈值；设置为map时，不同类别可以使用不同阈值，此时还需要正确设置class_names_file |
|  threshold_nms  |   浮点数   | 0.5 | 目标检测NMS IOU阈值 |
|  bgr2rgb  |   bool   | true | 解码器解出来的图像默认是bgr格式，是否需要将图像转换成rgb格式 |
|  mean  |   浮点数组   | 无 | 图像前处理均值，长度为3；计算方式为: y=(x-mean)/std；若bgr2rgb=true，数组中数组顺序需为r、g、b，否则需为b、g、r |
|  std  |   浮点数组   | 无 | 图像前处理方差，长度为3；计算方式同上；若bgr2rgb=true数组中数组顺序需为r、g、b，否则需为b、g、r |
|  stage    |   列表   | ["pre"]  | 标志前处理、推理、后处理三个阶段 |
| roi | map | 无 | 预设的ROI，配置了此参数时，只会对ROI框取的区域进行处理 |
| class_names_file | 字符串 | 无 | threshold_conf为浮点数时不生效，可以不设置；当threshold_conf为map时启用，class name文件的路径 |
|  shared_object |   字符串   |  "../../../build/lib/libyolox.so"  | libyolox 动态库路径 |
|     id      |    整数       | 0  | element id |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     name    |    字符串     | "yolox" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1 | 启动线程数 |

> **注意**：
stage参数，需要设置为"pre"，"infer"，"post" 其中之一或相邻项的组合，并且按前处理-推理-后处理的顺序连接element。将三个阶段分配在三个element上的目的是充分利用各项资源，提高检测效率。

# 多媒体插件 (multimedia)

---

## decode

# sophon-stream decode element

[English](README_EN.md) | 简体中文

sophon-stream decode element是sophon-stream框架中的一个插件，用于图片、视频、RTSP/RTMP/GB28181视频流解码，以供后续的分析和处理使用。

## 1. 特性
* 支持多种输入格式，如RTSP、RTMP、GB28181、本地视频、图片文件、BASE64、CAMERA等。
* 支持RTSP/RTMP/GB28181视频流断开重连。
* 支持本地视频与图片文件配置循环。
* 支持多路视频流高性能解码，支持硬件加速。
* 提供灵活的配置选项，如解码器参数、设备类型、线程数等。
* 可以与其他sophon-stream插件无缝集成，提供解码后的数据流。

## 2. 配置参数
sophon-stream解码器插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：
```json
{
  "configure": {},
  "shared_object": "../../../build/lib/libdecode.so",
  "device_id": 0,
  "id": 0,
  "name": "decode",
  "side": "sophgo",
  "thread_number": 1
}
```

|      参数名    |    类型    | 默认值 | 说明 |
|:-------------:| :-------: | :------------------:| :------------------------:|
|  shared_object |   字符串   |  "../../../build/lib/libdecode.so" | libdecode 动态库路径 |
|  device_id  |    整数       |  0 | tpu 设备号 |
|     id      |    整数       | 0  | element id |
|     name    |    字符串     | "decode" | element 名称 |
|     side    |    字符串     | "sophgo"| 设备类型 |
| thread_number |    整数     | 1| 启动线程数 |


此外，还需要注意decode中输入数据channel的设置

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
|source_type | 字符串  | 无  | 输入数据类型，"RTSP"代表RTSP视频流，“RTMP”代表RTMP视频流，“GB28181”代表GB28181视频流，“VIDEO”代表本地视频，“IMG_DIR”代表图片文件夹， “BASE64”代表base64数据 , “CAMERA”摄像头或者其他类型的视频输入设备|
|sample_interval | 整数  | 1  |抽帧数，如设置为5，表示每5帧有1帧会被后续处理，即为ObjectMata mFilter字段为false|
|loop_num | 整数  | 1  | 循环次数，仅适用于source_type为"VIDEO"和“IMG_DIR”，值为0时无限循环|
|fps | 浮点数  | 30 | 用于控制视频流的fps，fps=-1表示不控制fps；其它情况下，source_type为"IMG_DIR"或"BASE64"时由设置的值决定，其他source_type从视频流读取fps，设置的值不生效|
|base64_port | 整数  | 12348 | base64对应http端口 |
|skip_element| list | 无 | 设置该路数据是否跳过某些element，目前只对osd和encode生效。不设置时，认为不跳过任何element|
|sample_strategy|字符串|"DROP"|在有抽帧的情况下，设置被抽掉的帧是保留还是直接丢弃。"DROP"表示丢弃，"KEEP"表示保留|
|roi|字典|无|设置ROI时，将把解码结果进行裁剪并向下传递；否则默认传递原图|


其中，channel_id为输入视频的通道编号，与[编码器](../encode/README.md)输出channel_id相对应。例如，输入channel_id为20，使用编码器保存结果为本地视频时，文件名为20.avi。

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
>5. 输入CAMERA数据流的URL须以`/dev/video`开头
>6. 不推荐同时解码本地视频和网络流

---

## encode

# sophon-stream encode element

[English](README_EN.md) | 简体中文

sophon-stream encode element是sophon-stream框架中的一个插件，用于将处理后的图像信息编码为各类视频格式。

## 目录
- [sophon-stream encode element](#sophon-stream-encode-element)
  - [目录](#目录)
  - [1. 特性](#1-特性)
  - [2. 配置参数](#2-配置参数)
  - [3. rtsp使用说明](#3-rtsp使用说明)
  - [4. rtmp使用说明](#4-rtmp使用说明)
  - [5. 输出本地视频文件](#5-输出本地视频文件)
  - [6. 输出本地图片文件夹](#6-输出本地图片文件夹)
  - [7. WebSocket使用说明](#7-websocket使用说明)
  - [8. 推流服务器](#8-推流服务器)

## 1. 特性
* 支持多种输出格式，如RTSP、RTMP、本地视频文件、本地图片文件夹等。
* 支持多种视频编码格式，如H.264、H.265等。
* 支持多种像素格式，如I420、NV12等。
* 支持多路视频流高性能编码，支持硬件加速。
* 提供灵活的配置选项，如编码器参数、视频流端口、线程数等。
* 可以与其他sophon-stream插件无缝集成，提供编码后的数据流。

## 2. 配置参数
sophon-stream编码器插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "encode_type": "RTSP",
    "rtsp_port": "8554",
    "rtmp_port": "1935",
    "wss_port": "9000",
    "enc_fmt": "h264_bm",
    "pix_fmt": "I420",
    "ws_enc_type": "IMG_ONLY",
    "wss_backend": "WEBSOCKETPP",
    "fps": 25
  },
  "shared_object": "../../../build/lib/libencode.so",
  "device_id": 0,
  "id": 0,
  "name": "encode",
  "side": "sophgo",
  "thread_number": 4
}
```

|    参数名     |  类型  |              默认值               |                          说明                           |
| :-----------: | :----: | :-------------------------------: | :-----------------------------------------------------: |
|  encode_type  | 字符串 |                无                 | 编码格式，包括 “RTSP”、“RTMP”、“VIDEO”、“IMG_DIR”、"WS" |
|   rtsp_port   | 字符串 |                无                 |                        rtsp 端口                        |
|   rtmp_port   | 字符串 |                无                 |                        rtmp 端口                        |
|   wss_port    | 字符串 |                无                 |                websocket server起始端口                 |
|    enc_fmt    | 字符串 |                无                 |           编码格式，包括 "h264_bm"，“h265_bm”           |
|    pix_fmt    | 字符串 |                无                 |              像素格式，包括 "I420"，"NV12"              |
|  ws_enc_type  | 字符串 |           "IMG_ONLY"              | 当编码格式为WS时生效，设为"IMG_ONLY"时只对图片编码，设为"SERIALIZED"对ObjectMetadata作编码 |
| wss_backend   | 字符串 |          "WEBSOCKETPP"            | websocket server类型。支持"WEBSOCKETPP"和"BOOST"      |
|      fps      |  整数  |                25                 |                  RTSP、RTMP、VIDEO帧率                  |
|      ip       | 字符串 |             "localhost"           |                       流服务器地址                      |
|      prefix   | 字符串 |                ""                 |                       推流地址名称前缀                      |
|     width     | 整数   |                -1                 |         编码器输出的宽度，默认和输入图片相同              |
|     height     | 整数   |                -1                 |         编码器输出的高度，默认和输入图片相同              |
| shared_object | 字符串 | "../../../build/lib/libencode.so" |                  libencode 动态库路径                   |
|   device_id   |  整数  |                 0                 |                       tpu 设备号                        |
|      id       |  整数  |                 0                 |                       element id                        |
|     name      | 字符串 |             "encode"              |                      element 名称                       |
|     side      | 字符串 |             "sophgo"              |                        设备类型                         |
| thread_number |  整数  |                 1                 |          启动线程数，需要保证和处理码流数一致           |

> **注意**：
1. 需要保证插件线程数和处理码流数一致
2. encode_type为RTSP时，需保证rtsp_port不为空，encode_type为RTMP时，需保证rtmp_port不为空，encode_type为WS时，需保证wss_port不为空。
3. encode_type为VIDEO和IMG_DIR时，文件保存路径为`./results`

## 3. rtsp使用说明
需要本地启动推流服务器，具体用法见[6. 推流服务器](#8-推流服务器)
在`encode.json`中做出以下设置
```json
"encode_type": "RTSP",
"rtsp_port": "8554"
```

输出视频流URL的格式为：`rtsp://localhost:{rtsp_port}/live/{graph_id}_{channel_id}`

假设rtsp_port为8554，channel_id为0, graph_id为0, 此时URL为`rtsp://localhost:8554/live/0_0`

## 4. rtmp使用说明
需要本地启动推流服务器，具体用法见[8. 推流服务器](#8-推流服务器)

在`encode.json`中做出以下设置
```json
"encode_type": "RTMP",
"rtmp_port": "1935"
```

输出视频流URL格式为：`rtmp://localhost:{rtmp_port}/live/{channel_id}`

假设rtmp_port 为1935，channel_id为0, 此时URL为`rtmp://localhost:1935/live/0`

## 5. 输出本地视频文件
在`encode.json`中做出以下设置
```json
"encode_type": "VIDEO",
```

输出视频文件名为：`{channel_id}.avi`

假设channel_id为0, 此时文件名为`0.avi`

## 6. 输出本地图片文件夹
在`encode.json`中做出以下设置
```json
"encode_type": "IMG_DIR",
```

输出图片文件名为：`./results/{channel_id}/{mFrameId}.jpg`

假设channel_id为0, mFrameId为0，此时文件名为`./results/0/0.jpg`

## 7. WebSocket使用说明

在`encode.json`中做出以下设置
```json
"encode_type": "WS",
"wss_port": "9000"
```

输出websocket URL格式为：`ws://{host_ip}:{wss_port+channel_id}`

host_ip为127.0.0.1, wss_port为9000，channel_id为2，此时URL为`ws://127.0.0.1:9002`

## 8. 推流服务器
可以使用`mediamtx`作为推流服务器，启动步骤如下

首先去[官网](https://github.com/bluenviron/mediamtx/releases)下载对应的软件包然后解压。

对于在边缘设备上部署sophon-stream的场合，我们推荐使用arm64v8版本流服务器。即：

```bash
wget https://github.com/bluenviron/mediamtx/releases/download/v1.2.0/mediamtx_v1.2.0_linux_arm64v8.tar.gz
```

解压缩后打开`mediamtx.yml`配置文件，修改readTimeout与writeTimeout这两个参数，保存后退出
```yml
# timeout of read operations.
readTimeout: 120s
# timeout of write operations.
writeTimeout: 120s
```

然后启动mediamtx
```bash
./mediamtx
```

此时服务器启动成功
```bash
INF MediaMTX v0.23.7
INF [RTSP] listener opened on :8554 (TCP), :8000 (UDP/RTP), :8001 (UDP/RTCP)
INF [RTMP] listener opened on :1935
INF [HLS] listener opened on :8888
INF [WebRTC] listener opened on :8889 (HTTP)
```

mediamtx.yml中rtsp的默认TCP端口是8554，rtmp默认端口是1935，如果修改端口号，插件配置中相应端口配置也要修改成一致。

需要注意的是，mediamtx是一个示例服务器，并不具备高度的可扩展性和功能完整性。如果您需要构建一个稳定和功能丰富的实际RTSP流媒体服务器，可以选择使用成熟的开源或商业解决方案，如Live555、GStreamer、FFmpeg等，这些工具提供了更广泛和全面的RTSP功能支持。

如果您使用该流服务器进行推流，可以参考如下命令：
```bash
ffmpeg -stream_loop -1 -an -re -i <your video> -codec copy -f rtsp -rtsp_transport tcp rtsp://localhost:8554/1
```

> **注意**:
ws依赖boost库，如果make阶段在#include<boost/version.hpp>部分报错，请使用如下命令安装该库：
```bash
sudo apt-get update 
sudo apt-get install libboost-all-dev
```

---

## osd

# sophon-stream osd element

[English](README_EN.md) | 简体中文

sophon-stream osd element是sophon-stream框架中的一个插件，负责算法结果的可视化，支持目标检测、目标跟踪、实例分割算法结果可视化

## 目录
- [sophon-stream osd element](#sophon-stream-osd-element)
  - [目录](#目录)
  - [1. 特点](#1-特点)
  - [2. 配置参数](#2-配置参数)

## 1. 特点
* 支持目标检测、目标跟踪、实例分割算法结果的可视化
* DET模式下，当检测框带有分割掩码（mSegmentedObjectMetadatas）时，自动叠加绘制半透明掩码
* 分割掩码渲染需要使用 OPENCV draw_utils（BMCV模式不支持掩码渲染）

![track.jpg](pics/track.jpg)

## 2. 配置参数
sophon-stream osd插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "osd_type": "TRACK",
    "class_names_file": "../data/coco.names",
    "draw_utils": "OPENCV",
    "draw_interval": false,
    "put_text": false
  },
  "shared_object": "../../../build/lib/libosd.so",
  "device_id": 0,
  "id": 0,
  "name": "osd",
  "side": "sophgo",
  "thread_number": 1
}
```

|      参数名      |  类型  |              默认值               |                 说明                  |
| :--------------: | :----: | :-------------------------------: | :-----------------------------------: |
|     osd_type     | 字符串 |              "TRACK"              | 画图类型，包括 "DET"、"TRACK"、"POSE"、"ALGORITHM"、"TEXT" ，其中ALGORITHM代表使用draw_func_name所对应的osd函数，TEXT代表在原图任意位置使用硬件绘制文字。DET模式同时支持实例分割掩码渲染，当检测数据中包含分割掩码时自动叠加绘制|
| class_names_file | 字符串 |                无                 |         class name文件的路径          |
| recognice_names_file | 字符串 |                无             |         如果有识别子任务的话，表示识别类别名字文件的路径          |
|    draw_utils    | 字符串 |             "OPENCV"              |    画图工具，包括 "OPENCV"，"BMCV"    |
|  draw_interval   | 布尔值 |               false               |          是否画出未采样的帧           |
|     put_text     | 布尔值 |               false               |             是否输出文本              |
|    draw_func_name    | 字符串 |             "default"              |    对应不同ALGORITHM中的osd方式    |
|  heatmap_loss  |   字符串   | "MSELoss" | 姿态识别训练所使用的损失函数，暂只支持MSELoss |
|    tops     |  整数数组  |                 无                 |              在TEXT模式下，texts中每个字符串距离图片顶部的垂直距离               |
|    lefts     |  整数数组  |                 无                 |              在TEXT模式下，texts中每个字符串距离图片左侧的水平距离               |
|    texts     |  字符串数组  |                 无                 |              在TEXT模式下，要显示的文本内容组成的数组               |
|    font_library     |  字符串  |                 无                 |              在TEXT模式下使用的字体库文件的路径               |
|    r    |  整数  |                 0                 |              TEXT模式中文字颜色的r通道值               |
|    g    |  整数  |                 0                 |              TEXT模式中文字颜色的g通道值               |
|    b     |  整数  |                 0                 |              TEXT模式中文字颜色的b通道值               |
|  shared_object   | 字符串 | "../../../build/lib/libosd.so" |         libosd 动态库路径          |
|    device_id     |  整数  |                 0                 |              tpu 设备号               |
|        id        |  整数  |                 0                 |              element id               |
|       name       | 字符串 |               "osd"               |             element 名称              |
|       side       | 字符串 |             "sophgo"              |               设备类型                |
|  thread_number   |  整数  |                 4                 | 启动线程数，需要保证和处理码流数一致  |

> **注意**：
1. osd_type为"DET"时，需提供class_names_file文件地址
2. 实例分割掩码渲染仅支持OPENCV draw_utils，使用BMCV模式时不会绘制掩码
3. 掩码通过alpha混合叠加在原始图像上（权重：原图0.6，掩码0.4），不同类别使用不同颜色区分


# 工具插件 (tools)

---

## blank




```json
{
  "configure": {},
  "shared_object": "../../build/lib/libblank.so",
  "name": "blank",
  "side": "sophgo",
  "thread_number": 1
}
```
---

## blend

# sophon-stream blend element


sophon-stream blend element是sophon-stream框架中的一个插件，是一个用于图像拼接的插件。

## 1. 特性
* 图片融合模块（Image Blend）用于加速影像拼接的相关计算。对于有着不同图片输入数量的图片拼接任务，目前支持两路拼接。

## 2. 配置参数
sophon-stream dwa插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "wgt1": "../dwa_blend_encode/data/wgt/c01_alpha_444p_m2__0_2240x128.bin",
    "wgt2": "../dwa_blend_encode/data/wgt/c01_beta_444p_m2__0_2240x128.bin",
    "ovlp_lx": 2112,
    "ovlp_rx": 2239,
    "src_h":2240,
    "bd_lx0": 0,
    "bd_rx0": 0,
    "bd_lx1": 0,
    "bd_rx1": 0
  },
  "shared_object": "../../build/lib/libblend.so",
  "name": "blend",
  "side": "sophgo",
  "thread_number": 1
}
```

| 参数名        | 类型   | 默认值                                                           | 说明                            |
| ------------- | ------ | ---------------------------------------------------------------- | ------------------------------- |
| wgt1          | string | 无 | 左路的权重文件 |
| wgt2          | string | 无  | 右路的权重文件 |
| src_h          | int | 无  | 右路的权重文件 |
| ovlp_lx       | int    | 无                                                            | 重叠区域左边界点x坐标           |
| ovlp_rx       | int    | 无                                                            | 重叠区域右边界点x坐标           |
| bd_lx0        | int    | 无                                                                 | 左图左侧黑边宽度                |
| bd_rx0        | int    | 无                                                                 | 左图右侧黑边宽度                |
| bd_lx1        | int    | 无                                                                 | 右图左侧黑边宽度                |
| bd_rx1        | int    | 无                                                                 | 右图右侧黑边宽度                |
| shared_object | string | "../../../build/lib/libblend.so"                                   | libdwa动态库路径                |
| name          | string | "blend"                                                    | element名称                     |
| side          | string | "sophgo"                                                         | 设备类型                        |
| thread_number | int    | 1                                                                | 启动线程数                      |



---

## converger

# sophon-stream converger element

[English](README_EN.md) | 简体中文

sophon-stream converger element是sophon-stream框架中的一个插件，是一个专用作数据汇聚功能的工具。

## 1. 特性
* 必须与distributor element配合使用
* 保证输出的ObjectMetadata具有正确的时间顺序
* 支持多线程

## 2. 配置参数
sophon-stream converger插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "default_port": 0
    },
    "shared_object": "../../../build/lib/libconverger.so",
    "name": "converger",
    "side": "sophgo",
    "thread_number": 1
}
```

| 参数名        | 类型   | 默认值                               | 说明                            |
| ------------- | ------ | ------------------------------------ | ------------------------------- |
| default_port  | int    | 无                                   | 从数据分发element接收数据的端口 |
| shared_object | string | "../../../build/lib/libconverger.so" | libconverger动态库路径          |
| name          | string | "converger"                          | element名称                     |
| side          | string | "sophgo"                             | 设备类型                        |
| thread_number | int    | 1                                    | 启动线程数                      |

> **注意**
1. converger element从`default_port`接收到ObjectMetadata之后，会等待其所有的分支都更新完成，才会向后续element发送。
2. 发送前，将所有数据依序保存；发送时，将所有已经完成更新的数据依序发送。
3. converger element必须搭配distributor element使用。

---

## distributor

# sophon-stream distributor element

[English](README_EN.md) | 简体中文

sophon-stream distributor element是sophon-stream框架中的一个插件，是一个专用作数据分发功能的工具。

## 1. 特性
* 必须与converger element配合使用
* 支持按类别分发
* 支持按时间间隔分发
* 支持按帧间隔分发
* 支持多线程

## 2. 配置参数
sophon-stream distributor插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "default_port": 0,
        "rules" : [
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
            },
            {
                "frame_interval": 10,
                "routes": [
                    {
                        "classes":["cat"],
                        "port": 3
                    },
                    {
                        "classes":[],
                        "port" : 4
                    }
                ]
            }
        ],
        "class_names_file" : "../data/coco.names"
      },
      "shared_object": "../../../build/lib/libdistributor.so",
      "name": "distributor",
      "side": "sophgo",
      "thread_number": 1
}
```

| 参数名           | 类型   | 默认值                                 | 说明                       |
| ---------------- | ------ | -------------------------------------- | -------------------------- |
| default_port     | int    | 无                                     | 发往数据汇聚element的端口  |
| rules            | vector | []                                     | 当前element的所有分发逻辑  |
| time_interval    | float  | 无                                     | 分发的时间间隔，单位：秒   |
| frame_interval   | int    | 无                                     | 分发的帧间隔               |
| routes           | vector | []                                     | 当前interval下的分发路径   |
| classes          | vector | []                                     | 一组类别                   |
| port             | int    | 1                                      | 当前classes对应的分发端口  |
| class_names_file | string | ""                                     | 存放所有类别名称的文件目录 |
| shared_object    | string | "../../../build/lib/libdistributor.so" | libdistributor动态库路径   |
| name             | string | "distributor"                          | element名称                |
| side             | string | "sophgo"                               | 设备类型                   |
| thread_number    | int    | 1                                      | 启动线程数                 |

> **注意**：
1. 由于分发涉及crop功能，`time_interval`或`frame_interval`参数一般不建议设置太小，否则可能造成阻塞。
2. 向`default_port`分发时，不涉及`interval`参数。该端口连接到converger element，起到将分支数据汇聚起来的作用。
3. `classes`可以配置一组类别，此种情况下，该参数中的所有类别都发送往对应的端口。例如对所有动物，如`cat`和`dog`，统一做颜色分类。
4. 当`classes`项不为空时，默认对每个类别做crop后分发。若为空，则认为分发当前大图。
5. 分发规则视业务需求而定，可以单独配置时间间隔、也可以单独配置帧间隔，亦可二者结合，形成复杂的分发规则。
6. 设计上，当用户不填写`time_interval`或`frame_interval`参数时，会视为对每一帧都按照`routes`进行分发，即相当于`frame_interval == 1`的情况。但需要注意，同【注意事项1】，如此设置可能会造成阻塞。
7. distributor element必须搭配converger element使用。

---

## dpu

# sophon-stream dpu element


sophon-stream dpu element是sophon-stream框架中的一个插件，是一个用于深度估计的插件。

## 1. 特性
* 该API 使用 DPU 硬件资源, 实现半全局块匹配算法 SGBM(Semi-Global Block Maching) 跟 快速全局平滑算法FGS(Fast Global Smoothing)。

## 2. 配置参数
sophon-stream dpu插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "dpu_type": "DPU_SGBM",
    "dpu_mode": "DPU_SGBM_MUX0"
  },
  "shared_object": "../../build/lib/libdpu.so",
  "name": "dpu",
  "side": "sophgo",
  "thread_number": 2
}

```

| 参数名        | 类型   | 默认值                         | 说明                                 |
| ------------- | ------ | ------------------------------ | ------------------------------------ |
| dpu_type      | string | DPU_SGBM                       | 选择DPU_SGBM还是DPU_ONLINE(SGBM+FGS) |
| dpu_mode      | string | DPU_SGBM_MUX0                  | 选择是dpu_mode            |
| shared_object | string | "../../../build/lib/libdpu.so" | libdpu动态库路径                     |
| name          | string | "distributor"                  | element名称                          |
| side          | string | "sophgo"                       | 设备类型                             |
| thread_number | int    | 1                              | 启动线程数                           |


*参数说明：
每种dpu_type对应一种dpu_mode，dpu_mode的取值范围如下：*

- DPU_ONLINE
    * - DPU_ONLINE_MUX0
      - 该模式下，使用FGS处理左图和右图，输出一张8bit视差图（也可用于图像的降噪，类似于引导滤波）。
    * - DPU_ONLINE_MUX1
      - 该模式下，使用SGBM、FGS处理左图和右图，输出一张16bit深度图。
    * - DPU_ONLINE_MUX2
      - 该模式下，单独使用SGBM处理左图和右图，输出一张16bit深度图。

- DPU_SGBM
    * - DPU_SGBM_MUX0
      - 使用SGBM处理左图和右图，输出一张没有经过后处理的8bit视差图。
    * - DPU_SGBM_MUX1
      - 使用SGBM处理左图和右图，输出一张经过后处理的16bit视差图。
    * - DPU_SGBM_MUX2
      - 使用SGBM处理左图和右图，输出一张经过后处理的8bit视差图。
- DPU_FGS
    * - DPU_FGS_MUX0
      - 使用FGS处理左图和右图，输出一张8bit视差图（也可用于图像的降噪，类似于引导滤波）。
    * - DPU_FGS_MUX1
      - 使用FGS处理左图和右图，输出一张16bit深度图。
---

## dwa

# sophon-stream dwa element


sophon-stream dwa element是sophon-stream框架中的一个插件，是一个用于鱼眼展开和镜头畸变矫正的插件。

## 1. 特性
* 该插件主要包含dwa_fisheye和dwa_gdc功能。
* （1）畸变仿射(DWA)模块的鱼眼畸变校正功能，通过配置校正参数获取适当的校正模型来消除鱼眼镜头造成的图像畸变，从而使弯曲的图像呈现出人眼能够感受到的更真实的形式。
* （2）去畸变仿射(DWA)模块的几何畸变校正功能，通过校正镜头引起的图像畸变（针对桶形畸变 (Barrel Distortion) 及枕形畸变 (Pincushion Distortion) ），使图像中的直线变得更加准确和几何正确，提高图像的质量和可视化效果。其中，提供两种畸变校正的方式供用户选择，分别为：1. 用户根据图像畸变的类型及校正强度输入配置参数列表对图像进行校正；2. 用户使用 Grid_Info(输入输出图像坐标映射关系描述)文件校正图像，以获得更好的图像校正效果。

## 2. 配置参数
sophon-stream dwa插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
      "is_gray":true,
      "is_resize":true,
      "dst_h":1080,
      "dst_w":1920,
      "resize_h":1920,
      "resize_w":1920,
      "dwa_mode":"DWA_GDC_MODE",
      "use_grid": true,
      "grid_name": "../dwa_dpu_encode/data/gridinfo/rrr.dat",
    },
    "shared_object": "../../build/lib/libdwa.so",
    "name": "dwa",
    "side": "sophgo",
    "thread_number": 1
  }
```

| 参数名        | 类型   | 默认值                                    | 说明                                                             |
| ------------- | ------ | ----------------------------------------- | ---------------------------------------------------------------- |
| is_gray       | bool   | false                                      | 选择是否转换为灰度图，可供下一个插件的特殊格式需要               |
| is_resize     | bool   | false                                      | 选择是否缩放图像大小                                             |
| dst_h         | int    | 无                                      | 输出图像的高                                                 |
| dst_w         | int    | 无                                      | 输出图像的宽
| resize_h         | int    | 无                                      | dwa输入图像的高                                                 |
| resize_w         | int    | 无                                      | dwa输入图像的宽                                                 |
| dwa_mode      | string | 无                              | 选择使用鱼眼展开(DWA_FISHEYE_MODE)还是镜头畸变矫正(DWA_GDC_MODE) |
| use_grid      | bool   | 无                                      | 选择是否使用gridinfo进行畸变矫正                                 |
| grid_name     | string | 无 | 选择使用gridinfo的路径                                           |
| shared_object | string | "../../../build/lib/libdwa.so"            | libdwa动态库路径                                                 |
| name          | string | "dwa"                             | element名称                                                      |
| side          | string | "sophgo"                                  | 设备类型                                                         |
| thread_number | int    | 1                                         | 启动线程数                                                       |



---

## faiss

# sophon-stream faiss element

[English](README_EN.md) | 简体中文

sophon-stream faiss element是sophon-stream框架中的一个插件，是一个专用作计算查询向量与数据库向量的内积距离的工具。

## 1. 特性
* 该接口用于 Faiss::IndexFlatIP.search(), 在 BM1684X 上实现。考虑 BM1684X 上 TPU 的连续内存, 针对 100W 底库, 可以在单处理器上一次查询最多约 512 个 256 维的输入。

## 2. 配置参数
sophon-stream faiss插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "db_path":"../data/face_data/faiss_db_data.txt",
        "label_path":"../data/face_data/faiss_index_label.name"
    },
    "shared_object": "../../../build/lib/libfaiss.so",
    "name": "faiss",
    "side": "sophgo",
    "thread_number": 1
}
```

| 参数名        | 类型   | 默认值                                     | 说明               |
| ------------- | ------ | ------------------------------------------ | ------------------ |
| shared_object | string | "../../../build/lib/libfaiss.so"           | libfaiss动态库路径 |
| name          | string | "faiss"                                    | element名称        |
| side          | string | "sophgo"                                   | 设备类型           |
| db_path       | int    | "../data/face_data/faiss_db_data.txt"      | 数据库地址         |
| label_path    | string | "../data/face_data/faiss_index_label.name" | 数据库人脸标签     |


---

## filter

# sophon-stream http_push element

[English](README_EN.md) | 简体中文

sophon-stream filter element是sophon-stream框架中的一个插件，是一个专用作数据筛选得工具。

## 1. 配置参数
sophon-stream http_push插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "rules": [
            {
                "channel_id": 0,
                "filters": [
                    {
                        "alert_first_frames": 0,
                        "alert_frame_skip_nums": 6,
                        "areas": [
                            [
                                {
                                    "top": 0,
                                    "left": 0
                                },
                                {
                                    "top": 0,
                                    "left": 100000
                                },
                                {
                                    "top": 1000000,
                                    "left": 1000000
                                },
                                {
                                    "top": 100000,
                                    "left": 0
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
                        "type": 0,
                        "direction": [1,0],
                        "trajectory_interval": 5
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

| 参数名        | 类型   | 默认值                               | 说明                            |
| ------------- | ------ | ------------------------------------ | ------------------------------- |
| channel_id            | int | 0                           | 顺序和id与demo的json一致          |
| alert_first_frames            | int | 0                       | 每一路追踪到第几帧开始上报               |
| alert_frame_skip_nums           | int | 1                    | 从第一次上报开始每几帧上报一次          |
| top           | int | 1                    | 多边形顶点的x坐标,需要满足循序，个数为0代表不检测    |
| left           | int | 1                    | 多边形顶点的y坐标,需要满足循序，个数为0代表不检测    |
| classes           | list[int] | []                    | 筛选的类别，为空将会全部筛掉        |
| time_start           | string | 无                    | 开始时间，格式hh mm ss，为空将会全部筛掉  |
| time_end           | int | 无                   | 结束时间，格式hh mm ss,为空将会全部筛掉    |
| type           | int | 0                    | 筛选类型，recognize:0  track:1 classes:other        |
| shared_object | string | "../../../build/lib/libfilter.so" | libfilter动态库路径          |
| name          | string | "filter"                          | element名称                     |
| side          | string | "sophgo"                             | 设备类型                        |
| thread_number | int    | 1                                    | 启动线程数                      |
| direction     | list[int]  | 无                              | 预设方向[x,y]，如不设置则不限方向筛选，如设置则只筛选轨迹方向与预设方向夹角<=90°的框  |
| trajectory_interval | int  | 5                              | 间隔几帧计算一次目标运动方向                     |
---

## http_push

# sophon-stream http_push element

[English](README_EN.md) | 简体中文

sophon-stream http_push element是sophon-stream框架中的一个插件，是一个专用作ObjectMetadata序列化、base64编码和发送的工具，支持http和https协议。

## 1. 配置参数
sophon-stream http_push插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
    "configure": {
        "ip": "0.0.0.0",
        "port" : 8000,
        "path": "/stream/test"
    },
    "shared_object": "../../../build/lib/libhttp_push.so",
    "name": "http_push",
    "side": "sophgo",
    "thread_number": 1
}
```

| 参数名        | 类型   | 默认值                               | 说明                            |
| ------------- | ------ | ------------------------------------ | ------------------------------- |
| scheme        | string | "http"                            | 传输协议，用https时需填写https     |
| ip            | string | "0.0.0.0"                            | httplib::Client的ip            |
| port            | int | 8000                            | httplib::Client的端口            |
| cert            | string |                             | 服务器的证书文件路径，发送https请求时使用           |
| key            | string |                             | 服务器的密钥文件路径，发送https请求时使用           |
| cacert            | string |                             | 验证服务器证书的ca证书路径，发送https请求时使用           |
| veriry            | bool |                             | 是否验证证书，是填写true，否填写false           |
| path            | string | "/stream/test"                     | http请求的path            |
| shared_object | string | "../../../build/lib/libhttp_push.so" | libhttp_push动态库路径          |
| name          | string | "http_push"                          | element名称                     |
| side          | string | "sophgo"                             | 设备类型                        |
| thread_number | int    | 1                                    | 启动线程数                      |

> **注意**
1. http_push element 使用时需要保证启动线程数与输入码流路数一致

---

## ive

# sophon-stream ive element


sophon-stream ive element是sophon-stream框架中的一个插件，是一个用于深度估计后处理染色的插件。

## 1. 特性
* 该 API 使用ive硬件资源, 创建 Map（映射赋值）任务，对源图像中的每个像素，查找 Map 查找表中的值，赋予目标图像相应像素查找表中的值， 支持 U8->U8, U8->U16, U8->S16 三种模式的映射。

## 2. 配置参数
sophon-stream ive插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "ive_mapy": "../dwa_dpu_encode/data/maps/mapY.txt",
    "ive_mapu": "../dwa_dpu_encode/data/maps/mapU.txt",
    "ive_mapv": "../dwa_dpu_encode/data/maps/mapV.txt",
    "is_ive":false
  },
  "shared_object": "../../build/lib/libdpu.so",
  "name": "dpu",
  "side": "sophgo",
  "thread_number": 1
}
```

| 参数名        | 类型   | 默认值                                 | 说明                                 |
| ------------- | ------ | -------------------------------------- | ------------------------------------ |
| ive_mapy      | string | "../dwa_dpu_encode/data/maps/mapY.txt" | 给DPU结果进行染色的Y通道map文件      |
| ive_mapu      | string | "../dwa_dpu_encode/data/maps/mapU.txt" | 给DPU结果进行染色的U通道map文件      |
| ive_mapv      | string | "../dwa_dpu_encode/data/maps/mapV.txt" | 给DPU结果进行染色的V通道map文件      |
| is_ive        | bool   | true                                   | 选择是否对DPU结果进行染色            |
| shared_object | string | "../../../build/lib/libive.so"         | libive动态库路径                     |
| name          | string | "ive"                                  | element名称                          |
| side          | string | "sophgo"                               | 设备类型                             |
| thread_number | int    | 1                                      | 启动线程数                           |



---

## qt_display

# sophon-stream qt_display element

[English](README_EN.md) | 简体中文

sophon-stream qt_display element是sophon-stream框架中的一个插件，是一个用于qt显示的工具。

## 1. 配置参数
sophon-stream qt_display插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
      "width": 1920,
      "height": 1080,
      "rows": 2,
      "cols": 3
  },
  "shared_object": "../../build/lib/libqt_display.so",
  "name": "qt_display",
  "side": "sophgo",
  "thread_number": 4
}
```

| 参数名        | 类型   | 默认值                               | 说明                            |
| ------------- | ------ | ------------------------------------ | ------------------------------- |
| width         | int    | 1920                                  | 显示屏幕的宽            |
| height        | int    | 1080                                 | 显示屏幕的高            |
| rows          | int    | 2                                    | 每行显示路数的个数            |
| cols          | int    | 3                                    | 每列显示路数的个数            |
| shared_object | string | "../../../build/lib/libqt_display.so" | libqt_display动态库路径          |
| name          | string | "qt_display"                          | element名称                     |
| side          | string | "sophgo"                             | 设备类型                        |
| thread_number | int    | 4                                    | 启动线程数                      |

> **注意**
1. libqt_display element 使用时qt配置的显示路数(rows*cols)应该大于等于总路数
2. thread_number 应该与总路数一致
---

## resize

# sophon-stream resize element


sophon-stream resize element是sophon-stream框架中的一个插件，是一个用于尺寸变换的插件。

## 1. 特性
目前插件支持从原图裁剪指定区域的图像，并缩放到指定大小。

## 2. 配置参数
sophon-stream resize插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "dst_h": 512,
    "dst_w": 1024,
    "crop_w": 4096,
    "crop_h": 2048,
    "crop_top": 0,
    "crop_left": 0
  },
  "shared_object": "../../build/lib/libresize.so",
  "name": "resize",
  "side": "sophgo",
  "thread_number": 1
}

```

| 参数名        | 类型   | 默认值                         | 说明             |
| ------------- | ------ | ------------------------------| ---------------- |
| dst_h         | int    | 无                            | 输出图像的高度信息 |
| dst_w         | int    | 无                            | 输出图像的宽度信息 |
| crop_top      | int    | 无                            | 对输入图像进行裁剪操作时，从哪一行开始进行裁剪的位置信息 |
| crop_left     | int    | 无                            | 对输入图像进行裁剪操作时，从哪一列开始进行裁剪的位置信息 |
| crop_h        | int    | 无                            | 对输入图像进行裁剪操作时，裁剪出图像的高度信息 |
| crop_w        | int    | 无                            | 对输入图像进行裁剪操作时，裁剪出图像的宽度信息 |
| shared_object | string | "../../../build/lib/libresize.so" | libresize动态库路径 |
| name          | string | "resize"                       | element名称      |
| side          | string | "sophgo"                       | 设备类型         |
| thread_number | int    | 1                              | 启动线程数       |


---

## stitch

# sophon-stream stitch element


sophon-stream stitch element是sophon-stream框架中的一个插件，是一个用于拼接的插件。

## 1. 特性
暂时只能配合dpu使用。

## 2. 配置参数
sophon-stream stitch插件具有一些可配置的参数，可以根据需求进行设置。以下是一些常用的参数：

```json
{
  "configure": {
    "stitch_mode": "HORIZONTAL"
  },
  "shared_object": "../../build/lib/libstitch.so",
  "name": "stitch",
  "side": "sophgo",
  "thread_number": 1
}
```


| 参数名      | 类型   | 默认值 | 说明                                         |
| ----------- | ------ | ------ | -------------------------------------------- |
| stitch_mode | string | 无     | 设置图像的拼接模型，可选HORIZONTAL和VERTICAL |


## 3. 配置示例
## 3.1 stitch_demo
```json
{
  "channels": [
    {
      "channel_id": 2,
      "url": "../stitch/data/test_car_person_1080P.mp4",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 5,
      "fps": -1,
      "decode_id": 5000
    },
    {
      "channel_id": 3,
      "url": "../stitch/data/test_car_person_1080P.mp4",
      "source_type": "VIDEO",
      "sample_interval": 1,
      "loop_num": 5,
      "fps": -1,
      "decode_id": 5001
    }
  ],

  "engine_config_path": "../stitch/config/engine.json"
}
```

### 3.2 decode
```json
{
  "configure": {},
  "shared_object": "../../build/lib/libdecode.so",
  "name": "decode",
  "side": "sophgo",
  "thread_number": 1
}
```
### 3.3 resize
使用resize的原因是因为拼接后的图像较大，可以适当缩放一下再进行下一步。
```json
{
  "configure": {
    "dst_w":1920,
    "dst_h":1080,
    "crop_w": 3840,
    "crop_h": 1080,
    "crop_top": 0,
    "crop_left": 0
  },
  "shared_object": "../../build/lib/libresize.so",
  "name": "resize",
  "side": "sophgo",
  "thread_number": 1
}
```

### 3.4 engine
```json
[
    {
        "graph_id": 0,
        "device_id": 0,
        "graph_name": "stitch",
        "elements": [
            {
                "element_id": 5000,
                "element_config": "../stitch/config/decode.json",
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
                "element_config": "../stitch/config/decode.json",
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
                "element_id": 5002,
                "element_config": "../stitch/config/stitch.json"
            },
            {
                "element_id": 5003,
                "element_config": "../stitch/config/resize.json",
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
                "dst_element_id": 5002,
                "dst_port": 0
            },
            {
                "src_element_id": 5001,
                "src_port": 1,
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
